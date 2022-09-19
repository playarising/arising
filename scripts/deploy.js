const { ethers } = require("hardhat");

const CIVILIZATIONS = [
  {
    name: "Arising: Ard",
    symbol: "ARISING",
    url: "https://characters.playarising.com/ard/",
  },
  {
    name: "Arising: Hartheim",
    symbol: "ARISING",
    url: "https://characters.playarising.com/hartheim/",
  },
  {
    name: "Arising: I'Karans",
    symbol: "ARISING",
    url: "https://characters.playarising.com/ikarans/",
  },
  {
    name: "Arising: Shinkari",
    symbol: "ARISING",
    url: "https://characters.playarising.com/shinkari/",
  },
  {
    name: "Arising: Tark'i",
    symbol: "ARISING",
    url: "https://characters.playarising.com/tarki/",
  },
  {
    name: "Arising: Zhand",
    symbol: "ARISING",
    url: "https://characters.playarising.com/zhand/",
  },
];

const TOKEN = "0xc9dfa81f7e4A02FbA4FCce3D5AD95940F054d259";

async function main() {
  const Levels = await ethers.getContractFactory("Levels");
  const Civilizations = await ethers.getContractFactory("Civilizations");
  const BaseERC721 = await ethers.getContractFactory("BaseERC721");
  const Experience = await ethers.getContractFactory("Experience");
  const Names = await ethers.getContractFactory("Names");
  const Items = await ethers.getContractFactory("Items");
  const Stats = await ethers.getContractFactory("Stats");
  const BaseGadgetToken = await ethers.getContractFactory("BaseGadgetToken");
  const BaseFungibleItem = await ethers.getContractFactory("BaseFungibleItem");
  const Equipment = await ethers.getContractFactory("Equipment");
  const Forge = await ethers.getContractFactory("Forge");
  const Craft = await ethers.getContractFactory("Craft");
  const Quests = await ethers.getContractFactory("Quests");

  console.log("==> Deploying the Levels...");
  const levels = await Levels.deploy();
  await levels.deployed();

  console.log("==> Deploying the Civilizations...");
  const civilizations = await Civilizations.deploy(TOKEN);
  await civilizations.deployed();

  console.log("==> Configuring civilizations...");
  await civilizations.setMintPrice(ethers.utils.parseEther("8.99"));
  for (let i = 0; i < CIVILIZATIONS.length; i++) {
    const civ = await BaseERC721.deploy(
      CIVILIZATIONS[i].name,
      CIVILIZATIONS[i].symbol,
      CIVILIZATIONS[i].url
    );
    await civ.deployed();
    await civilizations.addCivilization(civ.address);
    await civ.transferOwnership(civilizations.address);
  }

  console.log("==> Deploying the Experience...");
  const experience = await Experience.deploy(
    civilizations.address,
    levels.address
  );
  await experience.deployed();

  console.log("==> Deploying the Names...");
  const names = await Names.deploy(civilizations.address, experience.address);
  await names.deployed();

  console.log("==> Deploying the Items...");
  const items = await Items.deploy();
  await items.deployed();

  console.log("==> Deploying the Refresher...");
  const refresher = await BaseGadgetToken.deploy(
    "Arising: Refresh Token",
    "REFRESHER",
    TOKEN,
    ethers.utils.parseEther("4.99")
  );
  await refresher.deployed();

  console.log("==> Deploying the Refresher...");
  const vitalizer = await BaseGadgetToken.deploy(
    "Arising: Vitalizer Token",
    "VITALIZER",
    TOKEN,
    ethers.utils.parseEther("49.99")
  );
  await refresher.deployed();

  console.log("==> Deploying the Stats...");
  const stats = await Stats.deploy(civilizations.address, experience.address);
  await stats.deployed();
  await stats.setRefreshToken(refresher.address);
  await stats.setVitalizerToken(vitalizer.address);

  console.log("==> Deploying the Gold...");
  const gold = await BaseFungibleItem.deploy(
    "Arising: Gold",
    "GOLD",
    civilizations.address
  );
  await gold.deployed();

  console.log("==> Deploying the Equipment...");
  const equipment = await Equipment.deploy(
    civilizations.address,
    experience.address,
    items.address
  );
  await equipment.deployed();

  console.log("==> Deploying the Forge...");
  const forge = await Forge.deploy(
    civilizations.address,
    experience.address,
    stats.address,
    gold.address,
    TOKEN,
    ethers.utils.parseEther("19.99")
  );
  await forge.deployed();

  console.log("==> Levels deployed:", levels.address);
  console.log("==> Civilizations deployed:", civilizations.address);
  console.log("==> Experience deployed:", civilizations.address);
  console.log("==> Names deployed:", names.address);
  console.log("==> Items deployed:", items.address);
  console.log("==> Refresher deployed:", refresher.address);
  console.log("==> Vitalizer deployed:", vitalizer.address);
  console.log("==> Stats deployed:", stats.address);
  console.log("==> Gold deployed:", stats.address);
  console.log("==> Equipment deployed:", stats.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
