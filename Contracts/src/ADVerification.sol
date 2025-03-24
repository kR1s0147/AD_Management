// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract AdVerification {
    mapping(address => mapping(uint256 => bool)) public adViewStatus;

    event AdVerified(address indexed user, uint256 adId, bool valid);

    function verifyAdView(address user, uint256 adId) external returns (bool) {
        require(!adViewStatus[user][adId], "Ad already viewed.");
        adViewStatus[user][adId] = true;
        emit AdVerified(user, adId, true);
        return true;
    }

    function hasViewedAd(address user, uint256 adId) external view returns (bool) {
        return adViewStatus[user][adId];
    }
}
