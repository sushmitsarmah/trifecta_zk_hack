/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {    
    struct Foo {
        a: u16,
        b: u16,
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
    ) -> i8 {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Pedersen64 hash_to_i8 tests
        let a: i8 = Pedersen64::hash_to_i8(addr_value);
        let b: i8 = Pedersen64::hash_to_i8(bool_value);
        let c: i8 = Pedersen64::hash_to_i8(field_value);
        let d: i8 = Pedersen64::hash_to_i8(group_value);
        let e: i8 = Pedersen64::hash_to_i8(i8_value);
        let f: i8 = Pedersen64::hash_to_i8(i16_value);
        let g: i8 = Pedersen64::hash_to_i8(i32_value);
        let j: i8 = Pedersen64::hash_to_i8(u8_value);
        let k: i8 = Pedersen64::hash_to_i8(u16_value);
        let l: i8 = Pedersen64::hash_to_i8(u32_value);
        let o: i8 = Pedersen64::hash_to_i8(scalar_value);
        // let p: i8 = Pedersen64::hash_to_i8(string_value);
        let q: i8 = Pedersen64::hash_to_i8(Foo { a: 1u16, b: 1u16, });
    
        return a;
    }
}
