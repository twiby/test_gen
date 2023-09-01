use test_gen::test_with;

struct MockStruct {
    _a: u32,
}

#[test_with(u32, String, MockStruct)]
fn test_struct<T>() {
    assert!(true);
}

#[test_with(u32, String, MockStruct)]
#[should_panic]
fn test_struct_fail<T>() {
    assert!(false);
}
