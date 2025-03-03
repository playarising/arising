// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

/**
 * @title INames
 * @notice Interface for the [Names](/docs/core/Names.md) contract.
 */
interface INames {
    /** @notice See [Names#pause](/docs/codex/Names.md#pause) */
    function pause() external;

    /** @notice See [Names#unpause](/docs/codex/Names.md#unpause) */
    function unpause() external;

    /** @notice See [Names#claimName](/docs/codex/Names.md#claimName) */
    function claimName(bytes memory _id, string memory _name) external;

    /** @notice See [Names#replaceName](/docs/codex/Names.md#replaceName) */
    function replaceName(bytes memory _id, string memory _new_name) external;

    /** @notice See [Names#clearName](/docs/codex/Names.md#clearName) */
    function clearName(bytes memory _id) external;

    /** @notice See [Names#getCharacterName](/docs/codex/Names.md#getCharacterName) */
    function getCharacterName(
        bytes memory _id
    ) external view returns (string memory _name);

    /** @notice See [Names#isNameAvailable](/docs/codex/Names.md#isNameAvailable) */
    function isNameAvailable(
        string memory _name
    ) external view returns (bool _available);

    /** @notice See [Names#isNameValid](/docs/codex/Names.md#isNameValid) */
    function isNameValid(
        string memory _name
    ) external pure returns (bool _available);

    /** @notice See [Names#toLowerCase](/docs/codex/Names.md#toLowerCase) */
    function toLowerCase(
        string memory _name
    ) external pure returns (string memory _lower_case);
}
