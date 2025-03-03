require("dotenv").config();

const argv = require("yargs/yargs")()
  .env("")
  .options({
    ci: {
      type: "boolean",
      default: false,
    },
    coverage: {
      type: "boolean",
      default: false,
    },
    gas: {
      alias: "enableGasReport",
      type: "boolean",
      default: false,
    },
    compiler: {
      alias: "compileVersion",
      type: "string",
      default: "0.8.25",
    },
  }).argv;

if (argv.enableGasReport) {
  require("hardhat-gas-reporter");
}

require("@nomiclabs/hardhat-waffle");
require("@nomiclabs/hardhat-etherscan");
require("@openzeppelin/hardhat-upgrades");
require("solidity-docgen");

module.exports = {
  defaultNetwork: "hardhat",
  networks: {
    hardhat: {},
    /* mainnet: {
      url: process.env.RPC_URL,
      gasPrice: 60000000000,
      accounts: [process.env.PRIVATE_KEY],
    }, */
  },
  etherscan: {
    apiKey: process.env.ETHERSCAN_API,
  },
  solidity: {
    compilers: [
      {
        version: argv.compiler,
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      },
    ],
  },
  gasReporter: {
    token: "MATIC",
    currency: "USD",
    outputFile: argv.ci ? "gas-report.txt" : undefined,
    coinmarketcap: process.env.COINMARKETCAP_API_KEY,
  },
  docgen: {
    path: "./docs",
    clear: true,
    runOnCompile: true,
    pages: "files",
    exclude: ["mocks/", "helpers/"],
  },
};

if (argv.coverage) {
  require("solidity-coverage");
  module.exports.networks.hardhat.initialBaseFeePerGas = 0;
}
