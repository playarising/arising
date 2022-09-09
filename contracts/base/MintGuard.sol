// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../interfaces/IMintGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Address.sol";

/**
 * @dev `MintGuard` a guard contract to prevent minting more than five characters among a set of collections.
 */
contract MintGuard is Ownable, IMintGuard {
    using Address for address;

    // =============================================== Storage ========================================================

    /** @dev Map to track the amount of tokens minted by address. **/
    mapping(address => uint256) private _minters;

    /** @dev Map to track the protected instances. **/
    mapping(address => bool) private _protected;

    /** @dev Array to track the protected instances. **/
    address[] protected;

    // ============================================== Modifiers =======================================================

    /**
     * @dev Checks if `msg.sender` is a protected contract.
     */
    modifier onlyProtected() {
        require(
            _protected[msg.sender],
            "MintGuard: contract is not defined on the list of protected contracts"
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @dev Adds an instance to the list of protected instances.
     * @param _contract The address of the instance.
     */
    function addProtected(address _contract) public onlyOwner {
        _protected[_contract] = true;
        protected.push(_contract);
    }

    /**
     * @dev Adds a mint to the mint counter for the address.
     * @param _minter The minter address.
     */
    function setMinter(address _minter) public onlyProtected {
        _minters[_minter] += 1;
    }

    // =============================================== Getters ========================================================

    /**
     * @dev Checks if an address is not able to mint more tokens.
     * @param _minter The minter address.
     */
    function hasMinted(address _minter) public view returns (bool) {
        require(
            !Address.isContract(_minter),
            "MintGuard: cannot mint from a contract"
        );
        return _minters[_minter] >= 5;
    }

    /**
     * @dev Returns the list of protected contracts.
     */
    function getProtected() public view returns (address[] memory) {
        return protected;
    }
}
