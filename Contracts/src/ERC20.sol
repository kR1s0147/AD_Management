// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract ADX is ERC20, Ownable {
    uint256 public constant INITIAL_SUPPLY = 1_000_000 * 10 ** 18;

    address public ADManagement ;

    constructor() ERC20("AdRewardToken", "ADX") Ownable(msg.sender) {
    }

    function mint(address to, uint256 amount) external onlyOwner {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) external onlyOwner {
        _burn(from, amount);
    }

    function setADManagement(address _ADManagement) external onlyOwner {
        require(_ADManagement != address(0), "Invalid address.");
        ADManagement = _ADManagement;
    }

    function getADManagement() external view returns (address) {
        return ADManagement;
    }

    function mintInitialSupply() external onlyOwner {
        _mint(ADManagement, INITIAL_SUPPLY);
        _mint(msg.sender, INITIAL_SUPPLY);
    }
}
