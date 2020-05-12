use std::ffi::c_void;
use std::fmt::{self, Display};
use std::marker::PhantomData;
use std::mem;
use std::ptr;

/// Binding to C++ `std::optional<T>`.
///
/// # Invariants
///
/// As an invariant of this API and the static analysis of the cxx::bridge
/// macro, in Rust code we can never obtain a `CxxOptional` by value. Instead in
/// Rust code we will only ever look at an optional behind a reference or smart
/// pointer, as in `&CxxOptional<T>` or `UniquePtr<CxxOptional<T>>`.
#[repr(C, packed)]
pub struct CxxOptional<T> {
    _private: [T; 0],
}

impl<T> CxxOptional<T>
where
    T: OptionalElement,
{
    /// Returns true if the option is a Some value.
    ///
    /// Matches the behavior of C++ [std::optional\<T\>::has_value][has_value].
    ///
    /// [has_value]: https://en.cppreference.com/w/cpp/utility/optional/operator_bool
    pub fn is_some(&self) -> bool {
        T::__has_value(self)
    }

    /// Returns true if the option is a None value.
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Returns a Rust Option representing either the contained value, or None.
    pub fn get(&self) -> Option<&T> {
        if self.is_some() {
            Some(unsafe { T::__get_unchecked(self) })
        } else {
            None
        }
    }

    /// Returns a reference to the contained value without doing bounds checking.
    ///
    /// This is generally not recommended, use with caution! Calling this method
    /// with no contained value is undefined behavior even if the resulting
    /// reference is not used.
    ///
    /// Matches the behavior of C++
    /// [std::optional\<T\>::operator\[\]][operator_\*].
    ///
    /// [operator_\*]: https://en.cppreference.com/w/cpp/utility/optional/operator*
    pub unsafe fn get_unchecked(&self) -> &T {
        T::__get_unchecked(self)
    }
}

pub struct TypeName<T> {
    element: PhantomData<T>,
}

impl<T> TypeName<T> {
    pub const fn new() -> Self {
        TypeName {
            element: PhantomData,
        }
    }
}

impl<T> Display for TypeName<T>
where
    T: OptionalElement,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "CxxOptional<{}>", T::__NAME)
    }
}

// Methods are private; not intended to be implemented outside of cxxbridge
// codebase.
#[doc(hidden)]
pub unsafe trait OptionalElement: Sized {
    const __NAME: &'static dyn Display;
    fn __has_value(v: &CxxOptional<Self>) -> bool;
    unsafe fn __get_unchecked(v: &CxxOptional<Self>) -> &Self;
    fn __unique_ptr_null() -> *mut c_void;
    unsafe fn __unique_ptr_raw(raw: *mut CxxOptional<Self>) -> *mut c_void;
    unsafe fn __unique_ptr_get(repr: *mut c_void) -> *const CxxOptional<Self>;
    unsafe fn __unique_ptr_release(repr: *mut c_void) -> *mut CxxOptional<Self>;
    unsafe fn __unique_ptr_drop(repr: *mut c_void);
}

macro_rules! impl_optional_element_for_primitive {
    ($ty:ident) => {
        const_assert_eq!(1, mem::align_of::<CxxOptional<$ty>>());

        unsafe impl OptionalElement for $ty {
            const __NAME: &'static dyn Display = &stringify!($ty);
            fn __has_value(v: &CxxOptional<$ty>) -> bool {
                extern "C" {
                    attr! {
                        #[link_name = concat!("cxxbridge03$std$optional$", stringify!($ty), "$has_value")]
                        fn __has_value(_: &CxxOptional<$ty>) -> bool;
                    }
                }
                unsafe { __has_value(v) }
            }
            unsafe fn __get_unchecked(v: &CxxOptional<$ty>) -> &$ty {
                extern "C" {
                    attr! {
                        #[link_name = concat!("cxxbridge03$std$optional$", stringify!($ty), "$get_unchecked")]
                        fn __get_unchecked(_: &CxxOptional<$ty>) -> *const $ty;
                    }
                }
                &*__get_unchecked(v)
            }
            fn __unique_ptr_null() -> *mut c_void {
                extern "C" {
                    attr! {
                        #[link_name = concat!("cxxbridge03$unique_ptr$std$optional$", stringify!($ty), "$null")]
                        fn __unique_ptr_null(this: *mut *mut c_void);
                    }
                }
                let mut repr = ptr::null_mut::<c_void>();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(raw: *mut CxxOptional<Self>) -> *mut c_void {
                extern "C" {
                    attr! {
                        #[link_name = concat!("cxxbridge03$unique_ptr$std$optional$", stringify!($ty), "$raw")]
                        fn __unique_ptr_raw(this: *mut *mut c_void, raw: *mut CxxOptional<$ty>);
                    }
                }
                let mut repr = ptr::null_mut::<c_void>();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(repr: *mut c_void) -> *const CxxOptional<Self> {
                extern "C" {
                    attr! {
                        #[link_name = concat!("cxxbridge03$unique_ptr$std$optional$", stringify!($ty), "$get")]
                        fn __unique_ptr_get(this: *const *mut c_void) -> *const CxxOptional<$ty>;
                    }
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(mut repr: *mut c_void) -> *mut CxxOptional<Self> {
                extern "C" {
                    attr! {
                        #[link_name = concat!("cxxbridge03$unique_ptr$std$optional$", stringify!($ty), "$release")]
                        fn __unique_ptr_release(this: *mut *mut c_void) -> *mut CxxOptional<$ty>;
                    }
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(mut repr: *mut c_void) {
                extern "C" {
                    attr! {
                        #[link_name = concat!("cxxbridge03$unique_ptr$std$optional$", stringify!($ty), "$drop")]
                        fn __unique_ptr_drop(this: *mut *mut c_void);
                    }
                }
                __unique_ptr_drop(&mut repr);
            }
        }
    };
}

impl_optional_element_for_primitive!(u8);
impl_optional_element_for_primitive!(u16);
impl_optional_element_for_primitive!(u32);
impl_optional_element_for_primitive!(u64);
impl_optional_element_for_primitive!(usize);
impl_optional_element_for_primitive!(i8);
impl_optional_element_for_primitive!(i16);
impl_optional_element_for_primitive!(i32);
impl_optional_element_for_primitive!(i64);
impl_optional_element_for_primitive!(isize);
impl_optional_element_for_primitive!(f32);
impl_optional_element_for_primitive!(f64);
