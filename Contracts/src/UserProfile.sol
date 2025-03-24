// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract UserProfile {
    struct User {
        address userAddress;
        uint256[] viewedAds;
        uint256 totalEarned;
        bool registered;
    }

    mapping(address => User) public users;

    event UserRegistered(address indexed user);
    event AdViewed(address indexed user, uint256 adId);

    modifier onlyRegistered() {
        require(users[msg.sender].registered, "User not registered.");
        _;
    }

    function registerUser() external {
        require(!users[msg.sender].registered, "Already registered.");
        users , 0, true);
        emit UserRegistered(msg.sender);
    }

    function recordAdView(uint256 adId) external onlyRegistered {
        users[msg.sender].viewedAds.push(adId);
        emit AdViewed(msg.sender, adId);
    }

    function getViewedAds(address user) external view returns (uint256[] memory) {
        return users[user].viewedAds;
    }
}
