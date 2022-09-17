# Solidity API

## IBaseGadgetToken

Interface for the [BaseGadgetToken](/docs/base/BaseGadgetToken.md) contract.

### pause

```solidity
function pause() external
```

See [BaseGadgetToken](/docs/base/BaseGadgetToken.md#pause)

### unpause

```solidity
function unpause() external
```

See [BaseGadgetToken](/docs/base/BaseGadgetToken.md#unpause)

### setPrice

```solidity
function setPrice(uint256 _price) external
```

See [BaseGadgetToken](/docs/base/BaseGadgetToken.md#setPrice)

### setToken

```solidity
function setToken(address _token) external
```

See [BaseGadgetToken](/docs/base/BaseGadgetToken.md#setToken)

### mint

```solidity
function mint(uint256 _amount) external
```

See [BaseGadgetToken](/docs/base/BaseGadgetToken.md#mint)

### mintFree

```solidity
function mintFree(address _receiver, uint256 _amount) external
```

See [BaseGadgetToken](/docs/base/BaseGadgetToken.md#mintFree)

### withdraw

```solidity
function withdraw() external
```

See [BaseGadgetToken](/docs/base/BaseGadgetToken.md#withdraw)

### getTotalCost

```solidity
function getTotalCost(uint256 _amount) external returns (uint256 _cost)
```

See [BaseGadgetToken](/docs/base/BaseGadgetToken.md#getTotalCost)
