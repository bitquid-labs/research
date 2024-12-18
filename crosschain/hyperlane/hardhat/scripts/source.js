const { ethers } = require("hardhat");

const OWNER = "0xDA01D79Ca36b493C7906F3C032D2365Fb3470aEC";
const DECIMALS = 18;
const INITIAL_SUPPLY = 200000000000000;
const NAME = "BitQuid";
const SYMBOL = "BQ";
const MAILBOX = "0x54148470292C24345fb828B003461a9444414517";
const DESTINATION_DOMAIN = 44787;
const ISM = "0x12ecb319c7f4e8ac5eb5226662aeb8528c5cefac";

const POOL = "0xC3614e27e1e30182CE0DC15F7E6E21562CF5F212";
const COVER = "0x9054b93ca6c02B4Fc6EF19D7cf0F45c002BcAE3e";

async function main() {
  console.log("Deploying on origin chain...");

  try {
    const Token = await ethers.getContractFactory("MockERC20");
    const token = await Token.deploy(NAME, SYMBOL, DECIMALS, INITIAL_SUPPLY);
    const tokenAddress = await token.getAddress();

    console.log(`Token Address: ${tokenAddress}`);

    const Governance = await ethers.getContractFactory("Governance");
    const governance = await Governance.deploy(
      MAILBOX,
      tokenAddress,
      POOL,
      5,
      OWNER,
      DESTINATION_DOMAIN
    );
    const govAddress = await governance.getAddress();

    console.log(`Gov Address: ${govAddress}`);

    console.log("Setting contracts...");

    await governance.setCoverContract(COVER);
    await governance.setInterchainSecurityModule(ISM);

    console.log("All contracts set");
  } catch (error) {
    console.error("An error occurred:", error);
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
