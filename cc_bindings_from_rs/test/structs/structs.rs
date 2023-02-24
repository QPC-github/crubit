// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `structs_test.cc`.

/// Test for a `#[repr(C)` struct.
pub mod repr_c {

    #[repr(C)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    pub fn create(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn get_x(p: Point) -> i32 {
        p.x
    }
}

/// Test for a struct using default layout (i.e. one without an explicit
/// `#[repr(C)]` or similar attribute).  Among other things, it tests that
/// building generated `..._cc_api_impl.rs` will not warn about
/// `improper_ctypes_definitions` (search for this warning name in `bindings.rs`
/// for a longer explanation of why suppressing this warning is okay).
pub mod default_repr {

    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    pub fn create(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn get_x(p: Point) -> i32 {
        p.x
    }
}

/// Test of ABI classification.
///
/// System V ABI can classify function parameter and return types into broad
/// categories like "integer", "sse2", or "memory".  Classification impacts how
/// a given value is passed (e.g. by value in `eax` or `xmm0` register, or by
/// pointer).  ABI classification of C++ structs generated
/// by `cc_bindings_from_rs` needs to match exactly the classification of the
/// Rust structs in the input crate (e.g. from this test).  Mismatched ABI
/// classification will lead to Undefined Behavior.
///
/// This is a regression test for b/270454629 - replacing fields with an opaque
/// blob of bytes (e.g. using `[u8; N]` instead of the actual field type) may
/// change the ABI classification of a struct.  The fields of structs below are
/// private (i.e. non-`pub`) to encourage `cc_bindings_from_rs` to treat them as
/// an opaque blob of bytes.
///
/// Optimizing compiler can make the disassembly of the `create` methods quite
/// empty (probably because the input argument uses the same register as the
/// return value.  To make the tests more sensitive to ABI choices, the
/// `multiply` method is used (to actually operate on the input arguments and to
/// have to calculate a *new* return value).
pub mod abi_classification {
    /// Expected ABI classification: integer.  (For indirect confirmation, see
    /// the disassembly at https://godbolt.org/z/b7eeGcrGn).
    pub struct StructInteger(i32);

    /// Expected ABI classification: SSE.  (For indirect confirmation, see the
    /// disassembly at https://godbolt.org/z/b7eeGcrGn).
    pub struct StructFloat(f32);

    /// Expected ABI classification: memory.  (For indirect confirmation, see
    /// the disassembly at https://godbolt.org/z/b7eeGcrGn).
    #[repr(packed(1))]
    pub struct StructMemory {
        _padding: u8,
        i: i32,
    }

    impl StructInteger {
        pub fn create(i: i32) -> Self {
            Self(i)
        }
        pub fn multiply(x: Self, y: Self) -> Self {
            Self(x.0 * y.0)
        }
        pub fn inspect(s: Self) -> i32 {
            s.0
        }
    }

    impl StructFloat {
        pub fn create(f: f32) -> Self {
            Self(f)
        }
        pub fn multiply(x: Self, y: Self) -> Self {
            Self(x.0 * y.0)
        }
        pub fn inspect(s: Self) -> f32 {
            s.0
        }
    }

    impl StructMemory {
        pub fn create(i: i32) -> Self {
            Self { _padding: 0, i }
        }
        pub fn multiply(x: Self, y: Self) -> Self {
            Self::create(x.i * y.i)
        }
        pub fn inspect(s: Self) -> i32 {
            s.i
        }
    }
}

/// This module provides test coverage for reordering the generated bindings in
/// a way that ensures that C++ structs are defined *before* being referring to
/// them when (say) declaring a function that returns the struct by value, or
/// takes it by value as an argument.
///
/// This module has been structured in a way that forces at least one submodule
/// to be broken up into 2 separate chunks.  Definition dependencies force
/// bindings from one of the structs to come first - let's assume that `m1::S1`
/// comes first (the case where `m2::S2` comes first is symmetrical -
/// all the same conclusions apply).  Before `m1::create_S2` can be declared,
/// `m1::S2` needs to be defined.  This means that the order will be: `m1::S1`,
/// ..., `m2::S2`, ..., `m1::create_S2` - the `m1` module has to be split into
/// two non-contiguous chunks (in the generated bindings):
///
///     ```cpp
///     namespace m1 {  // <- FIRST CHUNK OF `mod m1`
///         struct S1 { ... };
///     }
///
///     namespace m2 {
///         struct S2 { ... };
///     }
///
///     namespace m1 {  // <- SECOND CHUNK OF `mod m1`
///         S2 create_s2();
///     }
///     ```
pub mod reordering_defs {
    pub mod m1 {
        use super::m2::S2;
        pub struct S1(pub i32);
        pub fn create_s2() -> S2 {
            S2(123)
        }
        pub fn get_int_from_s2(s2: S2) -> i32 {
            s2.0
        }
    }
    pub mod m2 {
        use super::m1::S1;
        pub struct S2(pub i32);
        pub fn create_s1() -> S1 {
            S1(456)
        }
        pub fn get_int_from_s1(s1: S1) -> i32 {
            s1.0
        }
    }
}

/// This module provides coverage for emitting forward declarations.  In
/// particular, if we assume that the C++ bindings are emitted in the same order
/// as the Rust items below, then `S1` needs to be forward-declared (because
/// `get_int_from_s1` is *before* `S1`).
///
/// TODO(b/260725687): Using a cycle below should avoid the assumption above
/// about preserving the same order (because a cycle can't be
/// toposorted/reordered).  OTOH forming a cycle seems to depend on supporting
/// bindings for additional language features - either static methods
/// (b/260725279):
/// ```
///     // Cycle!:
///     pub struct S1(i32);
///     pub struct S2(i32);
///     impl S1 {
///         pub fn get_int_from_s2(s2: *const S2) { ... }
///     }
///     impl S2 {
///         pub fn get_int_from_s1(s1: *const S1) { ... }
///     }
/// ```
/// or fields (b/258233850):
/// ```
///     // Cycle!:
///     pub struct S1 {
///         ptr_to_s2: *const S2,
///     }
///     pub struct S2 {
///         ptr_to_s1: *const S1,
///     }
/// ```
pub mod fwd_decls {
    pub fn get_int_from_s1(s1: *const S1) -> i32 {
        #![allow(clippy::not_unsafe_ptr_arg_deref)]
        unsafe { (*s1).0 }
    }
    pub fn create_s1() -> S1 {
        S1(456)
    }
    pub struct S1(i32);
}
