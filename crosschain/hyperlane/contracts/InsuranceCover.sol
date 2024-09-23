// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./libraries/openzeppelin/IERC20.sol";
import "./libraries/openzeppelin/Ownable.sol";
import "./libraries/openzeppelin/ReentrancyGuard.sol";
import "./libraries/hyperlane/IMessageRecipient.sol";
import "./libraries/hyperlane/IMailbox.sol";
import "./CoverLib.sol";

interface ILP {
    struct Deposits {
        address lp;
        uint256 amount;
        uint256 poolId;
        uint256 dailyPayout;
        Status status;
        uint256 daysLeft;
        uint256 startDate;
        uint256 expiryDate;
        uint256 accruedPayout;
    }

    struct Pool {
        string poolName;
        CoverLib.RiskType riskType;
        uint256 apy;
        uint256 minPeriod;
        uint256 tvl;
        uint256 tcp; // Total claim paid to users
        bool isActive; // Pool status to handle soft deletion
        uint256 percentageSplitBalance;
        mapping(address => Deposits) deposits; // Mapping of user address to their deposit
    }

    enum Status {
        Active,
        Expired
    }

    function getUserDeposit(
        uint256 _poolId,
        address _user
    ) external view returns (Deposits memory);

    function getPool(
        uint256 _poolId
    )
        external
        view
        returns (
            string memory name,
            CoverLib.RiskType riskType,
            uint256 apy,
            uint256 minPeriod,
            uint256 tvl,
            bool isActive,
            uint256 percentageSplitBalance
        );

        function reducePercentageSplit(uint256 _poolId,uint256 __poolPercentageSplit) external ;
        function increasePercentageSplit(uint256 _poolId,uint256 __poolPercentageSplit) external ;
        function addPoolCover(uint256 _poolId, CoverLib.Cover memory _cover) external ;
        function updatePoolCovers(uint256 _poolId, CoverLib.Cover memory _cover) external ;
        function getPoolCovers(uint256 _poolId) external view returns (CoverLib.Cover[] memory);
}

