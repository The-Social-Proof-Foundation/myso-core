// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IMySocialTokenAdapter {
    function mySocialToken() external view returns (address);
    function decimals() external view returns (uint8);
    function authorizedBridges(address bridge) external view returns (bool);
    function mint(address recipient, uint256 amount) external;
    function burnFrom(address account, uint256 amount) external;
    function setAuthorizedBridge(address bridge, bool authorized) external;
}
