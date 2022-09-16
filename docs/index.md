# Solidity API

## BaseERC20Wrapper

This contract is a standard {ERC20} implementation with burnable and mintable
functions exposed to the contract owner. This contract is a wrapper for the {BaseFungibleItem} instance to convert
an internal fungible token to the ERC20 standard.

_Implementation of the {IBaseERC20Wrapper} interface._

### constructor

```solidity
constructor(string _name, string _symbol) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _name | string | Name of the ERC20 token. |
| _symbol | string | Symbol of the ERC20 token. |

### mint

```solidity
function mint(address _to, uint256 _amount) public
```

Creates tokens to the address provided.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _to | address | Address that receives the tokens. |
| _amount | uint256 | Amount to be minted. |

## BaseERC721

This contract is a {ERC721Enumerable} implementation for the different civilizations.
Exposes the mint function to the owner and some check functions.

_Implementation of the {IBaseERC721} interface._

### baseURI

```solidity
string baseURI
```

Constant for the base url of the token metadata.

### constructor

```solidity
constructor(string _name, string _symbol, string _uri) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _name | string | Name of the ERC20 token. |
| _symbol | string | Symbol of the ERC20 token. |
| _uri | string | Base url for the tokens metadata. |

### mint

```solidity
function mint(address to) public
```

Creates tokens to the address provided.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| to | address | Address that receives the tokens. |

### isApprovedOrOwner

```solidity
function isApprovedOrOwner(address spender, uint256 id) public view virtual returns (bool)
```

External function to check if an address is allowed to access a token.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| spender | address | Address that will access the token. |
| id | uint256 | ID of the token to be accessed. |

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | bool | bool |

### exists

```solidity
function exists(uint256 id) public view returns (bool)
```

External function to check if a token id is minted.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | uint256 | ID of the token to be checked. |

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | bool | bool |

### _baseURI

```solidity
function _baseURI() internal view virtual returns (string)
```

Internal function that overrides the {ERC721_baseURI} function
with an URI specified over the constructor.

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | string | string |

## BaseFungibleItem

This contract an imitation of the ERC20 standard to work around the character context.
It tracks balances of characters tokens. This also includes functions to wrap and unwrap to a
{BaseERC20Wrapper} instance.

_Implementation of the {IBaseFungibleItem} interface._

### name

```solidity
string name
```

Constant for the name of the item.

### image

```solidity
string image
```

Constant the url pointing to the image of the item.

### symbol

```solidity
string symbol
```

Constant for the symbol of the item.

### civilizations

```solidity
address civilizations
```

Constant for the address of the {Civilizations} instance.

### balances

```solidity
mapping(bytes => uint256) balances
```

Map to track the balances of characters.

### wrapper

```solidity
address wrapper
```

Constant for the address of the {BaseERC20Wrapper} instance.

### onlyAllowed

```solidity
modifier onlyAllowed(bytes _id)
```

Checks against the {Civilizations} instance if the {msg.sender} is the owner or
has allowance to access a composed ID.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _id | bytes | Composed ID of the token. |

### constructor

```solidity
constructor(string _name, string _symbol, string _image, address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _name | string | Name of the ERC20 token. |
| _symbol | string | Symbol of the ERC20 token. |
| _image | string | Url of the item image. |
| _civilizations | address | Address of the {Civilizations} instance. |

### mintTo

```solidity
function mintTo(bytes _id, uint256 _amount) public
```

Creates tokens to the character composed ID provided.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _id | bytes | Composed ID of the character. |
| _amount | uint256 | Amount of tokens to create. |

### consume

```solidity
function consume(bytes _id, uint256 _amount) public
```

Reduces tokens to the character composed ID provided.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _id | bytes | Composed ID of the character. |
| _amount | uint256 | Amount of tokens to create. |

### wrap

```solidity
function wrap(bytes _id, uint256 _amount) public
```

Converts the internal item to an ERC20 through the {BaseERC20Wrapper}.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _id | bytes | Composed ID of the character. |
| _amount | uint256 | Amount of tokens to create. |

### unwrap

```solidity
function unwrap(bytes _id, uint256 _amount) public
```

Converts the wrapped ERC20 token to an internal fungible item.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _id | bytes | Composed ID of the character. |
| _amount | uint256 | Amount of tokens to create. |

### balanceOf

```solidity
function balanceOf(bytes _id) public view returns (uint256)
```

External function to get the balance of the character composed ID provided.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _id | bytes | Composed ID of the character. |

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | uint256 |

### _mint

```solidity
function _mint(bytes _id, uint256 _amount) internal
```

Internal function to create tokens to the character composed ID provided without
without owner check.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _id | bytes | Composed ID of the character. |
| _amount | uint256 | Amount of tokens to create. |

## BaseGadgetToken

This contract implements an {ERC20Burnable} token to serve as utility tokens that
can be purchased by themselves.

_Implementation of the {IBaseGadgetToken} interface._

### token

```solidity
address token
```

Constant for address of the {ERC20} token used to purchase.

### price

```solidity
uint256 price
```

Constant for the price of each token (in wei).

### constructor

```solidity
constructor(string _name, string _symbol, address _token, uint256 _price) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _name | string | Name of the ERC20 token. |
| _symbol | string | Symbol of the ERC20 token. |
| _token | address | Address of the token used to purchase. |
| _price | uint256 | Price for each token. |

### pause

```solidity
function pause() public
```

Pauses the contract

### unpause

```solidity
function unpause() public
```

Resumes the contract

### setPrice

```solidity
function setPrice(uint256 _price) public
```

Changes the base price for each token.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _price | uint256 | Amount of tokens to charge for each token purchase (in wei). |

### setToken

```solidity
function setToken(address _token) public
```

Changes the token address to charge.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _token | address | Address of the new token to charge. |

### mint

```solidity
function mint(uint256 _amount) public
```

Creates tokens to the {msg.sender} by charging the total amount of tokens.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _amount | uint256 | Amount of tokens to purchase. |

### mintFree

```solidity
function mintFree(address _receiver, uint256 _amount) public
```

Creates tokens to the address provided.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _receiver | address | Address that receives the tokens. |
| _amount | uint256 | Amount of tokens to create. |

### withdraw

```solidity
function withdraw() public
```

Transfers the total amount of tokens stored in the contract to the owner .

