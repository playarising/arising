const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Civilizations", () => {
  before(async () => {
    const [owner] = await ethers.getSigners();

    this.owner = owner;

    const Coal = await ethers.getContractFactory("Coal");
    this.coal = await Coal.deploy();

    const Gold = await ethers.getContractFactory("Gold");
    this.gold = await Gold.deploy();

    const Iron = await ethers.getContractFactory("Iron");
    this.iron = await Iron.deploy();

    const Stone = await ethers.getContractFactory("Stone");
    this.stone = await Stone.deploy();

    const Wood = await ethers.getContractFactory("Wood");
    this.wood = await Wood.deploy();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.coal.name()).to.be.eq("Arising: Coal");
    expect(await this.gold.name()).to.be.eq("Arising: Gold");
    expect(await this.iron.name()).to.be.eq("Arising: Iron");
    expect(await this.stone.name()).to.be.eq("Arising: Stone");
    expect(await this.wood.name()).to.be.eq("Arising: Wood");

    expect(await this.coal.symbol()).to.be.eq("aCOAL");
    expect(await this.gold.symbol()).to.be.eq("aGOLD");
    expect(await this.iron.symbol()).to.be.eq("aIRON");
    expect(await this.stone.symbol()).to.be.eq("aSTONE");
    expect(await this.wood.symbol()).to.be.eq("aWOOD");
  });
});
