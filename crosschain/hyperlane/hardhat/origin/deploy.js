const { ethers } = require("hardhat");

const OWNER = "0xDA01D79Ca36b493C7906F3C032D2365Fb3470aEC";
const DECIMALS = 18;
const INITIAL_SUPPLY = 200000000000000;
const NAME = "BitQuid";
const SYMBOL = "BQ";
const MAILBOX = "";
const DESTINATION_DOMAIN = "";

const POOL = "";
const COVER = "";

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
      GOVTOKEN,
      poolAddress,
      5,
      OWNER,
      DESTINATION_DOMAIN
    );
    const govAddress = await governance.getAddress();

    console.log(`Gov Address: ${govAddress}`);

    console.log("Setting contracts...");

    await governance.setCoverContract(COVER);

    console.log("All contracts set");
  } catch (error) {
    console.error("An error occurred:", error);
  }
}
