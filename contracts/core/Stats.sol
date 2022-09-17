// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

import "../interfaces/ICivilizations.sol";
import "../interfaces/IExperience.sol";
import "../interfaces/IStats.sol";

/**
 * @title Stats
 * @notice This contract manages the stats points and pools for all the characters stored on the [Civilizations](/docs/core/Civilizations.md) instance.
 * The stats and the concept is based on the Cypher System for role playing games: http://cypher-system.com/.
 *
 * @notice Implementation of the [IStats](/docs/interfaces/IStats.md) interface.
 */
contract Stats is IStats, Ownable, Pausable {
    // =============================================== Storage ========================================================
    /** @dev Amount of seconds for refresh cooldown.  **/
    uint256 REFRESH_COOLDOWN_SECONDS = 86400; // 1 day.

    /** @dev Map to store the base stats from composed IDs. **/
    mapping(bytes => BasicStats) base;

    /** @dev Map to store the pool stats from composed ID. **/
    mapping(bytes => BasicStats) pool;

    /** @dev Map to store the the last refresh from composed ID. **/
    mapping(bytes => uint256) last_refresh;

    /** @dev Implementation of the `Refresher` **/
    address public refresher;

    /** @dev Implementation of the `Vitalizer` **/
    address public vitalizer;

    /** @dev Address of the `Civilizations` instance. **/
    address public civilizations;

    /** @dev Address of the `Experience` instance. **/
    address public experience;

    /** @dev Map to track the amount of points sacrificed by a character. **/
    mapping(bytes => uint256) public sacrifices;

    /** @dev Map to track the first refresher usage timestamp. **/
    mapping(bytes => uint256) public refresher_usage_time;

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
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    The address of the [Civilizations](/docs/core/Civilizations.md) instance.
     * @param _experience       The address of the [Experience](/docs/core/Experience.md) instance.
     */
    constructor(address _civilizations, address _experience) {
        civilizations = _civilizations;
        experience = _experience;
    }

    /** @notice Pauses the contract */
    function pause() public onlyOwner {
        _pause();
    }

    /** @notice Resumes the contract */
    function unpause() public onlyOwner {
        _unpause();
    }

    /** @dev Sets the `Refresher` instance.
     *  @param _token   address of the `Refresher` instance.
     */
    function setRefreshToken(address _token) public onlyOwner {
        refresher = _token;
    }

    /** @dev Sets the `Vitalizer` instance.
     *  @param _token   address of the `Vitalizer` instance.
     */
    function setVitalizerToken(address _token) public onlyOwner {
        vitalizer = _token;
    }

    /** @dev Reduces stats points from the pool.
     *  @param id         Composed ID of the token.
     *  @param stats      Amount of points reducing.
     */
    function consume(bytes memory id, BasicStats memory stats)
        public
        whenNotPaused
        onlyAllowed(id)
    {
        BasicStats storage currPool = pool[id];
        require(
            stats.might <= currPool.might,
            "Stats: cannot consume more might than currently available."
        );
        require(
            stats.speed <= currPool.speed,
            "Stats: cannot consume more speed than currently available."
        );
        require(
            stats.intellect <= currPool.intellect,
            "Stats: cannot consume more intellect than currently available."
        );

        pool[id].might -= stats.might;
        pool[id].speed -= stats.speed;
        pool[id].intellect -= stats.intellect;
    }

    /** @dev Reduces points to the base stats forever.
     *  @param id         Composed ID of the token.
     *  @param stats      Amount of points sacrificing.
     */
    function sacrifice(bytes memory id, BasicStats memory stats)
        public
        whenNotPaused
        onlyAllowed(id)
    {
        BasicStats storage currBase = base[id];
        require(
            stats.might <= currBase.might,
            "Stats: cannot sacrifice more might than currently available."
        );
        require(
            stats.speed <= currBase.speed,
            "Stats: cannot sacrifice more speed than currently available."
        );
        require(
            stats.intellect <= currBase.intellect,
            "Stats: cannot sacrifice more intellect than currently available."
        );

        base[id].might -= stats.might;
        base[id].speed -= stats.speed;
        base[id].intellect -= stats.intellect;

        if (pool[id].might > base[id].might) {
            pool[id].might = base[id].might;
        }

        if (pool[id].speed > base[id].speed) {
            pool[id].speed = base[id].speed;
        }

        if (pool[id].intellect > base[id].intellect) {
            pool[id].intellect = base[id].intellect;
        }

        sacrifices[id] += stats.might;
        sacrifices[id] += stats.speed;
        sacrifices[id] += stats.intellect;
    }

    /** @dev Performs a refresh filling the pool stats from the base stats.
     *  @param id   Composed ID of the token.
     */
    function refresh(bytes memory id) public whenNotPaused onlyAllowed(id) {
        uint256 last = last_refresh[id];
        require(
            last == 0 || getNextRefreshTime(id) <= block.timestamp,
            "Stats: not enough time has passed to refresh pool"
        );
        pool[id].might = base[id].might;
        pool[id].speed = base[id].speed;
        pool[id].intellect = base[id].intellect;
        last_refresh[id] = block.timestamp;
    }

    /** @dev Performs a refresh filling the pool stats from the base stats without cooldown spending `RefreshToken` (max 20 points per stat).
     *  @param id   Composed ID of the token.
     */
    function refreshWithToken(bytes memory id)
        public
        whenNotPaused
        onlyAllowed(id)
    {
        require(
            IERC20(refresher).balanceOf(msg.sender) >= 1,
            "Stats: not enough refresh tokens balance to perform a refresh."
        );
        require(
            IERC20(refresher).allowance(msg.sender, address(this)) >= 1,
            "Stats: not enough refresh tokens allowance to perform a refresh."
        );

        require(
            getNextRefreshWithTokenTime(id) <= block.timestamp,
            "Stats: already used a refresher for this day."
        );

        ERC20Burnable(refresher).burnFrom(msg.sender, 1);

        if ((base[id].might - pool[id].might) > 20) {
            pool[id].might += 20;
        } else {
            pool[id].might = base[id].might;
        }

        if ((base[id].speed - pool[id].speed) > 20) {
            pool[id].speed += 20;
        } else {
            pool[id].speed = base[id].speed;
        }

        if ((base[id].intellect - pool[id].intellect) > 20) {
            pool[id].intellect += 20;
        } else {
            pool[id].intellect = base[id].intellect;
        }

        refresher_usage_time[id] = block.timestamp;
    }

    /** @dev Consumes a vitalizer token to increase one point of a base stat.
     *  @param id         Composed ID of the token.
     *  @param stats      Amount of points increasing.
     */
    function consumeVitalizer(bytes memory id, BasicStats memory stats)
        public
        whenNotPaused
        onlyAllowed(id)
    {
        require(
            sacrifices[id] > 0,
            "Stats: user doesn't have sacrificed points to recover"
        );
        uint256 sum = stats.might + stats.speed + stats.intellect;
        require(
            sum == 1,
            "Stats: vitalizer should increase one point for a single stat."
        );
        require(
            IERC20(vitalizer).balanceOf(msg.sender) >= 1,
            "Stats: not enough vitalizer tokens balance to perform a vitalize."
        );
        require(
            IERC20(vitalizer).allowance(msg.sender, address(this)) >= 1,
            "Stats: not enough vitalizer tokens allowance to perform a vitalize."
        );

        ERC20Burnable(vitalizer).burnFrom(msg.sender, 1);

        base[id].might += stats.might;
        base[id].speed += stats.speed;
        base[id].intellect += stats.intellect;
        pool[id].might += stats.might;
        pool[id].speed += stats.speed;
        pool[id].intellect += stats.intellect;

        sacrifices[id] -= 1;
    }

    /** @dev Assigns the points to the base pool.
     *  @param id         Composed ID of the token.
     *  @param stats     Amount of points to assign.
     */
    function assignPoints(bytes memory id, BasicStats memory stats)
        public
        whenNotPaused
        onlyAllowed(id)
    {
        uint256 sum = stats.might + stats.speed + stats.intellect;
        uint256 available = getAvailablePoints(id);
        require(
            sum <= available,
            "Stats: can't assign more points than available."
        );
        base[id].might += stats.might;
        base[id].speed += stats.speed;
        base[id].intellect += stats.intellect;
        pool[id].might += stats.might;
        pool[id].speed += stats.speed;
        pool[id].intellect += stats.intellect;
    }

    // =============================================== Getters ========================================================

    /** @dev Returns the base stats of the composed ID.
     *  @param id   Composed ID of the token.
     */
    function getBaseStats(bytes memory id)
        public
        view
        returns (BasicStats memory)
    {
        return base[id];
    }

    /** @dev Returns the available pool stats of the composed ID.
     *  @param id   Composed ID of the token.
     */
    function getPoolStats(bytes memory id)
        public
        view
        returns (BasicStats memory)
    {
        return pool[id];
    }

    /** @dev Returns the amount of points available to assign.
     *  @param id   Composed ID of the token.
     */
    function getAvailablePoints(bytes memory id) public view returns (uint256) {
        BasicStats memory p = base[id];
        uint256 sum = p.intellect + p.might + p.speed;
        uint256 level = IExperience(experience).getLevel(id);
        uint256 assignableByLevel = _assignablePointsByLevel(level);
        return assignableByLevel - sum;
    }

    /** @dev Returns the time for the next free refresh.
     *  @param id   Composed ID of the token.
     */
    function getNextRefreshTime(bytes memory id) public view returns (uint256) {
        return last_refresh[id] + REFRESH_COOLDOWN_SECONDS;
    }

    /** @dev Returns the time of the refresh with tokens reset.
     *  @param id   Composed ID of the token.
     */
    function getNextRefreshWithTokenTime(bytes memory id)
        public
        view
        returns (uint256)
    {
        return refresher_usage_time[id] + REFRESH_COOLDOWN_SECONDS;
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
