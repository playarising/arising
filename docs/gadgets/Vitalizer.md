# Solidity API

## Vitalizer

This contract is an instance of [BaseGadgetToken](/docs/base/BaseGadgetToken.md) to reclaim sacrificed points on the [Stats](/docs/core/Stats.md) contract.

### constructor

```solidity
constructor(address _token, uint256 _price) public
```

Constructor.

Requirements:

| Name    | Type    | Description                            |
| ------- | ------- | -------------------------------------- |
| \_token | address | Address of the token used to purchase. |
| \_price | uint256 | Price for each token.                  |
