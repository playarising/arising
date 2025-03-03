const { ethers, upgrades } = require("hardhat");

const TOKEN = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174";

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
    name: "Arising: Ikarans",
    symbol: "ARISING",
    url: "https://characters.playarising.com/ikarans/",
  },
  {
    name: "Arising: Shinkari",
    symbol: "ARISING",
    url: "https://characters.playarising.com/shinkari/",
  },
  {
    name: "Arising: Tarki",
    symbol: "ARISING",
    url: "https://characters.playarising.com/tarki/",
  },
  {
    name: "Arising: Zhand",
    symbol: "ARISING",
    url: "https://characters.playarising.com/zhand/",
  },
];

const GADGETS = [
  {
    name: "Arising: Refresh Token",
    symbol: "REFRESH",
    price: "4990000",
  },
  {
    name: "Arising: Vitalize Token",
    symbol: "VITALIZE",
    price: "49900000",
  },
];

const RESOURCES = [
  {
    name: "Arising: Gold",
    symbol: "GOLD",
  },
  {
    name: "Arising: Wood",
    symbol: "WOOD",
  },
  {
    name: "Arising: Bones",
    symbol: "BONES",
  },
  {
    name: "Arising: Copper",
    symbol: "COPPER",
  },
  {
    name: "Arising: Bronze",
    symbol: "BRONZE",
  },
  {
    name: "Arising: Stone",
    symbol: "STONE",
  },
  {
    name: "Arising: Iron",
    symbol: "IRON",
  },
  {
    name: "Arising: Leather",
    symbol: "LEATHER",
  },
  {
    name: "Arising: Cotton",
    symbol: "COTTON",
  },
  {
    name: "Arising: Wool",
    symbol: "WOOL",
  },
  {
    name: "Arising: Silk",
    symbol: "SILK",
  },
  {
    name: "Arising: Silver",
    symbol: "SILVER",
  },
  {
    name: "Arising: Coal",
    symbol: "COAL",
  },
  {
    name: "Arising: Cobalt",
    symbol: "COBALT",
  },
  {
    name: "Arising: Platinum",
    symbol: "PLATINUM",
  },
  {
    name: "Arising: Adamantine",
    symbol: "ADAMANTINE",
  },
];

