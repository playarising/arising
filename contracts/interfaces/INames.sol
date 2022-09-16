// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title INames
 * @notice Interface for the {Names} contract.
 */
interface INames {
    function claimName(bytes memory id, string memory name) external;

    function replaceName(bytes memory id, string memory newName) external;

    function clearName(bytes memory id) external;

    function getTokenName(bytes memory id)
        external
        view
        returns (string memory);

    function isNameAvailable(string memory str) external view returns (bool);

    function isNameValid(string memory str) external pure returns (bool);

    function toLowerCase(string memory str)
        external
        pure
        returns (string memory);
}
