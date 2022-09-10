// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "../interfaces/IBaseERC721.sol";

/**
 * @dev `StatsManager` is a contract to manage the stats points and pools for a set of collections.
 *       The stats and the concept is created and modified based on the Cypher System for role playing games: http://cypher-system.com/.
 */

contract StatsManager is Ownable {
    // =============================================== Structs ========================================================

    /** @dev Struct to define the stats of a character.
     * @param might    The amount of points for the might stat.
     * @param speed    The amount of points for the speed stat.
     * @param intelect The amount of points for the intelect stat.
     */
    struct Stats {
        uint256 might;
        uint256 speed;
        uint256 intelect;
    }

    // =============================================== Storage ========================================================
    /** @dev Amount of seconds for refresh cooldown.  **/
    uint256 REFRESH_COOLDOWN_SECONDS = 86400; // 1 day.

    /** @dev Boolean to check if the implementation is usable. **/
    bool public initialized;

    /** @dev Map to store the civilizations implemented. **/
    mapping(uint256 => address) _civilizations;

    /** @dev Array to store the civilizations implemented. **/
    uint256[] civilizations;

    /** @dev Map to store the maps for each civilization and token id base stats. **/
    mapping(uint256 => mapping(uint256 => Stats)) base;

    /** @dev Map to store the maps for each civilization and token id pool stats. **/
    mapping(uint256 => mapping(uint256 => Stats)) pool;

    /** @dev Map to store the last refresh for each civilization and token id. **/
    mapping(uint256 => mapping(uint256 => uint256)) last_refresh;

    /** @dev Implementation of the `RefreshToken` **/
    address refresh_token;

    // =============================================== Events =========================================================
    // ============================================== Modifiers =======================================================

    /**
     * @dev Checks if `initialized` is enabled.
     */
    modifier onlyInitialized() {
        require(initialized, "BaseERC721: contract is not initialized");
        _;
    }

    // =============================================== Setters ========================================================

    /** @dev Enables the `StatsManager` implementation. */
    function setInitialized() public onlyOwner {
        require(
            refresh_token != address(0),
            "StatsManager: can't initialize when no refresh token set"
        );
        initialized = true;
    }

    /** @dev Sets the `RefreshToken` instance.
     *  @param _token   address of the `RefreshToken` instance.
     */
    function setRefreshToken(address _token) public onlyOwner {
        refresh_token = _token;
    }

    /** @dev Adds a civilization to the stats manager.
     *  @param _instance  Address of the `BaseERC721` instance.
     */
    function addCivilization(address _instance)
        public
        onlyOwner
        onlyInitialized
    {
        require(
            _instance != address(0),
            "StatsManager: instance address is null"
        );
        uint256 newId = civilizations.length + 1;
        _civilizations[newId] = _instance;
        civilizations.push(newId);
    }

    /** @dev Reduces stats points from the pool.
     *  @param civilization     ID of the civilization.
     *  @param id               ID of the token.
     *  @param might            Amount of might stat points reducing.
     *  @param speed            Amount of speed stat points reducing.
     *  @param intelect         Amount of intelect stat points reducing.
     */
    function consume(
        uint256 civilization,
        uint256 id,
        uint256 might,
        uint256 speed,
        uint256 intelect
    ) public onlyInitialized {
        require(
            _civilizations[civilization] != address(0),
            "StatsManager: civilization doesn't exist"
        );
        require(
            IBaseERC721(_civilizations[civilization]).isApprovedOrOwner(
                msg.sender,
                id
            ),
            "StatsManager: interaction is not from owner or allowed"
        );
        Stats storage currPool = pool[civilization][id];
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

        pool[civilization][id].might -= might;
        pool[civilization][id].speed -= speed;
        pool[civilization][id].intelect -= intelect;
    }

    /** @dev Performs a refresh filling the pool stats from the base stats.
     *  @param civilization     ID of the civilization.
     *  @param id               ID of the token.
     */
    function refresh(uint256 civilization, uint256 id) public onlyInitialized {
        require(
            _civilizations[civilization] != address(0),
            "StatsManager: civilization doesn't exist"
        );
        require(
            IBaseERC721(_civilizations[civilization]).isApprovedOrOwner(
                msg.sender,
                id
            ),
            "StatsManager: interaction is not from owner or allowed"
        );

        uint256 last = last_refresh[civilization][id];
        require(
            last == 0 || last + REFRESH_COOLDOWN_SECONDS <= block.timestamp,
            "StatsManager: not enough time has passed to refresh pool"
        );

        pool[civilization][id].might = base[civilization][id].might;
        pool[civilization][id].speed = base[civilization][id].speed;
        pool[civilization][id].intelect = base[civilization][id].intelect;
        last_refresh[civilization][id] = block.timestamp;
    }

    /** @dev Performs a refresh filling the pool stats from the base stats without cooldown spending `RefreshToken`.
     *  @param civilization     ID of the civilization.
     *  @param id               ID of the token.
     */
    function refreshWithToken(uint256 civilization, uint256 id)
        public
        onlyInitialized
    {
        require(
            _civilizations[civilization] != address(0),
            "StatsManager: civilization doesn't exist"
        );
        require(
            IBaseERC721(_civilizations[civilization]).isApprovedOrOwner(
                msg.sender,
                id
            ),
            "StatsManager: interaction is not from owner or allowed"
        );

        ERC20Burnable(refresh_token).burnFrom(msg.sender, 1);

        pool[civilization][id].might = base[civilization][id].might;
        pool[civilization][id].speed = base[civilization][id].speed;
        pool[civilization][id].intelect = base[civilization][id].intelect;
        last_refresh[civilization][id] = block.timestamp;
    }

    // =============================================== Getters ========================================================

    /** @dev Returns the base stats of the token from the civilization sent.
     *  @param id               ID of the token.
     *  @param civilization     ID of the civilization.
     */
    function getBaseStats(uint256 id, uint256 civilization)
        public
        view
        returns (Stats memory)
    {
        require(
            _civilizations[civilization] != address(0),
            "StatsManager: civilization doesn't exist"
        );
        return base[civilization][id];
    }

    /** @dev Returns the available pool stats of the token from the civilization sent.
     *  @param id               ID of the token.
     *  @param civilization     ID of the civilization.
     */
    function getPoolStats(uint256 id, uint256 civilization)
        public
        view
        returns (Stats memory)
    {
        require(
            _civilizations[civilization] != address(0),
            "StatsManager: civilization doesn't exist"
        );
        return pool[civilization][id];
    }
}
