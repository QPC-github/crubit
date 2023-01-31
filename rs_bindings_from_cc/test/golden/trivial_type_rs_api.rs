// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:trivial_type_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod ns {
    /// Implicitly defined special member functions are trivial on a struct with
    /// only trivial members.
    ///
    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=13
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct Trivial {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("Trivial"), crate::ns::Trivial);

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=13
    impl Default for Trivial {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialC1Ev(&mut tmp);
                tmp.assume_init()
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=13
    impl<'b> From<::ctor::RvalueReference<'b, Self>> for Trivial {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
            let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialC1EOS0_(&mut tmp, __param_0);
                tmp.assume_init()
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=13
    impl<'b> ::ctor::UnpinAssign<&'b Self> for Trivial {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialaSERKS0_(self, __param_0);
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=13
    impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for Trivial {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialaSEOS0_(self, __param_0);
            }
        }
    }

    /// Defaulted special member functions are trivial on a struct with only trivial
    /// members.
    ///
    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=19
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct TrivialWithDefaulted {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("TrivialWithDefaulted"),
        crate::ns::TrivialWithDefaulted
    );

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=20
    impl Default for TrivialWithDefaulted {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns20TrivialWithDefaultedC1Ev(&mut tmp);
                tmp.assume_init()
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=23
    impl<'b> ::ctor::UnpinAssign<&'b Self> for TrivialWithDefaulted {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns20TrivialWithDefaultedaSERKS0_(self, __param_0);
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=24
    impl<'b> From<::ctor::RvalueReference<'b, Self>> for TrivialWithDefaulted {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
            let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns20TrivialWithDefaultedC1EOS0_(
                    &mut tmp, __param_0,
                );
                tmp.assume_init()
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=25
    impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for TrivialWithDefaulted {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns20TrivialWithDefaultedaSEOS0_(self, __param_0);
            }
        }
    }

    /// This struct is trivial, and therefore trivially relocatable etc., but still
    /// not safe to pass by reference as it is not final.
    ///
    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=34
    #[::ctor::recursively_pinned]
    #[repr(C)]
    pub struct TrivialNonfinal {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("TrivialNonfinal"),
        crate::ns::TrivialNonfinal
    );

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=34
    impl ::ctor::CtorNew<()> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(args: ()) -> Self::CtorType {
            let () = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                        crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1Ev(
                            ::std::pin::Pin::into_inner_unchecked(dest),
                        );
                    },
                )
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=34
    impl<'b> ::ctor::CtorNew<&'b Self> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: &'b Self) -> Self::CtorType {
            let __param_0 = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                        crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1ERKS0_(
                            ::std::pin::Pin::into_inner_unchecked(dest),
                            __param_0,
                        );
                    },
                )
            }
        }
    }
    impl<'b> ::ctor::CtorNew<(&'b Self,)> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=34
    impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
            let __param_0 = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                        crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1EOS0_(
                            ::std::pin::Pin::into_inner_unchecked(dest),
                            __param_0,
                        );
                    },
                )
            }
        }
    }
    impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=34
    impl<'b> ::ctor::Assign<&'b Self> for TrivialNonfinal {
        #[inline(always)]
        fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalaSERKS0_(self, __param_0);
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=34
    impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for TrivialNonfinal {
        #[inline(always)]
        fn assign<'a>(
            self: ::std::pin::Pin<&'a mut Self>,
            __param_0: ::ctor::RvalueReference<'b, Self>,
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalaSEOS0_(self, __param_0);
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=38
    #[inline(always)]
    pub fn TakesByValue(trivial: crate::ns::Trivial) {
        unsafe { crate::detail::__rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(trivial) }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=39
    #[inline(always)]
    pub fn TakesWithDefaultedByValue(trivial: crate::ns::TrivialWithDefaulted) {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns25TakesWithDefaultedByValueENS_20TrivialWithDefaultedE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=40
    #[inline(always)]
    pub fn TakesTrivialNonfinalByValue(
        trivial: impl ::ctor::Ctor<Output = crate::ns::TrivialNonfinal>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(
                ::std::pin::Pin::into_inner_unchecked(::ctor::emplace!(trivial)),
            )
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=42
    #[inline(always)]
    pub fn TakesByReference<'a>(trivial: &'a mut crate::ns::Trivial) {
        unsafe { crate::detail::__rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE(trivial) }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=43
    #[inline(always)]
    pub fn TakesWithDefaultedByReference<'a>(trivial: &'a mut crate::ns::TrivialWithDefaulted) {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns29TakesWithDefaultedByReferenceERNS_20TrivialWithDefaultedE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=44
    #[inline(always)]
    pub fn TakesTrivialNonfinalByReference<'a>(
        trivial: ::std::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE(trivial)
        }
    }
}

// namespace ns

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN2ns7TrivialC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN2ns7TrivialC1EOS0_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::Trivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN2ns7TrivialaSERKS0_<'a, 'b>(
            __this: &'a mut crate::ns::Trivial,
            __param_0: &'b crate::ns::Trivial,
        ) -> &'a mut crate::ns::Trivial;
        pub(crate) fn __rust_thunk___ZN2ns7TrivialaSEOS0_<'a, 'b>(
            __this: &'a mut crate::ns::Trivial,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::Trivial>,
        ) -> &'a mut crate::ns::Trivial;
        pub(crate) fn __rust_thunk___ZN2ns20TrivialWithDefaultedC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::TrivialWithDefaulted>,
        );
        pub(crate) fn __rust_thunk___ZN2ns20TrivialWithDefaultedaSERKS0_<'a, 'b>(
            __this: &'a mut crate::ns::TrivialWithDefaulted,
            __param_0: &'b crate::ns::TrivialWithDefaulted,
        ) -> &'a mut crate::ns::TrivialWithDefaulted;
        pub(crate) fn __rust_thunk___ZN2ns20TrivialWithDefaultedC1EOS0_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::TrivialWithDefaulted>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialWithDefaulted>,
        );
        pub(crate) fn __rust_thunk___ZN2ns20TrivialWithDefaultedaSEOS0_<'a, 'b>(
            __this: &'a mut crate::ns::TrivialWithDefaulted,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialWithDefaulted>,
        ) -> &'a mut crate::ns::TrivialWithDefaulted;
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1ERKS0_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
            __param_0: &'b crate::ns::TrivialNonfinal,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1EOS0_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalaSERKS0_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
            __param_0: &'b crate::ns::TrivialNonfinal,
        ) -> ::std::pin::Pin<&'a mut crate::ns::TrivialNonfinal>;
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalaSEOS0_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,
        ) -> ::std::pin::Pin<&'a mut crate::ns::TrivialNonfinal>;
        #[link_name = "_ZN2ns12TakesByValueENS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(trivial: crate::ns::Trivial);
        #[link_name = "_ZN2ns25TakesWithDefaultedByValueENS_20TrivialWithDefaultedE"]
        pub(crate) fn __rust_thunk___ZN2ns25TakesWithDefaultedByValueENS_20TrivialWithDefaultedE(
            trivial: crate::ns::TrivialWithDefaulted,
        );
        pub(crate) fn __rust_thunk___ZN2ns27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(
            trivial: &mut crate::ns::TrivialNonfinal,
        );
        #[link_name = "_ZN2ns16TakesByReferenceERNS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE<'a>(
            trivial: &'a mut crate::ns::Trivial,
        );
        #[link_name = "_ZN2ns29TakesWithDefaultedByReferenceERNS_20TrivialWithDefaultedE"]
        pub(crate) fn __rust_thunk___ZN2ns29TakesWithDefaultedByReferenceERNS_20TrivialWithDefaultedE<
            'a,
        >(
            trivial: &'a mut crate::ns::TrivialWithDefaulted,
        );
        #[link_name = "_ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE<
            'a,
        >(
            trivial: ::std::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
        );
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::ns::Trivial>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::ns::Trivial>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::Trivial: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::Trivial: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::Trivial: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::ns::Trivial, trivial_field) == 0);
const _: () = assert!(::std::mem::size_of::<crate::ns::TrivialWithDefaulted>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::ns::TrivialWithDefaulted>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::TrivialWithDefaulted: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::TrivialWithDefaulted: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::TrivialWithDefaulted: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::ns::TrivialWithDefaulted, trivial_field) == 0);
const _: () = assert!(::std::mem::size_of::<crate::ns::TrivialNonfinal>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::ns::TrivialNonfinal>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::TrivialNonfinal: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::TrivialNonfinal: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::ns::TrivialNonfinal, trivial_field) == 0);
