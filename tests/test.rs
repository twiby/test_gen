use test_gen::test_with;

struct MockStruct {
    _a: u32,
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

#[test_with(u32, u64, char)]
fn test_vec<T>() {
    let vec = Vec::<T>::with_capacity(10);
    assert_eq!(vec.len(), 0);
    assert!(vec.capacity() >= 10);
}
#[test_with(u32, u64, char)]
#[should_panic]
fn test_vec_fail<T>() {
    let vec = Vec::<T>::with_capacity(10);
    assert_eq!(vec.len(), 0);
    assert!(vec.capacity() < 10);
}
