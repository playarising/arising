# Solidity API

## BaseERC20Wrapper

This contract is a standard `ERC20` implementation with burnable and mintable
functions exposed to the contract owner. This contract is a wrapper for the [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instance to convert
an internal fungible token to the `ERC20` standard.

Implementation of the [IBaseERC20Wrapper](/docs/interfaces/IBaseERC20Wrapper.md) interface.

### constructor

```solidity
constructor(string _name, string _symbol) public
```

Constructor.

Requirements:

| Name     | Type   | Description                  |
| -------- | ------ | ---------------------------- |
| \_name   | string | Name of the `ERC20` token.   |
| \_symbol | string | Symbol of the `ERC20` token. |

### mint

```solidity
function mint(address _to, uint256 _amount) public
```

Creates tokens to the address provided.

Requirements:

| Name     | Type    | Description                       |
| -------- | ------- | --------------------------------- |
| \_to     | address | Address that receives the tokens. |
| \_amount | uint256 | Amount to be minted.              |
