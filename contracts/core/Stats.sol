// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "../interfaces/ICivilizations.sol";

/**
 * @dev `Stats` is a contract to manage the stats points and pools for a set of collections.
 *       The stats and the concept is created and modified based on the Cypher System for role playing games: http://cypher-system.com/.
 */

contract Stats is Ownable {
    // =============================================== Structs ========================================================

    /** @dev Struct to define the stats of a character.
     * @param might    The amount of points for the might stat.
     * @param speed    The amount of points for the speed stat.
     * @param intelect The amount of points for the intelect stat.
     */
    struct CharacterStats {
        uint256 might;
        uint256 speed;
        uint256 intelect;
    }

    // =============================================== Storage ========================================================
    /** @dev Amount of seconds for refresh cooldown.  **/
    uint256 REFRESH_COOLDOWN_SECONDS = 86400; // 1 day.

    /** @dev Map to store the base stats from composed IDs. **/
    mapping(bytes => CharacterStats) base;

    /** @dev Map to store the pool stats from composed ID. **/
    mapping(bytes => CharacterStats) pool;

    /** @dev Map to store the the last refresh from composed ID. **/
    mapping(bytes => uint256) last_refresh;

    /** @dev Implementation of the `Refresher` **/
    address refresher;

    /** @dev Address of the `Civilizations` instance. **/
    address public civilizations;

    // =============================================== Modifiers ======================================================

    /**
     * @dev Checks if `msg.sender` is owner or allowed to manipulate a composed ID.
     */
    modifier onlyAllowed(bytes memory id) {
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, id),
            "BaseFungibleItem: require consumer to be owner or have allowance"
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations) {
        civilizations = _civilizations;
    }

    /** @dev Sets the `Refresher` instance.
     *  @param _token   address of the `Refresher` instance.
     */
    function setRefreshToken(address _token) public onlyOwner {
        refresher = _token;
    }

    /** @dev Reduces stats points from the pool.
     *  @param id               Composed ID of the token.
     *  @param might            Amount of might stat points reducing.
     *  @param speed            Amount of speed stat points reducing.
     *  @param intelect         Amount of intelect stat points reducing.
     */
    function consume(
        bytes memory id,
        uint256 might,
        uint256 speed,
        uint256 intelect
    ) public onlyAllowed(id) {
        CharacterStats storage currPool = pool[id];
        require(
            currPool.might - might > 0,
            "StatsManager: cannot consume less might than current available"
        );
        require(
            currPool.speed - speed > 0,
            "StatsManager: cannot consume less speed than current available"
        );
        require(
            currPool.intelect - intelect > 0,
            "StatsManager: cannot consume less intelect than current available"
        );

        pool[id].might -= might;
        pool[id].speed -= speed;
        pool[id].intelect -= intelect;
    }

    /** @dev Performs a refresh filling the pool stats from the base stats.
     *  @param id   Composed ID of the token.
     */
    function refresh(bytes memory id) public onlyAllowed(id) {
        uint256 last = last_refresh[id];
        require(
            last == 0 || last + REFRESH_COOLDOWN_SECONDS <= block.timestamp,
            "StatsManager: not enough time has passed to refresh pool"
        );
        pool[id].might = base[id].might;
        pool[id].speed = base[id].speed;
        pool[id].intelect = base[id].intelect;
        last_refresh[id] = block.timestamp;
    }

    /** @dev Performs a refresh filling the pool stats from the base stats without cooldown spending `RefreshToken`.
     *  @param id   Composed ID of the token.
     */
    function refreshWithToken(bytes memory id) public onlyAllowed(id) {
        ERC20Burnable(refresher).burnFrom(msg.sender, 1);

        pool[id].might = base[id].might;
        pool[id].speed = base[id].speed;
        pool[id].intelect = base[id].intelect;
        last_refresh[id] = block.timestamp;
    }

    // =============================================== Getters ========================================================

    /** @dev Returns the base stats of the composed ID.
     *  @param id   Composed ID of the token.
     */
    function getBaseStats(bytes memory id)
        public
        view
        returns (CharacterStats memory)
    {
        return base[id];
    }

    /** @dev Returns the available pool stats of the composed ID.
     *  @param id   Composed ID of the token.
     */
    function getPoolStats(bytes memory id)
        public
        view
        returns (CharacterStats memory)
    {
        return pool[id];
    }
}
