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

    const Cotton = await ethers.getContractFactory("Cotton");
    this.cotton = await Cotton.deploy(this.civilizations.address);
    await this.cotton.deployed();

    const Iron = await ethers.getContractFactory("Iron");
    this.iron = await Iron.deploy(this.civilizations.address);
    await this.iron.deployed();

    const Leather = await ethers.getContractFactory("Leather");
    this.leather = await Leather.deploy(this.civilizations.address);
    await this.leather.deployed();

    const Platinum = await ethers.getContractFactory("Platinum");
    this.platinum = await Platinum.deploy(this.civilizations.address);
    await this.platinum.deployed();

    const Silk = await ethers.getContractFactory("Silk");
    this.silk = await Silk.deploy(this.civilizations.address);
    await this.silk.deployed();

    const Silver = await ethers.getContractFactory("Silver");
    this.silver = await Silver.deploy(this.civilizations.address);
    await this.silver.deployed();

    const Stone = await ethers.getContractFactory("Stone");
    this.stone = await Stone.deploy(this.civilizations.address);
    await this.stone.deployed();

    const Wood = await ethers.getContractFactory("Wood");
    this.wood = await Wood.deploy(this.civilizations.address);
    await this.wood.deployed();

    const Wool = await ethers.getContractFactory("Wool");
    this.wool = await Wool.deploy(this.civilizations.address);
    await this.wool.deployed();

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

    const CottonFabric = await ethers.getContractFactory("CottonFabric");
    this.cotton_fabric = await CottonFabric.deploy(this.civilizations.address);
    await this.cotton_fabric.deployed();

    const GoldBar = await ethers.getContractFactory("GoldBar");
    this.gold_bar = await GoldBar.deploy(this.civilizations.address);
    await this.gold_bar.deployed();

    const HardenedLeather = await ethers.getContractFactory("HardenedLeather");
    this.hardened_leather = await HardenedLeather.deploy(
      this.civilizations.address
    );
    await this.hardened_leather.deployed();

    const IronBar = await ethers.getContractFactory("IronBar");
    this.iron_bar = await IronBar.deploy(this.civilizations.address);
    await this.iron_bar.deployed();

    const Ironstone = await ethers.getContractFactory("Ironstone");
    this.ironstone = await Ironstone.deploy(this.civilizations.address);
    await this.ironstone.deployed();

    const PlatinumBar = await ethers.getContractFactory("PlatinumBar");
    this.platinum_bar = await PlatinumBar.deploy(this.civilizations.address);
    await this.platinum_bar.deployed();

    const SilkFabric = await ethers.getContractFactory("SilkFabric");
    this.silk_fabric = await SilkFabric.deploy(this.civilizations.address);
    await this.silk_fabric.deployed();

    const SilverBar = await ethers.getContractFactory("SilverBar");
    this.silver_bar = await SilverBar.deploy(this.civilizations.address);
    await this.silver_bar.deployed();

    const SteelBar = await ethers.getContractFactory("SteelBar");
    this.steel_bar = await SteelBar.deploy(this.civilizations.address);
    await this.steel_bar.deployed();

    const WoodPlank = await ethers.getContractFactory("WoodPlank");
    this.wood_plank = await WoodPlank.deploy(this.civilizations.address);
    await this.wood_plank.deployed();

    const WoolFabric = await ethers.getContractFactory("WoolFabric");
    this.wool_fabric = await WoolFabric.deploy(this.civilizations.address);
    await this.wool_fabric.deployed();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.gold.name()).to.be.eq("Arising: Gold");

    expect(await this.adamantine.name()).to.be.eq("Arising: Adamantine");
    expect(await this.bronze.name()).to.be.eq("Arising: Bronze");
    expect(await this.coal.name()).to.be.eq("Arising: Coal");
    expect(await this.cobalt.name()).to.be.eq("Arising: Cobalt");
    expect(await this.cotton.name()).to.be.eq("Arising: Cotton");
    expect(await this.iron.name()).to.be.eq("Arising: Iron");
    expect(await this.leather.name()).to.be.eq("Arising: Leather");
    expect(await this.platinum.name()).to.be.eq("Arising: Platinum");
    expect(await this.silk.name()).to.be.eq("Arising: Silk");
    expect(await this.silver.name()).to.be.eq("Arising: Silver");
    expect(await this.stone.name()).to.be.eq("Arising: Stone");
    expect(await this.wood.name()).to.be.eq("Arising: Wood");
    expect(await this.wool.name()).to.be.eq("Arising: Wool");

    expect(await this.adamantine_bar.name()).to.be.eq(
      "Arising: Adamantine Bar"
    );
    expect(await this.bronze_bar.name()).to.be.eq("Arising: Bronze Bar");
    expect(await this.cobalt_bar.name()).to.be.eq("Arising: Cobalt Bar");
    expect(await this.cotton_fabric.name()).to.be.eq("Arising: Cotton Fabric");
    expect(await this.gold_bar.name()).to.be.eq("Arising: Gold Bar");
    expect(await this.hardened_leather.name()).to.be.eq(
      "Arising: Hardened Leather"
    );
    expect(await this.iron_bar.name()).to.be.eq("Arising: Iron Bar");
    expect(await this.ironstone.name()).to.be.eq("Arising: Ironstone");
    expect(await this.platinum_bar.name()).to.be.eq("Arising: Platinum Bar");
    expect(await this.silk_fabric.name()).to.be.eq("Arising: Silk Fabric");
    expect(await this.silver_bar.name()).to.be.eq("Arising: Silver Bar");
    expect(await this.steel_bar.name()).to.be.eq("Arising: Steel Bar");
    expect(await this.wood_plank.name()).to.be.eq("Arising: Wood Plank");
    expect(await this.wool_fabric.name()).to.be.eq("Arising: Wool Fabric");

    expect(await this.gold.symbol()).to.be.eq("GOLD");
    expect(await this.adamantine.symbol()).to.be.eq("ADAMANTINE");
    expect(await this.bronze.symbol()).to.be.eq("BRONZE");
    expect(await this.coal.symbol()).to.be.eq("COAL");
    expect(await this.cobalt.symbol()).to.be.eq("COBALT");
    expect(await this.cotton.symbol()).to.be.eq("COTTON");
    expect(await this.iron.symbol()).to.be.eq("IRON");
    expect(await this.leather.symbol()).to.be.eq("LEATHER");
    expect(await this.platinum.symbol()).to.be.eq("PLATINUM");
    expect(await this.silk.symbol()).to.be.eq("SILK");
    expect(await this.silver.symbol()).to.be.eq("SILVER");
    expect(await this.stone.symbol()).to.be.eq("STONE");
    expect(await this.wood.symbol()).to.be.eq("WOOD");
    expect(await this.wool.symbol()).to.be.eq("WOOL");

    expect(await this.adamantine_bar.symbol()).to.be.eq("ADAMANTINE_BAR");
    expect(await this.bronze_bar.symbol()).to.be.eq("BRONZE_BAR");
    expect(await this.cobalt_bar.symbol()).to.be.eq("COBALT_BAR");
    expect(await this.cotton_fabric.symbol()).to.be.eq("COTTON_FABRIC");
    expect(await this.gold_bar.symbol()).to.be.eq("GOLD_BAR");
    expect(await this.hardened_leather.symbol()).to.be.eq("HARDENED_LEATHER");
    expect(await this.iron_bar.symbol()).to.be.eq("IRON_BAR");
    expect(await this.ironstone.symbol()).to.be.eq("IRONSTONE");
    expect(await this.platinum_bar.symbol()).to.be.eq("PLATINUM_BAR");
    expect(await this.silk_fabric.symbol()).to.be.eq("SILK_FABRIC");
    expect(await this.silver_bar.symbol()).to.be.eq("SILVER_BAR");
    expect(await this.steel_bar.symbol()).to.be.eq("STEEL_BAR");
    expect(await this.wood_plank.symbol()).to.be.eq("WOOD_PLANK");
    expect(await this.wool_fabric.symbol()).to.be.eq("WOOL_FABRIC");

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
    expect(await this.cotton.image()).to.be.eq(
      "https://playarising.com/material/raw/cotton.png"
    );

    expect(await this.iron.image()).to.be.eq(
      "https://playarising.com/material/raw/iron.png"
    );
    expect(await this.leather.image()).to.be.eq(
      "https://playarising.com/material/raw/leather.png"
    );
    expect(await this.platinum.image()).to.be.eq(
      "https://playarising.com/material/raw/platinum.png"
    );
    expect(await this.silk.image()).to.be.eq(
      "https://playarising.com/material/raw/silk.png"
    );
    expect(await this.silver.image()).to.be.eq(
      "https://playarising.com/material/raw/silver.png"
    );
    expect(await this.stone.image()).to.be.eq(
      "https://playarising.com/material/raw/stone.png"
    );
    expect(await this.wood.image()).to.be.eq(
      "https://playarising.com/material/raw/wood.png"
    );
    expect(await this.wool.image()).to.be.eq(
      "https://playarising.com/material/raw/wool.png"
    );

    expect(await this.adamantine_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/adamantine_bar.png"
    );
    expect(await this.bronze_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/bronze_bar.png"
    );
    expect(await this.cobalt_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/cobalt_bar.png"
    );
    expect(await this.cotton_fabric.image()).to.be.eq(
      "https://playarising.com/material/basic/cotton_fabric.png"
    );
    expect(await this.gold_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/gold_bar.png"
    );
    expect(await this.hardened_leather.image()).to.be.eq(
      "https://playarising.com/material/basic/hardened_leather.png"
    );
    expect(await this.iron_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/iron_bar.png"
    );
    expect(await this.ironstone.image()).to.be.eq(
      "https://playarising.com/material/basic/ironstone.png"
    );
    expect(await this.platinum_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/platinum_bar.png"
    );
    expect(await this.silk_fabric.image()).to.be.eq(
      "https://playarising.com/material/basic/silk_fabric.png"
    );
    expect(await this.silver_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/silver_bar.png"
    );
    expect(await this.steel_bar.image()).to.be.eq(
      "https://playarising.com/material/basic/steel_bar.png"
    );
    expect(await this.wood_plank.image()).to.be.eq(
      "https://playarising.com/material/basic/wood_plank.png"
    );
    expect(await this.wool_fabric.image()).to.be.eq(
      "https://playarising.com/material/basic/wool_fabric.png"
    );
  });
});
