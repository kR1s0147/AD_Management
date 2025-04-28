// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "./ADRegistry.sol";
import "./UserProfile.sol";

contract DEADX is AdRegistry,UserProfile {

    constructor(address adxAddress, uint256 initialCreditFactor) AdRegistry(adxAddress, initialCreditFactor) UserProfile(msg.sender) {
        require(adxAddress != address(0), "Invalid ADX address.");
        require(initialCreditFactor > 0, "Invalid credit factor.");
    }

    event RewardsClaimed(address indexed user, uint256 amount);

    function claimRewards(address user) external returns(bool){
        require(userRewardPoints[user] > 0, "No rewards to claim.");
        uint256 rewardAmount = userRewardPoints[user]/creditFactor;
        userRewardPoints[user] = 0;
        require(adx.transfer(user, rewardAmount), "Transfer failed.");
        emit RewardsClaimed(user, rewardAmount);
        return true;
    }

    function update_credits( address[] memory ad_vendors,uint256[] memory credits) external onlyAdmin returns(bool){
        require(ad_vendors.length == credits.length, "Ad vendors and credits length mismatch.");
        for (uint256 i = 0; i < credits.length; i++) {
            AdVendors[msg.sender].Credits -= credits[i];
        }
        return true;
    }

    
}