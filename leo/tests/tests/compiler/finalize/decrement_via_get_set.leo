/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    mapping amounts: address => u128;

    async transition decrease_self(amount: u128) -> Future {
        return finalize_dec(self.caller, amount);
    }

    async function finalize_dec(addr: address, amount: u128) {
        let current_amount: u128 = Mapping::get_or_use(amounts, addr, 0u128);
        Mapping::set(amounts, addr, current_amount - amount);
    }
}