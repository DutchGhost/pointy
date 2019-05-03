use crate::{
    access::{Access, Shared, Unique},
    pointee::{Pointee, VTablePointer},
    trait_obj::TraitObject,
};

use core::{
    cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering},
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    slice,
    hash::{Hasher, Hash},
};

/// A pointer to any type T.
/// This pointer wraps normal pointers, slices, and trait objects.
#[derive(Debug)]
pub struct Ptr<
    'a,
    T: ?Sized + Pointee<Meta = Meta>,
    Meta: 'static + Copy = <T as Pointee>::Meta,
    A: Access = Shared,
> {
    /// The location of `T`
    location: *const T,

    /// Additional Metadata of the Pointee.
    meta: Meta,

    /// Shared / Unique access
    access: PhantomData<A>,

    /// We don't own T
    _mark: PhantomData<*const T>,

    /// Lifetime
    lifetime: PhantomData<&'a ()>,
}

unsafe impl <'a, T: ?Sized + Pointee + Send, A: Access> Send for Ptr<'a, T, T::Meta, A> {}
unsafe impl <'a, T: ?Sized + Pointee + Sync, A: Access> Sync for Ptr<'a, T, T::Meta, A> {}

/// if `A` Copy, the pointer is Copy.
impl<'a, T: ?Sized + Pointee, A: Access + Copy> Copy for Ptr<'a, T, T::Meta, A> {}

/// If `A` Clone, the pointer is Clone.
impl<'a, T: ?Sized + Pointee, A: Access + Clone + Copy> Clone for Ptr<'a, T, T::Meta, A> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized + Pointee, A: Access> Eq for Ptr<'a, T, T::Meta, A> {}

impl<'a, 'b, T: ?Sized + Pointee, A: Access, B: Access> PartialEq<Ptr<'b, T, T::Meta, B>>
    for Ptr<'a, T, T::Meta, A>
{
    #[inline]
    fn eq(&self, other: &Ptr<'b, T, T::Meta, B>) -> bool {
        self.location.eq(&other.location)
    }
}

impl<'a, T: ?Sized + Pointee, A: Access> Ord for Ptr<'a, T, T::Meta, A> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.location.cmp(&other.location)
    }
}

impl <'a, 'b, T: ?Sized + Pointee, A: Access, B: Access> PartialOrd<Ptr<'b, T, T::Meta, B>> for Ptr<'a, T, T::Meta, A> {
    fn partial_cmp(&self, other: &Ptr<'b, T, T::Meta, B>) -> Option<Ordering> {
        self.location.partial_cmp(&other.location)
    }
}

impl <'a, T: ?Sized + Pointee, A: Access> Hash for Ptr<'a, T, T::Meta, A> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.location.hash(state);
    }
}
impl<'a, T: ?Sized + Pointee, A: Access> Ptr<'a, T, T::Meta, A> {
    /// Constructs a new `Ptr` from `pointer`.
    #[inline]
    pub fn new<P>(pointer: P) -> Self
    where
        Self: From<P>,
    {
        Self::from(pointer)
    }
}

/// &T produces Shared
impl<'a, T: ?Sized + Pointee> From<&'a T> for Ptr<'a, T, T::Meta, Shared> {
    #[inline]
    fn from(reference: &'a T) -> Self {
        Self {
            location: reference,
            meta: reference.metadata(),
            access: PhantomData,
            _mark: PhantomData,
            lifetime: PhantomData,
        }
    }
}

/// &mut *can* produce Shared
impl<'a, T: ?Sized + Pointee> From<&'a mut T> for Ptr<'a, T, T::Meta, Shared> {
    #[inline]
    fn from(reference: &'a mut T) -> Self {
        Self::from(&*reference)
    }
}

/// &mut produces Unique
impl<'a, T: ?Sized + Pointee> From<&'a mut T> for Ptr<'a, T, T::Meta, Unique> {
    #[inline]
    fn from(reference: &'a mut T) -> Self {
        Self {
            location: reference,
            meta: (&*reference).metadata(),
            access: PhantomData,
            _mark: PhantomData,
            lifetime: PhantomData,
        }
    }
}

/// A pointer to a normal type T (Meta = ()) is dereferencable for any access specifier `A`.
impl<'a, T: Pointee<Meta = ()>, A: Access> Deref for Ptr<'a, T, (), A> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.location as *const T) }
    }
}

/// A pointer to a normal type T (Meta = ()) is only mutably dereferencabble by Unique access
impl<'a, T: Pointee<Meta = ()>> DerefMut for Ptr<'a, T, (), Unique> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.location as *mut T) }
    }
}

/// A pointer to a trait object (Meta = VTablePointer) is dereferencable for any access specifier `A`.
impl<'a, T: ?Sized + Pointee<Meta = VTablePointer>, A: Access> Deref
    for Ptr<'a, T, VTablePointer, A>
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let ptr = self.location as *mut ();

        // We create a TraitObject, and transmute it into T (the realy trait object).
        // We can't just use a normal transmute, because we dont know if T really is a trait object.
        unsafe { mem::transmute_copy(&TraitObject::new(ptr, self.meta)) }
    }
}

/// A pointer to a trait object (Meta = VTablePointer) is only mutable dereferencable by Unique access.
impl<'a, T: ?Sized + Pointee<Meta = VTablePointer>> DerefMut for Ptr<'a, T, VTablePointer, Unique> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.location as *mut ();

        // We create a TraitObject, and transmute it into T (the realy trait object).
        // We can't just use a normal transmute, because we dont know if T really is a trait object.
        unsafe { mem::transmute_copy(&TraitObject::new(ptr, self.meta)) }
    }
}

/// A pointer to a slic (Meta = usize) is dereferencable for any access specifier `A`.
impl<'a, T, A: Access> Deref for Ptr<'a, [T], <[T] as Pointee>::Meta, A> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        let ptr: *const T = self.location as *const T;
        unsafe { slice::from_raw_parts(ptr, self.meta) }
    }
}

/// A pointer to a trait object (Meta = usize) is only mutably dereferencable by Unique access.
impl<'a, T> DerefMut for Ptr<'a, [T], <[T] as Pointee>::Meta, Unique> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr: *mut T = self.location as *mut T;
        unsafe { slice::from_raw_parts_mut(ptr, self.meta) }
    }
}
