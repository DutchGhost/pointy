/// Fake stdlib's [TraitObject](https://doc.rust-lang.org/std/raw/struct.TraitObject.html)
/// That requires nightly, but this works on stable.
/// This has to be used with care, and might break in the future,
/// which is the reason all function on this struct are marked unsafe.
//#[repr(C)] here, compiler is not allowed to change layout.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TraitObject {
    data: *mut (),
    vtable: *mut (),
}

impl TraitObject {
    /// Creates a new TraitObject from `data` and `vtable`.
    #[inline]
    pub const unsafe fn new(data: *mut (), vtable: *mut ()) -> Self {
        Self { data, vtable }
    }

    /// Returns the data part of the trait object.
    #[inline]
    pub const unsafe fn data(&self) -> *mut () {
        self.data
    }

    /// Returns the vtable part of the trait object.
    #[inline]
    pub const unsafe fn vtable(&self) -> *mut () {
        self.vtable
    }
}
