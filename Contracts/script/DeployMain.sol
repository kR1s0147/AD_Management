// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "../src/ERC20.sol";
import "../src/ADRegistry.sol";
import "../src/UserProfile.sol";
import "../src/main.sol";

contract Deploy is Script {
    function run() external {
                // Load private key from environment variable or a .env file
        uint256 privateKey = vm.envUint("PRIVATE_KEY");

        // Start a new broadcast using the private key
        vm.startBroadcast(privateKey);

        // Deploy the ADX token
        ADX adx = new ADX();
        // Deploy the DEADX contract with the ADX token address and initial credit factor
        DEADX deadx = new DEADX(address(adx), 1000);
        // Set the ADManagement address in the ADX contract
        adx.setADManagement(address(deadx));

        adx.mintInitialSupply();
       
        // Stop the broadcast
        vm.stopBroadcast();
        // Print the deployed contract addresses
        console.log("ADX Token deployed at:", address(adx));
        console.log("DEADX contract deployed at:", address(deadx));
        // Print the initial supply of the ADX token
        console.log("Initial supply of ADX Token:", adx.INITIAL_SUPPLY());

    }
}
