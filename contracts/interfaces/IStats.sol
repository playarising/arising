// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

interface IStats {
    struct CharacterStats {
        uint256 might;
        uint256 speed;
        uint256 intelect;
    }

    function setRefreshToken(address _token) external;

    function setVitalizerToken(address _token) external;

    function consume(
        bytes memory id,
        uint256 might,
        uint256 speed,
        uint256 intelect
    ) external;

    function sacrifice(
        bytes memory id,
        uint256 might,
        uint256 speed,
        uint256 intelect
    ) external;

    function refresh(bytes memory id) external;

    function refreshWithToken(bytes memory id) external;

    function consumeVitalizer(
        bytes memory id,
        uint256 might,
        uint256 speed,
        uint256 intelect
    ) external;

    function assignPoints(
        bytes memory id,
        uint256 might,
        uint256 speed,
        uint256 intelect
    ) external;

    function getBaseStats(bytes memory id)
        external
        view
        returns (CharacterStats memory);

    function getPoolStats(bytes memory id)
        external
        view
        returns (CharacterStats memory);

    function getAvailablePoints(bytes memory id)
        external
        view
        returns (uint256);

    function getNextRefreshTime(bytes memory id)
        external
        view
        returns (uint256);

    function getNextRefreshWithTokenTime(bytes memory id)
        external
        view
        returns (uint256);
}
