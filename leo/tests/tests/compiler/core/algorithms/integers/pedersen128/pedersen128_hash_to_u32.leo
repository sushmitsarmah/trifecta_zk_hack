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
    ) -> u32 {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";
    
        // Pedersen128 hash_to_u32 tests
        let a: u32 = Pedersen128::hash_to_u32(addr_value);
        let b: u32 = Pedersen128::hash_to_u32(bool_value);
        let c: u32 = Pedersen128::hash_to_u32(field_value);
        let d: u32 = Pedersen128::hash_to_u32(group_value);
        let e: u32 = Pedersen128::hash_to_u32(i8_value);
        let f: u32 = Pedersen128::hash_to_u32(i16_value);
        let g: u32 = Pedersen128::hash_to_u32(i32_value);
        let h: u32 = Pedersen128::hash_to_u32(i64_value);
        let j: u32 = Pedersen128::hash_to_u32(u8_value);
        let k: u32 = Pedersen128::hash_to_u32(u16_value);
        let l: u32 = Pedersen128::hash_to_u32(u32_value);
        let m: u32 = Pedersen128::hash_to_u32(u64_value);
        let o: u32 = Pedersen128::hash_to_u32(scalar_value);
        // let p: u32 = Pedersen128::hash_to_u32(string_value);
        let q: u32 = Pedersen128::hash_to_u32(Foo { a: 1u32, b: 1u32, });
    
        return a;
    }
}
