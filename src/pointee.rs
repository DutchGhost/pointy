use crate::trait_obj::TraitObject;

use core::mem;

pub type VTablePointer = *mut ();

/// This trait provides metadata from a pointed-to type.
pub trait Pointee {
    /// The metadata
    type Meta: 'static + Copy;

    /// Acquires the metadata.
    fn metadata(&self) -> Self::Meta;
}

impl<T> Pointee for T {
    type Meta = ();

    #[inline]
    fn metadata(&self) -> Self::Meta {}
}

impl<T> Pointee for [T] {
    type Meta = usize;

    #[inline]
    fn metadata(&self) -> Self::Meta {
        self.len()
    }
}

impl<'a, T> Pointee for dyn FnMut() -> T + 'a {
    type Meta = VTablePointer;

    #[inline]
    fn metadata(&self) -> Self::Meta {
        unsafe {
            let trait_obj = mem::transmute::<&Self, TraitObject>(self);
            trait_obj.vtable()
        }
    }
}

impl<'a, T> Pointee for dyn Fn() -> T + 'a {
    type Meta = VTablePointer;

    #[inline]
    fn metadata(&self) -> Self::Meta {
        unsafe {
            let trait_obj = mem::transmute::<&Self, TraitObject>(self);
            trait_obj.vtable()
        }
    }
}
