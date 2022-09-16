# Solidity API

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

| Name     | Type   | Description                       |
| -------- | ------ | --------------------------------- |
| \_name   | string | Name of the ERC20 token.          |
| \_symbol | string | Symbol of the ERC20 token.        |
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
function isApprovedOrOwner(address spender, uint256 id) public view virtual returns (bool)
```

External function to check if an address is allowed to access a token.

Requirements:

| Name    | Type    | Description                         |
| ------- | ------- | ----------------------------------- |
| spender | address | Address that will access the token. |
| id      | uint256 | ID of the token to be accessed.     |

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0]  | bool | bool        |

### exists

```solidity
function exists(uint256 id) public view returns (bool)
```

External function to check if a token id is minted.

Requirements:

| Name | Type    | Description                    |
| ---- | ------- | ------------------------------ |
| id   | uint256 | ID of the token to be checked. |

| Name | Type | Description |
| ---- | ---- | ----------- |
| [0]  | bool | bool        |

### \_baseURI

```solidity
function _baseURI() internal view virtual returns (string)
```

Internal function that overrides the {ERC721_baseURI} function
with an URI specified over the constructor.

| Name | Type   | Description |
| ---- | ------ | ----------- |
| [0]  | string | string      |