### getTotalCost

```solidity
function getTotalCost(uint256 _amount) public view returns (uint256)
```

External function to get the total amount of tokens required to purchase an amount of tokens.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _amount | uint256 | Amount of tokens to purchase. |

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | uint256 |

### decimals

```solidity
function decimals() public view virtual returns (uint8)
```

Overrides the {ERC20.decimals} function to return 0 decimals.

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint8 | uint8 |

## Ard

Implementation of the {BaseERC721} contract for the Ard civilization.

### constructor

```solidity
constructor() public
```

Constructor.

## Hartheim

Implementation of the {BaseERC721} contract for the Hartheim civilization.

### constructor

```solidity
constructor() public
```

Constructor.

## IKarans

Implementation of the {BaseERC721} contract for the I'Karans civilization.

### constructor

```solidity
constructor() public
```

Constructor.

## Shinkari

Implementation of the {BaseERC721} contract for the Shinkari civilization.

### constructor

```solidity
constructor() public
```

Constructor.

## Tarki

Implementation of the {BaseERC721} contract for the Tark'i civilization.

### constructor

```solidity
constructor() public
```

Constructor.

## Zhand

Implementation of the {BaseERC721} contract for the Zhand civilization.

### constructor

```solidity
constructor() public
```

Constructor.

## Levels

This contract is a static storage with utility functions to determine the level
table for the {Experience} contract.

_Implementation of the {ILevels} interface._

### levels

```solidity
mapping(uint256 => struct ILevels.Level) levels
```

Map to track the levels.

### constructor

```solidity
constructor() public
```

Constructor.

_Initializes the lable table._

### getLevel

```solidity
function getLevel(uint256 _experience) public view returns (uint256)
```

External function to return the level number from an experience amount.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _experience | uint256 | Amount of experience to check. |

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | uint256 |

### getExperience

```solidity
function getExperience(uint256 _level) public view returns (uint256)
```

External function to return the total amount of experience required to reach a level.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _level | uint256 | Amount of experience to check. |

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | uint256 | uint256 |

## Civilizations

_`Civilizations` is the contract that stores all the usable civilizations._

### _civilizations

```solidity
mapping(address => uint256) _civilizations
```

_Map to store the civilizations implemented. *_

### civilizations

```solidity
address[] civilizations
```

_Array to store the civilizations implemented. *_

### _minters

```solidity
mapping(address => uint256) _minters
```

_Map to track the amount of tokens minted by address. *_

### token

```solidity
address token
```

_Address of the token used to charge the mint. *_

### _upgrades

```solidity
struct ICivilizations.UpgradedCharacters _upgrades
```

_Map to track the character upgrades. *_

### upgrades

```solidity
mapping(uint256 => struct ICivilizations.Upgrade) upgrades
```

_Map to track upgrades information. *_

### constructor

```solidity
constructor(address _token) public
```

_Constructor._

| Name | Type | Description |
| ---- | ---- | ----------- |
| _token | address | Address of the token to charge for upgrades. |

### pause

```solidity
function pause() public
```

_Pauses the contract_

### unpause

```solidity
function unpause() public
```

_Resumes the contract_

### setInitializeUpgrade

```solidity
function setInitializeUpgrade(uint256 upgrade, bool available) public
```

_Marks an upgrade to available.
 @param upgrade      The ID of the upgrade.
 @param available    The availability of the upgrade._

### setUpgradePrice

```solidity
function setUpgradePrice(uint256 upgrade, uint256 price) public
```

_Adds a civilization to the contract.
 @param upgrade  The ID of the upgrade.
 @param price    The amount of tokens to charge for the upgrade._

### setToken

```solidity
function setToken(address _token) public
```

_Sets the `token` address.
 @param _token   address of the token to use for charge._

### addCivilization

```solidity
function addCivilization(address _instance) public
```

_Adds a civilization to the contract.
 @param _instance  Address of the `BaseERC721` instance._

### mint

```solidity
function mint(address _instance) public
```

_Mints a token.
 @param _instance  Address of the `BaseERC721` instance to mint._

### buyUpgrade

```solidity
function buyUpgrade(bytes id, uint256 upgrade) public
```

_Marks a character upgrade as purchased._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the token. |
| upgrade | uint256 | Upgrade id. |

### withdraw

```solidity
function withdraw() public
```

_Transfers the total amount of `token` stored in the contract to `owner`._

### getID

```solidity
function getID(address _instance) public view returns (uint256)
```

_Returns the internal ID for a civilization.
 @param _instance  Address of the `BaseERC721` instance._

### getTokenUpgrades

```solidity
function getTokenUpgrades(bytes id) public view returns (struct ICivilizations.TokenUpgrades)
```

_Returns the upgrades for a composed ID.
 @param id Composed token id._

### getUpgradeInformation

```solidity
function getUpgradeInformation(uint256 upgrade) public view returns (struct ICivilizations.Upgrade)
```

_Returns the upgrades for a composed ID.
 @param upgrade ID of the upgrade._

### getCivilizations

```solidity
function getCivilizations() public view returns (address[])
```

_Returns the list of civilizations._

### getTokenID

```solidity
function getTokenID(address _instance, uint256 _id) public view returns (bytes)
```

_Returns a composed ID from a collection.
 @param _instance  Address of the `BaseERC721` instance.
 @param _id        The ID of the token inside the collection._

### isAllowed

```solidity
function isAllowed(address spender, bytes _id) public view returns (bool)
```

_Function to check if an address has allowance to a composed ID.
 @param spender   Address to check ownership or allowance.
 @param _id       Composed token id._

### exists

```solidity
function exists(bytes _id) public view returns (bool)
```

_Function to check if a composed ID is already minted.
 @param _id Composed token id._

### ownerOf

```solidity
function ownerOf(bytes _id) public view returns (address)
```

_Function to returns the actual owner of a composed ID.
 @param _id Composed token id._

### _addMinted

```solidity
function _addMinted(address _minter) internal
```

_Adds a mint to the mint counter for the address._

| Name | Type | Description |
| ---- | ---- | ----------- |
| _minter | address | The minter address. |

### _canMint

```solidity
function _canMint(address _minter) internal view returns (bool)
```

_Checks if an address is able to mint more tokens._

| Name | Type | Description |
| ---- | ---- | ----------- |
| _minter | address | The minter address. |

