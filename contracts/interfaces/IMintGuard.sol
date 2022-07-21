// SPDX-License-Identifier: MIT
pragma solidity 0.8.9;

interface IMintGuard {
    function addProtected(address _contract) external;

    function setMinter(address _minter) external;

    function hasMinted(address _minter) external returns (bool);
}
