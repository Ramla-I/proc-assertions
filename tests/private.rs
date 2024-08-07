#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;

mod simple_tests {
    #[test]
    fn test_assert_private_fields() {
        #[private_fields("field1")]
        struct TestStruct {
            field1: i32,
            pub field2: String,
        }
    }
}

// Uncommenting the following code should 
// trigger a compile-time error
// Are you sure? no compile-time error for me
#[private_fields]
struct InvalidStruct {
    pub field1: i32,
    field2: String,
}
