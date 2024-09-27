const { ethers } = require("hardhat");

const OWNER = "0xDA01D79Ca36b493C7906F3C032D2365Fb3470aEC";
const DESTINATION_DOMAIN = 44787;
const MAILBOX = "0xEf9F292fcEBC3848bF4bB92a96a04F9ECBb78E59";
const ISM = "0x2233a5ce12f814bd64c9cdd73410bb8693124d40";

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

    console.log("Setting contracts...");

    await coverContract.setInterchainSecurityModule(ISM);
    await insurancePool.setInterchainSecurityModule(ISM);

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
