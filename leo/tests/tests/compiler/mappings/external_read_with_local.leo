/*
namespace = "Compile"
expectation = "Pass"
*/

program registry.aleo {
    mapping users: address => bool;

    async transition register() -> Future {
        return finalize_register(self.caller);
    }
    async function finalize_register(addr: address) {
        Mapping::set(users, addr, true);
    }
}

// --- Next Program --- //

import registry.aleo;
program relay.aleo {
    mapping users: address => bool;

    async transition send(addr: address) -> Future {
        return finalize_send(addr);
    }
    async function finalize_send(addr: address) {
        let a:bool = Mapping::get(relay.aleo/users, addr);
        assert_eq(a, true);
    }
}