### _decomposeTokenID

```solidity
function _decomposeTokenID(bytes id) internal pure returns (uint256, uint256)
```

_Decompose a byte encoded token ID._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | The composed token id. |

## Craft

_`Craft` is the contract to manage item crafting for Arising._

### recipes

```solidity
mapping(uint256 => struct ICraft.Recipe) recipes
```

_Map to track available recipes on the forge. *_

### _recipes

```solidity
uint256[] _recipes
```

_Array to track all the recipes ids. *_

### gold

```solidity
address gold
```

_The address of the `Gold` instance. *_

### civilizations

```solidity
address civilizations
```

_Address of the `Civilizations` instance. *_

### experience

```solidity
address experience
```

_Address of the `Experience` instance. *_

### stats

```solidity
address stats
```

_Address of the `Stats` instance. *_

### items

```solidity
address items
```

_Address of the `Items` instance. *_

### slots

```solidity
mapping(bytes => struct ICraft.CraftSlot) slots
```

_Map to track craft slots and cooldowns for each character. *_

### onlyAllowed

```solidity
modifier onlyAllowed(bytes id)
```

_Checks if `msg.sender` is owner or allowed to manipulate a composed ID._

### constructor

```solidity
constructor(address _civilizations, address _experience, address _stats, address _gold, address _items) public
```

_Constructor._

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | The address of the `Civilizations` instance. |
| _experience | address | The address of the `Experience` instance. |
| _stats | address | The address of the `Experience` instance. |
| _gold | address | The address of the `Gold` instance. |
| _items | address | The address of the `Items` instance. |

### pause

```solidity
function pause() public
```

_Pauses the contract_

### unpause

```solidity
function unpause() public
```

_Resumes the contract_

### disableRecipe

```solidity
function disableRecipe(uint256 id) public
```

_Disables a craft recipe._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | uint256 | ID of the recipe. |

### enableRecipe

```solidity
function enableRecipe(uint256 id) public
```

_Enables a craft recipe._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | uint256 | ID of the recipe. |

### craft

```solidity
function craft(bytes id, uint256 recipe) public
```

_Craft a recipe._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |
| recipe | uint256 | ID of the recipe to craft. |

### claim

```solidity
function claim(bytes id) public
```

_Claims a recipe already crafted._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |

### getRecipe

```solidity
function getRecipe(uint256 id) public view returns (struct ICraft.Recipe)
```

_Reurns the recipe information of a recipe id._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | uint256 | ID of the recipe. |

### getCharacterSlot

```solidity
function getCharacterSlot(bytes id) public view returns (struct ICraft.CraftSlot)
```

_Reurns the craft slot information of a composed ID._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |

### _isSlotAvailable

```solidity
function _isSlotAvailable(bytes id) internal view returns (bool)
```

_Internal function to check if the craft slot is available to craft._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |

### _isSlotClaimable

```solidity
function _isSlotClaimable(bytes id) internal view returns (bool)
```

_Internal function to check if a craft slot is ready to claim._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |

### _assignRecipeToSlot

```solidity
function _assignRecipeToSlot(bytes id, struct ICraft.Recipe r) internal
```

_Internal function to assign a recipe to a slot for craft_

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |
| r | struct ICraft.Recipe | Recipe to be assigned. |

### _claim

```solidity
function _claim(bytes id) internal returns (uint256)
```

_Internal function claim a reward from the slot and return the reward experience.
     This function assumes the slot is claimable_

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |

## Equipment

_`Equipment` is the contract to equip gear for Arising characters._

### civilizations

```solidity
address civilizations
```

_Address of the `Civilizations` instance. *_

### experience

```solidity
address experience
```

_Address of the `Experience` instance. *_

### items

```solidity
address items
```

_Address of the `Items` instance. *_

### character_equipments

```solidity
mapping(bytes => mapping(enum IEquipment.EquipmentSlot => struct IEquipment.ItemEquiped)) character_equipments
```

_Map to track the equipment of characters. *_

### slots_types

```solidity
mapping(enum IEquipment.EquipmentSlot => mapping(enum IItems.ItemType => bool)) slots_types
```

_Map to track the slots that can be used by an item type. *_

### onlyAllowed

```solidity
modifier onlyAllowed(bytes id)
```

_Checks if `msg.sender` is owner or allowed to manipulate a composed ID._

### constructor

```solidity
constructor(address _civilizations, address _experience, address _items) public
```

_Constructor._

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | The address of the `Civilizations` instance. |
| _experience | address | The address of the `Experience` instance. |
| _items | address | The address of the `Items` instance. |

### pause

```solidity
function pause() public
```

_Pauses the contract_

### unpause

```solidity
function unpause() public
```

_Resumes the contract_

### equip

```solidity
function equip(bytes id, enum IEquipment.EquipmentSlot item_slot, uint256 item_id) public
```

_Assigns an item to an item slot. If it is already used, it replaces it._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |
| item_slot | enum IEquipment.EquipmentSlot | Slot from the equipment to remove. |
| item_id | uint256 | ID of the item to assign. |

### unequip

```solidity
function unequip(bytes id, enum IEquipment.EquipmentSlot item_slot) public
```

_Removes an item from the character equipment and transfers de ERC1155 token._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |
| item_slot | enum IEquipment.EquipmentSlot | Slot from the equipment to remove. |

### getCharacterEquipment

```solidity
function getCharacterEquipment(bytes id) public view returns (struct IEquipment.CharacterEquipment)
```

_Returns the full requipment of a character.
 @param id  Composed ID of the token._

### getCharacterTotalStatsModifiers

```solidity
function getCharacterTotalStatsModifiers(bytes id) public view returns (struct IStats.BasicStats, struct IStats.BasicStats)
```

_Returns the total modifiers from the equipment.
 @param id  Composed ID of the token._

### getCharacterTotalAttributes

```solidity
function getCharacterTotalAttributes(bytes id) public view returns (struct IItems.BaseAttributes, struct IItems.BaseAttributes)
```

_Returns the total attributes from the equipment.
 @param id  Composed ID of the token._

## Experience

_`Experience` is the contract to manage the storage of experience and missions from all the civilizations._

### experience

```solidity
mapping(bytes => uint256) experience
```

_Map to store the experience from composed ID. *_

### levels

```solidity
address levels
```

