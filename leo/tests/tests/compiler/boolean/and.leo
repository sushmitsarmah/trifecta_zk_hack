/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {    
    transition main(a: bool, b: bool) -> bool {
        return a && b;
    }}
