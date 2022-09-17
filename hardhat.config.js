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
      default: "0.8.17",
    },
  }).argv;

if (argv.enableGasReport) {
  require("hardhat-gas-reporter");
}

require("@nomiclabs/hardhat-waffle");
require("solidity-docgen");

module.exports = {
  defaultNetwork: "hardhat",
  networks: {
    hardhat: {
      blockGasLimit: 30000000,
    },
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
    currency: "USD",
    outputFile: argv.ci ? "gas-report.txt" : undefined,
  },
  docgen: {
    path: "./docs",
    clear: true,
    runOnCompile: true,
    pages: "files",
    exclude: ["mocks/", "materials/", "civilizations/"],
  },
};

if (argv.coverage) {
  require("solidity-coverage");
  module.exports.networks.hardhat.initialBaseFeePerGas = 0;
}
