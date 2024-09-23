// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// 0xDA01D79Ca36b493C7906F3C032D2365Fb3470aEC
// 0xb650bedaAAf173366D59d8ef74f571aCAFA0a6f1
library CoverLib {
    struct Cover {
        uint256 id;
        string coverName;
        RiskType riskType;
        string chains;
        uint256 capacity;
        uint256 cost;
        uint256 capacityAmount;
        uint256 coverValues;
        uint256 maxAmount;
        uint256 poolId;
        string CID;
    }

    struct GenericCoverInfo {
        address user;
        uint256 coverId;
        RiskType riskType;
        string coverName;
        uint256 coverValue; // This is the value of the cover purchased
        uint256 claimPaid;
        uint256 coverPeriod; // This is the period the cover is purchased for in days
        uint256 endDay; // When the cover expires
        bool isActive;
    }

    enum RiskType {
        Slashing,
        SmartContract,
        Stablecoin,
        Protocol
    }

    struct GenericCover {
        RiskType riskType;
        bytes coverData;
    }

        struct Voter {
        bool voted;
        bool vote;
        uint256 weight;
    }

    struct Proposal {
        uint256 id;
        uint256 votesFor;
        uint256 votesAgainst;
        uint256 createdAt;
        uint256 deadline;
        uint256 timeleft;
        ProposalStaus status;
        bool executed;
        ProposalParams proposalParam;
    }

    struct ProposalParams {
        address user;
        CoverLib.RiskType riskType;
        uint256 coverId;
        string txHash;
        string description;
        uint256 poolId;
        uint256 claimAmount;
    }

    enum ProposalStaus {
        Submitted, 
        Pending,
        Approved,
        Claimed,
        Rejected
    }

        struct Pool {
        string poolName;
        CoverLib.RiskType riskType;
        uint256 apy;
        uint256 minPeriod;
        uint256 tvl;
        uint256 tcp;
        bool isActive;
        uint256 percentageSplitBalance;
        mapping(address => Deposits) deposits;
    }

    struct NPool {
        string poolName;
        CoverLib.RiskType riskType;
        uint256 apy;
        uint256 minPeriod;
        uint256 tvl;
        uint256 tcp;
        bool isActive;
        uint256 percentageSplitBalance;
    }

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

    struct PoolInfo {
        string poolName;
        uint256 poolId;
        uint256 dailyPayout;
        uint256 depositAmount;
        uint256 apy;
        uint256 minPeriod;
        uint256 tvl;
        uint256 tcp;
        bool isActive;
        uint256 accruedPayout;
    }

    enum Status {
        Active,
        Withdrawn
    }


}
