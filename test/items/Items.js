const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Items", () => {
  before(async () => {
    const [owner, receiver] = await ethers.getSigners();

    this.owner = owner;
  });

  it("should deploy everything correctly", async () => {});
});
