// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title IStats
 * @notice Interface for the {Stats} contract.
 */
interface IStats {
    struct BasicStats {
        uint256 might;
        uint256 speed;
        uint256 intellect;
    }

    function setRefreshToken(address _token) external;

    function setVitalizerToken(address _token) external;

    function consume(bytes memory id, BasicStats memory stats) external;

    function sacrifice(bytes memory id, BasicStats memory stats) external;

    function refresh(bytes memory id) external;

    function refreshWithToken(bytes memory id) external;

    function consumeVitalizer(bytes memory id, BasicStats memory stats)
        external;

    function assignPoints(bytes memory id, BasicStats memory stats) external;

    function getBaseStats(bytes memory id)
        external
        view
        returns (BasicStats memory);

    function getPoolStats(bytes memory id)
        external
        view
        returns (BasicStats memory);

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
