/*
namespace = "Execute"
expectation = "Pass"
[[cases]]
program = "test.aleo"
function = "main"
input = ["0u32", "1u32"]
[[cases]]
program = "test.aleo"
function = "main"
input = ["1u32", "1u32"]
[[cases]]
program = "test.aleo"
function = "main"
input = ["1u32", "2u32"]
[[cases]]
program = "test.aleo"
function = "main"
input = ["2u32", "2u32"]
*/

program test.aleo {
    transition main(x: u32, y: u32) -> bool {
        assert_eq(x, y);
        return x == y;
    }
}
