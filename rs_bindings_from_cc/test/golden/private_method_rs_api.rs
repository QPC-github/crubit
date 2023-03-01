// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:private_method_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Generated from: rs_bindings_from_cc/test/golden/private_method.h;l=8
// Error while generating bindings for item 'Ptr':
// Class templates are not supported yet

/// Generated from: rs_bindings_from_cc/test/golden/private_method.h;l=17
#[::ctor::recursively_pinned]
#[repr(C)]
pub struct Outer {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Outer"), crate::Outer);

// Generated from: rs_bindings_from_cc/test/golden/private_method.h;l=17
// Error while generating bindings for item 'Outer::Outer':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/private_method.h;l=17
// Error while generating bindings for item 'Outer::Outer':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/private_method.h;l=17
// Error while generating bindings for item 'Outer::Outer':
// Parameter #0 is not supported: Unsupported type 'Outer &&': Unsupported type: && without lifetime

// Generated from: rs_bindings_from_cc/test/golden/private_method.h;l=17
// Error while generating bindings for item 'Outer::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/golden/private_method.h;l=17
// Error while generating bindings for item 'Outer::operator=':
// Parameter #0 is not supported: Unsupported type 'Outer &&': Unsupported type: && without lifetime

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_PRIVATE_METHOD_H_

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::Outer>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::Outer>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Outer: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Outer: Drop);
};
