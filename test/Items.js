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

    const Adamantine = await ethers.getContractFactory("Adamantine");
    this.adamantine = await Adamantine.deploy(this.civilizations.address);
    await this.adamantine.deployed();

    const Bronze = await ethers.getContractFactory("Bronze");
    this.bronze = await Bronze.deploy(this.civilizations.address);
    await this.bronze.deployed();

    const Platinum = await ethers.getContractFactory("Platinum");
    this.platinum = await Platinum.deploy(this.civilizations.address);
    await this.platinum.deployed();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.gold.name()).to.be.eq("Arising: Gold");
    expect(await this.coal.name()).to.be.eq("Arising: Coal");
    expect(await this.iron.name()).to.be.eq("Arising: Iron");
    expect(await this.stone.name()).to.be.eq("Arising: Stone");
    expect(await this.wood.name()).to.be.eq("Arising: Wood");
    expect(await this.adamantine.name()).to.be.eq("Arising: Adamantine");
    expect(await this.bronze.name()).to.be.eq("Arising: Bronze");
    expect(await this.platinum.name()).to.be.eq("Arising: Platinum");

    expect(await this.gold.symbol()).to.be.eq("aGOLD");
    expect(await this.coal.symbol()).to.be.eq("aCOAL");
    expect(await this.iron.symbol()).to.be.eq("aIRON");
    expect(await this.stone.symbol()).to.be.eq("aSTONE");
    expect(await this.wood.symbol()).to.be.eq("aWOOD");
    expect(await this.adamantine.symbol()).to.be.eq("aADAMANTINE");
    expect(await this.bronze.symbol()).to.be.eq("aBRONZE");
    expect(await this.platinum.symbol()).to.be.eq("aPLATINUM");

    expect(await this.gold.image()).to.be.eq(
      "https://playarising.com/gadgets/gold.png"
    );
    expect(await this.coal.image()).to.be.eq(
      "https://playarising.com/gadgets/raw/coal.png"
    );
    expect(await this.iron.image()).to.be.eq(
      "https://playarising.com/gadgets/raw/iron.png"
    );
    expect(await this.stone.image()).to.be.eq(
      "https://playarising.com/gadgets/raw/stone.png"
    );
    expect(await this.wood.image()).to.be.eq(
      "https://playarising.com/gadgets/raw/wood.png"
    );
    expect(await this.adamantine.image()).to.be.eq(
      "https://playarising.com/gadgets/raw/adamantine.png"
    );
    expect(await this.bronze.image()).to.be.eq(
      "https://playarising.com/gadgets/raw/bronze.png"
    );
    expect(await this.platinum.image()).to.be.eq(
      "https://playarising.com/gadgets/raw/platinum.png"
    );
  });
});
