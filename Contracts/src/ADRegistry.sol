// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "./ERC20.sol";

contract AdRegistry {
    struct AdVendor {
        bool isActive;
        uint256 Credits;
    }

    uint256 public creditFactor;

    address public owner;

    ADX public adx;

    modifier onlyOwner() {
        require(msg.sender == owner, "Not the contract owner.");
        _;
    }
    modifier onlyAdVendor() {
        require(AdVendors[msg.sender].isActive, "Not an AdVendor.");
        _;
    }

    constructor(address adxAddress, uint256 initialCreditFactor) {
        require(adxAddress != address(0), "Invalid ADX address.");
        require(initialCreditFactor > 0, "Invalid credit factor.");
        creditFactor = initialCreditFactor;
        adx = ADX(adxAddress);
        owner = msg.sender;
    }

    mapping(address => AdVendor) public AdVendors;

    event AdVendorRegistered(address indexed);
    event CreditsPurchased(address indexed advendor, uint256 amount);

    function register(address advendor) external returns (bool) {
        require(!AdVendors[advendor].isActive, "Already registered.");
        AdVendors[advendor] = AdVendor(true, 0);
        emit AdVendorRegistered(advendor);
        return true;
    }

    function BuyCredits(address advendor, uint256 amount) onlyAdVendor external returns (bool) {
        require(AdVendors[advendor].isActive, "Not registered.");
        require(amount > 0, "Invalid amount.");
        require(adx.balanceOf(msg.sender) >= amount, "Insufficient ADX balance.");
        require(adx.allowance(msg.sender, address(this)) >= amount, "Insufficient allowance.");
        require(adx.transferFrom(msg.sender, address(this), amount), "Transfer failed.");
        AdVendors[advendor].Credits += amount * creditFactor;
        emit CreditsPurchased(advendor, amount);
        return true;
    }

    function previewCredits(uint256 amount) external view returns (uint256) {
        return amount * creditFactor;
    }

    function getCreditFactor() external view returns (uint256) {
        return creditFactor;
    }

    function getCredits(address advendor) external view returns (uint256) {
        return AdVendors[advendor].Credits;
    }

    function setCreditFactor(uint256 factor) onlyOwner external returns (bool) {
        creditFactor = factor;
        return true;
    }

    function getAdVendorDetails(address advendor) external view returns (AdVendor memory) {
        return AdVendors[advendor];
    }

    function getAdVendorStatus(address advendor) external view returns (bool) {
        return AdVendors[advendor].isActive;
    }
}
