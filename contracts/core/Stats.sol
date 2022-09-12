// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "../interfaces/ICivilizations.sol";
import "../interfaces/IExperience.sol";
import "../interfaces/IStats.sol";

/**
 * @dev `Stats` is a contract to manage the stats points and pools for a set of collections.
 *       The stats and the concept is created and modified based on the Cypher System for role playing games: http://cypher-system.com/.
 */

contract Stats is Ownable, IStats {
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
    address public refresher;

    /** @dev Address of the `Civilizations` instance. **/
    address public civilizations;

    /** @dev Address of the `Experience` instance. **/
    address public experience;

    // =============================================== Modifiers ======================================================

    /**
     * @dev Checks if `msg.sender` is owner or allowed to manipulate a composed ID.
     */
    modifier onlyAllowed(bytes memory id) {
        require(
            ICivilizations(civilizations).exists(id),
            "Stats: can't get access to a non minted token."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, id),
            "Stats: msg.sender is not allowed to access this token."
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     * @param _experience       The address of the `Experience` instance.
     */
    constructor(address _civilizations, address _experience) {
        civilizations = _civilizations;
        experience = _experience;
    }

    /** @dev Sets the `Refresher` instance.
     *  @param _token   address of the `Refresher` instance.
     */
    function setRefreshToken(address _token) public onlyOwner {
        refresher = _token;
    }

    /** @dev Reduces stats points from the pool.
     *  @param id         Composed ID of the token.
     *  @param might      Amount of might stat points reducing.
     *  @param speed      Amount of speed stat points reducing.
     *  @param intelect   Amount of intelect stat points reducing.
     */
    function consume(
        bytes memory id,
        uint256 might,
        uint256 speed,
        uint256 intelect
    ) public onlyAllowed(id) {
        CharacterStats storage currPool = pool[id];
        require(
            might <= currPool.might,
            "Stats: cannot consume more might than currently available."
        );
        require(
            speed <= currPool.speed,
            "Stats: cannot consume more speed than currently available."
        );
        require(
            intelect <= currPool.intelect,
            "Stats: cannot consume more intelect than currently available."
        );

        pool[id].might -= might;
        pool[id].speed -= speed;
        pool[id].intelect -= intelect;
    }

    /** @dev Reduces points to the base stats forever.
     *  @param id         Composed ID of the token.
     *  @param might      Amount of might stat points reducing.
     *  @param speed      Amount of speed stat points reducing.
     *  @param intelect   Amount of intelect stat points reducing.
     */
    function sacrifice(
        bytes memory id,
        uint256 might,
        uint256 speed,
        uint256 intelect
    ) public onlyAllowed(id) {
        CharacterStats storage currBase = base[id];
        require(
            might <= currBase.might,
            "Stats: cannot sacrifice more might than currently available."
        );
        require(
            speed <= currBase.speed,
            "Stats: cannot sacrifice more speed than currently available."
        );
        require(
            intelect <= currBase.intelect,
            "Stats: cannot sacrifice more intelect than currently available."
        );

        base[id].might -= might;
        base[id].speed -= speed;
        base[id].intelect -= intelect;
    }

    /** @dev Performs a refresh filling the pool stats from the base stats.
     *  @param id   Composed ID of the token.
     */
    function refresh(bytes memory id) public onlyAllowed(id) {
        uint256 last = last_refresh[id];
        require(
            last == 0 || last + REFRESH_COOLDOWN_SECONDS <= block.timestamp,
            "Stats: not enough time has passed to refresh pool"
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

    /** @dev Assigns the points to the base pool.
     *  @param id         Composed ID of the token.
     *  @param might     Amount of might stat points assign.
     *  @param speed     Amount of speed stat points assign.
     *  @param intelect  Amount of intelect stat points assign.
     */
    function assignPoints(
        bytes memory id,
        uint256 might,
        uint256 speed,
        uint256 intelect
    ) public onlyAllowed(id) {
        uint256 sum = might + speed + intelect;
        uint256 available = getAvailablePoints(id);
        require(
            sum <= available,
            "Stats: can't assign more points than available."
        );
        base[id].might += might;
        base[id].speed += speed;
        base[id].intelect += intelect;
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

    /** @dev Returns the amount of points available to assign.
     *  @param id   Composed ID of the token.
     */
    function getAvailablePoints(bytes memory id) public view returns (uint256) {
        CharacterStats memory p = base[id];
        uint256 sum = p.intelect + p.might + p.speed;
        uint256 level = IExperience(experience).getLevel(id);
        uint256 assignableByLevel = _assignablePointsByLevel(level);
        return assignableByLevel - sum;
    }

    /** @dev Returns the amount of points available to assign.
     *  @param id   Composed ID of the token.
     */
    function getNextRefreshTime(bytes memory id) public view returns (uint256) {
        return last_refresh[id] + REFRESH_COOLDOWN_SECONDS;
    }

    // =============================================== Internal ========================================================
    /** @dev Returns the amount of total asignable points by level.
     *  @param level   Level number to check points.
     */
    function _assignablePointsByLevel(uint256 level)
        internal
        pure
        returns (uint256)
    {
        uint256 points = 6;
        return points + level;
    }
}
