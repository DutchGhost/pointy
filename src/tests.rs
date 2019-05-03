use super::{access::Unique, pointer::Ptr};

#[test]
fn normal_pointer_deref() {
    let n = 0;

    let pointer = Ptr::new(&n);

    assert_eq!(*pointer, 0);
}

#[test]
fn normal_pointer_deref_mut() {
    let mut n = 0;
    let mut pointer = Ptr::<_, _, Unique>::new(&mut n);

    *pointer = 42;

    assert_eq!(*pointer, 42);
}

#[test]
fn slice_deref() {
    let a = [1, 2, 3, 4, 5];

    let pointer = Ptr::new(&a);

    assert_eq!(*pointer, [1, 2, 3, 4, 5]);
}

#[test]
fn slice_deref_mut() {
    let mut a = [1, 2, 3, 4, 5];

    let mut pointer = Ptr::<_, _, Unique>::new(&mut a);

    for elem in pointer.iter_mut() {
        *elem += 1;
    }

    assert_eq!(*pointer, [2, 3, 4, 5, 6]);
}

#[test]
fn test_fn_trait() {
    let closure = || 0;

    let fn_trait_obj: &dyn Fn() -> i32 = &closure;

    let pointer = Ptr::new(fn_trait_obj);

    // Prove we can copy it.
    let copy = pointer;

    assert_eq!(pointer(), 0);
    assert_eq!(copy(), 0);
}

#[test]
fn test_fn_mut_trait() {
    use core::ops::DerefMut;

    let mut n = 0;
    let mut closure = || {
        n += 1;
        n
    };

    let fn_mut_trait_obj: &'_ mut dyn FnMut() -> i32 = &mut closure;

    let mut pointer = Ptr::<_, _, Unique>::new(fn_mut_trait_obj);

    // let copy = pointer; <-- can't copy!

    assert_eq!(pointer.deref_mut()(), 1);
    // drop(closure); <-- also can't drop, `closure` is still in use.
    assert_eq!(pointer.deref_mut()(), 2);
    assert_eq!(n, 2);
}
