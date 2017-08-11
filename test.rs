#![feature(prelude_import)]
#![no_std]
#![allow(dead_code, unused_macros)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;

extern crate num_traits;
extern crate typenum;

extern crate generic_array;
extern crate nodrop;

use std::{mem, ptr, slice};

use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ops::{Range, RangeFrom, RangeTo, RangeFull};
use std::ops::{Add, Sub};

use std::fmt::{Debug, Formatter, Result as FmtResult};

use nodrop::NoDrop;
use typenum::*;
use generic_array::{ArrayLength, GenericArray};

#[repr(C)]
#[structural_match]
pub struct NumericArray<T, N: ArrayLength<T>>(GenericArray<T, N>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::cmp::PartialEq, N: ::std::cmp::PartialEq + ArrayLength<T>>
 ::std::cmp::PartialEq for NumericArray<T, N> {
    #[inline]
    fn eq(&self, __arg_0: &NumericArray<T, N>) -> bool {
        match *__arg_0 {
            NumericArray(ref __self_1_0) =>
            match *self {
                NumericArray(ref __self_0_0) =>
                true && (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &NumericArray<T, N>) -> bool {
        match *__arg_0 {
            NumericArray(ref __self_1_0) =>
            match *self {
                NumericArray(ref __self_0_0) =>
                false || (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::cmp::Eq, N: ::std::cmp::Eq + ArrayLength<T>> ::std::cmp::Eq
 for NumericArray<T, N> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<GenericArray<T, N>>; }
    }
}

impl <T: Debug, N: ArrayLength<T>> Debug for NumericArray<T, N> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_tuple("NumericArray").field(&self.0).finish()
    }
}

impl <T: Copy, N: ArrayLength<T>> Clone for NumericArray<T, N> where
 N::ArrayType: Copy {
    fn clone(&self) -> NumericArray<T, N> { NumericArray{..*self} }
}

impl <T, N: ArrayLength<T>> Deref for NumericArray<T, N> {
    type
    Target
    =
    [T];

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl <T, N: ArrayLength<T>> DerefMut for NumericArray<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl <T, N: ArrayLength<T>> NumericArray<T, N> {
    /// Creates a new `NumericArray` instance from a `GenericArray` instance.
    /// 
    /// Example:
    /// 
    /// ```
    /// #[macro_use]
    /// extern crate generic_array;
    /// extern crate numeric_array;
    ///
    /// use numeric_array::NumericArray;
    ///
    /// fn main() {
    ///     let arr = NumericArray::new(arr![i32; 1, 2, 3, 4]);
    ///
    ///     println!("{:?}", arr); // Prints 'NumericArray([1, 2, 3, 4])'
    /// }
    /// ```
    pub fn new(arr: GenericArray<T, N>) -> NumericArray<T, N> {
        NumericArray(arr)
    }

    /// Moves all but the last element into a `NumericArray` with one less element than the current one.
    ///
    /// The last element is dropped.
    ///
    /// Example:
    ///
    /// ```ignore
    /// let a = NumericArray::new(arr![i32; 1, 2, 3, 4]);
    /// let b = NumericArray::new(arr![i32; 1, 2, 3]);
    /// 
    /// assert_eq!(a.shorten(), b);
    /// ```
    pub fn shorten(self) -> NumericArray<T, Sub1<N>> where N: Sub<B1>,
     Sub1<N>: ArrayLength<T> {
        use std::{mem, ptr};

        let mut shorter: GenericArray<T, Sub1<N>> =
            unsafe { mem::uninitialized() };

        for (dst, src) in shorter.iter_mut().zip(self.iter()) {
            unsafe { ptr::write(dst, ptr::read(src)); }
        }

        let _last = unsafe { ptr::read(&self.0[N::to_usize() - 1]) };

        mem::forget(self);

        NumericArray(shorter)
    }

    /// Moves all the current elements into a new array with one more element than the current one.
    ///
    /// The last element of the new array is set to `last`
    ///
    /// Example:
    ///
    /// ```ignore
    /// let a = NumericArray::new(arr![i32; 1, 2, 3, 4]);
    /// let b = NumericArray::new(arr![i32; 1, 2, 3]);
    ///
    /// assert_eq!(a, b.lengthen(4));
    /// ```
    pub fn lengthen(self, last: T) -> NumericArray<T, Add1<N>> where
     N: Add<B1>, Add1<N>: ArrayLength<T> {
        use std::{mem, ptr};

        let mut longer: GenericArray<T, Add1<N>> =
            unsafe { mem::uninitialized() };

        for (dst, src) in longer.iter_mut().zip(self.iter()) {
            unsafe { ptr::write(dst, ptr::read(src)); }
        }

        unsafe { ptr::write(&mut longer[N::to_usize()], last); }

        mem::forget(self);

        NumericArray(longer)
    }

    /// Maps the current array to a new one of the same size using the given function.
    pub fn map<U, F>(&self, f: F) -> NumericArray<U, N> where
     N: ArrayLength<U>, F: Fn(&T) -> U {
        let mut res: NoDrop<GenericArray<U, N>> =
            NoDrop::new(unsafe { mem::uninitialized() });

        for (dst, src) in res.iter_mut().zip(self.iter()) {
            unsafe { ptr::write(dst, f(src)); }
        }

        NumericArray(res.into_inner())
    }

    /// Same as `map`, but the values are moved rather than referenced.
    pub fn map_move<U, F>(self, f: F) -> NumericArray<U, N> where
     N: ArrayLength<U>, F: Fn(T) -> U {
        let mut res: NoDrop<GenericArray<U, N>> =
            NoDrop::new(unsafe { mem::uninitialized() });

        for (dst, src) in res.iter_mut().zip(self.iter()) {
            unsafe { ptr::write(dst, f(ptr::read(src))); }
        }

        mem::forget(self);

        NumericArray(res.into_inner())
    }

    /// Combines two same-length arrays and maps both values to a new array using the given function.
    pub fn zip<U, F>(&self, rhs: &Self, f: F) -> NumericArray<U, N> where
     N: ArrayLength<U>, F: Fn(&T, &T) -> U {
        let mut res: NoDrop<GenericArray<U, N>> =
            NoDrop::new(unsafe { mem::uninitialized() });

        for (dst, (lhs, rhs)) in
            res.iter_mut().zip(self.iter().zip(rhs.iter())) {
            unsafe { ptr::write(dst, f(lhs, rhs)); }
        }

        NumericArray(res.into_inner())
    }

    /// Same as `zip`, but `self` values are moved. The `rhs` array is still accessed by reference.
    pub fn zip_move<U, F>(self, rhs: &Self, f: F) -> NumericArray<U, N> where
     N: ArrayLength<U>, F: Fn(T, &T) -> U {
        let mut res: NoDrop<GenericArray<U, N>> =
            NoDrop::new(unsafe { mem::uninitialized() });

        for (dst, (lhs, rhs)) in
            res.iter_mut().zip(self.iter().zip(rhs.iter())) {
            unsafe { ptr::write(dst, f(ptr::read(lhs), rhs)); }
        }

        mem::forget(self);

        NumericArray(res.into_inner())
    }

    /// Like `zip` and `zip_move`, but moves both `self` and the `rhs` array.
    pub fn zip_move_both<U, F>(self, rhs: Self, f: F) -> NumericArray<U, N>
     where N: ArrayLength<U>, F: Fn(T, T) -> U {
        let mut res: NoDrop<GenericArray<U, N>> =
            NoDrop::new(unsafe { mem::uninitialized() });

        for (dst, (lhs, rhs)) in
            res.iter_mut().zip(self.iter().zip(rhs.iter())) {
            unsafe { ptr::write(dst, f(ptr::read(lhs), ptr::read(rhs))); }
        }

        mem::forget(self);
        mem::forget(rhs);

        NumericArray(res.into_inner())
    }
}






impl <T, N: ArrayLength<T>> AsRef<[T]> for NumericArray<T, N> {
    fn as_ref(&self) -> &[T] { self }
}

impl <T, N: ArrayLength<T>> Borrow<[T]> for NumericArray<T, N> {
    fn borrow(&self) -> &[T] { self }
}

impl <T, N: ArrayLength<T>> AsMut<[T]> for NumericArray<T, N> {
    fn as_mut(&mut self) -> &mut [T] { self }
}

impl <T, N: ArrayLength<T>> BorrowMut<[T]> for NumericArray<T, N> {
    fn borrow_mut(&mut self) -> &mut [T] { self }
}

impl <T, N: ArrayLength<T>> Index<usize> for NumericArray<T, N> {
    type
    Output
    =
    T;

    #[inline]
    fn index(&self, index: usize) -> &T { &(**self)[index] }
}

impl <T, N: ArrayLength<T>> IndexMut<usize> for NumericArray<T, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T { &mut (**self)[index] }
}

impl <T, N: ArrayLength<T>> Index<Range<usize>> for NumericArray<T, N> {
    type
    Output
    =
    [T];

    #[inline]
    fn index(&self, index: Range<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}

impl <T, N: ArrayLength<T>> Index<RangeTo<usize>> for NumericArray<T, N> {
    type
    Output
    =
    [T];

    #[inline]
    fn index(&self, index: RangeTo<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}

impl <T, N: ArrayLength<T>> Index<RangeFrom<usize>> for NumericArray<T, N> {
    type
    Output
    =
    [T];

    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}

impl <T, N: ArrayLength<T>> Index<RangeFull> for NumericArray<T, N> {
    type
    Output
    =
    [T];

    #[inline]
    fn index(&self, _index: RangeFull) -> &[T] { self }
}

impl <'a, T, N: ArrayLength<T>> IntoIterator for &'a NumericArray<T, N> {
    type
    Item
    =
    &'a T;
    type
    IntoIter
    =
    slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl <'a, T, N: ArrayLength<T>> IntoIterator for &'a mut NumericArray<T, N> {
    type
    Item
    =
    &'a mut T;
    type
    IntoIter
    =
    slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

macro_rules! impl_unary_ops(( $ ( $ op_trait : ident :: $ op : ident ) , * )
                            => {
                            $ (
                            impl < T , N : ArrayLength < T >> :: std :: ops ::
                            $ op_trait for NumericArray < T , N > where T : ::
                            std :: ops :: $ op_trait , N : ArrayLength << T as
                            :: std :: ops :: $ op_trait > :: Output > {
                            type Output = NumericArray << T as :: std :: ops
                            :: $ op_trait > :: Output , N > ; fn $ op ( self )
                            -> Self :: Output {
                            self . map_move (
                            :: std :: ops :: $ op_trait :: $ op ) } } ) * });


macro_rules! impl_binary_ops(( $ ( $ op_trait : ident :: $ op : ident ) , * )
                             => {
                             $ (
                             impl < T , N : ArrayLength < T >> :: std :: ops
                             :: $ op_trait < Self > for NumericArray < T , N >
                             where T : :: std :: ops :: $ op_trait , N :
                             ArrayLength << T as :: std :: ops :: $ op_trait >
                             :: Output > {
                             type Output = NumericArray << T as :: std :: ops
                             :: $ op_trait > :: Output , N > ; fn $ op (
                             self , rhs : Self ) -> Self :: Output {
                             self . zip_move_both (
                             rhs , :: std :: ops :: $ op_trait :: $ op ) } }
                             impl < T : Copy , N : ArrayLength < T >> :: std
                             :: ops :: $ op_trait < T > for NumericArray < T ,
                             N > where T : :: std :: ops :: $ op_trait , N :
                             ArrayLength << T as :: std :: ops :: $ op_trait >
                             :: Output > {
                             type Output = NumericArray << T as :: std :: ops
                             :: $ op_trait > :: Output , N > ; fn $ op (
                             self , rhs : T ) -> Self :: Output {
                             self . map_move (
                             | l | :: std :: ops :: $ op_trait :: $ op (
                             l , rhs ) ) } } ) * });




macro_rules! impl_assign_ops(( $ ( $ op_trait : ident :: $ op : ident ) , * )
                             => {
                             $ (
                             impl < T , N : ArrayLength < T >> :: std :: ops
                             :: $ op_trait < Self > for NumericArray < T , N >
                             where T : :: std :: ops :: $ op_trait {
                             fn $ op ( & mut self , rhs : Self ) {
                             for ( lhs , rhs ) in self . iter_mut (  ) . zip (
                             rhs . iter (  ) ) {
                             :: std :: ops :: $ op_trait :: $ op (
                             lhs , unsafe { ptr :: read ( rhs ) } ) ; } mem ::
                             forget ( rhs ) ; } } impl < T : Copy , N :
                             ArrayLength < T >> :: std :: ops :: $ op_trait <
                             T > for NumericArray < T , N > where T : :: std
                             :: ops :: $ op_trait {
                             fn $ op ( & mut self , rhs : T ) {
                             for lhs in self . iter_mut (  ) {
                             :: std :: ops :: $ op_trait :: $ op ( lhs , rhs )
                             ; } } } ) * });



macro_rules! impl_wrapping_ops(( $ ( $ op_trait : ident :: $ op : ident ) , *
                               ) => {
                               $ (
                               impl < T , N : ArrayLength < T >> num_traits ::
                               $ op_trait for NumericArray < T , N > where T :
                               num_traits :: $ op_trait {
                               fn $ op ( & self , rhs : & Self ) -> Self {
                               self . zip (
                               rhs , num_traits :: $ op_trait :: $ op ) } } )
                               * });

macro_rules! impl_checked_ops(( $ ( $ op_trait : ident :: $ op : ident ) , * )
                              => {
                              $ (
                              impl < T , N : ArrayLength < T >> num_traits ::
                              $ op_trait for NumericArray < T , N > where T :
                              num_traits :: $ op_trait {
                              fn $ op ( & self , rhs : & Self ) -> Option <
                              Self > {
                              let mut res : NoDrop < GenericArray < T , N >> =
                              NoDrop :: new (
                              unsafe { mem :: uninitialized (  ) } ) ; for (
                              dst , ( lhs , rhs ) ) in res . iter_mut (  ) .
                              zip ( self . iter (  ) . zip ( rhs . iter (  ) )
                              ) {
                              if let Some ( value ) = num_traits :: $ op_trait
                              :: $ op ( lhs , rhs ) {
                              unsafe { ptr :: write ( dst , value ) ; } } else
                              { return None ; } } Some (
                              NumericArray ( res . into_inner (  ) ) ) } } ) *
                              });
impl <T, N: ArrayLength<T>> ::std::ops::Neg for NumericArray<T, N> where
 T: ::std::ops::Neg, N: ArrayLength<<T as ::std::ops::Neg>::Output> {
    type
    Output
    =
    NumericArray<<T as ::std::ops::Neg>::Output, N>;
    fn neg(self) -> Self::Output { self.map_move(::std::ops::Neg::neg) }
}
impl <T, N: ArrayLength<T>> ::std::ops::Not for NumericArray<T, N> where
 T: ::std::ops::Not, N: ArrayLength<<T as ::std::ops::Not>::Output> {
    type
    Output
    =
    NumericArray<<T as ::std::ops::Not>::Output, N>;
    fn not(self) -> Self::Output { self.map_move(::std::ops::Not::not) }
}
impl <T, N: ArrayLength<T>> ::std::ops::Add<Self> for NumericArray<T, N> where
 T: ::std::ops::Add, N: ArrayLength<<T as ::std::ops::Add>::Output> {
    type
    Output
    =
    NumericArray<<T as ::std::ops::Add>::Output, N>;
    fn add(self, rhs: Self) -> Self::Output {
        self.zip_move_both(rhs, ::std::ops::Add::add)
    }
}
impl <T: Copy, N: ArrayLength<T>> ::std::ops::Add<T> for NumericArray<T, N>
 where T: ::std::ops::Add, N: ArrayLength<<T as ::std::ops::Add>::Output> {
    type
    Output
    =
    NumericArray<<T as ::std::ops::Add>::Output, N>;
    fn add(self, rhs: T) -> Self::Output {
        self.map_move(|l| ::std::ops::Add::add(l, rhs))
    }
}
impl <T, N: ArrayLength<T>> ::std::ops::Sub<Self> for NumericArray<T, N> where
 T: ::std::ops::Sub, N: ArrayLength<<T as ::std::ops::Sub>::Output> {
    type
    Output
    =
    NumericArray<<T as ::std::ops::Sub>::Output, N>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_move_both(rhs, ::std::ops::Sub::sub)
    }
}
impl <T: Copy, N: ArrayLength<T>> ::std::ops::Sub<T> for NumericArray<T, N>
 where T: ::std::ops::Sub, N: ArrayLength<<T as ::std::ops::Sub>::Output> {
    type
    Output
    =
    NumericArray<<T as ::std::ops::Sub>::Output, N>;
    fn sub(self, rhs: T) -> Self::Output {
        self.map_move(|l| ::std::ops::Sub::sub(l, rhs))
    }
}
impl <T, N: ArrayLength<T>> ::std::ops::Mul<Self> for NumericArray<T, N> where
 T: ::std::ops::Mul, N: ArrayLength<<T as ::std::ops::Mul>::Output> {
    type
    Output
    =
    NumericArray<<T as ::std::ops::Mul>::Output, N>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.zip_move_both(rhs, ::std::ops::Mul::mul)
    }
}
impl <T: Copy, N: ArrayLength<T>> ::std::ops::Mul<T> for NumericArray<T, N>
 where T: ::std::ops::Mul, N: ArrayLength<<T as ::std::ops::Mul>::Output> {
    type
    Output
    =
    NumericArray<<T as ::std::ops::Mul>::Output, N>;
    fn mul(self, rhs: T) -> Self::Output {
        self.map_move(|l| ::std::ops::Mul::mul(l, rhs))
    }
}
impl <T, N: ArrayLength<T>> ::std::ops::Div<Self> for NumericArray<T, N> where
 T: ::std::ops::Div, N: ArrayLength<<T as ::std::ops::Div>::Output> {
    type
    Output
    =
    NumericArray<<T as ::std::ops::Div>::Output, N>;
    fn div(self, rhs: Self) -> Self::Output {
        self.zip_move_both(rhs, ::std::ops::Div::div)
    }
}
impl <T: Copy, N: ArrayLength<T>> ::std::ops::Div<T> for NumericArray<T, N>
 where T: ::std::ops::Div, N: ArrayLength<<T as ::std::ops::Div>::Output> {
    type
    Output
    =
    NumericArray<<T as ::std::ops::Div>::Output, N>;
    fn div(self, rhs: T) -> Self::Output {
        self.map_move(|l| ::std::ops::Div::div(l, rhs))
    }
}
impl <T, N: ArrayLength<T>> ::std::ops::Rem<Self> for NumericArray<T, N> where
 T: ::std::ops::Rem, N: ArrayLength<<T as ::std::ops::Rem>::Output> {
    type
    Output
    =
    NumericArray<<T as ::std::ops::Rem>::Output, N>;
    fn rem(self, rhs: Self) -> Self::Output {
        self.zip_move_both(rhs, ::std::ops::Rem::rem)
    }
}
impl <T: Copy, N: ArrayLength<T>> ::std::ops::Rem<T> for NumericArray<T, N>
 where T: ::std::ops::Rem, N: ArrayLength<<T as ::std::ops::Rem>::Output> {
    type
    Output
    =
    NumericArray<<T as ::std::ops::Rem>::Output, N>;
    fn rem(self, rhs: T) -> Self::Output {
        self.map_move(|l| ::std::ops::Rem::rem(l, rhs))
    }
}
impl <T, N: ArrayLength<T>> ::std::ops::AddAssign<Self> for NumericArray<T, N>
 where T: ::std::ops::AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.iter_mut().zip(rhs.iter()) {
            ::std::ops::AddAssign::add_assign(lhs, unsafe { ptr::read(rhs) });
        }
        mem::forget(rhs);
    }
}
impl <T: Copy, N: ArrayLength<T>> ::std::ops::AddAssign<T> for
 NumericArray<T, N> where T: ::std::ops::AddAssign {
    fn add_assign(&mut self, rhs: T) {
        for lhs in self.iter_mut() {
            ::std::ops::AddAssign::add_assign(lhs, rhs);
        }
    }
}
impl <T, N: ArrayLength<T>> ::std::ops::SubAssign<Self> for NumericArray<T, N>
 where T: ::std::ops::SubAssign {
    fn sub_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.iter_mut().zip(rhs.iter()) {
            ::std::ops::SubAssign::sub_assign(lhs, unsafe { ptr::read(rhs) });
        }
        mem::forget(rhs);
    }
}
impl <T: Copy, N: ArrayLength<T>> ::std::ops::SubAssign<T> for
 NumericArray<T, N> where T: ::std::ops::SubAssign {
    fn sub_assign(&mut self, rhs: T) {
        for lhs in self.iter_mut() {
            ::std::ops::SubAssign::sub_assign(lhs, rhs);
        }
    }
}
impl <T, N: ArrayLength<T>> ::std::ops::MulAssign<Self> for NumericArray<T, N>
 where T: ::std::ops::MulAssign {
    fn mul_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.iter_mut().zip(rhs.iter()) {
            ::std::ops::MulAssign::mul_assign(lhs, unsafe { ptr::read(rhs) });
        }
        mem::forget(rhs);
    }
}
impl <T: Copy, N: ArrayLength<T>> ::std::ops::MulAssign<T> for
 NumericArray<T, N> where T: ::std::ops::MulAssign {
    fn mul_assign(&mut self, rhs: T) {
        for lhs in self.iter_mut() {
            ::std::ops::MulAssign::mul_assign(lhs, rhs);
        }
    }
}
impl <T, N: ArrayLength<T>> ::std::ops::DivAssign<Self> for NumericArray<T, N>
 where T: ::std::ops::DivAssign {
    fn div_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.iter_mut().zip(rhs.iter()) {
            ::std::ops::DivAssign::div_assign(lhs, unsafe { ptr::read(rhs) });
        }
        mem::forget(rhs);
    }
}
impl <T: Copy, N: ArrayLength<T>> ::std::ops::DivAssign<T> for
 NumericArray<T, N> where T: ::std::ops::DivAssign {
    fn div_assign(&mut self, rhs: T) {
        for lhs in self.iter_mut() {
            ::std::ops::DivAssign::div_assign(lhs, rhs);
        }
    }
}
impl <T, N: ArrayLength<T>> ::std::ops::RemAssign<Self> for NumericArray<T, N>
 where T: ::std::ops::RemAssign {
    fn rem_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.iter_mut().zip(rhs.iter()) {
            ::std::ops::RemAssign::rem_assign(lhs, unsafe { ptr::read(rhs) });
        }
        mem::forget(rhs);
    }
}
impl <T: Copy, N: ArrayLength<T>> ::std::ops::RemAssign<T> for
 NumericArray<T, N> where T: ::std::ops::RemAssign {
    fn rem_assign(&mut self, rhs: T) {
        for lhs in self.iter_mut() {
            ::std::ops::RemAssign::rem_assign(lhs, rhs);
        }
    }
}






