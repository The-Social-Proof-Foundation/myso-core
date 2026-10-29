// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {MySoNativeBridge} from "../contracts/MySoNativeBridge.sol";
import {BridgeCommittee} from "../contracts/BridgeCommittee.sol";
import {BridgeConfig} from "../contracts/BridgeConfig.sol";
import {BridgeLimiter} from "../contracts/BridgeLimiter.sol";
import {BridgeUtils} from "../contracts/utils/BridgeUtils.sol";
import {IMySocialTokenAdapter} from "../contracts/interfaces/IMySocialTokenAdapter.sol";
import {NativeBridgeDeployment} from "../script/deploy_native_testnet.s.sol";

contract NativeTestToken is ERC20 {
    address public presaleContract;
    constructor() ERC20("MySocial", "MySo") {}

    function setPresaleContract(address adapter) external {
        presaleContract = adapter;
    }

    function mint(address to, uint256 amount) external {
        require(msg.sender == presaleContract, "token: unauthorized");
        _mint(to, amount);
    }

    function burnFrom(address from, uint256 amount) external {
        require(msg.sender == presaleContract, "token: unauthorized");
        _burn(from, amount);
    }
}

contract NativeTestAdapter is IMySocialTokenAdapter {
    address public immutable mySocialToken;
    mapping(address => bool) public authorizedBridges;

    constructor(address token) {
        mySocialToken = token;
    }

    function decimals() external pure returns (uint8) {
        return 18;
    }

    function setAuthorizedBridge(address bridge, bool enabled) external {
        authorizedBridges[bridge] = enabled;
    }

    function mint(address recipient, uint256 amount) external {
        require(authorizedBridges[msg.sender], "adapter: unauthorized");
        NativeTestToken(mySocialToken).mint(recipient, amount);
    }

    function burnFrom(address account, uint256 amount) external {
        require(authorizedBridges[msg.sender], "adapter: unauthorized");
        NativeTestToken(mySocialToken).burnFrom(account, amount);
    }
}

