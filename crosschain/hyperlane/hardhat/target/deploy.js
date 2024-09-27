const { ethers } = require("hardhat");

const OWNER = "0xDA01D79Ca36b493C7906F3C032D2365Fb3470aEC";
const DESTINATION_DOMAIN = 7;
const MAILBOX = "";

async function main() {
  console.log("Deploying on target chain...");

  try {
    const InsurancePool = await ethers.getContractFactory("InsurancePool");
    const insurancePool = await InsurancePool.deploy(
      MAILBOX,
      OWNER,
      DESTINATION_DOMAIN
    );
    const poolAddress = await insurancePool.getAddress();

    console.log(`Pool Address: ${poolAddress}`);

    const InsuraceCover = await ethers.getContractFactory("InsuranceCover");
    const coverContract = await InsuraceCover.deploy(
      MAILBOX,
      poolAddress,
      OWNER,
      DESTINATION_DOMAIN
    );

    const coverAddress = await coverContract.getAddress();
    console.log(`Cover Address: ${coverAddress}`);
  } catch (error) {
    console.error("An error occurred:", error);
  }
}
