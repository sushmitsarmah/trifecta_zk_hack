/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    transition foo(a: u8, b: u8, flag: bool) -> u8 {
        let start: (u8, u8) = (a, b);
        for i: u8 in 0u8..16u8 {
            start = (start.0 + start.1, start.1 + 1u8);
            if flag {
                start = (start.1, start.0 + start.0);
            }
        }
        return start.0 + start.1;
    }
}