_Address of the `Levels` implementation. *_

### civilizations

```solidity
address civilizations
```

_Address of the `Civilizations` implementation. *_

### authorized

```solidity
mapping(address => bool) authorized
```

_Map to store the list of authorized addresses to assign experience. *_

### onlyAuthorized

```solidity
modifier onlyAuthorized()
```

_Checks if `msg.sender` is authorized to assign experience._

### constructor

```solidity
constructor(address _levels, address _civilizations) public
```

_Constructor._

| Name | Type | Description |
| ---- | ---- | ----------- |
| _levels | address | The address of the `Levels` instance. |
| _civilizations | address | The address of the `Civilizations` instance. |

### assignExperience

```solidity
function assignExperience(bytes id, uint256 amount) public
```

_Adds experience to the character from a composed ID.
 @param id   Composed ID of the token._

### addAuthority

```solidity
function addAuthority(address authority) public
```

_Adds an authority to assign experience.
 @param authority   Address of the authority to assign._

### removeAuthority

```solidity
function removeAuthority(address authority) public
```

_Removes an authority to assign experience.
 @param authority   Address of the authority to remove._

### getExperience

```solidity
function getExperience(bytes id) public view returns (uint256)
```

_Returns the experience points of the token from a composed ID.
 @param id   Composed ID of the token._

### getLevel

```solidity
function getLevel(bytes id) public view returns (uint256)
```

_Returns the level of a token from a composed ID.
 @param id   Composed ID of the token._

### getExperienceForNextLevel

```solidity
function getExperienceForNextLevel(bytes id) public view returns (uint256)
```

_Returns the amount of experience required to reach the next level.
 @param id   Composed ID of the token._

## Forge

_`Forge` is a contract to convert the raw material to craftable pieces._

### civilizations

```solidity
address civilizations
```

_Address of the `Civilizations` instance. *_

### experience

```solidity
address experience
```

_Address of the `Experience` instance. *_

### stats

```solidity
address stats
```

_Address of the `Stats` instance. *_

### recipes

```solidity
mapping(uint256 => struct IForge.Recipe) recipes
```

_Map to track available recipes on the forge. *_

### _recipes

```solidity
uint256[] _recipes
```

_Array to track all the recipes ids. *_

### forges

```solidity
mapping(bytes => struct IForge.Forges) forges
```

_Map to track forges and cooldowns for each character. *_

### token

```solidity
address token
```

_Address of the token used to charge the mint. *_

### price

```solidity
uint256 price
```

_Price of forge upgrades. *_

### gold

```solidity
address gold
```

_The address of the `Gold` instance. *_

### onlyAllowed

```solidity
modifier onlyAllowed(bytes id)
```

_Checks if `msg.sender` is owner or allowed to manipulate a composed ID._

### constructor

```solidity
constructor(address _civilizations, address _experience, address _stats, address _gold, address _token, uint256 _price) public
```

_Constructor._

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | The address of the `Civilizations` instance. |
| _experience | address | The address of the `Experience` instance. |
| _stats | address | The address of the `Experience` instance. |
| _gold | address | The address of the `Gold` instance. |
| _token | address | Address of the token to charge for forge upgrades. |
| _price | uint256 | Price of forge upgrades. |

### pause

```solidity
function pause() public
```

_Pauses the contract_

### unpause

```solidity
function unpause() public
```

_Resumes the contract_

### disableRecipe

```solidity
function disableRecipe(uint256 id) public
```

_Disables a forge recipe._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | uint256 | ID of the recipe. |

### enableRecipe

```solidity
function enableRecipe(uint256 id) public
```

_Enables a forge recipe._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | uint256 | ID of the recipe. |

### addRecipe

```solidity
function addRecipe(address[] materials, uint256[] amounts, struct IStats.BasicStats stats, uint256 cooldown, uint256 level_required, uint256 cost, uint256 experience_reward, address reward) public
```

_Adds a new recipe to the forge._

| Name | Type | Description |
| ---- | ---- | ----------- |
| materials | address[] | Addresses of the raw resources for the creation. |
| amounts | uint256[] | Amounts for each raw resource. |
| stats | struct IStats.BasicStats | Stat cost for the recipe. |
| cooldown | uint256 | Cooldown in seconds for the recipe. |
| level_required | uint256 | Minimum level required. |
| cost | uint256 | Gold cost of the recipe. |
| experience_reward | uint256 | Amount of experience rewarded. |
| reward | address | Address of the reward contract. |

### buyUpgrade

```solidity
function buyUpgrade(bytes id) public
```

_Upgrades character to use another forge slot._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the token. |

### forge

```solidity
function forge(bytes id, uint256 recipe, uint256 _forge) public
```

_Forges a recipe._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |
| recipe | uint256 | ID of the recipe to forge. |
| _forge | uint256 | Number of the forge to use. |

### claim

```solidity
function claim(bytes id, uint256 _forge) public
```

_Claims a recipe already forged._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |
| _forge | uint256 | Number of the forge to use. |

### withdraw

```solidity
function withdraw() public
```

_Transfers the total amount of `token` stored in the contract to `owner`._

### getRecipe

```solidity
function getRecipe(uint256 id) public view returns (struct IForge.Recipe)
```

_Reurns the recipe information of a recipe id._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | uint256 | ID of the recipe. |

### getCharacterForge

```solidity
function getCharacterForge(bytes id, uint256 _forge) public view returns (struct IForge.Forge)
```

_Reurns the forge information of a composed ID._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |
| _forge | uint256 | ID of the forge. |

### getCharacterForgesUpgrades

```solidity
function getCharacterForgesUpgrades(bytes id) public view returns (bool[3])
```

_Reurns an array of booleans for the character forges upgraded._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |

### getCharacterForgesAvailability

```solidity
function getCharacterForgesAvailability(bytes id) public view returns (bool[3])
```

_Reurns an array of booleans for the character forges available to use._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |

### _isForgeAvailable

```solidity
function _isForgeAvailable(bytes id, uint256 _forge) internal view returns (bool)
```

_Internal function to check if a forge id is available to use._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |
| _forge | uint256 | Number of the forge to use. |

### _isForgeClaimable

```solidity
function _isForgeClaimable(bytes id, uint256 _forge) internal view returns (bool)
```

