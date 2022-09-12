// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

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

    function setInitialized() external;

    function setInitializeUpgrade(uint256 upgrade, uint256 price) external;

    function setUpgradePrice(uint256 upgrade, uint256 price) external;

    function setToken(address _token) external;

    function addCivilization(address _instance) external;

    function mint(address _instance) external;

    function buyUpgrade(bytes memory id, uint256 upgrade) external;

    function getID(address _instance) external view returns (uint256);

    function getTokenUpgrades(bytes memory id)
        external
        view
        returns (
            bool,
            bool,
            bool
        );

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
}
