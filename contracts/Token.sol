pragma solidity ^0.8.0;

contract Token {
    string public name = 'My hardhat Token';
    string public symbol = 'MHT';
    uint public totalSupply = 1000000;
    address public owner;
    uint pos0;
    mapping(address => uint) balances;

    constructor() {
    	pos0 = 11;
        balances[msg.sender] = totalSupply;
        owner = msg.sender;
    }

    function transfer(address to, uint amount) external {
        require(balances[msg.sender] >= amount, 'Not enough tokens');
        balances[msg.sender] -= amount;
        balances[to] += amount;
    }

    function balanceOf(address account) external view returns(uint) {
        return balances[account];
    }
    
    event Solution(uint indexed _num);
    
    function solution() public view returns (string memory) {
        return "olalalalalal";
    }

   function set_pos0() public {
        pos0 = 2;
        emit Solution(2);
    }
}
