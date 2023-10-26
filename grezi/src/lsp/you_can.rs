#![allow(clippy::needless_doctest_main)]
#![warn(unused_extern_crates)]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]

use helix_core::ropey::RopeSlice;

#[inline(always)]
pub unsafe fn borrow_unchecked<
    'original,
    'unbounded,
    Ref: BorrowUnchecked<'original, 'unbounded>,
>(
    reference: Ref,
) -> Ref::Unbounded {
    unsafe { BorrowUnchecked::borrow_unchecked(reference) }
}

#[doc(hidden)]
pub unsafe trait BorrowUnchecked<'original, 'unbounded> {
    type Unbounded;

    unsafe fn borrow_unchecked(self) -> Self::Unbounded;
}

unsafe impl<'original, 'unbounded, T: 'unbounded> BorrowUnchecked<'original, 'unbounded>
    for &'original T
{
    type Unbounded = &'unbounded T;

    #[inline(always)]
    unsafe fn borrow_unchecked(self) -> Self::Unbounded {
        unsafe { ::core::mem::transmute(self) }
    }
}

unsafe impl<'original, 'unbounded> BorrowUnchecked<'original, 'unbounded> for RopeSlice<'original> {
    type Unbounded = RopeSlice<'unbounded>;

    #[inline(always)]
    unsafe fn borrow_unchecked(self) -> Self::Unbounded {
        unsafe { ::core::mem::transmute(self) }
    }
}

unsafe impl<'original, 'unbounded, T: 'unbounded> BorrowUnchecked<'original, 'unbounded>
    for &'original mut T
{
    type Unbounded = &'unbounded mut T;

    #[inline(always)]
    unsafe fn borrow_unchecked(self) -> Self::Unbounded {
        unsafe { ::core::mem::transmute(self) }
    }
}
