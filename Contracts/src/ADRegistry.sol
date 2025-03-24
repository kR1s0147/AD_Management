// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract AdRegistry {
    struct Ad {
        uint256 adId;
        address advertiser;
        string adData; // IPFS hash or URL for ad content
        uint256 budget;
        uint256 rewardPerView;
        uint256 totalViews;
        bool isActive;
    }

    mapping(uint256 => Ad) public ads;
    uint256 public adCounter;

    event AdCreated(uint256 indexed adId, address indexed advertiser);
    event AdUpdated(uint256 indexed adId, bool isActive);

    modifier onlyAdvertiser(uint256 adId) {
        require(ads[adId].advertiser == msg.sender, "Not the ad owner.");
        _;
    }

    function createAd(string memory adData, uint256 budget, uint256 rewardPerView) external {
        adCounter++;
        ads[adCounter] = Ad(adCounter, msg.sender, adData, budget, rewardPerView, 0, true);
        emit AdCreated(adCounter, msg.sender);
    }

    function updateAdStatus(uint256 adId, bool isActive) external onlyAdvertiser(adId) {
        ads[adId].isActive = isActive;
        emit AdUpdated(adId, isActive);
    }

    function getAd(uint256 adId) external view returns (Ad memory) {
        return ads[adId];
    }
}
