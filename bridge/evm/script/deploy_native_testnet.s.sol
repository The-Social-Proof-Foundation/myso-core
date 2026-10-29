// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script, console} from "forge-std/Script.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {BridgeCommittee} from "../contracts/BridgeCommittee.sol";
import {BridgeConfig} from "../contracts/BridgeConfig.sol";
import {BridgeLimiter} from "../contracts/BridgeLimiter.sol";
import {BridgeVault} from "../contracts/BridgeVault.sol";
import {MySoNativeBridge} from "../contracts/MySoNativeBridge.sol";

/// @dev Wire committee/config and ownership atomically so initializeConfig cannot be front-run.
contract NativeBridgeDeployment {
    address public immutable committee;
    address public immutable config;
    address public immutable limiter;
    address public immutable vault;
    address public immutable bridge;

    constructor(
        address[4] memory implementations,
        address[] memory members,
        uint16[] memory stakes,
        address adapter,
        address token,
        uint64 price,
        uint64 limit
    ) {
        require(block.chainid == 84532, "Deployment: Base Sepolia only");
        require(price > 0 && limit > 0, "Deployment: invalid price or limit");
        uint256 total;
        for (uint256 i; i < stakes.length; i++) {
            require(stakes[i] > 0, "Deployment: zero stake");
            total += stakes[i];
        }
        require(total == 10000, "Deployment: committee must total 10000");
        committee = address(
            new ERC1967Proxy(
                implementations[0],
                abi.encodeCall(BridgeCommittee.initialize, (members, stakes, uint16(10000)))
            )
        );
        address[] memory tokens = new address[](1);
        tokens[0] = adapter;
        uint64[] memory prices = new uint64[](1);
        prices[0] = price;
        uint8[] memory ids = new uint8[](1);
        ids[0] = 0;
        uint8[] memory decimals = new uint8[](1);
        decimals[0] = 9;
        uint8[] memory routes = new uint8[](1);
        routes[0] = 2;
        config = address(
            new ERC1967Proxy(
                implementations[1],
                abi.encodeCall(
                    BridgeConfig.initialize,
                    (committee, uint8(12), tokens, prices, ids, decimals, routes)
                )
            )
        );
        BridgeCommittee(committee).initializeConfig(config);
        vault = address(new BridgeVault(0x4200000000000000000000000000000000000006));
        uint64[] memory limits = new uint64[](1);
        limits[0] = limit;
        limiter = address(
            new ERC1967Proxy(
                implementations[2],
                abi.encodeCall(BridgeLimiter.initialize, (committee, routes, limits))
            )
        );
        bridge = address(
            new ERC1967Proxy(
                implementations[3],
                abi.encodeCall(
                    MySoNativeBridge.initialize, (committee, vault, limiter, adapter, token)
                )
            )
        );
        BridgeVault(payable(vault)).transferOwnership(bridge);
        BridgeLimiter(limiter).transferOwnership(bridge);
    }
}

contract DeployNativeTestnet is Script {
    function run() external {
        require(block.chainid == 84532, "Deployment: Base Sepolia only");
        string memory data = vm.readFile(vm.envString("BRIDGE_PLAN_CONFIG"));
        address[] memory members = abi.decode(vm.parseJson(data, ".committeeMembers"), (address[]));
        uint256[] memory raw = abi.decode(vm.parseJson(data, ".committeeMemberStake"), (uint256[]));
        require(members.length == raw.length && members.length > 0, "Deployment: invalid committee");
        uint16[] memory stakes = new uint16[](raw.length);
        for (uint256 i; i < raw.length; i++) {
            require(raw[i] <= 10000, "Deployment: invalid stake");
            stakes[i] = uint16(raw[i]);
        }
        uint256 price = vm.parseJsonUint(data, ".tokenPrice");
        uint256 limit = vm.parseJsonUint(data, ".dailyLimitUsd8");
        require(
            price <= type(uint64).max && limit <= type(uint64).max,
            "Deployment: price or limit overflow"
        );
        vm.startBroadcast(vm.envAddress("BRIDGE_DEPLOYER"));
        address[4] memory implementations = [
            address(new BridgeCommittee()),
            address(new BridgeConfig()),
            address(new BridgeLimiter()),
            address(new MySoNativeBridge())
        ];
        NativeBridgeDeployment deployed = new NativeBridgeDeployment(
            implementations,
            members,
            stakes,
            vm.parseJsonAddress(data, ".adapter"),
            vm.parseJsonAddress(data, ".token"),
            uint64(price),
            uint64(limit)
        );
        vm.stopBroadcast();
        console.log("[Deployed] MySoBridge:", deployed.bridge());
        console.log("[Deployed] BridgeCommittee:", deployed.committee());
        console.log("[Deployed] BridgeConfig:", deployed.config());
        console.log("[Deployed] BridgeLimiter:", deployed.limiter());
        console.log("[Deployed] BridgeVault:", deployed.vault());
        console.log("[Deployed] Deployment:", address(deployed));
    }
}
