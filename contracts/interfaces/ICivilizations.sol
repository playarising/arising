// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title ICivilizations
 * @notice Interface for the [Civilizations](/docs/core/Civilizations.md) contract.
 */
interface ICivilizations {
    /**
     * @notice Internal struct to store the global state of an upgrade.
     *
     * Requirements:
     * @param price         Price to purchase the upgrade.
     * @param available     Status of the purchase mechanism for the upgrade.
     */
    struct Upgrade {
        uint256 price;
        bool available;
    }

    /**
     * @notice Internal struct to store the maps for the character upgrades.
     *
     * Requirements:
     * @param upgrade_1     Map to store the upgrade 1 purchases.
     * @param upgrade_2     Map to store the upgrade 2 purchases.
     * @param upgrade_3     Map to store the upgrade 3 purchases.
     */
    struct UpgradedCharacters {
        mapping(bytes => bool) upgrade_1;
        mapping(bytes => bool) upgrade_2;
        mapping(bytes => bool) upgrade_3;
    }

    /**
     * @notice Internal struct to return the status of the character upgrades.
     *
     * Requirements:
     * @param upgrade_1     Boolean to return if is purchased for the character.
     * @param upgrade_2     Boolean to return if is purchased for the character.
     * @param upgrade_3     Boolean to return if is purchased for the character.
     */
    struct CharacterUpgrades {
        bool upgrade_1;
        bool upgrade_2;
        bool upgrade_3;
    }

    /** @notice See [Civilizations#pause](/docs/core/Civilizations.md#pause) */
    function pause() external;

    /** @notice See [Civilizations#unpause](/docs/core/Civilizations.md#unpause) */
    function unpause() external;

    /** @notice See [Civilizations#setInitializeUpgrade](/docs/core/Civilizations.md#setInitializeUpgrade) */
    function setInitializeUpgrade(uint256 _upgrade_id, bool _available)
        external;

    /** @notice See [Civilizations#setUpgradePrice](/docs/core/Civilizations.md#setUpgradePrice) */
    function setUpgradePrice(uint256 _upgrade_id, uint256 _price) external;

    /** @notice See [Civilizations#setToken](/docs/core/Civilizations.md#setToken) */
    function setToken(address _token) external;

    /** @notice See [Civilizations#addCivilization](/docs/core/Civilizations.md#addCivilization) */
    function addCivilization(address _civilization) external;

    /** @notice See [Civilizations#mint](/docs/core/Civilizations.md#mint) */
    function mint(address _civilization) external;

    /** @notice See [Civilizations#buyUpgrade](/docs/core/Civilizations.md#buyUpgrade) */
    function buyUpgrade(bytes memory _id, uint256 _upgrade_id) external;

    /** @notice See [Civilizations#withdraw](/docs/core/Civilizations.md#withdraw) */
    function withdraw() external;

    /** @notice See [Civilizations#getID](/docs/core/Civilizations.md#getID) */
    function getID(address _civilization) external view returns (uint256);

    /** @notice See [Civilizations#getTokenUpgrades](/docs/core/Civilizations.md#getTokenUpgrades) */
    function getTokenUpgrades(bytes memory _id)
        external
        view
        returns (CharacterUpgrades memory);

    /** @notice See [Civilizations#getUpgradeInformation](/docs/core/Civilizations.md#getUpgradeInformation) */
    function getUpgradeInformation(uint256 _upgrade_id)
        external
        view
        returns (Upgrade memory);

    /** @notice See [Civilizations#getCivilizations](/docs/core/Civilizations.md#getCivilizations) */
    function getCivilizations() external view returns (address[] memory);

    /** @notice See [Civilizations#getTokenID](/docs/core/Civilizations.md#getTokenID) */
    function getTokenID(address _civilization, uint256 _token_id)
        external
        view
        returns (bytes memory);

    /** @notice See [Civilizations#isAllowed](/docs/core/Civilizations.md#isAllowed) */
    function isAllowed(address _spender, bytes memory _id)
        external
        view
        returns (bool);

    /** @notice See [Civilizations#exists](/docs/core/Civilizations.md#exists) */
    function exists(bytes memory _id) external view returns (bool);

    /** @notice See [Civilizations#ownerOf](/docs/core/Civilizations.md#ownerOf) */
    function ownerOf(bytes memory _id) external view returns (address);
}