contract MySoNativeBridgeTest is Test {
    MySoNativeBridge bridge;
    BridgeCommittee committee;
    BridgeConfig config;
    BridgeLimiter limiter;
    NativeTestToken token;
    IMySocialTokenAdapter adapter;
    address user = address(0xBEEF);

    function makeTokenAdapter() internal virtual {
        token = new NativeTestToken();
        adapter = new NativeTestAdapter(address(token));
        token.setPresaleContract(address(adapter));
    }

    function authorize(bool enabled) internal virtual {
        adapter.setAuthorizedBridge(address(bridge), enabled);
    }

    function setUp() public virtual {
        vm.chainId(84532);
        vm.warp(1_800_000_000);
        makeTokenAdapter();
        address[] memory members = new address[](3);
        uint16[] memory stakes = new uint16[](3);
        for (uint256 i; i < 3; i++) {
            members[i] = vm.addr(i + 1);
            stakes[i] = i == 1 ? 3334 : 3333;
        }
        address[4] memory impls = [
            address(new BridgeCommittee()),
            address(new BridgeConfig()),
            address(new BridgeLimiter()),
            address(new MySoNativeBridge())
        ];
        NativeBridgeDeployment deployed = new NativeBridgeDeployment(
            impls, members, stakes, address(adapter), address(token), 100_000_000, 10_000_000_000
        );
        bridge = MySoNativeBridge(deployed.bridge());
        committee = BridgeCommittee(deployed.committee());
        config = BridgeConfig(deployed.config());
        limiter = BridgeLimiter(deployed.limiter());
        authorize(true);
    }

    function message(uint64 nonce, uint8 version, uint64 amount)
        internal
        view
        returns (BridgeUtils.Message memory m)
    {
        bytes memory payload = abi.encodePacked(
            uint8(32), bytes32(uint256(7)), uint8(12), uint8(20), user, uint8(0), amount
        );
        if (version == 2) {
            payload = bytes.concat(payload, abi.encodePacked(uint64(block.timestamp * 1000)));
        }
        m = BridgeUtils.Message(0, version, nonce, 2, payload);
    }

    function signatures(BridgeUtils.Message memory m, uint256 count)
        internal
        pure
        returns (bytes[] memory sigs)
    {
        sigs = new bytes[](count);
        for (uint256 i; i < count; i++) {
            (uint8 v, bytes32 r, bytes32 s) = vm.sign(i + 1, BridgeUtils.computeHash(m));
            sigs[i] = abi.encodePacked(r, s, v);
        }
    }

    function claim(uint64 nonce, uint64 amount) internal {
        BridgeUtils.Message memory m = message(nonce, 1, amount);
        bridge.transferBridgedTokensWithSignatures(signatures(m, 2), m);
    }

    function testMintBurnRoundTrip() public {
        uint256 beforeSupply = token.totalSupply();
        uint256 beforeBalance = token.balanceOf(user);
        claim(0, 1_000_000_000);
        assertEq(token.balanceOf(user), beforeBalance + 1 ether);
        assertEq(token.totalSupply(), beforeSupply + 1 ether);
        vm.startPrank(user);
        token.approve(address(bridge), 1 ether);
        bridge.bridgeERC20(0, 1 ether, abi.encode(bytes32(uint256(7))), 2);
        vm.stopPrank();
        assertEq(token.balanceOf(user), beforeBalance);
        assertEq(token.totalSupply(), beforeSupply);
        assertEq(token.balanceOf(address(bridge)), 0);
        assertEq(bridge.nonces(0), 1);
    }

    function testV2MintAndBurn() public {
        BridgeUtils.Message memory m = message(3, 2, 1_000_000_000);
        uint256 beforeBalance = token.balanceOf(user);
        bridge.transferBridgedTokensWithSignaturesV2(signatures(m, 2), m);
        assertEq(token.balanceOf(user), beforeBalance + 1 ether);
        vm.startPrank(user);
        token.approve(address(bridge), 1 ether);
        bridge.bridgeERC20V2(0, 1 ether, abi.encode(bytes32(uint256(7))), 2);
        vm.stopPrank();
        assertEq(token.balanceOf(user), beforeBalance);
    }

    function testReplayCannotMintTwice() public {
        claim(4, 1);
        vm.expectRevert("NativeBridge: message already processed");
        claim(4, 1);
    }

    function testDifferentVersionCannotReplayNonce() public {
        claim(4, 1);
        BridgeUtils.Message memory m = message(4, 2, 1);
        bytes[] memory sigs = signatures(m, 2);
        vm.expectRevert("NativeBridge: message already processed");
        bridge.transferBridgedTokensWithSignaturesV2(sigs, m);
    }

    function testInsufficientVotingStakeCannotMint() public {
        BridgeUtils.Message memory m = message(0, 1, 1);
        bytes[] memory sigs = signatures(m, 1);
        vm.expectRevert("BridgeCommittee: Insufficient stake amount");
        bridge.transferBridgedTokensWithSignatures(sigs, m);
    }

    function testDuplicateSignerCannotMint() public {
        BridgeUtils.Message memory m = message(0, 1, 1);
        bytes[] memory sigs = signatures(m, 2);
        sigs[1] = sigs[0];
        vm.expectRevert("BridgeCommittee: Duplicate signature provided");
        bridge.transferBridgedTokensWithSignatures(sigs, m);
    }

    function testUnapprovedBridgeCannotMint() public {
        authorize(false);
        BridgeUtils.Message memory m = message(0, 1, 1);
        bytes[] memory sigs = signatures(m, 2);
        vm.expectRevert();
        bridge.transferBridgedTokensWithSignatures(sigs, m);
        assertFalse(bridge.isTransferProcessed(0));
    }

    function testBurnRequiresUserApprovalAndRollsBack() public {
        claim(0, 1_000_000_000);
        uint256 beforeBalance = token.balanceOf(user);
        vm.prank(user);
        vm.expectRevert();
        bridge.bridgeERC20(0, 1 ether, abi.encode(bytes32(uint256(7))), 2);
        assertEq(token.balanceOf(user), beforeBalance);
        assertEq(bridge.nonces(0), 0);
    }

    function testDustAndZeroAmountsCannotBurn() public {
        vm.expectRevert("NativeBridge: invalid amount");
        bridge.bridgeERC20(0, 1, abi.encode(bytes32(uint256(7))), 2);
        vm.expectRevert("NativeBridge: invalid amount");
        bridge.bridgeERC20(0, 0, abi.encode(bytes32(uint256(7))), 2);
    }

    function testWrongRouteCannotBurn() public {
        vm.expectRevert("NativeBridge: only MYSO route supported");
        bridge.bridgeERC20(0, 1 ether, abi.encode(bytes32(uint256(7))), 1);
        vm.expectRevert("NativeBridge: only MYSO route supported");
        bridge.bridgeERC20(3, 1 ether, abi.encode(bytes32(uint256(7))), 2);
    }

    function testWrongSourceCannotMint() public {
        BridgeUtils.Message memory m = message(0, 1, 1);
        m.chainID = 1;
        bytes[] memory sigs = signatures(m, 2);
        vm.expectRevert("NativeBridge: only MYSO route supported");
        bridge.transferBridgedTokensWithSignatures(sigs, m);
    }

    function testDailyLimitAppliesToMint() public {
        BridgeUtils.Message memory m = message(0, 1, 101_000_000_000);
        bytes[] memory sigs = signatures(m, 2);
        vm.expectRevert("NativeBridge: limit exceeded");
        bridge.transferBridgedTokensWithSignatures(sigs, m);
    }

    function testEmergencyPauseAndUnpause() public {
        BridgeUtils.Message memory emergency = BridgeUtils.Message(2, 1, 0, 12, hex"00");
        bridge.executeEmergencyOpWithSignatures(signatures(emergency, 2), emergency);
        assertTrue(bridge.paused());
        BridgeUtils.Message memory transfer = message(0, 1, 1);
        bytes[] memory sigs = signatures(transfer, 2);
        vm.expectRevert();
        bridge.transferBridgedTokensWithSignatures(sigs, transfer);
        vm.expectRevert();
        bridge.bridgeERC20(0, 1 ether, abi.encode(bytes32(uint256(7))), 2);
        emergency = BridgeUtils.Message(2, 1, 1, 12, hex"01");
        bridge.executeEmergencyOpWithSignatures(signatures(emergency, 2), emergency);
        assertFalse(bridge.paused());
        claim(0, 1);
    }

    function testV2FutureTimestampCannotMint() public {
        BridgeUtils.Message memory m = message(0, 1, 1);
        m.version = 2;
        m.payload = bytes.concat(m.payload, abi.encodePacked(uint64((block.timestamp + 60) * 1000)));
        bytes[] memory sigs = signatures(m, 2);
        vm.expectRevert("NativeBridge: future message");
        bridge.transferBridgedTokensWithSignaturesV2(sigs, m);
    }

    function testUnauthorizedBurnRollsBackTokenTransfer() public {
        claim(0, 1_000_000_000);
        uint256 beforeBalance = token.balanceOf(user);
        uint256 beforeSupply = token.totalSupply();
        vm.prank(user);
        token.approve(address(bridge), 1 ether);
        authorize(false);
        vm.prank(user);
        vm.expectRevert();
        bridge.bridgeERC20(0, 1 ether, abi.encode(bytes32(uint256(7))), 2);
        assertEq(token.balanceOf(user), beforeBalance);
        assertEq(token.totalSupply(), beforeSupply);
        assertEq(token.allowance(user, address(bridge)), 1 ether);
        assertEq(bridge.nonces(0), 0);
    }

    function testNoInitializerTakeover() public {
        address currentVault = bridge.vault();
        vm.expectRevert();
        bridge.initialize(
            address(committee), currentVault, address(limiter), address(adapter), address(token)
        );
    }

    function testZeroRecipientCannotBurn() public {
        vm.expectRevert("NativeBridge: invalid recipient");
        bridge.bridgeERC20(0, 1 ether, abi.encode(bytes32(0)), 2);
    }
}