contract InsuranceCover is ReentrancyGuard, IMessageRecipient, Ownable {
    using CoverLib for *;
    error LpNotActive();
    error InsufficientPoolBalance();
    error NoClaimableReward();
    error InvalidCoverDuration();
    error CoverNotAvailable();
    error NameAlreadyExists();
    error InvalidAmount();
    error UnsupportedCoverType();
    error WrongPool();

    uint public coverFeeBalance;
    ILP public lpContract;
    IMailbox public mailbox;
    address public mailboxAddress;
    address public lpAddress;
    address public governance;

    mapping(uint256 => bool) public coverExists;
    mapping(address => mapping (uint256 => uint256)) public NextLpClaimTime;

    mapping(address => mapping(uint256 => CoverLib.GenericCoverInfo)) public userCovers;
    mapping(uint256 => CoverLib.Cover) public covers;

    uint256 public coverCount;

    event ReceivedMessage(
        uint32 indexed origin,
        bytes32 indexed sender,
        string message
    );
    event CoverCreated(
        string name,
        CoverLib.RiskType riskType
    );
    event CoverPurchased(
        address indexed user,
        uint256 coverValue,
        uint256 coverFee,
        CoverLib.RiskType riskType
    );
    event PayoutClaimed(
        address indexed user,
        uint256 indexed poolId,
        uint256 amount
    );
    event CoverUpdated(
        uint256 indexed coverId,
        string coverName,
        CoverLib.RiskType riskType
    );

    constructor(
        address _mailbox,
        address _lpContract,
        address _initialOwner,
        address _governance
    ) Ownable(_initialOwner) {
        mailbox = IMailbox(_mailbox);
        mailboxAddress = _mailbox;
        lpContract = ILP(_lpContract);
        lpAddress = _lpContract;
        governance = _governance;
    }

    function createCover(
        string memory _cid,
        CoverLib.RiskType _riskType,
        string memory _coverName,
        string memory _chains,
        uint256 _capacity,
        uint256 _cost,
        uint256 _poolId
    ) public onlyOwner {
        CoverLib.Cover[] memory coversInPool = lpContract.getPoolCovers(_poolId);
        for (uint256 i = 0; i < coversInPool.length; i++) {
            if (keccak256(abi.encodePacked(coversInPool[i].coverName)) == keccak256(abi.encodePacked(_coverName))) {
                revert NameAlreadyExists();
            }
        }
        (, CoverLib.RiskType risk, , , uint256 tvl, , uint256 _percentageSplitBalance) = lpContract.getPool(_poolId);

        if (risk != _riskType || _capacity > _percentageSplitBalance) {
            revert WrongPool();
        }

        uint256 _maxAmount = tvl * (_capacity * 1e18 / 100) / 1e18;

        lpContract.reducePercentageSplit(_poolId, _capacity);

        coverCount++;
        CoverLib.Cover memory cover =  CoverLib.Cover({
            id: coverCount,
            coverName: _coverName,
            riskType: _riskType,
            chains: _chains,
            capacity: _capacity,
            cost: _cost,
            capacityAmount: _maxAmount,
            coverValues: 0,
            maxAmount: _maxAmount,
            poolId: _poolId,
            CID: _cid
        });
        covers[coverCount] = cover;
        lpContract.addPoolCover(_poolId, cover);
        coverExists[coverCount] = true;

        emit CoverCreated(_coverName, _riskType);
    }

    function updateCover(
        uint256 _coverId,
        string memory _coverName, 
        CoverLib.RiskType _riskType, 
        string memory _cid,
        string memory _chains,
        uint256 _capacity,
        uint256 _cost,
        uint256 _poolId
        ) public onlyOwner {
        (, CoverLib.RiskType risk, , , uint256 tvl, , uint256 _percentageSplitBalance) = lpContract.getPool(_poolId);
        
        if (risk != _riskType || _capacity > _percentageSplitBalance) {
            revert WrongPool();
        }

        CoverLib.Cover storage cover = covers[_coverId];

        uint256 _maxAmount = tvl * (_capacity * 1e18 / 100) / 1e18;

        if (cover.coverValues > _maxAmount) {
            revert WrongPool();
        }

        CoverLib.Cover[] memory coversInPool = lpContract.getPoolCovers(_poolId);
        for (uint256 i = 0; i < coversInPool.length; i++) {
            if (keccak256(abi.encodePacked(coversInPool[i].coverName)) == keccak256(abi.encodePacked(_coverName)) &&
                coversInPool[i].id != _coverId) {
                revert NameAlreadyExists();
            }
        }

        uint256 oldCoverCapacity = cover.capacity;

        cover.coverName = _coverName;
        cover.chains = _chains;
        cover.capacity = _capacity;
        cover.cost = _cost;
        cover.CID = _cid;
        cover.capacityAmount = _maxAmount;
        cover.poolId = _poolId;
        cover.maxAmount = _maxAmount - cover.coverValues;

        if (oldCoverCapacity > _capacity) {
            uint256 difference = oldCoverCapacity - _capacity;
            lpContract.increasePercentageSplit(_poolId, difference);
        } else if (oldCoverCapacity < _capacity) {
            uint256 difference = _capacity - oldCoverCapacity;
            lpContract.reducePercentageSplit(_poolId, difference);
        }

        lpContract.updatePoolCovers(_poolId, cover);

        emit CoverUpdated(_coverId, _coverName, _riskType);
    }

    function purchaseCover(
        uint256 _coverId,
        uint256 _coverValue,
        uint256 _coverPeriod
    ) public payable nonReentrant {
        if (msg.value <= 0) {
            revert InvalidAmount();
        }
        if (_coverPeriod <= 27 || _coverPeriod >= 366) {
            revert InvalidCoverDuration();
        }
        if (!coverExists[_coverId]) {
            revert CoverNotAvailable();
        }

        CoverLib.Cover storage cover = covers[_coverId];
        if (_coverValue > cover.maxAmount) {
            revert InsufficientPoolBalance();
        }

        uint256 newCoverValues = cover.coverValues + _coverValue;

        if (newCoverValues > cover.capacityAmount) {
            revert InsufficientPoolBalance();
        }

        cover.coverValues = newCoverValues;
        cover.maxAmount = cover.capacityAmount - newCoverValues;

        cover.maxAmount = (cover.capacityAmount - cover.coverValues);
        CoverLib.GenericCoverInfo storage userCover = userCovers[msg.sender][_coverId];
        
        if (userCover.coverValue == 0) {
            userCovers[msg.sender][_coverId] = CoverLib.GenericCoverInfo({
                user: msg.sender,
                coverId: _coverId,
                riskType: cover.riskType,
                coverName: cover.coverName,
                coverValue: _coverValue,
                claimPaid: 0,
                coverPeriod: _coverPeriod,
                endDay: block.timestamp + (_coverPeriod * 1 days),
                isActive: true
            });
        } else {
            require((userCover.coverPeriod + _coverPeriod) < 366, "Total cover period must be less than 365");
            userCover.coverValue += _coverValue;
            userCover.coverPeriod += _coverPeriod;
            userCover.endDay += (userCover.coverPeriod * 1 days);
        }

        coverFeeBalance += msg.value;

        emit CoverPurchased(msg.sender, _coverValue, msg.value, cover.riskType);
    }

    function getAllUserCovers(address user) external view returns (CoverLib.GenericCoverInfo[] memory) {
        CoverLib.GenericCoverInfo[] memory userCoverList = new CoverLib.GenericCoverInfo[](coverCount);
        uint256 actualCount = 0;

        for (uint256 i = 1; i <= coverCount; i++) {
            if (userCovers[user][i].coverValue > 0) {
                userCoverList[actualCount] = userCovers[user][i];
                actualCount++;
            }
        }

        assembly {
            mstore(userCoverList, actualCount)
        }

        return userCoverList;
    }

    function getAllAvailableCovers() external view returns (CoverLib.Cover[] memory) {
        CoverLib.Cover[] memory availableCovers = new CoverLib.Cover[](coverCount);
        uint256 actualCount = 0;

        for (uint256 i = 1; i <= coverCount; i++) {
            if (coverExists[i]) {
                availableCovers[actualCount] = covers[i];
                actualCount++;
            }
        }
        assembly {
            mstore(availableCovers, actualCount)
        }
        return availableCovers;
    }

    function getUserCoverInfo(address user, uint256 _coverId) external view returns (CoverLib.GenericCoverInfo memory) {
        return userCovers[user][_coverId];
    }

    function updateUserCoverValue(address user, uint256 _coverId, uint256 _claimPaid) public onlyMailboxOrGovernance nonReentrant {
        userCovers[user][_coverId].coverValue -= _claimPaid;
        userCovers[user][_coverId].claimPaid += _claimPaid;
    }

    function deleteExpiredUserCovers(address _user) external nonReentrant {
        for (uint256 i = 1; i <= coverCount; i++) {
            CoverLib.GenericCoverInfo storage userCover = userCovers[_user][i];
            if (userCover.isActive && block.timestamp > userCover.endDay) {
                userCover.isActive = false;
                delete userCovers[_user][i];
            }
        }
    }

    function getCoverFeeBalance() external view returns (uint256) {
        return coverFeeBalance;
    }

    function increaseCoverFeeBalance() public payable onlyOwner nonReentrant {
        require(msg.value > 0, "Value must be greater than 0");
        coverFeeBalance += msg.value;
    }

    function updateMaxAmount(uint256 _coverId) public onlyPool nonReentrant {
        CoverLib.Cover storage cover = covers[_coverId];
        (, , , , uint256 tvl, , ) = lpContract.getPool(cover.poolId);
        require(tvl > 0, "TVL is zero");
        require(cover.capacity > 0, "Invalid cover capacity");
        uint256 amount = tvl * (cover.capacity * 1e18 / 100) / 1e18;
        covers[_coverId].capacityAmount = amount;
        covers[_coverId].maxAmount = (covers[_coverId].capacityAmount - covers[_coverId].coverValues);
    }

    function claimPayoutForLP(uint256 _poolId) external nonReentrant {
        ILP.Deposits memory depositInfo = lpContract.getUserDeposit(_poolId, msg.sender);
        if (depositInfo.status != ILP.Status.Active) {
            revert LpNotActive();
        }
        
        uint256 lastClaimTime;
        if (NextLpClaimTime[msg.sender][_poolId] == 0) {
            lastClaimTime = depositInfo.startDate;
        } else {
            lastClaimTime = NextLpClaimTime[msg.sender][_poolId];
        }

        uint256 currentTime = block.timestamp;
        if (currentTime > depositInfo.expiryDate) {
            currentTime = depositInfo.expiryDate;
        }

        // uint256 claimableDays = (currentTime - lastClaimTime) / 1 days;   Uncomment

        uint256 claimableDays = (currentTime - lastClaimTime) / 5 minutes;

        if (claimableDays <= 0) {
            revert NoClaimableReward();
        }
        uint256 claimableAmount = depositInfo.dailyPayout * claimableDays;

        if (claimableAmount > coverFeeBalance) {
            revert InsufficientPoolBalance();
        }
        NextLpClaimTime[msg.sender][_poolId] = block.timestamp;

        (bool success, ) = msg.sender.call{value: claimableAmount}("");
        require(success, "Transfer failed");

        coverFeeBalance -= claimableAmount;

        emit PayoutClaimed(msg.sender, _poolId, claimableAmount);
    }

    function getDepositClaimableDays(address user, uint256 _poolId) public view returns (uint256) {
        ILP.Deposits memory depositInfo = lpContract.getUserDeposit(_poolId, user);

        uint256 lastClaimTime;
        if (NextLpClaimTime[user][_poolId] == 0) {
            lastClaimTime = depositInfo.startDate;
        } else {
            lastClaimTime = NextLpClaimTime[user][_poolId];
        }
        uint256 currentTime = block.timestamp;
        if (currentTime > depositInfo.expiryDate) {
            currentTime = depositInfo.expiryDate;
        }
        uint256 claimableDays = (currentTime - lastClaimTime) / 5 minutes;

        return claimableDays;
    }

    function getLastClaimTime(address user, uint256 _poolId)public view returns (uint256) {
        return NextLpClaimTime[user][_poolId];
    } 

    modifier onlyMailboxOrGovernance() {
        require(msg.sender == mailboxAddress || msg.sender == governance, "Not authorized");
        _;
    }

    modifier onlyPool() {
        require(msg.sender == lpAddress, "Not authorized");
        _;
    }

    function handle(uint32 _origin, bytes32 _sender, bytes memory _message) external payable override {
        require(msg.sender == address(mailbox), "Sender must be mailbox");

        (string memory functionName, bytes memory param) = abi.decode(_message, (string, bytes));

        if (keccak256(abi.encodePacked(functionName)) == keccak256(abi.encodePacked("updateUserCoverValue"))) {
            (address user, uint256 coverId, uint256 claimPaid) = abi.decode(param, (address, uint256, uint256));
            updateUserCoverValue(user, coverId, claimPaid);
        }

        emit ReceivedMessage(_origin, _sender, string(_message));
    }
}