async function main() {
  console.log("==> Deploy Levels Upgradeable");
  const Levels = await ethers.getContractFactory("Levels");

  const levels = await upgrades.deployProxy(Levels, [], {
    kind: "uups",
  });
  await levels.deployed();
  console.log("==> Levels deployed:", levels.address);

  console.log("==> Deploy Civilizations Upgradeable");
  const Civilizations = await ethers.getContractFactory("Civilizations");

  const civ = await upgrades.deployProxy(Civilizations, [TOKEN], {
    kind: "uups",
  });
  await civ.deployed();
  console.log("==> Civilizations deployed:", civ.address);

  const BaseERC721 = await ethers.getContractFactory("BaseERC721");
  console.log("==> Deploy BaseERC721 Upgradeable Beacon");
  const baseErc721Beacon = await upgrades.deployBeacon(BaseERC721);
  await baseErc721Beacon.deployed();
  console.log(
    "==> BaseERC721 Upgradeable Beacon deployed:",
    baseErc721Beacon.address,
  );

  console.log("==> Deploy Beacon Proxies for Civilizations");
  for (const civilization of CIVILIZATIONS) {
    const instance = await upgrades.deployBeaconProxy(
      baseErc721Beacon.address,
      BaseERC721,
      [civilization.name, civilization.symbol, civilization.url, civ.address],
    );
    await instance.deployed();
    await (await instance.addAuthority(civ.address)).wait();
    await (await civ.addCivilization(instance.address)).wait();
    console.log("==>", civilization.name, "deployed:", instance.address);
  }

  console.log("==> Deploy Experience Upgradeable");
  const Experience = await ethers.getContractFactory("Experience");

  const experience = await upgrades.deployProxy(
    Experience,
    [civ.address, levels.address],
    {
      kind: "uups",
    },
  );
  await experience.deployed();
  console.log("==> Experience deployed:", experience.address);

  console.log("==> Deploy Names Upgradeable");
  const Names = await ethers.getContractFactory("Names");

  const names = await upgrades.deployProxy(
    Names,
    [civ.address, experience.address],
    {
      kind: "uups",
    },
  );
  await names.deployed();
  console.log("==> Names deployed:", names.address);

  console.log("==> Deploy Items Upgradeable");
  const Items = await ethers.getContractFactory("Items");

  const items = await upgrades.deployProxy(Items, [], {
    kind: "uups",
  });
  await items.deployed();
  console.log("==> Items deployed:", items.address);

  console.log("==> Deploy Equipment Upgradeable");
  const Equipment = await ethers.getContractFactory("Equipment");

  const equipment = await upgrades.deployProxy(
    Equipment,
    [civ.address, experience.address, items.address],
    {
      kind: "uups",
    },
  );
  await equipment.deployed();
  console.log("==> Equipment deployed:", equipment.address);

  const BaseGadgetToken = await ethers.getContractFactory("BaseGadgetToken");
  console.log("==> Deploy BaseGadgetToken Upgradeable Beacon");
  const baseGadgetTokenBeacon = await upgrades.deployBeacon(BaseGadgetToken);
  await baseGadgetTokenBeacon.deployed();
  console.log(
    "==> BaseGadgetToken Upgradeable Beacon deployed:",
    baseGadgetTokenBeacon.address,
  );

  console.log("==> Deploy Beacon Proxies for Gadgets");
  const gadgets = [];
  for (const gadget of GADGETS) {
    const instance = await upgrades.deployBeaconProxy(
      baseGadgetTokenBeacon.address,
      BaseGadgetToken,
      [gadget.name, gadget.symbol, TOKEN, gadget.price],
    );
    await instance.deployed();
    gadgets.push(instance.address);
    console.log("==>", gadget.name, "deployed:", instance.address);
  }

  console.log("==> Deploy Stats Upgradeable");
  const Stats = await ethers.getContractFactory("Stats");

  const stats = await upgrades.deployProxy(
    Stats,
    [
      civ.address,
      experience.address,
      equipment.address,
      gadgets[0],
      gadgets[1],
    ],
    {
      kind: "uups",
    },
  );
  await stats.deployed();
  console.log("==> Stats deployed:", stats.address);

  console.log("==> Deploy Craft Upgradeable");
  const Craft = await ethers.getContractFactory("Craft");

  const craft = await upgrades.deployProxy(
    Craft,
    [civ.address, experience.address, stats.address, items.address],
    {
      kind: "uups",
    },
  );
  await craft.deployed();
  await (await items.addAuthority(craft.address)).wait();
  console.log("==> Craft deployed:", craft.address);

  console.log("==> Deploy Forge Upgradeable");
  const Forge = await ethers.getContractFactory("Forge");

  const forge = await upgrades.deployProxy(
    Forge,
    [civ.address, experience.address, stats.address, TOKEN, "19990000"],
    {
      kind: "uups",
    },
  );
  await forge.deployed();
  console.log("==> Forge deployed:", forge.address);

  console.log("==> Deploy Quests Upgradeable");
  const Quests = await ethers.getContractFactory("Quests");

  const quests = await upgrades.deployProxy(
    Quests,
    [civ.address, experience.address, stats.address],
    {
      kind: "uups",
    },
  );
  await quests.deployed();
  await (await experience.addAuthority(quests.address)).wait();
  console.log("==> Quests deployed:", quests.address);

  const BaseFungibleItem = await ethers.getContractFactory("BaseFungibleItem");
  console.log("==> Deploy BaseFungibleItem Upgradeable Beacon");
  const baseFungibleItemBeacon = await upgrades.deployBeacon(BaseFungibleItem);
  await baseFungibleItemBeacon.deployed();
  console.log(
    "==> BaseFungibleItem Upgradeable Beacon deployed:",
    baseFungibleItemBeacon.address,
  );

  console.log("==> Deploy Beacon Proxies for Resources");
  for (const resource of RESOURCES) {
    const instance = await upgrades.deployBeaconProxy(
      baseFungibleItemBeacon.address,
      BaseFungibleItem,
      [resource.name, resource.symbol, civ.address],
    );
    await instance.deployed();
    await (await instance.addAuthority(quests.address)).wait();
    console.log("==>", resource.name, "deployed:", instance.address);
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
