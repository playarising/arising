// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title Helper
 * @notice Contract to do bulk functions and fetch from multiple contracts
 */

contract Helper {
    // =============================================== Structs ========================================================

    // =============================================== Storage ========================================================

    /** @notice Address of the [Civilizations](/docs/core/Civilizations.md) instance. */
    address public civilizations;
    /** @notice Address of the [Craft](/docs/core/Craft.md) instance. */
    address public craft;
    /** @notice Address of the [Equipment](/docs/core/Equipment.md) instance. */
    address public equipment;
    /** @notice Address of the [Experience](/docs/core/Experience.md) instance. */
    address public experience;
    /** @notice Address of the [Forge](/docs/core/Forge.md) instance. */
    address public forge;
    /** @notice Address of the [Names](/docs/core/Names.md) instance. */
    address public names;
    /** @notice Address of the [Quests](/docs/core/Quests.md) instance. */
    address public quests;
    /** @notice Address of the [Stats](/docs/core/Stats.md) instance. */
    address public stats;

    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    The address of the [Civilizations](/docs/core/Civilizations.md) instance.
     * @param _craft            The address of the [Craft](/docs/core/Craft.md) instance.
     * @param _equipment        The address of the [Equipment](/docs/core/Equipment.md) instance.
     * @param _experience       The address of the [Experience](/docs/core/Experience.md) instance.
     * @param _forge            The address of the [Forge](/docs/core/Forge.md) instance.
     * @param _names            The address of the [Names](/docs/core/Names.md) instance.
     * @param _quests           The address of the [Quests](/docs/core/Quests.md) instance.
     * @param _stats            The address of the [Stats](/docs/core/Stats.md) instance.
     */
    constructor(
        address _civilizations,
        address _craft,
        address _equipment,
        address _experience,
        address _forge,
        address _names,
        address _quests,
        address _stats
    ) {
        civilizations = _civilizations;
        craft = _craft;
        equipment = _equipment;
        experience = _experience;
        forge = _forge;
        names = _names;
        quests = _quests;
        stats = _stats;
    }
}
