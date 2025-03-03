// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import "../interfaces/ICivilizations.sol";
import "../interfaces/IExperience.sol";
import "../interfaces/IStats.sol";
import "../interfaces/IEquipment.sol";

/**
 * @title Stats
 * @notice This contract manages the stats points and pools for all the characters stored on the [Civilizations](/docs/core/Civilizations.md) instance.
 * The stats and the concept is based on the Cypher System for role playing games: http://cypher-system.com/.
 *
 * @notice Implementation of the [IStats](/docs/interfaces/IStats.md) interface.
 */
contract Stats is
    IStats,
    Initializable,
    PausableUpgradeable,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    // =============================================== Storage ========================================================

    /** @notice Constant amount of seconds for refresh cooldown.  **/
    uint256 public REFRESH_COOLDOWN_SECONDS;

    /** @notice Map track the base stats for characters. */
    mapping(bytes => BasicStats) base;

    /** @notice Map track the pool stats for characters. */
    mapping(bytes => BasicStats) pool;

    /** @notice Map track the last refresh timestamps of the characters. */
    mapping(bytes => uint256) last_refresh;

    /** @notice Address of the Refresher [BaseGadgetToken](/docs/base/BaseGadgetToken.md) instance. */
    address public refresher;

    /** @notice Address of the Vitalizer [BaseGadgetToken](/docs/base/BaseGadgetToken.md) instance. */
    address public vitalizer;

    /** @notice Address of the [Civilizations](/docs/core/Civilizations.md) instance. */
    address public civilizations;

    /** @notice Address of the [Experience](/docs/core/Experience.md) instance. */
    address public experience;

    /** @notice Address of the [Equipment](/docs/core/Equipment.md) instance. */
    address public equipment;

    /** @notice Map to track the amount of points sacrificed by a character. */
    mapping(bytes => uint256) public sacrifices;

    /** @notice Map to track the first refresher token usage timestamps. */
    mapping(bytes => uint256) public refresher_usage_time;

    // =============================================== Modifiers ======================================================

    /**
     * @notice Checks against the [Civilizations](/docs/core/Civilizations.md) instance if the `msg.sender` is the owner or
     * has allowance to access a composed ID.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     */
    modifier onlyAllowed(bytes memory _id) {
        require(
            ICivilizations(civilizations).exists(_id),
            "Stats: onlyAllowed() token not minted."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, _id),
            "Stats: onlyAllowed() msg.sender is not allowed to access this token."
        );
        _;
    }

    // =============================================== Events =========================================================

    /**
     * @notice Event emmited when the character base or pool points change.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     * @param _pool_stats   Pool stat points.
     * @param _base_stats   Pool stat points
     */
    event ChangedPoints(
        bytes _id,
        BasicStats _base_stats,
        BasicStats _pool_stats
    );

    // =============================================== Setters ========================================================

    /**
     * @notice Initialize.
     *
     * Requirements:
     * @param _civilizations    The address of the [Civilizations](/docs/core/Civilizations.md) instance.
     * @param _experience       The address of the [Experience](/docs/core/Experience.md) instance.
     * @param _equipment       The address of the [Equipment](/docs/core/Equipment.md) instance.
     * @param _refresher    Address of the Refresher [BaseGadgetToken](/docs/base/BaseGadgetToken.md) instance.
     * @param _vitalizer    Address of the Vitalizer [BaseGadgetToken](/docs/base/BaseGadgetToken.md) instance.
     */
    function initialize(
        address _civilizations,
        address _experience,
        address _equipment,
        address _refresher,
        address _vitalizer
    ) public initializer {
        __Ownable_init();
        __Pausable_init();
        __UUPSUpgradeable_init();

        civilizations = _civilizations;
        experience = _experience;
        equipment = _equipment;
        refresher = _refresher;
        vitalizer = _vitalizer;
        REFRESH_COOLDOWN_SECONDS = 86400; // 1 day
    }

    /** @notice Pauses the contract */
    function pause() public onlyOwner {
        _pause();
    }

    /** @notice Resumes the contract */
    function unpause() public onlyOwner {
        _unpause();
    }

    /**
     * @notice Changes the amount of seconds of cooldown between refreshes.
     *
     * Requirements:
     * @param _cooldown     Amount of seconds to wait between refreshes.
     */
    function setRefreshCooldown(uint256 _cooldown) public onlyOwner {
        REFRESH_COOLDOWN_SECONDS = _cooldown;
    }

    /**
     * @notice Removes the amount of points available on the character pool stats.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _stats    Stats to consume.
     */
    function consume(
        bytes memory _id,
        BasicStats memory _stats
    ) public whenNotPaused onlyAllowed(_id) {
        BasicStats memory _modifiers = IEquipment(equipment)
            .getCharacterTotalStatsModifiers(_id);

        BasicStats memory _consumes;

        if (_modifiers.might < _stats.might) {
            _consumes.might = _stats.might - _modifiers.might;
        }

        if (_modifiers.speed <= _stats.speed) {
            _consumes.speed = _stats.speed - _modifiers.speed;
        }

        if (_modifiers.intellect <= _stats.intellect) {
            _consumes.intellect = _stats.intellect - _modifiers.intellect;
        }

        BasicStats storage _pool = pool[_id];

        require(
            _consumes.might <= _pool.might,
            "Stats: consume() not enough might."
        );
        require(
            _consumes.speed <= _pool.speed,
            "Stats: consume() not enough speed."
        );
        require(
            _consumes.intellect <= _pool.intellect,
            "Stats: consume() not enough intellect."
        );

        pool[_id].might -= _consumes.might;
        pool[_id].speed -= _consumes.speed;
        pool[_id].intellect -= _consumes.intellect;

        emit ChangedPoints(_id, base[_id], pool[_id]);
    }

    /**
     * @notice Removes the amount of points available on the character base stats.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _stats    Stats to consume.
     */
    function sacrifice(
        bytes memory _id,
        BasicStats memory _stats
    ) public whenNotPaused onlyAllowed(_id) {
        BasicStats storage _base = base[_id];
        require(
            _stats.might <= _base.might,
            "Stats: sacrifice() not enough might."
        );
        require(
            _stats.speed <= _base.speed,
            "Stats: sacrifice() not enough speed."
        );
        require(
            _stats.intellect <= _base.intellect,
            "Stats: sacrifice() not enough intellect."
        );

        base[_id].might -= _stats.might;
        base[_id].speed -= _stats.speed;
        base[_id].intellect -= _stats.intellect;

        if (pool[_id].might > base[_id].might) {
            pool[_id].might = base[_id].might;
        }

        if (pool[_id].speed > base[_id].speed) {
            pool[_id].speed = base[_id].speed;
        }

        if (pool[_id].intellect > base[_id].intellect) {
            pool[_id].intellect = base[_id].intellect;
        }

        sacrifices[_id] += _stats.might;
        sacrifices[_id] += _stats.speed;
        sacrifices[_id] += _stats.intellect;
        emit ChangedPoints(_id, base[_id], pool[_id]);
    }

    /**
     * @notice Refills the pool stats for the character.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     */
    function refresh(bytes memory _id) public whenNotPaused onlyAllowed(_id) {
        uint256 _last = last_refresh[_id];
        require(
            _last == 0 || getNextRefreshTime(_id) <= block.timestamp,
            "Stats: refresh() not enough time has passed to refresh pool."
        );
        pool[_id].might = base[_id].might;
        pool[_id].speed = base[_id].speed;
        pool[_id].intellect = base[_id].intellect;
        last_refresh[_id] = block.timestamp;
        emit ChangedPoints(_id, base[_id], pool[_id]);
    }

    /**
     * @notice Refills the pool stats for the character spending a Refresher [BaseGadgetToken](/docs/base/BaseGadgetToken.md) token.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     */
    function refreshWithToken(
        bytes memory _id
    ) public whenNotPaused onlyAllowed(_id) {
        require(
            IERC20(refresher).balanceOf(msg.sender) >= 1,
            "Stats: refreshWithToken() not enough refresh tokens balance."
        );
        require(
            IERC20(refresher).allowance(msg.sender, address(this)) >= 1,
            "Stats: refreshWithToken() not enough refresh tokens allowance."
        );
        require(
            getNextRefreshWithTokenTime(_id) <= block.timestamp,
            "Stats: refreshWithToken() no more refresh with tokens available."
        );

        ERC20Burnable(refresher).burnFrom(msg.sender, 1);

        if ((base[_id].might - pool[_id].might) > 20) {
            pool[_id].might += 20;
        } else {
            pool[_id].might = base[_id].might;
        }

        if ((base[_id].speed - pool[_id].speed) > 20) {
            pool[_id].speed += 20;
        } else {
            pool[_id].speed = base[_id].speed;
        }

        if ((base[_id].intellect - pool[_id].intellect) > 20) {
            pool[_id].intellect += 20;
        } else {
            pool[_id].intellect = base[_id].intellect;
        }

        refresher_usage_time[_id] = block.timestamp;
        emit ChangedPoints(_id, base[_id], pool[_id]);
    }

    /**
     * @notice Recovers a sacrificed point spending a Vitalizer [BaseGadgetToken](/docs/base/BaseGadgetToken.md) token.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _stats    Stats to sacrifice.
     */
    function vitalize(
        bytes memory _id,
        BasicStats memory _stats
    ) public whenNotPaused onlyAllowed(_id) {
        require(
            sacrifices[_id] > 0,
            "Stats: vitalize() not enough sacrificed points."
        );
        uint256 sum = _stats.might + _stats.speed + _stats.intellect;
        require(sum == 1, "Stats: vitalize() too many points to recover.");
        require(
            IERC20(vitalizer).balanceOf(msg.sender) >= 1,
            "Stats: vitalize() not enough vitalizer tokens balance."
        );
        require(
            IERC20(vitalizer).allowance(msg.sender, address(this)) >= 1,
            "Stats: vitalize() not enough vitalizer tokens allowance."
        );

        ERC20Burnable(vitalizer).burnFrom(msg.sender, 1);

        base[_id].might += _stats.might;
        base[_id].speed += _stats.speed;
        base[_id].intellect += _stats.intellect;
        pool[_id].might += _stats.might;
        pool[_id].speed += _stats.speed;
        pool[_id].intellect += _stats.intellect;

        sacrifices[_id] -= 1;
        emit ChangedPoints(_id, base[_id], pool[_id]);
    }

    /**
     * @notice Increases points of the base pool based on new levels.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _stats    Stats to increase.
     */
    function assignPoints(
        bytes memory _id,
        BasicStats memory _stats
    ) public whenNotPaused onlyAllowed(_id) {
        uint256 sum = _stats.might + _stats.speed + _stats.intellect;
        uint256 available = getAvailablePoints(_id);
        require(
            sum <= available,
            "Stats: assignPoints() too many points selected."
        );
        base[_id].might += _stats.might;
        base[_id].speed += _stats.speed;
        base[_id].intellect += _stats.intellect;
        pool[_id].might += _stats.might;
        pool[_id].speed += _stats.speed;
        pool[_id].intellect += _stats.intellect;
        emit ChangedPoints(_id, base[_id], pool[_id]);
    }

    // =============================================== Getters ========================================================

    /**
     * @notice External function that returns the base points of a character.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     *
     * @return _stats   Base stats of the character.
     */
    function getBaseStats(
        bytes memory _id
    ) public view returns (BasicStats memory _stats) {
        return base[_id];
    }

    /**
     * @notice External function that returns the available pool points of a character.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     *
     * @return _stats   Available pool stats of the character.
     */
    function getPoolStats(
        bytes memory _id
    ) public view returns (BasicStats memory _stats) {
        return pool[_id];
    }

    /**
     * @notice External function that returns the assignable points of a character.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _points      Number of points available to assign.
     */
    function getAvailablePoints(
        bytes memory _id
    ) public view returns (uint256 _points) {
        BasicStats memory _base = base[_id];
        uint256 _sum = _base.intellect + _base.might + _base.speed;
        uint256 level = IExperience(experience).getLevel(_id);
        uint256 assignableByLevel = _assignablePointsByLevel(level);
        return assignableByLevel - _sum;
    }

    /**
     * @notice External function that returns the next refresher timestamp for a character.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _timestamp   Timestamp when the next refresh is available.
     */
    function getNextRefreshTime(
        bytes memory _id
    ) public view returns (uint256 _timestamp) {
        return last_refresh[_id] + REFRESH_COOLDOWN_SECONDS;
    }

    /**
     * @notice External function that returns the next refresher timestamp for a character when using a Refresher [BaseGadgetToken](/docs/base/BaseGadgetToken.md) token.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _timestamp   Timestamp when the next refresh is available.
     */
    function getNextRefreshWithTokenTime(
        bytes memory _id
    ) public view returns (uint256 _timestamp) {
        return refresher_usage_time[_id] + REFRESH_COOLDOWN_SECONDS;
    }

    // =============================================== Internal ========================================================

    /**
     * @notice Internal function to get the amount of points assignable by a provided level.
     *
     * Requirements:
     * @param _level     Level to get the assignable points.
     *
     * @return _points   Amount of points spendable for this level.
     */
    function _assignablePointsByLevel(
        uint256 _level
    ) internal pure returns (uint256 _points) {
        uint256 points = 6;
        return points + _level;
    }

    /** @notice Internal function make sure upgrade proxy caller is the owner. */
    function _authorizeUpgrade(
        address newImplementation
    ) internal virtual override onlyOwner {}
}
