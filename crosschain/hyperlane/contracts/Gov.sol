// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./libraries/openzeppelin/IERC20.sol";
import "./libraries/openzeppelin/Ownable.sol";
import "./libraries/openzeppelin/ReentrancyGuard.sol";
import "./libraries/hyperlane/IMessageRecipient.sol";
import "./libraries/hyperlane/IMailbox.sol";
import "./CoverLib.sol";

contract Governance is ReentrancyGuard, IMessageRecipient, Ownable {
    error VotingTimeElapsed();

    uint256 public proposalCounter;
    uint256 public votingDuration;
    uint256 public destinationDomain;
    mapping(uint256 => CoverLib.Proposal) public proposals;
    mapping(uint256 => mapping(address => CoverLib.Voter)) public voters;
    uint256[] public proposalIds;
    mapping (uint256 => bool) poolStatus;

    event ReceivedMessage(
        uint32 indexed origin,
        bytes32 indexed sender,
        string message
    );
    event ProposalCreated(
        uint256 indexed proposalId,
        address indexed creator,
        string description,
        CoverLib.RiskType riskType,
        uint256 claimAmount,
        CoverLib.ProposalStaus status
    );
    event VoteCast(
        address indexed voter,
        uint256 indexed proposalId,
        bool vote,
        uint256 weight
    );
    event ProposalExecuted(uint256 indexed proposalId, bool approved);


    IERC20 public governanceToken;
    IMailbox public mailbox;
    address public coverContract;
    address public poolContract;
    address public mailboxAddress;

    constructor(
        address _mailbox,
        address _governanceToken,
        address _insurancePool,
        uint256 _votingDuration,
        address _initialOwner
    ) Ownable(_initialOwner) {
        mailbox = IMailbox(_mailbox);
        mailboxAddress = _mailbox;
        governanceToken = IERC20(_governanceToken);
        poolContract = _insurancePool;
        votingDuration = _votingDuration * 1 minutes;
    }

    function createProposal(CoverLib.ProposalParams memory params) external {
        require(poolStatus[params.poolId], "Pool does not exist");
        require(params.claimAmount > 0, "Claim amount must be greater than 0");

        proposalCounter++;

        proposals[proposalCounter] = CoverLib.Proposal({
            id: proposalCounter,
            votesFor: 0,
            votesAgainst: 0,
            createdAt: block.timestamp,
            deadline: 0,
            timeleft: 0,
            executed: false,
            status: CoverLib.ProposalStaus.Submitted,
            proposalParam: params
        });

        proposalIds.push(proposalCounter); // Track the proposal ID

        emit ProposalCreated(
            proposalCounter,
            params.user,
            params.description,
            params.riskType,
            params.claimAmount,
            CoverLib.ProposalStaus.Submitted
        );
    }

    function vote(uint256 _proposalId, bool _vote) external {
        require(!voters[_proposalId][msg.sender].voted, "Already voted");
        CoverLib.Proposal storage proposal = proposals[_proposalId];
        require(proposal.createdAt != 0, "Proposal does not exist");
        
        if (proposal.status == CoverLib.ProposalStaus.Submitted) {
            proposal.status = CoverLib.ProposalStaus.Pending;
            proposal.deadline = block.timestamp + votingDuration;
            proposal.timeleft = (proposal.deadline - block.timestamp) / 1 minutes;
        } else if (block.timestamp >= proposal.deadline) {
            proposal.timeleft = 0;
            revert VotingTimeElapsed();
        }

        proposal.timeleft = (proposal.deadline - block.timestamp) / 1 minutes;
        uint256 voterWeight = governanceToken.balanceOf(msg.sender);
        require(voterWeight > 0, "No voting weight");

        voters[_proposalId][msg.sender] = CoverLib.Voter({
            voted: true,
            vote: _vote,
            weight: voterWeight
        });

        if (_vote) {
            proposal.votesFor += voterWeight;
        } else {
            proposal.votesAgainst += voterWeight;
        }

        emit VoteCast(msg.sender, _proposalId, _vote, voterWeight);
    }

    function executeProposal(uint256 _proposalId) external onlyOwner nonReentrant {
        CoverLib.Proposal storage proposal = proposals[_proposalId];
        require(proposal.status == CoverLib.ProposalStaus.Pending, "Proposal not pending");
        require(
            block.timestamp > proposal.deadline,
            "Voting period is still active"
        );
        require(!proposal.executed, "Proposal already executed");
        proposal.executed = true;

        if (proposal.votesFor > proposal.votesAgainst) {

            proposals[_proposalId].status = CoverLib.ProposalStaus.Approved;

            mailbox.dispatch(uint32(destinationDomain), addressToBytes32(coverContract), abi.encode("updateUserCoverValue", abi.encode(
                proposal.proposalParam.user,
                proposal.proposalParam.coverId,
                proposal.proposalParam.claimAmount
            )));

            CoverLib.Proposal memory mproposal = proposal;
            mailbox.dispatch(uint32(destinationDomain), addressToBytes32(poolContract), abi.encode("approvedProposals", abi.encode(mproposal)));

            emit ProposalExecuted(_proposalId, true);
        } else {
            proposals[_proposalId].status = CoverLib.ProposalStaus.Rejected;
            emit ProposalExecuted(_proposalId, false);
        }
    }

    function updateProposalStatusToClaimed(uint256 proposalId) public onlyMailboxOrCoverContract nonReentrant {
        proposals[proposalId].status = CoverLib.ProposalStaus.Claimed;
    }

    function setVotingDuration(uint256 _newDuration) external onlyOwner {
        require(_newDuration > 0, "Voting duration must be greater than 0");
        votingDuration = _newDuration;
    }

    function getProposalCount() public view returns (uint256) {
        return proposalCounter;
    }

    function getProposalDetails(
        uint256 _proposalId
    ) public returns (CoverLib.Proposal memory) {
        if (block.timestamp >= proposals[_proposalId].deadline) {
            proposals[_proposalId].timeleft = 0;
        } else {
            proposals[_proposalId].timeleft = (proposals[_proposalId].deadline - block.timestamp) / 1 minutes;
        }
        return proposals[_proposalId];
    }

    function getAllProposals() public view returns (CoverLib.Proposal[] memory) {
        CoverLib.Proposal[] memory result = new CoverLib.Proposal[](proposalIds.length);
        for (uint256 i = 0; i < proposalIds.length; i++) {
            result[i] = proposals[proposalIds[i]];
            if (block.timestamp >= result[i].deadline) {
                result[i].timeleft = 0;
            } else {
                result[i].timeleft = (result[i].deadline - block.timestamp) / 1 minutes;
            }
        }
        return result;
    }

        function getActiveProposals() public view returns (CoverLib.Proposal[] memory) {
        CoverLib.Proposal[] memory result = new CoverLib.Proposal[](proposalIds.length);
        for (uint256 i = 0; i < proposalIds.length; i++) {
            if (proposals[proposalIds[i]].status == CoverLib.ProposalStaus.Submitted || proposals[proposalIds[i]].deadline >= block.timestamp) {
                result[i] = proposals[proposalIds[i]];
                if (block.timestamp == result[i].deadline) {
                    result[i].timeleft = 0;
                } else {
                    result[i].timeleft = (result[i].deadline - block.timestamp) / 1 minutes;
                }
            }
        }
        return result;
    }

    function getPastProposals() public view returns (CoverLib.Proposal[] memory) {
        CoverLib.Proposal[] memory result = new CoverLib.Proposal[](proposalIds.length);
        for (uint256 i = 0; i < proposalIds.length; i++) {
            if (proposals[proposalIds[i]].status != CoverLib.ProposalStaus.Submitted && proposals[proposalIds[i]].deadline < block.timestamp) {
                result[i] = proposals[proposalIds[i]];
                result[i].timeleft = 0;
            }
        }
        return result;
    }

    function setCoverContract(address _coverContract) external onlyOwner {
        require(coverContract == address(0), "Governance already set");
        require(
            _coverContract != address(0),
            "Governance address cannot be zero"
        );
        coverContract = _coverContract;
    }

    modifier onlyMailboxOrCoverContract() {
        require(msg.sender == mailboxAddress || msg.sender == coverContract, "Not authorized");
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

        if (keccak256(abi.encodePacked(functionName)) == keccak256(abi.encodePacked("getProposalDetails"))) {
            uint256 proposalId = abi.decode(param, (uint256));
            getProposalDetails(proposalId);
        }

        if (keccak256(abi.encodePacked(functionName)) == keccak256(abi.encodePacked("updateProposalStatusToClaimed"))) {
            uint256 proposalId = abi.decode(param, (uint256));
            updateProposalStatusToClaimed(proposalId);
        }

        if (keccak256(abi.encodePacked(functionName)) == keccak256(abi.encodePacked("PoolActive"))) {
            uint256 poolId = abi.decode(param, (uint256));
            poolStatus[poolId] = true;
        }

        if (keccak256(abi.encodePacked(functionName)) == keccak256(abi.encodePacked("PoolDeactivated"))) {
            uint256 poolId = abi.decode(param, (uint256));
            poolStatus[poolId] = false;
        }

        emit ReceivedMessage(_origin, _sender, string(_message));
    }
}