_Internal function to check if a forge id is ready to claim._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |
| _forge | uint256 | Number of the forge to use. |

### _assignRecipeToForge

```solidity
function _assignRecipeToForge(bytes id, uint256 _forge, struct IForge.Recipe r) internal
```

_Internal function to assign a recipe to a forge to create. This function assumes the forge trying to accessing
     is available (upgraded) and usable._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |
| _forge | uint256 | Number of the forge to use. |
| r | struct IForge.Recipe | Recipe to be assigned. |

### _claimForge

```solidity
function _claimForge(bytes id, uint256 _forge) internal returns (uint256)
```

_Internal function claim a reward from a forge._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |
| _forge | uint256 | Number of the forge to use. |

### _getForgeFromID

```solidity
function _getForgeFromID(bytes id, uint256 _forge) internal view returns (bool, struct IForge.Forge)
```

_Internal function to return a forge instance from a number._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the character. |
| _forge | uint256 | Number of the forge to use. |

## Names

_`Names` is a contract manage the names of Arising characters.
      Some checks are based on the original Rarity names contract https://github.com/rarity-adventure/rarity-names/blob/main/contracts/rarity_names.sol
      created by https://twitter.com/mat_nadler._

### civilizations

```solidity
address civilizations
```

_Address of the `Civilizations` implementation. *_

### experience

```solidity
address experience
```

_Address of the `Experience` implementation. *_

### names

```solidity
mapping(bytes => string) names
```

_Map storing the names for each character. *_

### claimed_names

```solidity
mapping(string => bool) claimed_names
```

_Names claimed. *_

### onlyAllowed

```solidity
modifier onlyAllowed(bytes id)
```

_Checks if `msg.sender` is owner or allowed to manipulate a composed ID._

### constructor

```solidity
constructor(address _civilizations, address _experience) public
```

_Constructor._

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | The address of the `Civilizations` instance. |
| _experience | address | The address of the `Experience` instance. |

### pause

```solidity
function pause() public
```

_Pauses the contract_

### unpause

```solidity
function unpause() public
```

_Resumes the contract_

### claimName

```solidity
function claimName(bytes id, string name) public
```

_Assigns a name to a character and stores to prevent duplicates.
 @param id         Composed ID of the token._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes |  |
| name | string | Name to claim. |

### replaceName

```solidity
function replaceName(bytes id, string newName) public
```

_Changes a name for a character.
 @param id    Composed ID of the token._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes |  |
| newName | string | Name to replace with. |

### clearName

```solidity
function clearName(bytes id) public
```

_Removes the name of the character.
 @param id    Composed ID of the token._

### getTokenName

```solidity
function getTokenName(bytes id) public view returns (string)
```

_Returns the name of the composed ID._

| Name | Type | Description |
| ---- | ---- | ----------- |
| id | bytes | Composed ID of the token. |

### isNameAvailable

```solidity
function isNameAvailable(string str) public view returns (bool)
```

_Checks if a given name is available to use._

| Name | Type | Description |
| ---- | ---- | ----------- |
| str | string | String to checked. |

### isNameValid

```solidity
function isNameValid(string str) public pure returns (bool)
```

_Checks if a given name is valid (Alphanumeric and spaces without leading or trailing space)._

| Name | Type | Description |
| ---- | ---- | ----------- |
| str | string | String to checked. |

### toLowerCase

```solidity
function toLowerCase(string str) public pure returns (string)
```

_Converts a string to lowercase._

| Name | Type | Description |
| ---- | ---- | ----------- |
| str | string | String to convert. |

## Quests

_`Quests` is a contract to manage the different missions characters can do._

### civilizations

```solidity
address civilizations
```

_Address of the `Civilizations` instance. *_

### experience

```solidity
address experience
```

_Address of the `Experience` instance. *_

### constructor

```solidity
constructor(address _civilizations, address _experience) public
```

_Constructor._

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | The address of the `Civilizations` instance. |
| _experience | address | The address of the `Experience` instance. |

### pause

```solidity
function pause() public
```

_Pauses the contract_

### unpause

```solidity
function unpause() public
```

_Resumes the contract_

## Stats

_`Stats` is a contract to manage the stats points and pools for a set of collections.
      The stats and the concept is created and modified based on the Cypher System for role playing games: http://cypher-system.com/._

### REFRESH_COOLDOWN_SECONDS

```solidity
uint256 REFRESH_COOLDOWN_SECONDS
```

_Amount of seconds for refresh cooldown.  *_

### base

```solidity
mapping(bytes => struct IStats.BasicStats) base
```

_Map to store the base stats from composed IDs. *_

### pool

```solidity
mapping(bytes => struct IStats.BasicStats) pool
```

_Map to store the pool stats from composed ID. *_

### last_refresh

```solidity
mapping(bytes => uint256) last_refresh
```

_Map to store the the last refresh from composed ID. *_

### refresher

```solidity
address refresher
```

_Implementation of the `Refresher` *_

### vitalizer

```solidity
address vitalizer
```

_Implementation of the `Vitalizer` *_

### civilizations

```solidity
address civilizations
```

_Address of the `Civilizations` instance. *_

### experience

```solidity
address experience
```

_Address of the `Experience` instance. *_

### sacrifices

```solidity
mapping(bytes => uint256) sacrifices
```

_Map to track the amount of points sacrificed by a character. *_

### refresher_usage_time

```solidity
mapping(bytes => uint256) refresher_usage_time
```

_Map to track the first refresher usage timestamp. *_

### onlyAllowed

```solidity
modifier onlyAllowed(bytes id)
```

_Checks if `msg.sender` is owner or allowed to manipulate a composed ID._

### constructor

```solidity
constructor(address _civilizations, address _experience) public
```

_Constructor._

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | The address of the `Civilizations` instance. |
| _experience | address | The address of the `Experience` instance. |

### pause

```solidity
function pause() public
```

_Pauses the contract_

### unpause

```solidity
function unpause() public
```

_Resumes the contract_

### setRefreshToken

```solidity
function setRefreshToken(address _token) public
```

_Sets the `Refresher` instance.
 @param _token   address of the `Refresher` instance._

### setVitalizerToken

```solidity
function setVitalizerToken(address _token) public
```

_Sets the `Vitalizer` instance.
 @param _token   address of the `Vitalizer` instance._

### consume

