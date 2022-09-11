// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

interface IExperience {
    function assignExperience(bytes memory id, uint256 amount) external;

    function addAuthority(address authority) external;

    function removeAuthority(address authority) external;

    function getExperience(bytes memory id) external view returns (uint256);

    function getLevel(bytes memory id) external view returns (uint256);

    function getExperienceForNextLevel(bytes memory id)
        external
        view
        returns (uint256);
}
