/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {    
    record Token {
        owner: address,
        amount: u128,
    }
    
    mapping amounts: address => u128;
    mapping tokens: address => Token;
    
    async transition decrease_self(amount: u128) -> Future {
        return finalize_decrease_self(self.caller, amount);
    }
    
    async function finalize_decrease_self(addr: address, amount: u128) {
        Mapping::get(tokens, true);
        tokens.get(true);
        Mapping::get(amounts, 1u8);
        amounts.get(1u8);
    }
}
