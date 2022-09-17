# Solidity API

## BaseERC721

This contract is a `ERC721Enumerable` implementation for the different civilizations.
Exposes the mint function to the owner and some check functions.

_Implementation of the [IBaseERC721](/docs/interfaces/IBaseERC721.md) interface._

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

| Name     | Type   | Description                       |
| -------- | ------ | --------------------------------- |
| \_name   | string | Name of the `ERC721` token.       |
| \_symbol | string | Symbol of the `ERC721` token.     |
| \_uri    | string | Base url for the tokens metadata. |

### mint

```solidity
function mint(address to) public
```

Creates tokens to the address provided.

Requirements:

| Name | Type    | Description                       |
| ---- | ------- | --------------------------------- |
| to   | address | Address that receives the tokens. |

### isApprovedOrOwner

```solidity
function isApprovedOrOwner(address spender, uint256 id) public view virtual returns (bool _approved)
```

External function to check if an address is allowed to access a token.

Requirements:

| Name    | Type    | Description                         |
| ------- | ------- | ----------------------------------- |
| spender | address | Address that will access the token. |
| id      | uint256 | ID of the token to be accessed.     |

| Name       | Type | Description                                                      |
| ---------- | ---- | ---------------------------------------------------------------- |
| \_approved | bool | Boolean to return if the address is allowed to access the token. |

### exists

```solidity
function exists(uint256 id) public view returns (bool _exist)
```

External function to check if a token id is minted.

Requirements:

| Name | Type    | Description                    |
| ---- | ------- | ------------------------------ |
| id   | uint256 | ID of the token to be checked. |

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
