/*
namespace = "Execute"
expectation = "Pass"
[[cases]]
program = "test.aleo"
function = "main"
input = ["0group"]
[[cases]]
program = "test.aleo"
function = "main"
input = ["2group"]
*/

program test.aleo {
    transition main(a: group) -> (field, field) {
        let b: field = group::to_x_coordinate(a);
        let c: field = a as field;
        let e: field = a.to_x_coordinate();
        assert_eq(b, c);
        assert_eq(b, e);
        let d: field = group::to_y_coordinate(a);
        let f: field = a.to_y_coordinate();
        assert_eq(d, f);
        return (b, d);
    }
}
