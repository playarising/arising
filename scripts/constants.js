import { ethers } from "hardhat";

const GADGETS_PAYMENT_TOKEN_ADDRESS = "";

export const CIVILIZATIONS = [
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

export const GADGETS = [
  {
    name: "Arising: Refresh Token",
    symbol: "REFRESHER",
    token: GADGETS_PAYMENT_TOKEN_ADDRESS,
    price: ethers.utils.parseEther("4.99"),
  },
  {
    name: "Arising: Vitalizer Token",
    symbol: "VITALIZER",
    token: GADGETS_PAYMENT_TOKEN_ADDRESS,
    price: ethers.utils.parseEther("49.99"),
  },
];

export const MATERIALS = [
  {
    name: "Arising: Gold",
    symbol: "GOLD",
  },
  {
    name: "Arising: Adamantine",
    symbol: "ADAMANTINE",
  },
  {
    name: "Arising: Bronze",
    symbol: "BRONZE",
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
    name: "Arising: Cotton",
    symbol: "COTTON",
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
    name: "Arising: Platinum",
    symbol: "PLATINUM",
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
    name: "Arising: Stone",
    symbol: "STONE",
  },
  {
    name: "Arising: Wood",
    symbol: "WOOD",
  },
  {
    name: "Arising: Wool",
    symbol: "WOOL",
  },
  {
    name: "Arising: Adamantine Bar",
    symbol: "ADAMANTINE_BAR",
  },
  {
    name: "Arising: Bronze Bar",
    symbol: "BRONZE_BAR",
  },
  {
    name: "Arising: Cobalt Bar",
    symbol: "COBALT_BAR",
  },
  {
    name: "Arising: Cotton Fabric",
    symbol: "COTTON_FABRIC",
  },
  {
    name: "Arising: Gold Bar",
    symbol: "GOLD_BAR",
  },
  {
    name: "Arising: Hardened Leather",
    symbol: "HARDENED_LEATHER",
  },
  {
    name: "Arising: Iron Bar",
    symbol: "IRON_BAR",
  },
  {
    name: "Arising: Ironstone",
    symbol: "IRONSTONE",
  },
  {
    name: "Arising: Platinum Bar",
    symbol: "PLATINUM_BAR",
  },
  {
    name: "Arising: Silk Fabric",
    symbol: "SILK_FABRIC",
  },
  {
    name: "Arising: Silver Bar",
    symbol: "SILVER_BAR",
  },
  {
    name: "Arising: Steel Bar",
    symbol: "STEEL_BAR",
  },
  {
    name: "Arising: Wood Plank",
    symbol: "WOOD_PLANK",
  },
  {
    name: "Arising: Wool Fabric",
    symbol: "WOOL_FABRIC",
  },
];
