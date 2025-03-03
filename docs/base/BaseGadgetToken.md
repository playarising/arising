# Solidity API

## BaseGadgetToken

This contract implements an `ERC20BurnableUpgradeable` token to serve as utility tokens that
can be purchased by themselves.

Implementation of the [IBaseGadgetToken](/docs/interfaces/IBaseGadgetToken.md) interface.

### token

```solidity
address token
```

Constant for address of the `ERC20` token used to purchase.

### price

```solidity
uint256 price
```

Constant for the price of each token (in wei).

### initialize

```solidity
function initialize(string _name, string _symbol, address _token, uint256 _price) public
```

Initialize.

Requirements:

| Name     | Type    | Description                            |
| -------- | ------- | -------------------------------------- |
| \_name   | string  | Name of the `ERC20` token.             |
| \_symbol | string  | Symbol of the `ERC20` token.           |
| \_token  | address | Address of the token used to purchase. |
| \_price  | uint256 | Price for each token.                  |

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

| Name    | Type    | Description                                                  |
| ------- | ------- | ------------------------------------------------------------ |
| \_price | uint256 | Amount of tokens to charge for each token purchase (in wei). |

### setToken

```solidity
function setToken(address _token) public
```

Changes the token address to charge.

Requirements:

| Name    | Type    | Description                         |
| ------- | ------- | ----------------------------------- |
| \_token | address | Address of the new token to charge. |

### mint

```solidity
function mint(uint256 _amount) public
```

Creates tokens to the `msg.sender` by charging the total amount of tokens.

Requirements:

| Name     | Type    | Description                   |
| -------- | ------- | ----------------------------- |
| \_amount | uint256 | Amount of tokens to purchase. |

### mintFree

```solidity
function mintFree(address _receiver, uint256 _amount) public
```

Creates tokens to the address provided.

Requirements:

| Name       | Type    | Description                       |
| ---------- | ------- | --------------------------------- |
| \_receiver | address | Address that receives the tokens. |
| \_amount   | uint256 | Amount of tokens to create.       |

### getTotalCost

```solidity
function getTotalCost(uint256 _amount) public view returns (uint256 _cost)
```

External function to get the total amount of tokens required to purchase an amount of tokens.

Requirements:

| Name     | Type    | Description                   |
| -------- | ------- | ----------------------------- |
| \_amount | uint256 | Amount of tokens to purchase. |

| Name   | Type    | Description                    |
| ------ | ------- | ------------------------------ |
| \_cost | uint256 | Total cost to purchase tokens. |

### decimals

```solidity
function decimals() public view virtual returns (uint8 _decimals)
```

Overrides the `ERC20.decimals` function to return 0 decimals.

| Name       | Type  | Description                      |
| ---------- | ----- | -------------------------------- |
| \_decimals | uint8 | Amount of decimals of the token. |
