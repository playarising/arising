// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../interfaces/IMintGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Address.sol";

contract MintGuard is Ownable, IMintGuard {
    using Address for address;

    // Address minted counts
    mapping(address => uint256) private _minters;

    // Protected contracts
    mapping(address => bool) private _protected;
    address[] protected;

    /**
     * @dev Throws if called from an address or a contract not specified by the owner.
     */
    modifier onlyProtected() {
        require(
            _protected[msg.sender],
            "MintGuard: contract is not defined on the list of protected contracts"
        );
        _;
    }

    /**
     * @dev Returns the list of protected contracts
     */
    function getProtected() public view returns (address[] memory) {
        return protected;
    }

    /**
     * @dev Adds a contract address to the list of modifiers for the `_minters` list
     */
    function addProtected(address _contract) public onlyOwner {
        _protected[_contract] = true;
        protected.push(_contract);
    }

    /**
     * @dev Adds a mint count to an address an address as minter to prevent multiple mints from the same address
     */
    function setMinter(address _minter) public onlyProtected {
        _minters[_minter] += 1;
    }

    /**
     * @dev Checks if an address has already minted.
     * This call will also add an extra layer to prevent contracts to mint (is not entirely safe, but prevents bots to spam-mint).
     */
    function hasMinted(address _minter) public view returns (bool) {
        require(
            !Address.isContract(_minter),
            "MintGuard: cannot mint from a contract"
        );
        return _minters[_minter] >= 5;
    }
}
