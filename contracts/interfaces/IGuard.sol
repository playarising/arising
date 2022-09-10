// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

interface IGuard {
    function addProtected(address _contract) external;

    function setMinter(address _minter) external;

    function hasMinted(address _minter) external returns (bool);
}
