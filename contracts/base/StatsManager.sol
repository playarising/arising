// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/access/Ownable.sol";
import "../interfaces/IBaseERC721.sol";
import "../interfaces/IRefresherToken.sol";

/*
 * StatsManager is a contract to manage the stats points and pools for a set of collections.
 * All Stats are based on the Cypher System for role playing games: http://cypher-system.com/
 */
contract StatsManager is Ownable {
    // Seconds required to pass between refreshes
    uint256 SECONDS_BETWEEN_REFRESH = 86400; // 1 day.

    // Boolean to initialize the mint capabilities.
    bool public initialized;

    // Stats are the base attributes for a character
    struct Stats {
        uint256 might;
        uint256 speed;
        uint256 intelect;
    }

    // Maps to store the civilizations, base and pool information
    mapping(uint256 => address) civilizations;
    uint256[] civilizations_ids;

    mapping(uint256 => mapping(uint256 => Stats)) base;
    mapping(uint256 => mapping(uint256 => Stats)) pool;

    // Map to include timestamps of last refresh
    mapping(uint256 => mapping(uint256 => uint256)) last_refresh;

    // Array for all civilizations
    address[] all_civilizations;

    // Address of the gadget refresher
    address refresher_token;

    /**
     * @dev Throws if called when the contract is not initialized.
     */
    modifier onlyInitialized() {
        require(initialized, "BaseERC721: contract is not initialized");
        _;
    }

    constructor(address _refresher_token) {
        refresher_token = _refresher_token;
    }

    /**
     * @dev Enabled the collection to be minted
     */
    function setInitialized() public onlyOwner {
        initialized = true;
    }

    function getBaseStats(uint256 id, uint256 civilization)
        public
        view
        returns (Stats memory)
    {
        require(
            civilizations[civilization] != address(0),
            "StatsManager: civilization doesn't exist"
        );
        return base[civilization][id];
    }

    function getPoolStats(uint256 id, uint256 civilization)
        public
        view
        returns (Stats memory)
    {
        require(
            civilizations[civilization] != address(0),
            "StatsManager: civilization doesn't exist"
        );
        return pool[civilization][id];
    }

    function addCivilization(address instance)
        public
        onlyOwner
        onlyInitialized
    {
        require(
            instance != address(0),
            "StatsManager: instance address is null"
        );
        uint256 newId = civilizations_ids.length + 1;
        civilizations[newId] = instance;
        civilizations_ids.push(newId);
    }

    function consume(
        uint256 civilization,
        uint256 id,
        uint256 might,
        uint256 speed,
        uint256 intelect
    ) public onlyInitialized {
        require(
            civilizations[civilization] != address(0),
            "StatsManager: civilization doesn't exist"
        );
        require(
            IBaseERC721(civilizations[civilization]).isApprovedOrOwner(
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

    function refresh(uint256 civilization, uint256 id) public onlyInitialized {
        require(
            civilizations[civilization] != address(0),
            "StatsManager: civilization doesn't exist"
        );
        require(
            IBaseERC721(civilizations[civilization]).isApprovedOrOwner(
                msg.sender,
                id
            ),
            "StatsManager: interaction is not from owner or allowed"
        );

        uint256 last = last_refresh[civilization][id];
        require(
            last == 0 || last + SECONDS_BETWEEN_REFRESH <= block.timestamp,
            "StatsManager: not enough time has passed to refresh pool"
        );

        pool[civilization][id].might = base[civilization][id].might;
        pool[civilization][id].speed = base[civilization][id].speed;
        pool[civilization][id].intelect = base[civilization][id].intelect;
        last_refresh[civilization][id] = block.timestamp;
    }

    function refreshWithToken(uint256 civilization, uint256 id)
        public
        onlyInitialized
    {
        require(
            civilizations[civilization] != address(0),
            "StatsManager: civilization doesn't exist"
        );
        require(
            IBaseERC721(civilizations[civilization]).isApprovedOrOwner(
                msg.sender,
                id
            ),
            "StatsManager: interaction is not from owner or allowed"
        );

        IRefresherToken(refresher_token).burnFrom(msg.sender, 1);

        pool[civilization][id].might = base[civilization][id].might;
        pool[civilization][id].speed = base[civilization][id].speed;
        pool[civilization][id].intelect = base[civilization][id].intelect;
        last_refresh[civilization][id] = block.timestamp;
    }
}
