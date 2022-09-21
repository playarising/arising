const { ethers } = require("hardhat");

const MATERIALS = [
  {
    name: "Arising: Wood",
    symbol: "WOOD",
  },
  {
    name: "Arising: Bones",
    symbol: "BONES",
  },
  {
    name: "Arising: Cooper",
    symbol: "COOPER",
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

const CIVILIZATIONS = "0xEe77f82B049BB4E94494Cd0228f2f9B6fD1f5f63";
const QUESTS = "0xa81fD563cb930cB36a8fe74039f3aD16B47Bb49F";

async function main() {
  const BaseFungibleItem = await ethers.getContractFactory("BaseFungibleItem");
  for (let i = 0; i < MATERIALS.length; i++) {
    const material = await BaseFungibleItem.deploy(
      MATERIALS[i].name,
      MATERIALS[i].symbol,
      CIVILIZATIONS
    );
    await material.deployed();
    await (await material.addAuthority(QUESTS)).wait();
    console.log("==> Deployed:", MATERIALS[i].name, material.address);
  }

  const gold = await BaseFungibleItem.deploy(
    "Arising: Gold",
    "GOLD",
    CIVILIZATIONS
  );
  await gold.deployed();
  await (await gold.addAuthority(QUESTS)).wait();
  console.log("==> Deployed: Arising: Gold", gold.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
