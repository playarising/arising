const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Materials", () => {
  before(async () => {
    const [owner] = await ethers.getSigners();

    this.owner = owner;

    const Civilizations = await ethers.getContractFactory("Civilizations");
    this.civilizations = await Civilizations.deploy(this.owner.address);
    await this.civilizations.deployed();

    const Gold = await ethers.getContractFactory("Gold");
    this.gold = await Gold.deploy(this.civilizations.address);
    await this.gold.deployed();

    const Adamantine = await ethers.getContractFactory("Adamantine");
    this.adamantine = await Adamantine.deploy(this.civilizations.address);
    await this.adamantine.deployed();

    const Bronze = await ethers.getContractFactory("Bronze");
    this.bronze = await Bronze.deploy(this.civilizations.address);
    await this.bronze.deployed();

    const Coal = await ethers.getContractFactory("Coal");
    this.coal = await Coal.deploy(this.civilizations.address);
    await this.coal.deployed();

    const Cobalt = await ethers.getContractFactory("Cobalt");
    this.cobalt = await Cobalt.deploy(this.civilizations.address);
    await this.cobalt.deployed();

    const Iron = await ethers.getContractFactory("Iron");
    this.iron = await Iron.deploy(this.civilizations.address);
    await this.iron.deployed();

    const Platinum = await ethers.getContractFactory("Platinum");
    this.platinum = await Platinum.deploy(this.civilizations.address);
    await this.platinum.deployed();

    const Silver = await ethers.getContractFactory("Silver");
    this.silver = await Silver.deploy(this.civilizations.address);
    await this.silver.deployed();

    const Stone = await ethers.getContractFactory("Stone");
    this.stone = await Stone.deploy(this.civilizations.address);
    await this.stone.deployed();

    const Wood = await ethers.getContractFactory("Wood");
    this.wood = await Wood.deploy(this.civilizations.address);
    await this.wood.deployed();

    const AdamantineBar = await ethers.getContractFactory("AdamantineBar");
    this.adamantine_bar = await AdamantineBar.deploy(
      this.civilizations.address
    );
    await this.adamantine_bar.deployed();

    const BronzeBar = await ethers.getContractFactory("BronzeBar");
    this.bronze_bar = await BronzeBar.deploy(this.civilizations.address);
    await this.bronze_bar.deployed();

    const CobaltBar = await ethers.getContractFactory("CobaltBar");
    this.cobalt_bar = await CobaltBar.deploy(this.civilizations.address);
    await this.cobalt_bar.deployed();

    const GoldBar = await ethers.getContractFactory("GoldBar");
    this.gold_bar = await GoldBar.deploy(this.civilizations.address);
    await this.gold_bar.deployed();

    const IronBar = await ethers.getContractFactory("IronBar");
    this.iron_bar = await IronBar.deploy(this.civilizations.address);
    await this.iron_bar.deployed();

    const PlatinumBar = await ethers.getContractFactory("PlatinumBar");
    this.platinum_bar = await PlatinumBar.deploy(this.civilizations.address);
    await this.platinum_bar.deployed();

    const SilverBar = await ethers.getContractFactory("SilverBar");
    this.silver_bar = await SilverBar.deploy(this.civilizations.address);
    await this.silver_bar.deployed();

    const SteelBar = await ethers.getContractFactory("SteelBar");
    this.steel_bar = await SteelBar.deploy(this.civilizations.address);
    await this.steel_bar.deployed();

    const WoodPlank = await ethers.getContractFactory("WoodPlank");
    this.wood_plank = await WoodPlank.deploy(this.civilizations.address);
    await this.wood_plank.deployed();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.gold.name()).to.be.eq("Arising: Gold");

    expect(await this.adamantine.name()).to.be.eq("Arising: Adamantine");
    expect(await this.bronze.name()).to.be.eq("Arising: Bronze");
    expect(await this.coal.name()).to.be.eq("Arising: Coal");
    expect(await this.cobalt.name()).to.be.eq("Arising: Cobalt");
    expect(await this.iron.name()).to.be.eq("Arising: Iron");
    expect(await this.platinum.name()).to.be.eq("Arising: Platinum");
    expect(await this.silver.name()).to.be.eq("Arising: Silver");
    expect(await this.stone.name()).to.be.eq("Arising: Stone");
    expect(await this.wood.name()).to.be.eq("Arising: Wood");

    expect(await this.adamantine_bar.name()).to.be.eq(
      "Arising: Adamantine Bar"
    );
    expect(await this.bronze_bar.name()).to.be.eq("Arising: Bronze Bar");
    expect(await this.cobalt_bar.name()).to.be.eq("Arising: Cobalt Bar");
    expect(await this.gold_bar.name()).to.be.eq("Arising: Gold Bar");
    expect(await this.iron_bar.name()).to.be.eq("Arising: Iron Bar");
    expect(await this.platinum_bar.name()).to.be.eq("Arising: Platinum Bar");
    expect(await this.silver_bar.name()).to.be.eq("Arising: Silver Bar");
    expect(await this.steel_bar.name()).to.be.eq("Arising: Steel Bar");
    expect(await this.wood_plank.name()).to.be.eq("Arising: Wood Plank");

    expect(await this.gold.symbol()).to.be.eq("aGOLD");
    expect(await this.adamantine.symbol()).to.be.eq("aADAMANTINE");
    expect(await this.bronze.symbol()).to.be.eq("aBRONZE");
    expect(await this.coal.symbol()).to.be.eq("aCOAL");
    expect(await this.cobalt.symbol()).to.be.eq("aCOBALT");
    expect(await this.iron.symbol()).to.be.eq("aIRON");
    expect(await this.platinum.symbol()).to.be.eq("aPLATINUM");
    expect(await this.silver.symbol()).to.be.eq("aSILVER");
    expect(await this.stone.symbol()).to.be.eq("aSTONE");
    expect(await this.wood.symbol()).to.be.eq("aWOOD");

    expect(await this.adamantine_bar.symbol()).to.be.eq("aADAMANTINEBAR");
    expect(await this.bronze_bar.symbol()).to.be.eq("aBRONZEBAR");
    expect(await this.cobalt_bar.symbol()).to.be.eq("aCOBALTBAR");
    expect(await this.gold_bar.symbol()).to.be.eq("aGOLDBAR");
    expect(await this.iron_bar.symbol()).to.be.eq("aIRONBAR");
    expect(await this.platinum_bar.symbol()).to.be.eq("aPLATINUMBAR");
    expect(await this.silver_bar.symbol()).to.be.eq("aSILVERBAR");
    expect(await this.steel_bar.symbol()).to.be.eq("aSTEELBAR");
    expect(await this.wood_plank.symbol()).to.be.eq("aWOODPLANK");

    expect(await this.gold.image()).to.be.eq(
      "https://playarising.com/gadgets/gold.png"
    );

    expect(await this.adamantine.image()).to.be.eq(
      "https://playarising.com/material/raw/adamantine.png"
    );
    expect(await this.bronze.image()).to.be.eq(
      "https://playarising.com/material/raw/bronze.png"
    );
    expect(await this.coal.image()).to.be.eq(
      "https://playarising.com/material/raw/coal.png"
    );
    expect(await this.cobalt.image()).to.be.eq(
      "https://playarising.com/material/raw/cobalt.png"
    );
    expect(await this.platinum.image()).to.be.eq(
      "https://playarising.com/material/raw/platinum.png"
    );
    expect(await this.silver.image()).to.be.eq(
      "https://playarising.com/material/raw/silver.png"
    );
    expect(await this.iron.image()).to.be.eq(
      "https://playarising.com/material/raw/iron.png"
    );
    expect(await this.stone.image()).to.be.eq(
      "https://playarising.com/material/raw/stone.png"
    );
    expect(await this.wood.image()).to.be.eq(
      "https://playarising.com/material/raw/wood.png"
    );

    expect(await this.adamantine_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/adamantinebar.png"
    );
    expect(await this.bronze_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/bronzebar.png"
    );
    expect(await this.cobalt_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/cobaltbar.png"
    );
    expect(await this.gold_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/goldbar.png"
    );
    expect(await this.iron_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/ironbar.png"
    );
    expect(await this.platinum_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/platinumbar.png"
    );
    expect(await this.silver_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/silverbar.png"
    );
    expect(await this.steel_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/steelbar.png"
    );
    expect(await this.wood_plank.image()).to.be.eq(
      "https://playarising.com/material/basic/woodplank.png"
    );
  });
});
