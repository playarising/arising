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

    /** @dev Implementation of the `Vitalizer` **/
    address public vitalizer;

    /** @dev Address of the `Civilizations` instance. **/
    address public civilizations;

    /** @dev Address of the `Experience` instance. **/
    address public experience;

    /** @dev Map to track the usage of vitality tokens. **/
    mapping(bytes => uint256) public vitality_uses;

    /** @dev Map to track the daily usages of refresher tokens. **/
    mapping(bytes => uint256) public refresher_counts;

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

    /** @dev Sets the `Vitalizer` instance.
     *  @param _token   address of the `Vitalizer` instance.
     */
    function setVitalizerToken(address _token) public onlyOwner {
        vitalizer = _token;
    }

    /** @dev Reduces stats points from the pool.
     *  @param id         Composed ID of the token.
     *  @param might      Amount of might stat points reducing.
     *  @param speed      Amount of speed stat points reducing.
     *  @param intellect   Amount of intellect stat points reducing.
     */
    function consume(
        bytes memory id,
        uint256 might,
        uint256 speed,
        uint256 intellect
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
            intellect <= currPool.intellect,
            "Stats: cannot consume more intellect than currently available."
        );

        pool[id].might -= might;
        pool[id].speed -= speed;
        pool[id].intellect -= intellect;
    }

    /** @dev Reduces points to the base stats forever.
     *  @param id         Composed ID of the token.
     *  @param might      Amount of might stat points reducing.
     *  @param speed      Amount of speed stat points reducing.
     *  @param intellect   Amount of intellect stat points reducing.
     */
    function sacrifice(
        bytes memory id,
        uint256 might,
        uint256 speed,
        uint256 intellect
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
            intellect <= currBase.intellect,
            "Stats: cannot sacrifice more intellect than currently available."
        );

        base[id].might -= might;
        base[id].speed -= speed;
        base[id].intellect -= intellect;

        if (pool[id].might > base[id].might) {
            pool[id].might = base[id].might;
        }

        if (pool[id].speed > base[id].speed) {
            pool[id].speed = base[id].speed;
        }

        if (pool[id].intellect > base[id].intellect) {
            pool[id].intellect = base[id].intellect;
        }
    }

    /** @dev Performs a refresh filling the pool stats from the base stats.
     *  @param id   Composed ID of the token.
     */
    function refresh(bytes memory id) public onlyAllowed(id) {
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
    function refreshWithToken(bytes memory id) public onlyAllowed(id) {
        require(
            IERC20(refresher).balanceOf(msg.sender) >= 1,
            "Stats: not enough refresh tokens balance to perform a refresh."
        );
        require(
            IERC20(refresher).allowance(msg.sender, address(this)) >= 1,
            "Stats: not enough refresh tokens allowance to perform a refresh."
        );

        require(
            refresher_counts[id] < 5 ||
                getNextRefreshWithTokenTime(id) <= block.timestamp,
            "Stats: already five refreshers used for the day."
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

        if (getNextRefreshWithTokenTime(id) <= block.timestamp) {
            refresher_counts[id] = 0;
            refresher_usage_time[id] = block.timestamp;
        }

        refresher_counts[id] += 1;
    }

    /** @dev Consumes a vitalizer token to increase one point of a base stat.
     *  @param id         Composed ID of the token.
     *  @param might      Amount of might stat points increasing.
     *  @param speed      Amount of speed stat points increasing.
     *  @param intellect   Amount of intellect stat points increasing.
     */
    function consumeVitalizer(
        bytes memory id,
        uint256 might,
        uint256 speed,
        uint256 intellect
    ) public onlyAllowed(id) {
        require(
            vitality_uses[id] < 10,
            "Stats: character already used all possible vitalize consumes."
        );
        uint256 sum = might + speed + intellect;
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

        base[id].might += might;
        base[id].speed += speed;
        base[id].intellect += intellect;
        pool[id].might += might;
        pool[id].speed += speed;
        pool[id].intellect += intellect;

        vitality_uses[id] += 1;
    }

    /** @dev Assigns the points to the base pool.
     *  @param id         Composed ID of the token.
     *  @param might     Amount of might stat points assign.
     *  @param speed     Amount of speed stat points assign.
     *  @param intellect  Amount of intellect stat points assign.
     */
    function assignPoints(
        bytes memory id,
        uint256 might,
        uint256 speed,
        uint256 intellect
    ) public onlyAllowed(id) {
        uint256 sum = might + speed + intellect;
        uint256 available = getAvailablePoints(id);
        require(
            sum <= available,
            "Stats: can't assign more points than available."
        );
        base[id].might += might;
        base[id].speed += speed;
        base[id].intellect += intellect;
        pool[id].might += might;
        pool[id].speed += speed;
        pool[id].intellect += intellect;
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
