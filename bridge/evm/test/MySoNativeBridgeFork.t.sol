// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {MySoNativeBridgeTest, NativeTestToken} from "./MySoNativeBridge.t.sol";
import {IMySocialTokenAdapter} from "../contracts/interfaces/IMySocialTokenAdapter.sol";

/// @notice Run explicitly with --fork-url https://sepolia.base.org and a fixed --fork-block-number.
/// All owner impersonation and mint/burn operations exist only in the local fork.
contract MySoNativeBridgeForkTest is MySoNativeBridgeTest {
    function makeTokenAdapter() internal override {
        token = NativeTestToken(0x8a9e9Ad05010aD980a1d24b61bC2a099B13D42a2);
        adapter = IMySocialTokenAdapter(0x643e8c1Cf3Ef9f76D4EF2a81C461Fe39c2F2C7f8);
        require(
            address(token).code.length > 0 && address(adapter).code.length > 0,
            "Run fork suite against Base Sepolia"
        );
    }

    function authorize(bool enabled) internal override {
        (bool ok, bytes memory data) =
            address(adapter).staticcall(abi.encodeWithSignature("owner()"));
        require(ok, "Cannot read adapter owner");
        vm.prank(abi.decode(data, (address)));
        adapter.setAuthorizedBridge(address(bridge), enabled);
    }
}
