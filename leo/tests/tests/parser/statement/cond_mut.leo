/*
namespace = "Compile"
expectation = "Pass"
*/

function main(y: bool) -> bool {
    let x: i8 = 100i8;
    if false {
        x = 1i8;
        x = x * 100i8;
    }
    
    return (x == 100i8) == y;
}
