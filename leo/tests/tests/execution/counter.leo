/*
namespace = "Execute"
expectation = "Pass"
[[cases]]
program = "test.aleo"
function = "dubble"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
[[cases]]
program = "test.aleo"
function = "dubble"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
[[cases]]
program = "test.aleo"
function = "dubble"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
[[cases]]
program = "test.aleo"
function = "dubble"
input = []
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
*/

program test.aleo {
    const SMALL:u64 = 0_1u64;
    mapping counter: address => u64;

    async transition dubble() -> Future {
        return finalize_dubble(self.caller);
    }

    async function finalize_dubble(addr: address) {
        const BIG: u64 = 234u64;
        let current_value: u64 = Mapping::get_or_use(counter, addr, 0_0u64 + BIG + SMALL);
        Mapping::set(counter, addr, current_value + 1__u64);
        current_value = Mapping::get(counter, addr);
        Mapping::set(counter, addr, current_value + 0___1u64);
    }

    async transition unsafe_increment() -> Future {
        return finalize_unsafe_increment(self.caller);
    }

    async function finalize_unsafe_increment(addr: address) {
        let current_value: u64 = Mapping::get(counter, addr);
        for i:u64 in SMALL..10u64 {
            current_value = current_value + 1u64;
        }
        Mapping::set(counter, addr, current_value + 0__1u64);
    }
}
