// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {
    PausableUpgradeable
} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {CommitteeUpgradeable} from "./utils/CommitteeUpgradeable.sol";
import {BridgeUtils} from "./utils/BridgeUtils.sol";
import {IBridgeConfig} from "./interfaces/IBridgeConfig.sol";
import {BridgeUtilsV2} from "./utils/BridgeUtilsV2.sol";
import {IMySoBridge} from "./interfaces/IMySoBridge.sol";
import {IBridgeLimiter} from "./interfaces/IBridgeLimiter.sol";
import {IMySocialTokenAdapter} from "./interfaces/IMySocialTokenAdapter.sol";

/// @notice Fresh-deployment bridge for genesis MYSO (id 0, chain 2) and its existing Base
/// Sepolia representation. This is NOT an upgrade implementation for historical proxies.
contract MySoNativeBridge is IMySoBridge, CommitteeUpgradeable, PausableUpgradeable {
    using SafeERC20 for IERC20;

    mapping(uint64 nonce => bool processed) public isTransferProcessed;
    address public vault;
    IBridgeLimiter public limiter;
    IMySocialTokenAdapter public nativeAdapter;
    IERC20 public nativeToken;
    uint256 public constant DECIMAL_SCALE = 1e9;

    event TokensDepositedV2(
        uint8 indexed sourceChainID,
        uint64 indexed nonce,
        uint8 indexed destinationChainID,
        uint8 tokenID,
        uint64 mysoAdjustedAmount,
        address senderAddress,
        bytes recipientAddress,
        uint256 timestampSeconds
    );

    function initialize(
        address _committee,
        address _vault,
        address _limiter,
        address adapter,
        address token
    ) external initializer {
        require(
            _committee.code.length > 0 && _vault.code.length > 0 && _limiter.code.length > 0,
            "NativeBridge: invalid contracts"
        );
        require(
            adapter.code.length > 0 && token.code.length > 0,
            "NativeBridge: invalid token contracts"
        );
        __CommitteeUpgradeable_init(_committee);
        __Pausable_init();
        vault = _vault;
        limiter = IBridgeLimiter(_limiter);
        nativeAdapter = IMySocialTokenAdapter(adapter);
        nativeToken = IERC20(token);
        require(
            nativeAdapter.mySocialToken() == token && nativeAdapter.decimals() == 18
                && IERC20Metadata(token).decimals() == 18,
            "NativeBridge: adapter mismatch"
        );
        _checkRegistry();
    }

    function _checkRegistry() internal view {
        IBridgeConfig config = committee.config();
        require(config.chainID() == 12 && config.isChainSupported(2), "NativeBridge: invalid route");
        require(
            config.tokenAddressOf(0) == address(nativeAdapter) && config.tokenMySoDecimalOf(0) == 9,
            "NativeBridge: registry mismatch"
        );
    }

    function bridgeERC20(uint8 tokenID, uint256 amount, bytes memory recipient, uint8 target)
        external
        whenNotPaused
        nonReentrant
    {
        uint64 adjusted = _burn(tokenID, amount, recipient, target);
        emit TokensDeposited(12, nonces[0]++, 2, 0, adjusted, msg.sender, recipient);
    }

    function bridgeERC20V2(uint8 tokenID, uint256 amount, bytes memory recipient, uint8 target)
        external
        whenNotPaused
        nonReentrant
    {
        uint64 adjusted = _burn(tokenID, amount, recipient, target);
        emit TokensDepositedV2(
            12, nonces[0]++, 2, 0, adjusted, msg.sender, recipient, block.timestamp
        );
    }

    function _burn(uint8 tokenID, uint256 amount, bytes memory recipient, uint8 target)
        internal
        returns (uint64)
    {
        _checkRegistry();
        require(tokenID == 0 && target == 2, "NativeBridge: only MYSO route supported");
        require(
            recipient.length == 32 && abi.decode(recipient, (bytes32)) != bytes32(0),
            "NativeBridge: invalid recipient"
        );
        require(
            amount > 0 && amount % DECIMAL_SCALE == 0 && amount / DECIMAL_SCALE <= type(uint64).max,
            "NativeBridge: invalid amount"
        );
        uint256 beforeBalance = nativeToken.balanceOf(address(this));
        uint256 beforeSupply = nativeToken.totalSupply();
        // Standard ERC20 approval is to the bridge proxy, never to the metadata adapter.
        nativeToken.safeTransferFrom(msg.sender, address(this), amount);
        require(
            nativeToken.balanceOf(address(this)) == beforeBalance + amount,
            "NativeBridge: transfer mismatch"
        );
        nativeAdapter.burnFrom(address(this), amount);
        require(
            nativeToken.balanceOf(address(this)) == beforeBalance
                && nativeToken.totalSupply() + amount == beforeSupply,
            "NativeBridge: burn mismatch"
        );
        return uint64(amount / DECIMAL_SCALE);
    }

    function transferBridgedTokensWithSignatures(
        bytes[] memory signatures,
        BridgeUtils.Message memory message
    )
        external
        whenNotPaused
        nonReentrant
        verifyMessageAndSignatures(message, signatures, BridgeUtils.TOKEN_TRANSFER)
    {
        require(message.version == 1, "NativeBridge: invalid message version");
        BridgeUtils.TokenTransferPayload memory p =
            BridgeUtils.decodeTokenTransferPayload(message.payload);
        _mint(message, p.targetChain, p.tokenID, p.recipientAddress, p.amount, p.senderAddress);
    }

    function transferBridgedTokensWithSignaturesV2(
        bytes[] memory signatures,
        BridgeUtils.Message memory message
    )
        external
        whenNotPaused
        nonReentrant
        verifyMessageAndSignatures(message, signatures, BridgeUtils.TOKEN_TRANSFER)
    {
        require(message.version == 2, "NativeBridge: invalid message version");
        BridgeUtilsV2.TokenTransferPayloadV2 memory p =
            BridgeUtilsV2.decodeTokenTransferPayloadV2(message.payload);
        require(p.timestampMs <= block.timestamp * 1000, "NativeBridge: future message");
        // Keep the limiter for old messages too; do not exempt mature mint authorizations.
        _mint(message, p.targetChain, p.tokenID, p.recipientAddress, p.amount, p.senderAddress);
    }

    function _mint(
        BridgeUtils.Message memory message,
        uint8 target,
        uint8 tokenID,
        address recipient,
        uint64 amount,
        bytes memory sender
    ) internal {
        _checkRegistry();
        require(
            message.chainID == 2 && target == 12 && tokenID == 0,
            "NativeBridge: only MYSO route supported"
        );
        require(!isTransferProcessed[message.nonce], "NativeBridge: message already processed");
        require(amount > 0 && recipient != address(0), "NativeBridge: invalid claim");
        uint256 adjusted = uint256(amount) * DECIMAL_SCALE;
        require(!limiter.willAmountExceedLimit(2, 0, adjusted), "NativeBridge: limit exceeded");
        isTransferProcessed[message.nonce] = true;
        limiter.recordBridgeTransfers(2, 0, adjusted);
        uint256 beforeBalance = nativeToken.balanceOf(recipient);
        uint256 beforeSupply = nativeToken.totalSupply();
        nativeAdapter.mint(recipient, adjusted);
        require(
            nativeToken.balanceOf(recipient) == beforeBalance + adjusted
                && nativeToken.totalSupply() == beforeSupply + adjusted,
            "NativeBridge: mint mismatch"
        );
        emit TokensClaimed(2, message.nonce, 12, 0, adjusted, sender, recipient);
    }

    function executeEmergencyOpWithSignatures(
        bytes[] memory signatures,
        BridgeUtils.Message memory message
    )
        external
        nonReentrant
        verifyMessageAndSignatures(message, signatures, BridgeUtils.EMERGENCY_OP)
    {
        bool freezing = BridgeUtils.decodeEmergencyOpPayload(message.payload);
        if (freezing) _pause();
        else _unpause();
        emit EmergencyOperation(message.nonce, freezing);
    }
}
