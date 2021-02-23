/**
 * @type import('hardhat/config').HardhatUserConfig
 */
require('@nomiclabs/hardhat-waffle');
module.exports = {
  solidity: "0.8.0",
  networks: {
    hardhat: {
      forking: {
        url: "https://eth-mainnet.alchemyapi.io/v2/ALCHAMY_PRIVATE_KEY"
      }
    }
  }
      // rinkeby: {
    //     url: INFURA_URL,
    //     accounts: [`0x${PRIVATE_KEY}`]
    // }
};
