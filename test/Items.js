const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Items", () => {
  before(async () => {
    const [owner] = await ethers.getSigners();

    this.owner = owner;

    const Civilizations = await ethers.getContractFactory("Civilizations");
    this.civilizations = await Civilizations.deploy(1, this.owner.address);
    await this.civilizations.deployed();

    const Coal = await ethers.getContractFactory("Coal");
    this.coal = await Coal.deploy(this.civilizations.address);
    await this.coal.deployed();

    const Gold = await ethers.getContractFactory("Gold");
    this.gold = await Gold.deploy(this.civilizations.address);
    await this.gold.deployed();

    const Iron = await ethers.getContractFactory("Iron");
    this.iron = await Iron.deploy(this.civilizations.address);
    await this.iron.deployed();

    const Stone = await ethers.getContractFactory("Stone");
    this.stone = await Stone.deploy(this.civilizations.address);
    await this.stone.deployed();

    const Wood = await ethers.getContractFactory("Wood");
    this.wood = await Wood.deploy(this.civilizations.address);
    await this.wood.deployed();
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
