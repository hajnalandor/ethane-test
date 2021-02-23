/**
 * @type import('hardhat/config').HardhatUserConfig
 */
require('@nomiclabs/hardhat-waffle');
// const INFURA_URL = `https://rinkeby.infura.io/v3/638142df229445c79c493a42f5ca8a74`;
// const PRIVATE_KEY=`54b1d02f24b5c1de4bb330f4d6e19893252575c28ecf3bd9d724853448d33c8a`;
module.exports = {
  solidity: "0.8.0",
  networks: {
    hardhat: {
      forking: {
        url: "https://eth-mainnet.alchemyapi.io/v2/-rJvDU7Ctx1euBT9ww0902_LXPnnrVyz"
      }
    }
  }
      // rinkeby: {
    //     url: INFURA_URL,
    //     accounts: [`0x${PRIVATE_KEY}`]
    // }
};
