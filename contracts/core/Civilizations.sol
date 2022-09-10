// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @dev `Civilizations` is the contract that stores all the usable civilizations.
 */
contract Civilizations is Ownable {
    // =============================================== Storage ========================================================
    /** @dev Map to store the civilizations implemented. **/
    mapping(address => uint256) _civilizations;

    /** @dev Array to store the civilizations implemented. **/
    address[] civilizations;

    // =============================================== Setters ========================================================

    /** @dev Adds a civilization to the contract.
     *  @param _instance  Address of the `BaseERC721` instance.
     */
    function addCivilization(address _instance) public onlyOwner {
        require(
            _instance != address(0),
            "Civilizations: instance address is null"
        );
        uint256 newId = civilizations.length + 1;
        _civilizations[_instance] = newId;
        civilizations.push(_instance);
    }

    // =============================================== Getters ========================================================

    /** @dev Returns the internal ID for a civilization.
     *  @param _instance  Address of the `BaseERC721` instance.
     */
    function getID(address _instance) public view returns (uint256) {
        require(
            _instance != address(0),
            "Civilizations: instance address is null."
        );
        require(
            _civilizations[_instance] != 0,
            "Civilizations: instance is not an Arising civilization."
        );
        return _civilizations[_instance];
    }

    /** @dev Returns the list of civilizations.
     *  @param _instance  Address of the `BaseERC721` instance.
     */
    function getCivilizations(address _instance)
        public
        view
        returns (address[] memory)
    {
        require(
            _instance != address(0),
            "Civilizations: instance address is null"
        );
        return civilizations;
    }

    /** @dev Returns a unique token ID from a collection.
     *  @param _instance  Address of the `BaseERC721` instance.
     *  @param _id        The ID of the token inside the collection.
     */
    function getTokenID(address _instance, uint256 _id)
        public
        view
        returns (string memory)
    {
        require(
            _instance != address(0),
            "Civilizations: instance address is null"
        );
        uint256 id = _civilizations[_instance];
        require(
            id != 0,
            "Civilizations: instance is not an Arising civilization."
        );
        return
            string(
                abi.encodePacked(
                    Strings.toString(id),
                    "-",
                    Strings.toString(_id)
                )
            );
    }
}
