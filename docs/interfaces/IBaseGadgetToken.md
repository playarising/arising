# Solidity API

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
function getTotalCost(uint256 _amount) external returns (uint256 _cost)
```

See {BaseGadgetToken.getTotalCost}
