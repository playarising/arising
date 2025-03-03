# Solidity API

## BaseFungibleItem

This contract an imitation of the `ERC20` standard to work around the character context.
It tracks balances of characters tokens. This also includes functions to wrap and unwrap to a
[BaseERC20Wrapper](/docs/base/BaseERC20Wrapper.md) instance.

Implementation of the [IBaseFungibleItem](/docs/interfaces/IBaseFungibleItem.md) interface.

### name

```solidity
string name
```

Constant for the name of the item.

### symbol

```solidity
string symbol
```

Constant for the symbol of the item.

### civilizations

```solidity
address civilizations
```

Constant for the address of the [Civilizations](/docs/core/Civilizations.md) instance.

### balances

```solidity
mapping(bytes => uint256) balances
```

Map to track the balances of characters.

### wrapper

```solidity
address wrapper
```

Constant for the address of the [BaseERC20Wrapper](/docs/base/BaseERC20Wrapper.md) instance.

### enable_wrap

```solidity
bool enable_wrap
```

Constant to enable/disable the token wrap.

### authorized

```solidity
mapping(address => bool) authorized
```

Map to store the list of authorized addresses to mint items.

### onlyAllowed

```solidity
modifier onlyAllowed(bytes _id)
```

Checks against the [Civilizations](/docs/core/Civilizations.md) instance if the `msg.sender` is the owner or
has allowance to access a composed ID.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

### onlyEnabled

```solidity
modifier onlyEnabled()
```

Checks if the wrap functionality is enabled.

### onlyAuthorized

```solidity
modifier onlyAuthorized()
```

Checks if the `msg.sender` is authorized to mint items.

### initialize

```solidity
function initialize(string _name, string _symbol, address _civilizations) public
```

Initialize.

Requirements:

| Name            | Type    | Description                                                           |
| --------------- | ------- | --------------------------------------------------------------------- |
| \_name          | string  | Name of the `ERC20` token.                                            |
| \_symbol        | string  | Symbol of the `ERC20` token.                                          |
| \_civilizations | address | Address of the [Civilizations](/docs/core/Civilizations.md) instance. |

### addAuthority

```solidity
function addAuthority(address _authority) public
```

Assigns a new address as an authority to mint items.

Requirements:

| Name        | Type    | Description                |
| ----------- | ------- | -------------------------- |
| \_authority | address | Address to give authority. |

### removeAuthority

```solidity
function removeAuthority(address _authority) public
```

Removes an authority to mint items.

Requirements:

| Name        | Type    | Description                |
| ----------- | ------- | -------------------------- |
| \_authority | address | Address to give authority. |

### setWrapFunction

```solidity
function setWrapFunction(bool _enabled) public
```

Enables or disables the wrap function for the token.

Requirements:

| Name      | Type | Description                     |
| --------- | ---- | ------------------------------- |
| \_enabled | bool | Enable or diable wrap function. |

### mintTo

```solidity
function mintTo(bytes _id, uint256 _amount) public
```

Creates tokens to the character composed ID provided.

Requirements:

| Name     | Type    | Description                   |
| -------- | ------- | ----------------------------- |
| \_id     | bytes   | Composed ID of the character. |
| \_amount | uint256 | Amount of tokens to create.   |

### consume

```solidity
function consume(bytes _id, uint256 _amount) public
```

Reduces tokens to the character composed ID provided.

Requirements:

| Name     | Type    | Description                   |
| -------- | ------- | ----------------------------- |
| \_id     | bytes   | Composed ID of the character. |
| \_amount | uint256 | Amount of tokens to create.   |

### wrap

```solidity
function wrap(bytes _id, uint256 _amount) public
```

Converts the internal item to an `ERC20` through the [BaseERC20Wrapper](/docs/base/BaseERC20Wrapper.md).

Requirements:

| Name     | Type    | Description                   |
| -------- | ------- | ----------------------------- |
| \_id     | bytes   | Composed ID of the character. |
| \_amount | uint256 | Amount of tokens to create.   |

### unwrap

```solidity
function unwrap(bytes _id, uint256 _amount) public
```

Converts the wrapped `ERC20` token to an internal fungible item.

Requirements:

| Name     | Type    | Description                   |
| -------- | ------- | ----------------------------- |
| \_id     | bytes   | Composed ID of the character. |
| \_amount | uint256 | Amount of tokens to create.   |

### balanceOf

```solidity
function balanceOf(bytes _id) public view returns (uint256 _balance)
```

External function to get the balance of the character composed ID provided.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name      | Type    | Description                                             |
| --------- | ------- | ------------------------------------------------------- |
| \_balance | uint256 | Amount of tokens of the character from the composed ID. |

### \_mint

```solidity
function _mint(bytes _id, uint256 _amount) internal
```

Internal function to create tokens to the character composed ID provided without
without owner check.

Requirements:

| Name     | Type    | Description                   |
| -------- | ------- | ----------------------------- |
| \_id     | bytes   | Composed ID of the character. |
| \_amount | uint256 | Amount of tokens to create.   |
