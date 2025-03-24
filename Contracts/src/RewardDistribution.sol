// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "./TokenContract.sol";
import "./AdRegistry.sol";
import "./UserProfile.sol";

contract RewardDistributor {
    TokenContract public token;
    AdRegistry public adRegistry;
    UserProfile public userProfile;

    event RewardDistributed(address indexed user, uint256 amount);

    constructor(address tokenAddress, address adRegistryAddress, address userProfileAddress) {
        token = TokenContract(tokenAddress);
        adRegistry = AdRegistry(adRegistryAddress);
        userProfile = UserProfile(userProfileAddress);
    }

    function distributeRewards(address user, uint256 adId) external {
        require(userProfile.users(user).registered, "User not registered.");
        AdRegistry.Ad memory ad = adRegistry.getAd(adId);
        require(ad.isActive, "Ad is not active.");
        require(ad.budget >= ad.rewardPerView, "Insufficient budget.");

        adRegistry.ads(adId).totalViews++;
        adRegistry.ads(adId).budget -= ad.rewardPerView;
        token.transfer(user, ad.rewardPerView);

        emit RewardDistributed(user, ad.rewardPerView);
    }
}
