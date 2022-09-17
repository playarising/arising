// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title ICivilizations
 * @notice Interface for the [Civilizations](/docs/core/Civilizations.md) contract.
 */
interface ICivilizations {
    struct Upgrade {
        uint256 price;
        bool available;
    }

    struct UpgradedCharacters {
        mapping(bytes => bool) upgrade_1;
        mapping(bytes => bool) upgrade_2;
        mapping(bytes => bool) upgrade_3;
    }

    struct TokenUpgrades {
        bool upgrade_1;
        bool upgrade_2;
        bool upgrade_3;
    }

    function setInitializeUpgrade(uint256 upgrade, bool available) external;

    function setUpgradePrice(uint256 upgrade, uint256 price) external;

    function setToken(address _token) external;

    function addCivilization(address _instance) external;

    function mint(address _instance) external;

    function buyUpgrade(bytes memory id, uint256 upgrade) external;

    function withdraw() external;

    function getID(address _instance) external view returns (uint256);

    function getTokenUpgrades(bytes memory id)
        external
        view
        returns (TokenUpgrades memory);

    function getUpgradeInformation(uint256 upgrade)
        external
        view
        returns (Upgrade memory);

    function getCivilizations() external view returns (address[] memory);

    function getTokenID(address _instance, uint256 _id)
        external
        view
        returns (bytes memory);

    function isAllowed(address spender, bytes memory _id)
        external
        view
        returns (bool);

    function exists(bytes memory _id) external view returns (bool);

    function ownerOf(bytes memory _id) external view returns (address);
}
