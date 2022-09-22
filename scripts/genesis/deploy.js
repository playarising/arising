const { ethers } = require("hardhat");

const TOKEN = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174";

async function main() {
  const Levels = await ethers.getContractFactory("Levels");

  console.log("==> Deploying Levels");
  const levels = await Levels.deploy();
  await levels.deployed();

  const Civilizations = await ethers.getContractFactory("Civilizations");

  console.log("==> Deploying Civilizations");
  const civ = await Civilizations.deploy(TOKEN);
  await civ.deployed();

  const BaseERC721 = await ethers.getContractFactory("BaseERC721");

  console.log("==> Deploying Ard");
  const ard = await BaseERC721.deploy(
    "Arising: Ard",
    "ARISING",
    "https://characters.playarising.com/ard/",
    civ.address
  );
  await ard.deployed();

  console.log("==> Deploying Hartheim");
  const hartheim = await BaseERC721.deploy(
    "Arising: Hartheim",
    "ARISING",
    "https://characters.playarising.com/hartheim/",
    civ.address
  );
  await hartheim.deployed();

  console.log("==> Deploying I'Karans");
  const ikarans = await BaseERC721.deploy(
    "Arising: IKarans",
    "ARISING",
    "https://characters.playarising.com/ikarans/",
    civ.address
  );
  await ikarans.deployed();

  console.log("==> Deploying Shinkari");
  const shinkari = await BaseERC721.deploy(
    "Arising: Shinkari",
    "ARISING",
    "https://characters.playarising.com/shinkari/",
    civ.address
  );
  await shinkari.deployed();

  console.log("==> Deploying Tark'i");
  const tarki = await BaseERC721.deploy(
    "Arising: Tarki",
    "ARISING",
    "https://characters.playarising.com/tarki/",
    civ.address
  );
  await tarki.deployed();

  console.log("==> Deploying Zhand");
  const zhand = await BaseERC721.deploy(
    "Arising: Zhand",
    "ARISING",
    "https://characters.playarising.com/zhand/",
    civ.address
  );
  await zhand.deployed();

  const Experience = await ethers.getContractFactory("Experience");

  console.log("==> Deploying Experience");
  const experience = await Experience.deploy(civ.address, levels.address);
  await experience.deployed();

  const Names = await ethers.getContractFactory("Names");

  console.log("==> Deploying Names");
  const names = await Names.deploy(civ.address, experience.address);
  await names.deployed();

  const Items = await ethers.getContractFactory("Items");

  console.log("==> Deploying Items");
  const items = await Items.deploy();
  await items.deployed();

  const BaseGadgetToken = await ethers.getContractFactory("BaseGadgetToken");

  console.log("==> Deploying Refresher");
  const refresher = await BaseGadgetToken.deploy(
    "Arising: Refresh Token",
    "REFRESHER",
    TOKEN,
    "4990000"
  );
  await refresher.deployed();

  console.log("==> Deploying Vitalizer");
  const vitalizer = await BaseGadgetToken.deploy(
    "Arising: Vitalizer Token",
    "VITALIZER",
    TOKEN,
    "49990000"
  );
  await vitalizer.deployed();

  const Equipment = await ethers.getContractFactory("Equipment");

  console.log("==> Deploying Equipment");
  const equipment = await Equipment.deploy(
    civ.address,
    experience.address,
    items.address
  );
  await equipment.deployed();

  const Stats = await ethers.getContractFactory("Stats");

  console.log("==> Deploying Stats");
  const stats = await Stats.deploy(
    civ.address,
    experience.address,
    equipment.address
  );
  await stats.deployed();

  const Forge = await ethers.getContractFactory("Forge");

  console.log("==> Deploying Forge");
  const forge = await Forge.deploy(
    civ.address,
    experience.address,
    stats.address,
    TOKEN,
    "19990000"
  );
  await forge.deployed();

  const Craft = await ethers.getContractFactory("Craft");

  console.log("==> Deploying Craft");
  const craft = await Craft.deploy(
    civ.address,
    experience.address,
    stats.address,
    items.address
  );
  await craft.deployed();

  const Quests = await ethers.getContractFactory("Quests");

  console.log("==> Deploying Quests");
  const quests = await Quests.deploy(
    civ.address,
    experience.address,
    stats.address
  );
  await quests.deployed();

  console.log("==> Levels deployed:", levels.address);
  console.log("==> Civilizations deployed:", civ.address);
  console.log("==> Ard deployed:", ard.address);
  console.log("==> Hartheim deployed:", hartheim.address);
  console.log("==> I'Karans deployed:", ikarans.address);
  console.log("==> Shinkari deployed:", shinkari.address);
  console.log("==> Tark'i deployed:", tarki.address);
  console.log("==> Zhand deployed:", zhand.address);
  console.log("==> Experience deployed:", experience.address);
  console.log("==> Names deployed:", names.address);
  console.log("==> Items deployed:", items.address);
  console.log("==> Refresher deployed:", refresher.address);
  console.log("==> Vitalizer deployed:", vitalizer.address);
  console.log("==> Stats deployed:", stats.address);
  console.log("==> Equipment deployed:", equipment.address);
  console.log("==> Forge deployed:", forge.address);
  console.log("==> Craft deployed:", craft.address);
  console.log("==> Quests deployed:", quests.address);

  await (await civ.setMintPrice("8990000")).wait();
  await (await stats.setRefreshToken(refresher.address)).wait();
  await (await stats.setVitalizerToken(vitalizer.address)).wait();

  await (await items.addAuthority(craft.address)).wait();
  await (await experience.addAuthority(quests.address)).wait();

  await (await ard.addAuthority(civ.address)).wait();
  await (await hartheim.addAuthority(civ.address)).wait();
  await (await ikarans.addAuthority(civ.address)).wait();
  await (await shinkari.addAuthority(civ.address)).wait();
  await (await tarki.addAuthority(civ.address)).wait();
  await (await zhand.addAuthority(civ.address)).wait();

  await (await civ.addCivilization(ard.address)).wait();
  await (await civ.addCivilization(hartheim.address)).wait();
  await (await civ.addCivilization(ikarans.address)).wait();
  await (await civ.addCivilization(shinkari.address)).wait();
  await (await civ.addCivilization(tarki.address)).wait();
  await (await civ.addCivilization(zhand.address)).wait();

  try {
    await run("verify:verify", {
      address: levels.address,
      constructorArguments: [],
    });
  } catch (e) {}

  try {
    await run("verify:verify", {
      address: civ.address,
      constructorArguments: [TOKEN],
    });
  } catch (e) {}

  try {
    await run("verify:verify", {
      address: experience.address,
      constructorArguments: [civ.address, levels.address],
    });
  } catch (e) {}

  try {
    await run("verify:verify", {
      address: names.address,
      constructorArguments: [civ.address, experience.address],
    });
  } catch (e) {}

  try {
    await run("verify:verify", {
      address: items.address,
      constructorArguments: [],
    });
  } catch (e) {}

  try {
    await run("verify:verify", {
      address: refresher.address,
      constructorArguments: [
        "Arising: Refresh Token",
        "REFRESHER",
        TOKEN,
        "4990000",
      ],
    });
  } catch (e) {}

  try {
    await run("verify:verify", {
      address: vitalizer.address,
      constructorArguments: [
        "Arising: Vitalizer Token",
        "VITALIZER",
        TOKEN,
        "49990000",
      ],
    });
  } catch (e) {}

  try {
    await run("verify:verify", {
      address: stats.address,
      constructorArguments: [
        civ.address,
        experience.address,
        equipment.address,
      ],
    });
  } catch (e) {}

  try {
    await run("verify:verify", {
      address: equipment.address,
      constructorArguments: [civ.address, experience.address, items.address],
    });
  } catch (e) {}

  try {
    await run("verify:verify", {
      address: forge.address,
      constructorArguments: [
        civ.address,
        experience.address,
        stats.address,
        TOKEN,
        "19990000",
      ],
    });
  } catch (e) {}

  try {
    await run("verify:verify", {
      address: quests.address,
      constructorArguments: [civ.address, experience.address, stats.address],
    });
  } catch (e) {}

  try {
    await run("verify:verify", {
      address: craft.address,
      constructorArguments: [
        civ.address,
        experience.address,
        stats.address,
        items.address,
      ],
    });
  } catch (e) {}

  try {
    await run("verify:verify", {
      address: zhand.address,
      constructorArguments: [
        "Arising: Zhand",
        "ARISING",
        "https://characters.playarising.com/zhand/",
        civ.address,
      ],
    });
  } catch (e) {}
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
