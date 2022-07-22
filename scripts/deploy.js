const { ethers } = require("hardhat");

const PAYMENT_RECEIVER = "";

const ARD_CAP = 0;
const ZHAND_CAP = 0;
const IKARANS_CAP = 0;
const TARKI_CAP = 0;
const HEARTHEIM_CAP = 0;
const SHINKARI_CAP = 0;

async function main() {
  const MintGuard = await ethers.getContractFactory("MintGuard");

  console.log("==> Deploying MintGuard...");
  const guard = await MintGuard.deploy();
  await guard.deployed();
  console.log("==> MintGuard deployed", guard.address);

  const Ard = await ethers.getContractFactory("Ard");
  const ard = await Ard.deploy(guard.address, ARD_CAP, PAYMENT_RECEIVER);
  await ard.deployed();

  const Zhand = await ethers.getContractFactory("Zhand");
  const zhand = await Zhand.deploy(guard.address, ZHAND_CAP, PAYMENT_RECEIVER);
  await zhand.deployed();

  const Ikarans = await ethers.getContractFactory("I'Karans");
  const ikarans = await Ikarans.deploy(
    guard.address,
    IKARANS_CAP,
    PAYMENT_RECEIVER
  );
  await ikarans.deployed();

  const Tarki = await ethers.getContractFactory("Tark'i");
  const tarki = await Tarki.deploy(guard.address, TARKI_CAP, PAYMENT_RECEIVER);
  await tarki.deployed();

  const Heartheim = await ethers.getContractFactory("Heartheim");
  const heartheim = await Heartheim.deploy(
    guard.address,
    HEARTHEIM_CAP,
    PAYMENT_RECEIVER
  );
  await heartheim.deployed();

  const Shinkari = await ethers.getContractFactory("Shinkari");
  const shinkari = await Shinkari.deploy(
    guard.address,
    SHINKARI_CAP,
    PAYMENT_RECEIVER
  );
  await shinkari.deployed();

  console.log("==> Deployment finished");
  console.log("==> Ard", ard.address);
  console.log("==> Zhand", zhand.address);
  console.log("==> I'karans", ikarans.address);
  console.log("==> Tark'i", tarki.address);
  console.log("==> Heartheim", heartheim.address);
  console.log("==> Shinkari", shinkari.address);

  console.log("==> Adding protected contracts to MintGuard");
  await guard.addProtected(ard.address);
  await guard.addProtected(zhand.address);
  await guard.addProtected(ikarans.address);
  await guard.addProtected(tarki.address);
  await guard.addProtected(heartheim.address);
  await guard.addProtected(shinkari.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
