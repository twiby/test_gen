#![deny(warnings)]
use typed_test_gen::test_with;

struct MockStruct {
    _a: u32,
}

enum MockEnum {
    _FirstVariant,
    _SecondVariant,
    _ThirdVariant,
}

union MockUnion {
    _a: u32,
    _b: u64,
}

struct MockGenericStruct<T> {
    _a: T,
}

mod module {
    pub struct MockStructInModule {
        _a: u32,
    }

    pub struct MockGenericStructInModule<T> {
        _a: T,
    }
}

#[test_with(
    u32,
    String,
    MockStruct,
    MockEnum,
    MockUnion,
    MockGenericStruct<u32>,
    MockGenericStruct<u64>,
    module::MockStructInModule,
    module::MockGenericStructInModule<u32>,
    module::MockGenericStructInModule<u64>,
    MockGenericStruct<module::MockGenericStructInModule<u32>>
)]
fn test_struct<T>() {
    assert!(true);
}

#[test_with(
    u32,
    String,
    MockStruct,
    MockEnum,
    MockUnion,
    MockGenericStruct<u32>,
    MockGenericStruct<u64>,
    module::MockStructInModule,
    module::MockGenericStructInModule<u32>,
    module::MockGenericStructInModule<u64>,
    MockGenericStruct<module::MockGenericStructInModule<u32>>
)]
#[should_panic]
fn test_struct_fail<T>() {
    assert!(false);
}

#[test_with(
    u32,
    String,
    MockStruct,
    MockEnum,
    MockUnion,
    MockGenericStruct<u32>,
    MockGenericStruct<u64>,
    module::MockStructInModule,
    module::MockGenericStructInModule<u32>,
    module::MockGenericStructInModule<u64>,
    MockGenericStruct<module::MockGenericStructInModule<u32>>
)]
#[ignore]
fn test_ignored<T>() {
    unreachable!("This test should actually be ignored");
}

#[test_with(u32)]
fn with_result<T>() -> Result<(), T> {
    Ok(())
}

#[test_with(u32)]
#[should_panic]
fn fails_with_result<T: Default>() -> Result<(), T> {
    Err(T::default())
}
