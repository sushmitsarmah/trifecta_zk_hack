/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {
    struct Foo {
        a: u32,
        b: u32,
    }

    transition main(
        i8_value: i8,
        i16_value: i16,
        i32_value: i32,
        i64_value: i64,
        u8_value: u8,
        u16_value: u16,
        u32_value: u32,
        u64_value: u64,
    ) -> bool {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";

        // Pedersen128 commit_to_address tests
        let a: address = Pedersen128::commit_to_address(addr_value, scalar_value);
        let b: address = Pedersen128::commit_to_address(bool_value, scalar_value);
        let c: address = Pedersen128::commit_to_address(field_value, scalar_value);
        let d: address = Pedersen128::commit_to_address(group_value, scalar_value);
        let e: address = Pedersen128::commit_to_address(i8_value, scalar_value);
        let f: address = Pedersen128::commit_to_address(i16_value, scalar_value);
        let g: address = Pedersen128::commit_to_address(i32_value, scalar_value);
        let h: address = Pedersen128::commit_to_address(i64_value, scalar_value);
        let j: address = Pedersen128::commit_to_address(u8_value, scalar_value);
        let k: address = Pedersen128::commit_to_address(u16_value, scalar_value);
        let l: address = Pedersen128::commit_to_address(u32_value, scalar_value);
        let m: address = Pedersen128::commit_to_address(u64_value, scalar_value);
        let o: address = Pedersen128::commit_to_address(scalar_value, scalar_value);
        // let j11: address = Pedersen128::commit_to_address(string_value);
        let p: address = Pedersen128::commit_to_address(Foo { a: 1u32, b: 1u32 }, scalar_value);

        return a == j;
    }}