impl <T, N: ArrayLength<T>> num_traits::Saturating for NumericArray<T, N>
 where T: num_traits::Saturating {
    fn saturating_add(self, rhs: Self) -> Self {
        self.zip_move_both(rhs, num_traits::Saturating::saturating_add)
    }

    fn saturating_sub(self, rhs: Self) -> Self {
        self.zip_move_both(rhs, num_traits::Saturating::saturating_sub)
    }
}
impl <T, N: ArrayLength<T>> num_traits::WrappingAdd for NumericArray<T, N>
 where T: num_traits::WrappingAdd {
    fn wrapping_add(&self, rhs: &Self) -> Self {
        self.zip(rhs, num_traits::WrappingAdd::wrapping_add)
    }
}
impl <T, N: ArrayLength<T>> num_traits::WrappingSub for NumericArray<T, N>
 where T: num_traits::WrappingSub {
    fn wrapping_sub(&self, rhs: &Self) -> Self {
        self.zip(rhs, num_traits::WrappingSub::wrapping_sub)
    }
}
impl <T, N: ArrayLength<T>> num_traits::WrappingMul for NumericArray<T, N>
 where T: num_traits::WrappingMul {
    fn wrapping_mul(&self, rhs: &Self) -> Self {
        self.zip(rhs, num_traits::WrappingMul::wrapping_mul)
    }
}
impl <T, N: ArrayLength<T>> num_traits::CheckedAdd for NumericArray<T, N>
 where T: num_traits::CheckedAdd {
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        let mut res: NoDrop<GenericArray<T, N>> =
            NoDrop::new(unsafe { mem::uninitialized() });
        for (dst, (lhs, rhs)) in
            res.iter_mut().zip(self.iter().zip(rhs.iter())) {
            if let Some(value) = num_traits::CheckedAdd::checked_add(lhs, rhs)
                   {
                unsafe { ptr::write(dst, value); }
            } else { return None; }
        }
        Some(NumericArray(res.into_inner()))
    }
}
impl <T, N: ArrayLength<T>> num_traits::CheckedSub for NumericArray<T, N>
 where T: num_traits::CheckedSub {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        let mut res: NoDrop<GenericArray<T, N>> =
            NoDrop::new(unsafe { mem::uninitialized() });
        for (dst, (lhs, rhs)) in
            res.iter_mut().zip(self.iter().zip(rhs.iter())) {
            if let Some(value) = num_traits::CheckedSub::checked_sub(lhs, rhs)
                   {
                unsafe { ptr::write(dst, value); }
            } else { return None; }
        }
        Some(NumericArray(res.into_inner()))
    }
}
impl <T, N: ArrayLength<T>> num_traits::CheckedMul for NumericArray<T, N>
 where T: num_traits::CheckedMul {
    fn checked_mul(&self, rhs: &Self) -> Option<Self> {
        let mut res: NoDrop<GenericArray<T, N>> =
            NoDrop::new(unsafe { mem::uninitialized() });
        for (dst, (lhs, rhs)) in
            res.iter_mut().zip(self.iter().zip(rhs.iter())) {
            if let Some(value) = num_traits::CheckedMul::checked_mul(lhs, rhs)
                   {
                unsafe { ptr::write(dst, value); }
            } else { return None; }
        }
        Some(NumericArray(res.into_inner()))
    }
}
impl <T, N: ArrayLength<T>> num_traits::CheckedDiv for NumericArray<T, N>
 where T: num_traits::CheckedDiv {
    fn checked_div(&self, rhs: &Self) -> Option<Self> {
        let mut res: NoDrop<GenericArray<T, N>> =
            NoDrop::new(unsafe { mem::uninitialized() });
        for (dst, (lhs, rhs)) in
            res.iter_mut().zip(self.iter().zip(rhs.iter())) {
            if let Some(value) = num_traits::CheckedDiv::checked_div(lhs, rhs)
                   {
                unsafe { ptr::write(dst, value); }
            } else { return None; }
        }
        Some(NumericArray(res.into_inner()))
    }
}



#[inline(never)]
pub fn black_box<T>(val: T) -> T {
    use std::{mem, ptr};

    let ret = unsafe { ptr::read_volatile(&val) };
    mem::forget(val);
    ret
}