```solidity
function consume(bytes id, struct IStats.BasicStats stats) public
```

_Reduces stats points from the pool.
 @param id         Composed ID of the token.
 @param stats      Amount of points reducing._

### sacrifice

```solidity
function sacrifice(bytes id, struct IStats.BasicStats stats) public
```

_Reduces points to the base stats forever.
 @param id         Composed ID of the token.
 @param stats      Amount of points sacrificing._

### refresh

```solidity
function refresh(bytes id) public
```

_Performs a refresh filling the pool stats from the base stats.
 @param id   Composed ID of the token._

### refreshWithToken

```solidity
function refreshWithToken(bytes id) public
```

_Performs a refresh filling the pool stats from the base stats without cooldown spending `RefreshToken` (max 20 points per stat).
 @param id   Composed ID of the token._

### consumeVitalizer

```solidity
function consumeVitalizer(bytes id, struct IStats.BasicStats stats) public
```

_Consumes a vitalizer token to increase one point of a base stat.
 @param id         Composed ID of the token.
 @param stats      Amount of points increasing._

### assignPoints

```solidity
function assignPoints(bytes id, struct IStats.BasicStats stats) public
```

_Assigns the points to the base pool.
 @param id         Composed ID of the token.
 @param stats     Amount of points to assign._

### getBaseStats

```solidity
function getBaseStats(bytes id) public view returns (struct IStats.BasicStats)
```

_Returns the base stats of the composed ID.
 @param id   Composed ID of the token._

### getPoolStats

```solidity
function getPoolStats(bytes id) public view returns (struct IStats.BasicStats)
```

_Returns the available pool stats of the composed ID.
 @param id   Composed ID of the token._

### getAvailablePoints

```solidity
function getAvailablePoints(bytes id) public view returns (uint256)
```

_Returns the amount of points available to assign.
 @param id   Composed ID of the token._

### getNextRefreshTime

```solidity
function getNextRefreshTime(bytes id) public view returns (uint256)
```

_Returns the time for the next free refresh.
 @param id   Composed ID of the token._

### getNextRefreshWithTokenTime

```solidity
function getNextRefreshWithTokenTime(bytes id) public view returns (uint256)
```

_Returns the time of the refresh with tokens reset.
 @param id   Composed ID of the token._

### _assignablePointsByLevel

```solidity
function _assignablePointsByLevel(uint256 level) internal pure returns (uint256)
```

_Returns the amount of total asignable points by level.
 @param level   Level number to check points._

## Gold

This contract is an instance of {BaseFungibleItem} to serve as the main currency of the whole internal ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Refresher

This contract is an instance of {BaseGadgetToken} to perform paid refreshes for the {Stats} contract.

### constructor

```solidity
constructor(address _token, uint256 _price) public
```

_Constructor.

Requirements:_

| Name | Type | Description |
| ---- | ---- | ----------- |
| _token | address | Address of the token used to purchase. |
| _price | uint256 | Price for each token. |

## Vitalizer

This contract is an instance of {BaseGadgetToken} to reclaim sacrificed points on the {Stats} contract.

### constructor

```solidity
constructor(address _token, uint256 _price) public
```

_Constructor.

Requirements:_

| Name | Type | Description |
| ---- | ---- | ----------- |
| _token | address | Address of the token used to purchase. |
| _price | uint256 | Price for each token. |

## IBaseERC20Wrapper

Interface for the {BaseERC20Wrapper} contract.

### mint

```solidity
function mint(address _to, uint256 _amount) external
```

See {BaseERC20Wrapper.mint}

## IBaseERC721

Interface for the {BaseERC721} contract.

### mint

```solidity
function mint(address _to) external
```

See {BaseERC721.mint}

### isApprovedOrOwner

```solidity
function isApprovedOrOwner(address _spender, uint256 _id) external view returns (bool)
```

See {BaseERC721.isApprovedOrOwner}

### exists

```solidity
function exists(uint256 _id) external view returns (bool)
```

See {BaseERC721.exists}

## IBaseFungibleItem

Interface for the {BaseFungibleItem} contract.

### mintTo

```solidity
function mintTo(bytes _id, uint256 _amount) external
```

See {BaseFungibleItem.mintTo}

### consume

```solidity
function consume(bytes _id, uint256 _amount) external
```

See {BaseFungibleItem.consume}

### wrap

```solidity
function wrap(bytes _id, uint256 _amount) external
```

See {BaseFungibleItem.wrap}

### unwrap

```solidity
function unwrap(bytes _id, uint256 _amount) external
```

See {BaseFungibleItem.unwrap}

### balanceOf

```solidity
function balanceOf(bytes _id) external view returns (uint256)
```

See {BaseFungibleItem.balanceOf}

## IBaseGadgetToken

Interface for the {BaseGadgetToken} contract.

### pause

```solidity
function pause() external
```

See {BaseGadgetToken.pause}

### unpause

```solidity
function unpause() external
```

See {BaseGadgetToken.unpause}

### setPrice

```solidity
function setPrice(uint256 _price) external
```

See {BaseGadgetToken.setPrice}

### setToken

```solidity
function setToken(address _token) external
```

See {BaseGadgetToken.setToken}

### mint

```solidity
function mint(uint256 _amount) external
```

See {BaseGadgetToken.mint}

### mintFree

```solidity
function mintFree(address _receiver, uint256 _amount) external
```

See {BaseGadgetToken.mintFree}

### withdraw

```solidity
function withdraw() external
```

See {BaseGadgetToken.withdraw}

### getTotalCost

```solidity
function getTotalCost(uint256 _amount) external returns (uint256)
```

See {BaseGadgetToken.getTotalCost}

## ICivilizations

Interface for the {Civilizations} contract.

### Upgrade

```solidity
struct Upgrade {
  uint256 price;
  bool available;
}
```

### UpgradedCharacters

```solidity
struct UpgradedCharacters {
  mapping(bytes => bool) upgrade_1;
  mapping(bytes => bool) upgrade_2;
  mapping(bytes => bool) upgrade_3;
}
```

### TokenUpgrades

```solidity
struct TokenUpgrades {
  bool upgrade_1;
  bool upgrade_2;
  bool upgrade_3;
}
```

### setInitializeUpgrade

```solidity
function setInitializeUpgrade(uint256 upgrade, bool available) external
```

### setUpgradePrice

