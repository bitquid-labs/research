// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./libraries/openzeppelin/IERC20.sol";
import "./libraries/openzeppelin/Ownable.sol";
import "./libraries/openzeppelin/ReentrancyGuard.sol";
import "./libraries/hyperlane/IMessageRecipient.sol";
import "./libraries/hyperlane/IMailbox.sol";
import {IInterchainSecurityModule, ISpecifiesInterchainSecurityModule} from "./libraries/hyperlane/IInterchainSecurityModule.sol";
import "./CoverLib.sol";

interface ICover {
    function updateMaxAmount(uint256 _coverId) external ;
    function getDepositClaimableDays(address user, uint256 _poolId) external view returns (uint256);
    function getLastClaimTime(address user, uint256 _poolId) external view returns (uint256);
}

contract InsurancePool is ReentrancyGuard, IMessageRecipient, ISpecifiesInterchainSecurityModule, Ownable {
    using CoverLib for *;
    error LpNotActive();

    mapping (uint256 => CoverLib.Cover[]) poolToCovers;
    mapping(uint256 => CoverLib.Pool) public pools;
    mapping(uint256 => CoverLib.Proposal) internal approvedProposals;
    uint256 public poolCount;
    address public governance;
    ICover public ICoverContract;
    IMailbox public mailbox;
    address public coverContract;
    address public initialOwner;
    uint256 public destinationDomain;
    IInterchainSecurityModule public interchainSecurityModule;

    event ReceivedMessage(uint32 indexed origin, bytes32 indexed sender, string message);
    event Deposited(address indexed user, uint256 amount, string pool);
    event Withdraw(address indexed user, uint256 amount, string pool);
    event ClaimPaid(address indexed recipient, string pool, uint256 amount);
    event PoolCreated(uint256 indexed id, string poolName);
    event PoolUpdated(uint256 indexed poolId, uint256 apy, uint256 _minPeriod);
    event ClaimAttempt(uint256, uint256, address);

    constructor(address _mailbox, address _initialOwner, uint256 _destinationDomain) Ownable(_initialOwner) {
        mailbox = IMailbox(_mailbox);
        initialOwner = _initialOwner;
        destinationDomain = _destinationDomain;
    }

    function createPool(
        CoverLib.RiskType _riskType,
        string memory _poolName,
        uint256 _apy,
        uint256 _minPeriod
    ) public onlyOwner {
        poolCount += 1;
        CoverLib.Pool storage newPool = pools[poolCount];
        newPool.poolName = _poolName;
        newPool.apy = _apy;
        newPool.minPeriod = _minPeriod;
        newPool.tvl = 0;
        newPool.isActive = true;
        newPool.riskType = _riskType;
        newPool.percentageSplitBalance = 100;

        mailbox.dispatch(uint32(destinationDomain), addressToBytes32(governance), abi.encode("PoolActive", abi.encode(poolCount)));

        emit PoolCreated(poolCount, _poolName);
    }

    function updatePool(
        uint256 _poolId,
        uint256 _apy,
        uint256 _minPeriod
    ) public onlyOwner {
        require(pools[_poolId].isActive, "Pool does not exist or is inactive");
        require(_apy > 0, "Invalid APY");
        require(_minPeriod > 0, "Invalid minimum period");

        pools[_poolId].apy = _apy;
        pools[_poolId].minPeriod = _minPeriod;

        emit PoolUpdated(_poolId, _apy, _minPeriod);
    }

    function reducePercentageSplit(uint256 _poolId, uint256 __poolPercentageSplit) public onlyCover {
        pools[_poolId].percentageSplitBalance -= __poolPercentageSplit;
    }

    function increasePercentageSplit(uint256 _poolId, uint256 __poolPercentageSplit) public onlyCover {
        pools[_poolId].percentageSplitBalance += __poolPercentageSplit;
    }

    function deactivatePool(uint256 _poolId) public onlyOwner {
        if (!pools[_poolId].isActive) {
            revert LpNotActive();
        }
        pools[_poolId].isActive = false;
        mailbox.dispatch(uint32(destinationDomain), addressToBytes32(governance), abi.encode("PoolDeactivated", abi.encode(poolCount)));
    }

    function getPool(
        uint256 _poolId
    )
        public
        view
        returns (
            string memory name,
            CoverLib.RiskType riskType,
            uint256 apy,
            uint256 minPeriod,
            uint256 tvl,
            bool isActive,
            uint256 percentageSplitBalance
        )
    {
        CoverLib.Pool storage pool = pools[_poolId];
        return (
            pool.poolName,
            pool.riskType,
            pool.apy,
            pool.minPeriod,
            pool.tvl,
            pool.isActive,
            pool.percentageSplitBalance
        );
    }

    function getAllPools() public view returns (CoverLib.NPool[] memory) {
        CoverLib.NPool[] memory result = new CoverLib.NPool[](poolCount);
        for (uint256 i = 1; i <= poolCount; i++) {
            CoverLib.Pool storage pool = pools[i];
            result[i - 1] = CoverLib.NPool({
                poolName: pool.poolName,
                riskType: pool.riskType,
                apy: pool.apy,
                minPeriod: pool.minPeriod,
                tvl: pool.tvl,
                tcp: pool.tcp,
                isActive: pool.isActive,
                percentageSplitBalance: pool.percentageSplitBalance
            });
        }
        return result;
    }

    function updatePoolCovers(uint256 _poolId, CoverLib.Cover memory _cover) public onlyCover {
        for (uint i = 0; i < poolToCovers[_poolId].length; i++) {
            if (poolToCovers[_poolId][i].id == _cover.id) {
                poolToCovers[_poolId][i] = _cover;
                break;
            }
        }
    }

    function addPoolCover(uint256 _poolId, CoverLib.Cover memory _cover) public onlyCover {
        poolToCovers[_poolId].push(_cover);
    }

    function getPoolCovers(uint256 _poolId) public view returns (CoverLib.Cover[] memory) {
        return poolToCovers[_poolId];
    }

    function getPoolsByAddress(address _userAddress)
        public
        view
        returns (CoverLib.PoolInfo[] memory)
    {
        uint256 resultCount = 0;
        for (uint256 i = 1; i <= poolCount; i++) {
            CoverLib.Pool storage pool = pools[i];
            if (pool.deposits[_userAddress].amount > 0) {
                resultCount++;
            }
        }

        CoverLib.PoolInfo[] memory result = new CoverLib.PoolInfo[](resultCount);

        uint256 resultIndex = 0;

        for (uint256 i = 1; i <= poolCount; i++) {
            CoverLib.Pool storage pool = pools[i];
            CoverLib.Deposits memory userDeposit = pools[i].deposits[_userAddress];
            uint256 claimableDays = ICoverContract.getDepositClaimableDays(_userAddress, i);
            uint256 accruedPayout = userDeposit.dailyPayout * claimableDays;
            if (pool.deposits[_userAddress].amount > 0) {
                result[resultIndex++] = CoverLib.PoolInfo({
                    poolName: pool.poolName,
                    poolId: i,
                    dailyPayout: pool.deposits[_userAddress].dailyPayout,
                    depositAmount: pool.deposits[_userAddress].amount,
                    apy: pool.apy,
                    minPeriod: pool.minPeriod,
                    tvl: pool.tvl,
                    tcp: pool.tcp,
                    isActive: pool.isActive,
                    accruedPayout: accruedPayout
                });
            }
        }
        return result;
    }

    function withdraw(uint256 _poolId) public nonReentrant {
        CoverLib.Pool storage selectedPool = pools[_poolId];
        CoverLib.Deposits storage userDeposit = selectedPool.deposits[msg.sender];

        require(userDeposit.amount > 0, "No deposit found for this address");
        require(userDeposit.status == CoverLib.Status.Active, "Deposit is not active");
        require(
            block.timestamp >= userDeposit.expiryDate,
            "Deposit period has not ended"
        );

        userDeposit.status = CoverLib.Status.Withdrawn;
        selectedPool.tvl -= userDeposit.amount;
        CoverLib.Cover[] memory poolCovers = getPoolCovers(_poolId);
        for (uint i = 0; i < poolCovers.length; i++) {
            ICoverContract.updateMaxAmount(poolCovers[i].id);
        }

        (bool success, ) = msg.sender.call{value: userDeposit.amount}("");
        require(success, "Withdrawal failed");

        emit Withdraw(msg.sender, userDeposit.amount, selectedPool.poolName);
    }

    function deposit(
        uint256 _poolId
    ) public payable nonReentrant {
        CoverLib.Pool storage selectedPool = pools[_poolId];

        require(msg.value > 0, "Amount must be greater than 0");
        require(selectedPool.isActive, "Pool is inactive or does not exist");

        if (selectedPool.deposits[msg.sender].amount > 0) {
            uint256 amount = selectedPool.deposits[msg.sender].amount + msg.value;
            selectedPool.deposits[msg.sender].amount = amount;
            selectedPool.deposits[msg.sender].expiryDate = block.timestamp + (selectedPool.minPeriod * 1 days);
            selectedPool.deposits[msg.sender].dailyPayout = (amount * selectedPool.apy) / 100 / 365;
            selectedPool.deposits[msg.sender].daysLeft = (selectedPool.minPeriod * 1 days) ;
        } else {
            uint256 dailyPayout = (msg.value * selectedPool.apy) / 100 / 365;
            selectedPool.deposits[msg.sender] = CoverLib.Deposits({
                lp: msg.sender,
                amount: msg.value,
                poolId: _poolId,
                dailyPayout: dailyPayout,
                status: CoverLib.Status.Active,
                daysLeft: selectedPool.minPeriod,
                startDate: block.timestamp,
                expiryDate: block.timestamp + (selectedPool.minPeriod * 1 minutes),
                accruedPayout: 0
            });
        }

        selectedPool.tvl += msg.value;
        CoverLib.Cover[] memory poolCovers = getPoolCovers(_poolId);
        for (uint i = 0; i < poolCovers.length; i++) {
            ICoverContract.updateMaxAmount(poolCovers[i].id);
        }

        emit Deposited(msg.sender, msg.value, selectedPool.poolName);
    }

    function claimProposalFunds(
        uint256 _proposalId
    ) public nonReentrant {
        CoverLib.Proposal memory proposal = approvedProposals[_proposalId];
        CoverLib.ProposalParams memory proposalParam = proposal.proposalParam;
        require(proposal.status == CoverLib.ProposalStaus.Approved && proposal.executed, "Proposal not approved");
        CoverLib.Pool storage pool = pools[proposalParam.poolId];
        require(msg.sender == proposalParam.user,"Not a valid proposal");
        require(pool.isActive, "Pool is not active");
        require(pool.tvl >= proposalParam.claimAmount, "Not enough funds in the pool"); 

        pool.tcp += proposalParam.claimAmount;
        pool.tvl -= proposalParam.claimAmount;
        CoverLib.Cover[] memory poolCovers = getPoolCovers(proposalParam.poolId);
        for (uint i = 0; i < poolCovers.length; i++) {
            ICoverContract.updateMaxAmount(poolCovers[i].id);
        }
        
        mailbox.dispatch(uint32(destinationDomain), addressToBytes32(governance), abi.encode("updateProposalStatusToClaimed", abi.encode(_proposalId)));

        emit ClaimAttempt(proposalParam.poolId, proposalParam.claimAmount, proposalParam.user);

        (bool success, ) = msg.sender.call{value: proposalParam.claimAmount}("");
        require(success, "Transfer failed");

        emit ClaimPaid(msg.sender, pool.poolName, proposalParam.claimAmount);
    }

    function getUserDeposit(
        uint256 _poolId,
        address _user
    ) public view returns (CoverLib.Deposits memory) {
        CoverLib.Deposits memory userDeposit = pools[_poolId].deposits[_user];
        uint256 claimTime = ICoverContract.getLastClaimTime(_user, _poolId);
        uint lastClaimTime;
        if (claimTime == 0) {
            lastClaimTime = userDeposit.startDate;
        } else {
            lastClaimTime = claimTime;
        }
        uint256 currentTime = block.timestamp;
        if (currentTime > userDeposit.expiryDate) {
            currentTime = userDeposit.expiryDate;
        }
        uint256 claimableDays = (currentTime - lastClaimTime) / 5 minutes;
        userDeposit.accruedPayout = userDeposit.dailyPayout * claimableDays;
        if (userDeposit.expiryDate <= block.timestamp) {
            userDeposit.daysLeft = 0;
        } else {
            uint256 timeLeft = userDeposit.expiryDate - block.timestamp;
            userDeposit.daysLeft = (timeLeft + 1 days - 1) / 1 days; // Round up
        }
        return userDeposit;
    }

    function getPoolTVL(uint256 _poolId) public view returns (uint256) {
        return pools[_poolId].tvl;
    }

    function poolActive(uint256 poolId) public view returns (bool) {
        CoverLib.Pool storage pool = pools[poolId];
        return pool.isActive;
    }

    function setGovernance(address _governance) external onlyOwner {
        require(governance == address(0), "Governance already set");
        require(_governance != address(0), "Governance address cannot be zero");
        governance = _governance;
    }

    function setCover(address _coverContract) external onlyOwner {
        require(coverContract == address(0), "Governance already set");
        require(_coverContract != address(0), "Governance address cannot be zero");
        ICoverContract = ICover(_coverContract);
        coverContract = _coverContract;
    }

    function setInterchainSecurityModule(address _ism) external onlyOwner {
        interchainSecurityModule = IInterchainSecurityModule(_ism);
    }

    receive() external payable {}

    modifier onlyCover() {
        require(
            msg.sender == coverContract || msg.sender == initialOwner,
            "Caller is not the governance contract"
        );
        _;
    }

    function addressToBytes32(address _addr) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(_addr)));
    }

    function setDestinationDomain(uint32 _destinationDomain) external onlyOwner {
        destinationDomain = _destinationDomain;
    }

    function handle(uint32 _origin, bytes32 _sender, bytes memory _message) external payable override {
        require(msg.sender == address(mailbox), "Sender must be mailbox");

        (string memory functionName, bytes memory param) = abi.decode(_message, (string, bytes));

        if (keccak256(abi.encodePacked(functionName)) == keccak256(abi.encodePacked("poolActive"))) {
            uint256 poolId = abi.decode(param, (uint256));
            poolActive(poolId);
        }

        if (keccak256(abi.encodePacked(functionName)) == keccak256(abi.encodePacked("approvedProposals"))) {
            CoverLib.Proposal memory proposal = abi.decode(param, (CoverLib.Proposal));
            approvedProposals[proposal.id] = proposal;
        }

        emit ReceivedMessage(_origin, _sender, string(_message));
    }
}