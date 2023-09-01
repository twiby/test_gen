use test_gen::test_with;

struct MockStruct {
    _a: u32,
}

#[test_with(u32, String, MockStruct)]
fn fwell<T>() {
    assert!(true);
}

#[test_with(u32, String, MockStruct)]
#[should_panic]
fn fwell2<T>() {
    assert!(false);
}
