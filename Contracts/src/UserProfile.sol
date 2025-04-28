// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract UserProfile {

    mapping(address => uint256) public userRewardPoints;

    address public admin;

    constructor(address _admin) {
        admin = _admin;
    }

    modifier onlyAdmin() {
        require(msg.sender == admin, "Not the contract admin.");
        _;
    }
    event UserRewardPointsUpdated(address indexed user, uint256 points);

    function UpdateUserRewards(address[] memory users , uint256[] memory rewardPoints) onlyAdmin external returns(bool) {
        require(users.length == rewardPoints.length, "Users and reward points length mismatch.");
        for (uint256 i = 0; i < users.length; i++) {
            userRewardPoints[users[i]] += rewardPoints[i];
            emit UserRewardPointsUpdated(users[i], userRewardPoints[users[i]]);
        }
        return true;
    }

    function getUserRewardPoints(address user) external view returns (uint256) {
        return userRewardPoints[user];
    }

    function getAdmin() external view returns (address) {
        return admin;
    }

    function setAdmin(address newAdmin) external onlyAdmin {
        require(newAdmin != address(0), "Invalid address.");
        admin = newAdmin;
    }
}