```solidity
function setUpgradePrice(uint256 upgrade, uint256 price) external
```

### setToken

```solidity
function setToken(address _token) external
```

### addCivilization

```solidity
function addCivilization(address _instance) external
```

### mint

```solidity
function mint(address _instance) external
```

### buyUpgrade

```solidity
function buyUpgrade(bytes id, uint256 upgrade) external
```

### withdraw

```solidity
function withdraw() external
```

### getID

```solidity
function getID(address _instance) external view returns (uint256)
```

### getTokenUpgrades

```solidity
function getTokenUpgrades(bytes id) external view returns (struct ICivilizations.TokenUpgrades)
```

### getUpgradeInformation

```solidity
function getUpgradeInformation(uint256 upgrade) external view returns (struct ICivilizations.Upgrade)
```

### getCivilizations

```solidity
function getCivilizations() external view returns (address[])
```

### getTokenID

```solidity
function getTokenID(address _instance, uint256 _id) external view returns (bytes)
```

### isAllowed

```solidity
function isAllowed(address spender, bytes _id) external view returns (bool)
```

### exists

```solidity
function exists(bytes _id) external view returns (bool)
```

### ownerOf

```solidity
function ownerOf(bytes _id) external view returns (address)
```

## ICraft

Interface for the {Craft} contract.

### Recipe

```solidity
struct Recipe {
  uint256 id;
  address[] materials;
  uint256[] material_amounts;
  struct IStats.BasicStats stats_required;
  uint256 cooldown;
  uint256 level_required;
  uint256 experience_reward;
  uint256 cost;
  bool available;
  uint256 item_reward;
}
```

### CraftSlot

```solidity
struct CraftSlot {
  uint256 cooldown;
  uint256 last_recipe;
  bool claimed;
}
```

## IEquipment

Interface for the {Equipment} contract.

### ItemEquiped

```solidity
struct ItemEquiped {
  uint256 id;
  bool equiped;
}
```

### EquipmentSlot

```solidity
enum EquipmentSlot {
  HELMET,
  SHOULDER_GUARDS,
  ARM_GUARDS,
  HANDS,
  RING,
  NECKLACE,
  CHEST,
  LEGS,
  BELT,
  FEET,
  CAPE,
  LEFT_HAND,
  RIGHT_HAND
}
```

### CharacterEquipment

```solidity
struct CharacterEquipment {
  struct IEquipment.ItemEquiped helmet;
  struct IEquipment.ItemEquiped shoulder_guards;
  struct IEquipment.ItemEquiped arm_guards;
  struct IEquipment.ItemEquiped hands;
  struct IEquipment.ItemEquiped rings;
  struct IEquipment.ItemEquiped necklace;
  struct IEquipment.ItemEquiped chest;
  struct IEquipment.ItemEquiped legs;
  struct IEquipment.ItemEquiped belt;
  struct IEquipment.ItemEquiped feet;
  struct IEquipment.ItemEquiped cape;
  struct IEquipment.ItemEquiped left_hand;
  struct IEquipment.ItemEquiped right_hand;
}
```

## IExperience

Interface for the {Experience} contract.

### assignExperience

```solidity
function assignExperience(bytes id, uint256 amount) external
```

### addAuthority

```solidity
function addAuthority(address authority) external
```

### removeAuthority

```solidity
function removeAuthority(address authority) external
```

### getExperience

```solidity
function getExperience(bytes id) external view returns (uint256)
```

### getLevel

```solidity
function getLevel(bytes id) external view returns (uint256)
```

### getExperienceForNextLevel

```solidity
function getExperienceForNextLevel(bytes id) external view returns (uint256)
```

## IForge

Interface for the {Forge} contract.

### Recipe

```solidity
struct Recipe {
  uint256 id;
  address[] materials;
  uint256[] material_amounts;
  struct IStats.BasicStats stats_required;
  uint256 cooldown;
  uint256 level_required;
  address reward;
  uint256 experience_reward;
  uint256 cost;
  bool available;
}
```

### Forge

```solidity
struct Forge {
  bool available;
  uint256 cooldown;
  uint256 last_recipe;
  bool last_recipe_claimed;
}
```

### Forges

```solidity
struct Forges {
  struct IForge.Forge forge_1;
  struct IForge.Forge forge_2;
  struct IForge.Forge forge_3;
}
```

### disableRecipe

```solidity
function disableRecipe(uint256 id) external
```

### enableRecipe

```solidity
function enableRecipe(uint256 id) external
```

### addRecipe

```solidity
function addRecipe(address[] _materials, uint256[] _amounts, struct IStats.BasicStats stats, uint256 cooldown, uint256 level_required, uint256 cost, uint256 exp_reward, address reward) external
```

### buyUpgrade

```solidity
function buyUpgrade(bytes id) external
```

### forge

```solidity
function forge(bytes id, uint256 recipe, uint256 _forge) external
```

### claim

```solidity
function claim(bytes id, uint256 _forge) external
```

### withdraw

```solidity
function withdraw() external
```

### getRecipe

```solidity
function getRecipe(uint256 id) external view returns (struct IForge.Recipe)
```

### getCharacterForge

```solidity
function getCharacterForge(bytes id, uint256 _forge) external view returns (struct IForge.Forge)
```

### getCharacterForgesUpgrades

```solidity
function getCharacterForgesUpgrades(bytes id) external view returns (bool[3])
```

### getCharacterForgesAvailability

```solidity
function getCharacterForgesAvailability(bytes id) external view returns (bool[3])
```

## IItems

Interface for the {Items} contract.

### Item

```solidity
struct Item {
  uint256 id;
  uint256 level_required;
  enum IItems.ItemType item_type;
  struct IItems.StatsModifiers stat_modifiers;
  struct IItems.Attributes attributes;
  bool available;
}
```

### ItemType

```solidity
enum ItemType {
  HELMET,
  SHOULDER_GUARDS,
  ARM_GUARDS,
  HANDS,
  RING,
  NECKLACE,
  CHEST,
  LEGS,
  BELT,
  FEET,
  CAPE,
  ONE_HANDED,
  TWO_HANDED
}
```

### StatsModifiers

```solidity
struct StatsModifiers {
  uint256 might;
  uint256 might_reducer;
  uint256 speed;
  uint256 speed_reducer;
  uint256 intellect;
  uint256 intellect_reducer;
}
```

