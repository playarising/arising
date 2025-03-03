# Solidity API

## BaseERC721

This contract is a `ERC721Enumerable` implementation for the different civilizations.
Exposes the mint function to the owner and some check functions.

Implementation of the [IBaseERC721](/docs/interfaces/IBaseERC721.md) interface.

### baseURI

```solidity
string baseURI
```

Constant for the base url of the token metadata.

### civilizations

```solidity
address civilizations
```

Address of the [Civilizations](/docs/core/Civilizations.md) instance.

### authorized

```solidity
mapping(address => bool) authorized
```

Map to store the list of authorized addresses to mint tokens.

### onlyAuthorized

```solidity
modifier onlyAuthorized()
```

Checks against if the `msg.sender` is authorized to mint tokens.

### initialize

```solidity
function initialize(string _name, string _symbol, string _uri, address _civilizations) public
```

Initialize.

Requirements:

| Name            | Type    | Description                                                               |
| --------------- | ------- | ------------------------------------------------------------------------- |
| \_name          | string  | Name of the `ERC721` token.                                               |
| \_symbol        | string  | Symbol of the `ERC721` token.                                             |
| \_uri           | string  | Base url for the tokens metadata.                                         |
| \_civilizations | address | The address of the [Civilizations](/docs/core/Civilizations.md) instance. |

### addAuthority

```solidity
function addAuthority(address _authority) public
```

Assigns a new address as an authority to mint tokens.

Requirements:

| Name        | Type    | Description                |
| ----------- | ------- | -------------------------- |
| \_authority | address | Address to give authority. |

### removeAuthority

```solidity
function removeAuthority(address _authority) public
```

Removes an authority to mint tokens.

Requirements:

| Name        | Type    | Description                |
| ----------- | ------- | -------------------------- |
| \_authority | address | Address to give authority. |

### mint

```solidity
function mint(address _to) public returns (uint256)
```

Creates tokens to the address provided.

Requirements:

| Name | Type    | Description                       |
| ---- | ------- | --------------------------------- |
| \_to | address | Address that receives the tokens. |

| Name | Type    | Description                         |
| ---- | ------- | ----------------------------------- |
| [0]  | uint256 | \_token_id The ID of the new token. |

### isApprovedOrOwner

```solidity
function isApprovedOrOwner(address _spender, uint256 _token_id) public view virtual returns (bool _approved)
```

External function to check if an address is allowed to access a token.

Requirements:

| Name       | Type    | Description                         |
| ---------- | ------- | ----------------------------------- |
| \_spender  | address | Address that will access the token. |
| \_token_id | uint256 | ID of the token to be accessed.     |

| Name       | Type | Description                                                      |
| ---------- | ---- | ---------------------------------------------------------------- |
| \_approved | bool | Boolean to return if the address is allowed to access the token. |

### exists

```solidity
function exists(uint256 _token_id) public view returns (bool _exist)
```

External function to check if a token id is minted.

Requirements:

| Name       | Type    | Description                    |
| ---------- | ------- | ------------------------------ |
| \_token_id | uint256 | ID of the token to be checked. |

| Name    | Type | Description                                      |
| ------- | ---- | ------------------------------------------------ |
| \_exist | bool | Boolean to check if the token is already minted. |

### \_baseURI

```solidity
function _baseURI() internal view virtual returns (string _uri)
```

Internal function that overrides the `ERC721_baseURI` function
with an URI specified over the constructor.

| Name  | Type   | Description                   |
| ----- | ------ | ----------------------------- |
| \_uri | string | Base URL from the constructor |

### \_afterTokenTransfer

```solidity
function _afterTokenTransfer(address from, address to, uint256 tokenId) internal virtual
```

Internal function that overrides the `ERC721_afterTokenTransfer` function to emit the transfer event.
