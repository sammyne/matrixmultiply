use std::alloc;
use std::alloc::{handle_alloc_error, Layout};
use std::{cmp, mem};

//#[cfg(test)]
#[cfg(feature = "with-testing")]
use std::ops::{Deref, DerefMut};
//#[cfg(test)]
#[cfg(feature = "with-testing")]
use std::slice;

pub(crate) struct Alloc<T> {
    ptr: *mut T,
    len: usize,
    align: usize,
}

impl<T> Alloc<T> {
    #[inline]
    pub unsafe fn new(nelem: usize, align: usize) -> Self {
        let align = cmp::max(align, mem::align_of::<T>());
        #[cfg(debug_assertions)]
        let layout = Layout::from_size_align(mem::size_of::<T>() * nelem, align).unwrap();
        #[cfg(not(debug_assertions))]
        let layout = Layout::from_size_align_unchecked(mem::size_of::<T>() * nelem, align);
        dprint!("Allocating nelem={}, layout={:?}", nelem, layout);
        let ptr = alloc::alloc(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        Alloc {
            ptr: ptr as *mut T,
            len: nelem,
            align,
        }
    }

    //#[cfg(test)]
    pub fn init_with(mut self, elt: T) -> Alloc<T>
    where
        T: Copy,
    {
        for elt1 in &mut self[..] {
            *elt1 = elt;
        }
        self
    }

    #[inline]
    pub fn ptr_mut(&mut self) -> *mut T {
        self.ptr
    }
}

impl<T> Drop for Alloc<T> {
    fn drop(&mut self) {
        unsafe {
            let layout =
                Layout::from_size_align_unchecked(mem::size_of::<T>() * self.len, self.align);
            alloc::dealloc(self.ptr as _, layout);
        }
    }
}

//#[cfg(test)]
#[cfg(feature = "with-testing")]
impl<T> Deref for Alloc<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

//#[cfg(test)]
#[cfg(feature = "with-testing")]
impl<T> DerefMut for Alloc<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