### BaseAttributes

```solidity
struct BaseAttributes {
  uint256 atk;
  uint256 def;
  uint256 range;
  uint256 mag_atk;
  uint256 mag_def;
  uint256 rate;
}
```

### Attributes

```solidity
struct Attributes {
  uint256 atk;
  uint256 atk_reducer;
  uint256 def;
  uint256 def_reducer;
  uint256 range;
  uint256 range_reducer;
  uint256 mag_atk;
  uint256 mag_atk_reducer;
  uint256 mag_def;
  uint256 mag_def_reducer;
  uint256 rate;
  uint256 rate_reducer;
}
```

### addItem

```solidity
function addItem(uint256 level_required, enum IItems.ItemType item_type, struct IItems.StatsModifiers stat_modifiers, struct IItems.Attributes attributes) external
```

### disableItem

```solidity
function disableItem(uint256 id) external
```

### enableItem

```solidity
function enableItem(uint256 id) external
```

### getItem

```solidity
function getItem(uint256 id) external view returns (struct IItems.Item)
```

### mint

```solidity
function mint(address to, uint256 id) external
```

## ILevels

Interface for the {Levels} contract.

### Level

```solidity
struct Level {
  uint256 min;
  uint256 max;
}
```

### getLevel

```solidity
function getLevel(uint256 _experience) external view returns (uint256)
```

See {Levels.getLevel}

### getExperience

```solidity
function getExperience(uint256 _level) external view returns (uint256)
```

See {Levels.getExperience}

## INames

Interface for the {Names} contract.

### claimName

```solidity
function claimName(bytes id, string name) external
```

### replaceName

```solidity
function replaceName(bytes id, string newName) external
```

### clearName

```solidity
function clearName(bytes id) external
```

### getTokenName

```solidity
function getTokenName(bytes id) external view returns (string)
```

### isNameAvailable

```solidity
function isNameAvailable(string str) external view returns (bool)
```

### isNameValid

```solidity
function isNameValid(string str) external pure returns (bool)
```

### toLowerCase

```solidity
function toLowerCase(string str) external pure returns (string)
```

## IQuests

Interface for the {Quests} contract.

## IStats

Interface for the {Stats} contract.

### BasicStats

```solidity
struct BasicStats {
  uint256 might;
  uint256 speed;
  uint256 intellect;
}
```

### setRefreshToken

```solidity
function setRefreshToken(address _token) external
```

### setVitalizerToken

```solidity
function setVitalizerToken(address _token) external
```

### consume

```solidity
function consume(bytes id, struct IStats.BasicStats stats) external
```

### sacrifice

```solidity
function sacrifice(bytes id, struct IStats.BasicStats stats) external
```

### refresh

```solidity
function refresh(bytes id) external
```

### refreshWithToken

```solidity
function refreshWithToken(bytes id) external
```

### consumeVitalizer

```solidity
function consumeVitalizer(bytes id, struct IStats.BasicStats stats) external
```

### assignPoints

```solidity
function assignPoints(bytes id, struct IStats.BasicStats stats) external
```

### getBaseStats

```solidity
function getBaseStats(bytes id) external view returns (struct IStats.BasicStats)
```

### getPoolStats

```solidity
function getPoolStats(bytes id) external view returns (struct IStats.BasicStats)
```

### getAvailablePoints

```solidity
function getAvailablePoints(bytes id) external view returns (uint256)
```

### getNextRefreshTime

```solidity
function getNextRefreshTime(bytes id) external view returns (uint256)
```

### getNextRefreshWithTokenTime

```solidity
function getNextRefreshWithTokenTime(bytes id) external view returns (uint256)
```

## Items

This contract is an standard {ERC1155} implementation with internal mappings to store items additional
information for the characters usage.

_Implementation of the {IItems} interface._

### items

```solidity
mapping(uint256 => struct IItems.Item) items
```

Map to track the extra items data.

### _items

```solidity
uint256[] _items
```

Array to track a full list of item IDs.

### constructor

```solidity
constructor() public
```

Constructor.

### mint

```solidity
function mint(address _to, uint256 _id) public
```

Creates tokens to the address provided.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _to | address | Address that receives the tokens. |
| _id | uint256 | ID of the item to be created. |

### addItem

```solidity
function addItem(uint256 _level_required, enum IItems.ItemType _item_type, struct IItems.StatsModifiers _stats_modifiers, struct IItems.Attributes _attributes) public
```

Adds the item data to relate with a specific token ID.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _level_required | uint256 | Minimum level for a character to use the item. |
| _item_type | enum IItems.ItemType | Type of the item defined by the enum {ItemType}. |
| _stats_modifiers | struct IItems.StatsModifiers | Item modifiers for the character stats. |
| _attributes | struct IItems.Attributes | Specific item attributes. |

### disableItem

```solidity
function disableItem(uint256 _id) public
```

Disables an item from beign equiped.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _id | uint256 | ID of the item. |

### enableItem

```solidity
function enableItem(uint256 _id) public
```

Enables an item to be equiped.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _id | uint256 | ID of the item. |

### getItem

```solidity
function getItem(uint256 _id) public view returns (struct IItems.Item)
```

Returns the full information of an Item.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _id | uint256 | ID of the item. |

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0] | struct IItems.Item | IItem.Item |

## AdamantineBar

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## BronzeBar

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## CobaltBar

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## CottonFabric

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## GoldBar

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## HardenedLeather

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## IronBar

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Ironstone

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## PlatinumBar

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## SilkFabric

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## SilverBar

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## SteelBar

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## WoodPlank

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## WoolFabric

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Adamantine

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Bronze

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Coal

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Cobalt

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Cotton

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Iron

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Leather

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Platinum

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Silk

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Silver

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Stone

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Wood

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## Wool

This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.

### constructor

```solidity
constructor(address _civilizations) public
```

Constructor.

Requirements:

| Name | Type | Description |
| ---- | ---- | ----------- |
| _civilizations | address | Address of the {Civilizations} instance. |

## MockMinter

### civilizations

```solidity
address civilizations
```

### constructor

```solidity
constructor(address _civilizations) public
```

### mintMock

```solidity
function mintMock(address _instance) public
```

## MockToken

### constructor

```solidity
constructor(uint256 supply) public
```

