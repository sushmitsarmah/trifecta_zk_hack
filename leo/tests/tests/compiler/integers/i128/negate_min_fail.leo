/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {    
    // TODO: This test passes, but constant propagation should detect an overflow.
    
    transition main(y: bool) -> i128 {
        let a: i128 = -170141183460469231731687303715884105728i128;
    
        return -a; // This line attempts to return 170141183460469231731687303715884105728i128 however, the largest i128 value is 170141183460469231731687303715884105727i128.
    }
}
