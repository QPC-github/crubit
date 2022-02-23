#![cfg(test)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


use anyhow::Result;
use ir::*;
use ir_testing::*;
use itertools::Itertools;
use quote::quote;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use token_stream_matchers::{assert_ir_matches, assert_ir_not_matches};

// TODO(mboehme): If we start needing to match on parts of the IR in tests,
// check out the crate https://crates.io/crates/galvanic-assert.

fn assert_cc_produces_ir_items_ignoring_decl_ids(cc_src: &str, mut expected: Vec<Item>) {
    let actual = ir_from_cc(cc_src).unwrap();

    // Filter out predefined builtin types.
    let actual_items = actual
        .items()
        .filter(|i| {
            !matches!(i, Item::TypeAlias(typedef)
                if typedef.identifier.identifier == "__builtin_ms_va_list")
        })
        .collect_vec();

    for (expected_item, actual_item) in expected.iter_mut().zip(actual_items.iter()) {
        // TODO(hlopko): Handle MappedTypes as well.
        match (expected_item, actual_item) {
            (Item::Record(ref mut expected_record), Item::Record(actual_record)) => {
                expected_record.id = actual_record.id;
            }
            (Item::Enum(ref mut expected_enum), Item::Enum(actual_enum)) => {
                expected_enum.id = actual_enum.id;
            }
            (_, _) => (),
        }
    }

    assert_eq!(actual_items, expected.iter().collect_vec());
}

#[test]
fn test_function() {
    assert_cc_produces_ir_items_ignoring_decl_ids(
        "int Add(int a, int b);",
        vec![Item::Func(Func {
            name: UnqualifiedIdentifier::Identifier(ir_id("Add")),
            owning_target: "//test:testing_target".into(),
            mangled_name: "_Z3Addii".to_string(),
            doc_comment: None,
            return_type: ir_int(),
            params: vec![ir_int_param("a"), ir_int_param("b")],
            lifetime_params: vec![],
            is_inline: false,
            member_func_metadata: None,
            source_loc: SourceLoc {
                filename: "ir_from_cc_virtual_header.h".to_string(),
                line: 3,
                column: 1,
            },
        })],
    );
}

#[test]
fn test_function_with_unnamed_parameters() {
    assert_cc_produces_ir_items_ignoring_decl_ids(
        "int multiply(int, int);",
        vec![Item::Func(Func {
            name: UnqualifiedIdentifier::Identifier(ir_id("multiply")),
            owning_target: "//test:testing_target".into(),
            mangled_name: "_Z8multiplyii".to_string(),
            doc_comment: None,
            return_type: ir_int(),
            params: vec![ir_int_param("__param_0"), ir_int_param("__param_1")],
            lifetime_params: vec![],
            is_inline: false,
            member_func_metadata: None,
            source_loc: SourceLoc {
                filename: "ir_from_cc_virtual_header.h".to_string(),
                line: 3,
                column: 1,
            },
        })],
    );
}

#[test]
fn test_functions_from_dependency_are_not_emitted() -> Result<()> {
    let ir = ir_from_cc_dependency("int Add(int a, int b);", "int Multiply(int a, int b);")?;
    assert_ir_matches!(ir, quote! { Func { name: "Add" ... } });
    assert_ir_not_matches!(ir, quote! { Func { name: "Multiply" ... } });
    Ok(())
}

#[test]
fn test_dont_import_record_nested_in_func() {
    let ir = ir_from_cc("inline void f() { struct S{}; }").unwrap();
    assert_ir_not_matches!(ir, quote! { Record { identifier: "S" ... } });
}

#[test]
fn test_dont_import_class_template_or_specialization() {
    let ir = ir_from_cc("template <class T> struct Template{}; template<> struct Template<int>{};")
        .unwrap();
    assert_ir_not_matches!(ir, quote! { Record { identifier: "Template" ... } });
}

#[test]
fn test_function_template_not_supported_yet() {
    let ir = ir_from_cc("template<typename SomeParam> void SomeFunctionTemplate() {};").unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
          name: "SomeFunctionTemplate",
          message: "Function templates are not supported yet" ...
        }}
    );
}

#[test]
fn test_union_not_supported_yet() {
    let ir = ir_from_cc("union SomeUnion {};").unwrap();
    assert_ir_not_matches!(ir, quote! { Record { identifier: "SomeUnion" ... } });
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "SomeUnion",
            message: "Unions are not supported yet" ...
        }}
    );
}

#[test]
fn test_record_member_variable_access_specifiers() {
    let ir = ir_from_cc(
        "
        struct SomeStruct {
          public:
            int public_int;
          protected:
            int protected_int;
          private:
            int private_int;
        };
    ",
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            Record {
                identifier: "SomeStruct" ...
                fields: [
                    Field {
                        identifier: "public_int" ...
                        access: Public ...
                    },
                    Field {
                        identifier: "protected_int" ...
                        access: Protected ...
                    },
                    Field {
                        identifier: "private_int" ...
                        access: Private ...
                    },
                ] ...
            }
        }
    );
}

#[test]
fn test_record_private_member_functions_not_present() {
    let ir = ir_from_cc(
        "
        struct SomeStruct {
          public:
            int public_method();
          protected:
            int protected_method();
          private:
            int private_method();
        };
    ",
    )
    .unwrap();

    assert_ir_matches!(ir, quote! { Func { name: "public_method" ... } });
    assert_ir_not_matches!(ir, quote! { Func { name: "protected_method" ... } });
    assert_ir_not_matches!(ir, quote! { Func { name: "private_method" ... } });
}

#[test]
fn test_record_private_static_member_functions_not_present() {
    let ir = ir_from_cc(
        "
        struct SomeStruct {
          public:
            static int public_method();
          protected:
            static int protected_method();
          private:
            static int private_method();
        };
    ",
    )
    .unwrap();

    assert_ir_matches!(ir, quote! { Func { name: "public_method" ... } });
    assert_ir_not_matches!(ir, quote! { Func { name: "protected_method" ... } });
    assert_ir_not_matches!(ir, quote! { Func { name: "private_method" ... } });
}

#[test]
fn test_record_special_member_access_specifiers() {
    let ir = ir_from_cc(
        "
        struct SomeStruct {
          private:
            SomeStruct(SomeStruct& s);
          protected:
            SomeStruct(SomeStruct&& s);
          public:
            ~SomeStruct();
        };
    ",
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            Record {
                identifier: "SomeStruct" ...
                copy_constructor: SpecialMemberFunc { ... access: Private ... },
                move_constructor: SpecialMemberFunc { ... access: Protected ... },
                destructor: SpecialMemberFunc { ... access: Public ... } ...
            }
        }
    );
}

#[test]
fn test_record_special_member_definition() {
    let ir = ir_from_cc(
        "
        struct SomeStruct {
          private:
            SomeStruct(SomeStruct& s);
          protected:
            SomeStruct(SomeStruct&& s) = delete;
        };
    ",
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            Record {
                identifier: "SomeStruct" ...
                copy_constructor: SpecialMemberFunc { definition: NontrivialUserDefined ... },
                move_constructor: SpecialMemberFunc { definition: Deleted ... },
                destructor: SpecialMemberFunc { definition: Trivial ... } ...
            }
        }
    );
}

#[test]
fn test_pointer_member_variable() {
    let ir = ir_from_cc(
        "struct SomeStruct {
            SomeStruct* ptr;
        };",
    )
    .unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            Field {
                identifier: "ptr" ...
                type_: MappedType {
                    rs_type: RsType {
                        name: Some("*mut") ...
                        type_args: [RsType {
                            name: None ...
                            type_args: [],
                            decl_id: Some(...),
                        }],
                        decl_id: None,
                    },
                    cc_type: CcType {
                        name: Some("*") ...
                        type_args: [CcType {
                            name: None ...
                            type_args: [],
                            decl_id: Some(...),
                        }],
                        decl_id: None,
                    },
                } ...
            }
        }
    );
}

#[test]
fn test_doc_comment() -> Result<()> {
    let ir = ir_from_cc(
        r#"
            /// Doc comment
            ///
            ///  * with three slashes
            struct DocCommentSlashes {};

            //! Doc comment
            //!
            //!  * with slashes and bang
            struct DocCommentBang {};

            /** Multiline comment

                * with two stars */
            struct MultilineCommentTwoStars {};

            // Line comment
            //
            //  * with two slashes
            struct LineComment {};

            /* Multiline comment

                * with one star */
            struct MultilineOneStar {};
        "#,
    )?;
    let comments: HashMap<_, _> = ir
        .records()
        .map(|r| (r.identifier.identifier.as_str(), r.doc_comment.as_ref().unwrap()))
        .collect();

    assert_eq!(comments["DocCommentSlashes"], "Doc comment\n\n * with three slashes");
    assert_eq!(comments["DocCommentBang"], "Doc comment\n\n * with slashes and bang");

    // TODO(forster): The bullet point is not retained in this
    // case. Instead we get the space at the end. Not sure if this
    // can be fixed easily...
    assert_eq!(comments["MultilineCommentTwoStars"], "Multiline comment\n\n with two stars ");
    assert_eq!(comments["LineComment"], "Line comment\n\n * with two slashes");

    // TODO(forster): The bullet point is not retained in this
    // case. Instead we get the space at the end. Not sure if this
    // can be fixed easily...
    assert_eq!(comments["MultilineOneStar"], "Multiline comment\n\n with one star ");

    Ok(())
}

#[test]
fn test_doc_comment_vs_tooling_directives() -> Result<()> {
    let ir = ir_from_cc(
        r#" // Doc comment for `f1`
            // NOLINTNEXTLINE(google3-readability-pass-trivial-by-value)
            void f1();

            // Doc comment for `f2`
            // // NOLINT
            void f2();

            // // NOLINT
            static void f3();

            // Mid-sentence usage: [...] this is why we need NOLINT / virtual [...].
            void f4();

            // No closing paren still suppresses
            // NOLINTNEXTLINE(google3-readability
            void f5();

            // Multiple, comma-separated directives listed in parens
            // NOLINTNEXTLINE(foo,bar)
            void f6();
        "#,
    )?;

    let comments: HashMap<&str, Option<&str>> = ir
        .functions()
        .map(|f| {
            if let UnqualifiedIdentifier::Identifier(id) = &f.name {
                (id.identifier.as_str(), f.doc_comment.as_deref())
            } else {
                panic!("No constructors/destructors expected in this test.")
            }
        })
        .collect();

    assert_eq!(comments["f1"], Some("Doc comment for `f1`"));
    assert_eq!(comments["f2"], Some("Doc comment for `f2`"));
    assert_eq!(comments["f3"], None);
    assert_eq!(
        comments["f4"],
        Some("Mid-sentence usage: [...] this is why we need NOLINT / virtual [...].")
    );
    assert_eq!(comments["f5"], Some("No closing paren still suppresses"));
    assert_eq!(comments["f6"], Some("Multiple, comma-separated directives listed in parens"));
    Ok(())
}

// TOOD(rosica): Reenable once b/208377928 is fixed. (Also disallow dead code.)
// #[test]
#[allow(dead_code)]
fn test_type_conversion() -> Result<()> {
    // TODO(mboehme): Add tests for the corresponding versions of the types in
    // the `std` namespace. We currently can't do this because we can't include
    // C++ standard library headers such as <cstdint>, only builtin headers such
    // as <stdint.h> (see b/214344126).
    let ir = ir_from_cc(
        r#"
            #include <stdint.h>
            #include <stddef.h>

            // We mock types from the C++ standard library because it's hard to
            // make headers that aren't part of the compiler available to a unit test.
            namespace std {
              using ::int8_t;
              using ::int16_t;
              using ::int32_t;
              using ::int64_t;

              using ::uint8_t;
              using ::uint16_t;
              using ::uint32_t;
              using ::uint64_t;

              using ::ptrdiff_t;
              using ::size_t;
              using ::intptr_t;
              using ::uintptr_t;
            }

            struct S {
                bool b;

                char c;
                unsigned char uc;
                signed char sc;
                char16_t c16;
                char32_t c32;
                wchar_t wc;

                short s;
                int i;
                long l;
                long long ll;

                unsigned short us;
                unsigned int ui;
                unsigned long ul;
                unsigned long long ull;

                signed short ss;
                signed int si;
                signed long sl;
                signed long long sll;

                int8_t i8;
                int16_t i16;
                int32_t i32;
                int64_t i64;
                std::int8_t std_i8;
                std::int16_t std_i16;
                std::int32_t std_i32;
                std::int64_t std_i64;

                uint8_t u8;
                uint16_t u16;
                uint32_t u32;
                uint64_t u64;
                std::uint8_t std_u8;
                std::uint16_t std_u16;
                std::uint32_t std_u32;
                std::uint64_t std_u64;

                ptrdiff_t pt;
                size_t st;
                intptr_t ip;
                uintptr_t up;
                std::ptrdiff_t std_pt;
                std::size_t std_st;
                std::intptr_t std_ip;
                std::uintptr_t std_up;

                float f;
                double d;
            };
        "#,
    )?;
    let fields = ir.records().next().unwrap().fields.iter();
    let type_mapping: HashMap<_, _> = fields
        .map(|f| {
            (
                f.type_.cc_type.name.as_ref().unwrap().as_str(),
                f.type_.rs_type.name.as_ref().unwrap().as_str(),
            )
        })
        .collect();

    assert_eq!(type_mapping["bool"], "bool");

    assert_eq!(type_mapping["char"], "i8");
    assert_eq!(type_mapping["unsigned char"], "u8");
    assert_eq!(type_mapping["signed char"], "i8");
    assert_eq!(type_mapping["char16_t"], "u16");
    // We cannot map C++ char32_t or wchar_t to Rust char,
    // because Rust requires that chars are valid UTF scalar values.
    assert_eq!(type_mapping["char32_t"], "u32");
    assert_eq!(type_mapping["wchar_t"], "i32");

    assert_eq!(type_mapping["short"], "i16");
    assert_eq!(type_mapping["int"], "i32");
    assert_eq!(type_mapping["long"], "i64");
    assert_eq!(type_mapping["long long"], "i64");

    assert_eq!(type_mapping["unsigned short"], "u16");
    assert_eq!(type_mapping["unsigned int"], "u32");
    assert_eq!(type_mapping["unsigned long"], "u64");
    assert_eq!(type_mapping["unsigned long long"], "u64");

    assert_eq!(type_mapping["short"], "i16");
    assert_eq!(type_mapping["int"], "i32");
    assert_eq!(type_mapping["long"], "i64");
    assert_eq!(type_mapping["long long"], "i64");

    assert_eq!(type_mapping["int8_t"], "i8");
    assert_eq!(type_mapping["int16_t"], "i16");
    assert_eq!(type_mapping["int32_t"], "i32");
    assert_eq!(type_mapping["int64_t"], "i64");
    assert_eq!(type_mapping["std::int8_t"], "i8");
    assert_eq!(type_mapping["std::int16_t"], "i16");
    assert_eq!(type_mapping["std::int32_t"], "i32");
    assert_eq!(type_mapping["std::int64_t"], "i64");

    assert_eq!(type_mapping["uint8_t"], "u8");
    assert_eq!(type_mapping["uint16_t"], "u16");
    assert_eq!(type_mapping["uint32_t"], "u32");
    assert_eq!(type_mapping["uint64_t"], "u64");
    assert_eq!(type_mapping["std::uint8_t"], "u8");
    assert_eq!(type_mapping["std::uint16_t"], "u16");
    assert_eq!(type_mapping["std::uint32_t"], "u32");
    assert_eq!(type_mapping["std::uint64_t"], "u64");

    assert_eq!(type_mapping["ptrdiff_t"], "isize");
    assert_eq!(type_mapping["size_t"], "usize");
    assert_eq!(type_mapping["intptr_t"], "isize");
    assert_eq!(type_mapping["uintptr_t"], "usize");
    assert_eq!(type_mapping["std::ptrdiff_t"], "isize");
    assert_eq!(type_mapping["std::size_t"], "usize");
    assert_eq!(type_mapping["std::intptr_t"], "isize");
    assert_eq!(type_mapping["std::uintptr_t"], "usize");

    assert_eq!(type_mapping["float"], "f32");
    assert_eq!(type_mapping["double"], "f64");

    Ok(())
}

#[test]
fn test_typedef() -> Result<()> {
    let ir = ir_from_cc(
        r#"
            typedef int MyTypedefDecl;
            using MyTypeAliasDecl = int;
        "#,
    )?;

    let int = quote! {
      MappedType {
        rs_type: RsType {
          name: Some("i32"),
          lifetime_args: [],
          type_args: [],
          decl_id: None,
        },
        cc_type: CcType {
          name: Some("int"),
          is_const: false,
          type_args: [],
          decl_id: None,
        },
      }
    };
    assert_ir_matches!(
        ir,
        quote! {
          TypeAlias {
            identifier: "MyTypedefDecl",
            id: DeclId(...),
            owning_target: BlazeLabel("//test:testing_target"),
            underlying_type: #int,
          }
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
          TypeAlias {
            identifier: "MyTypeAliasDecl",
            id: DeclId(...),
            owning_target: BlazeLabel("//test:testing_target"),
            underlying_type: #int,
          }
        }
    );

    Ok(())
}

#[test]
fn test_well_known_types_check_namespaces() -> Result<()> {
    // Check that we don't treat a type called `int32_t` in a user-defined
    // namespace as if it was the standard type `int32_t`.
    // Because we don't support namespaces yet, the outcome of this should be
    // that `f()` is unsupported, rather than being imported with a parameter
    // type of `i32`.
    // Once we support namespaces, change this test to check that `f()` is
    // imported with the correct paramter type `my_namespace::int32_t`.
    let ir = ir_from_cc(
        r#"
            namespace my_namespace {
              using int32_t = int;
            }
            void f(my_namespace::int32_t i);
        "#,
    )?;
    assert_strings_contain(
        ir.unsupported_items().map(|i| i.name.as_str()).collect_vec().as_slice(),
        "f",
    );
    Ok(())
}

#[test]
fn test_dont_import_typedef_nested_in_func() {
    let ir = ir_from_cc("inline void f() { typedef int MyTypedefDecl; }").unwrap();
    assert_ir_not_matches!(ir, quote! { TypeAlias { identifier: "MyTypedefDecl" ... } });
}

#[test]
fn test_typedef_nested_in_record_not_supported() {
    let ir = ir_from_cc("struct S { typedef int MyTypedefDecl; };").unwrap();
    assert_strings_contain(
        ir.unsupported_items().map(|i| i.name.as_str()).collect_vec().as_slice(),
        "S::MyTypedefDecl",
    );
}

#[test]
fn test_records_nested_in_records_not_supported_yet() {
    let ir = ir_from_cc("struct SomeStruct { struct NestedStruct {}; };").unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
          name: "SomeStruct::NestedStruct",
          message: "Nested classes are not supported yet" ...
        }}
    );
}
#[test]
fn test_do_not_import_static_member_functions_when_record_not_supported_yet() {
    // only using nested struct as an example of a record we cannot import yet.
    let ir = ir_from_cc(
        "
        struct SomeStruct {
          struct NestedStruct {
            static void StaticMemberFunction();
          };
        };",
    )
    .unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
          name: "SomeStruct::NestedStruct::StaticMemberFunction" ...
        }}
    );
}

#[test]
fn test_do_not_import_nonstatic_member_functions_when_record_not_supported_yet() {
    // only using nested struct as an example of a record we cannot import yet.
    let ir = ir_from_cc(
        "
        struct SomeStruct {
          struct NestedStruct {
            void NonStaticMemberFunction();
          };
        };",
    )
    .unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
          name: "SomeStruct::NestedStruct::NonStaticMemberFunction" ...
        }}
    );
}

#[test]
fn test_dont_import_injected_class_name() {
    let ir = ir_from_cc("struct SomeStruct {};").unwrap();
    let names = ir.records().map(|r| &r.identifier.identifier).filter(|n| n.contains("SomeStruct"));
    // if we do support nested structs, we should not emit record for injected class
    // name
    assert_eq!(names.count(), 1);
    // if we don't support nested structs, we should not emit unsupported item for
    // injected class name
    assert_ir_not_matches!(
        ir,
        quote! { UnsupportedItem {
          name: "SomeStruct::SomeStruct",
          message: "Nested classes are not supported yet" ...
        }}
    );
}

#[test]
fn test_integer_typedef_usage() -> Result<()> {
    // This is a regression test. We used to incorrectly desugar typedefs of
    // builtin types and treat them as if they were the underlying builtin type.
    // As a result, this test would produce a binding for f(MyTypedef) with a
    // parameter of type `int` instead of `MyTypedef`. This test therefore
    // checks that the type has a `decl_id` but doesn't have a `name`. More
    // specific checks are done in the code generation tests.
    let ir = ir_from_cc(
        r#"
            typedef int MyTypedef;
            void f(MyTypedef my_typedef);
        "#,
    )?;
    assert_ir_matches!(
        ir,
        quote! { Func {
         name: "f", ...
         params: [
           FuncParam {
             type_: MappedType {
               rs_type: RsType {
                 name: None, ...
                 decl_id: Some(...), ...
               },
               cc_type: CcType {
                 name: None, ...
                 decl_id: Some(...), ...
               },
             },
             identifier: "my_typedef",
           }], ...
        } }
    );

    Ok(())
}

#[test]
fn test_struct_forward_declaration() {
    let ir = ir_from_cc("struct Struct;").unwrap();
    assert!(!ir.records().any(|r| r.identifier.identifier == "Struct"));
}

#[test]
fn test_member_function_params() {
    let ir = ir_from_cc(
        r#"
            struct Struct {
                void Foo(int x, int y);
            };
        "#,
    )
    .unwrap();
    let foo_func =
        ir.functions().find(|f| f.name == UnqualifiedIdentifier::Identifier(ir_id("Foo"))).unwrap();
    let param_names: Vec<_> = foo_func.params.iter().map(|p| &p.identifier.identifier).collect();
    assert_eq!(param_names, vec!["__this", "x", "y"]);
}

fn assert_member_function_with_predicate_has_instance_method_metadata<F: FnMut(&Func) -> bool>(
    ir: &IR,
    record_name: &str,
    mut func_predicate: F,
    expected_metadata: &Option<ir::InstanceMethodMetadata>,
) {
    let record =
        ir.records().find(|r| r.identifier.identifier == record_name).expect("Struct not found");
    let function = ir.functions().find(|f| func_predicate(*f));
    let meta = function
        .expect("Function not found")
        .member_func_metadata
        .as_ref()
        .expect("Member function should specify member_func_metadata");
    assert_eq!(meta.record_id, record.id);
    assert_eq!(&meta.instance_method_metadata, expected_metadata);
}

fn assert_member_function_has_instance_method_metadata(
    name: &str,
    definition: &str,
    expected_metadata: &Option<ir::InstanceMethodMetadata>,
) {
    let mut file = String::new();
    file += "struct Struct {\n  ";
    file += definition;
    file += "\n};";
    let ir = ir_from_cc(&file).unwrap();

    assert_member_function_with_predicate_has_instance_method_metadata(
        &ir,
        "Struct",
        |f| f.name == UnqualifiedIdentifier::Identifier(ir_id(name)),
        expected_metadata,
    );
}

#[test]
fn test_member_function_static() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "static void Function();",
        &None,
    );
}

#[test]
fn test_member_function() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "void Function();",
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::Unqualified,
            is_const: false,
            is_virtual: false,
            is_explicit_ctor: false,
        }),
    );
}

#[test]
fn test_member_function_const() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "void Function() const;",
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::Unqualified,
            is_const: true,
            is_virtual: false,
            is_explicit_ctor: false,
        }),
    );
}

#[test]
fn test_member_function_virtual() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "virtual void Function();",
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::Unqualified,
            is_const: false,
            is_virtual: true,
            is_explicit_ctor: false,
        }),
    );
}

#[test]
fn test_member_function_lvalue() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "void Function() &;",
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::LValue,
            is_const: false,
            is_virtual: false,
            is_explicit_ctor: false,
        }),
    );
}

#[test]
fn test_member_function_rvalue() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "void Function() &&;",
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::RValue,
            is_const: false,
            is_virtual: false,
            is_explicit_ctor: false,
        }),
    );
}

#[test]
fn test_member_function_explicit_constructor() {
    let ir = ir_from_cc(
        r#"
        struct SomeStruct {
          explicit SomeStruct(int i);
          SomeStruct() = delete;
          SomeStruct(const SomeStruct&) = delete;
        }; "#,
    )
    .unwrap();
    assert_member_function_with_predicate_has_instance_method_metadata(
        &ir,
        "SomeStruct",
        |f| f.name == UnqualifiedIdentifier::Constructor,
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::Unqualified,
            is_const: false,
            is_virtual: false,
            is_explicit_ctor: true,
        }),
    );
}

#[test]
fn test_member_function_implicit_constructor() {
    let ir = ir_from_cc(
        r#"
        struct SomeStruct {
          SomeStruct(int i);
          SomeStruct() = delete;
          SomeStruct(const SomeStruct&) = delete;
        }; "#,
    )
    .unwrap();
    assert_member_function_with_predicate_has_instance_method_metadata(
        &ir,
        "SomeStruct",
        |f| f.name == UnqualifiedIdentifier::Constructor,
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::Unqualified,
            is_const: false,
            is_virtual: false,
            is_explicit_ctor: false,
        }),
    );
}

fn get_func_names(definition: &str) -> Vec<ir::UnqualifiedIdentifier> {
    let ir = ir_from_cc(definition).unwrap();
    ir.functions().map(|f| f.name.clone()).collect()
}

#[test]
fn test_identifier_function_name() {
    assert_eq!(
        get_func_names("void Function();"),
        vec![ir::UnqualifiedIdentifier::Identifier(ir::Identifier {
            identifier: "Function".into()
        })],
    );
}

#[test]
fn test_constructor_function_name() {
    assert!(
        get_func_names("struct Struct {Struct();};")
            .contains(&ir::UnqualifiedIdentifier::Constructor)
    );
}

#[test]
fn test_destructor_function_name() {
    assert!(
        get_func_names("struct Struct {~Struct();};")
            .contains(&ir::UnqualifiedIdentifier::Destructor)
    );
}

#[test]
fn test_unsupported_items_are_emitted() -> Result<()> {
    // We will have to rewrite this test to use something else that is unsupported
    // once we start importing structs from namespaces.
    let ir = ir_from_cc("namespace my_namespace { struct StructFromNamespaceIsUnsupported {}; }")?;
    assert_strings_contain(
        ir.unsupported_items().map(|i| i.name.as_str()).collect_vec().as_slice(),
        "my_namespace::StructFromNamespaceIsUnsupported",
    );
    Ok(())
}

#[test]
fn test_unsupported_items_are_emitted_from_reopened_namespace() -> Result<()> {
    // Once we actually support namespaces, change this test to check that we
    // emit the struct from the reopened namespace.
    let ir = ir_from_cc(
        r#"namespace my_namespace {}
         namespace my_namespace {
           struct StructFromNamespaceIsUnsupported {};
         }"#,
    )?;
    assert_strings_contain(
        ir.unsupported_items().map(|i| i.name.as_str()).collect_vec().as_slice(),
        "my_namespace::StructFromNamespaceIsUnsupported",
    );
    Ok(())
}

#[test]
fn test_unsupported_items_from_dependency_are_not_emitted() -> Result<()> {
    // We will have to rewrite this test to use something else that is unsupported
    // once we start importing structs from namespaces.
    let ir = ir_from_cc_dependency(
        "struct MyOtherStruct { my_namespace::StructFromNamespaceIsUnsupported my_struct; };",
        "namespace my_namespace { struct StructFromNamespaceIsUnsupported {}; }",
    )?;
    let names = ir.unsupported_items().map(|i| i.name.as_str()).collect_vec();
    assert_strings_dont_contain(names.as_slice(), "my_namespace::StructFromNamespaceIsUnsupported");
    assert_strings_contain(names.as_slice(), "MyOtherStruct");
    Ok(())
}

#[test]
fn test_user_of_unsupported_type_is_unsupported() -> Result<()> {
    // We will have to rewrite this test to use something else that is unsupported
    // once we start importing structs from namespaces.
    let ir = ir_from_cc(
        r#"namespace my_namespace { struct Unsupported {}; }
           void f(my_namespace::Unsupported* unsupported);
           struct S { my_namespace::Unsupported unsupported; };"#,
    )?;
    let names = ir.unsupported_items().map(|i| i.name.as_str()).collect_vec();
    assert_strings_contain(&names, "my_namespace::Unsupported");
    assert_strings_contain(&names, "f");
    assert_strings_contain(&names, "S");
    Ok(())
}

fn assert_strings_contain(strings: &[&str], expected_string: &str) {
    assert!(
        strings.iter().any(|s| *s == expected_string),
        "Value '{}' was unexpectedly missing from {:?}",
        expected_string,
        strings
    );
}

fn assert_strings_dont_contain(strings: &[&str], unexpected_string: &str) {
    assert!(
        strings.iter().all(|s| *s != unexpected_string),
        "Value '{}' was unexpectedly found in {:?}",
        unexpected_string,
        strings
    );
}

#[test]
fn test_elided_lifetimes() {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct S {
          int& f(int& i);
        };"#,
    )
    .unwrap();
    let func = retrieve_func(&ir, "f");
    let lifetime_params = &func.lifetime_params;
    assert_eq!(lifetime_params.iter().map(|p| &p.name).collect_vec(), vec!["a", "b"]);
    let a_id = lifetime_params[0].id;
    let b_id = lifetime_params[1].id;
    assert_eq!(func.return_type.rs_type.lifetime_args, vec![a_id]);

    assert_eq!(func.params[0].identifier, ir_id("__this"));
    assert_eq!(func.params[0].type_.rs_type.name, Some("&mut".to_string()));
    assert_eq!(func.params[0].type_.rs_type.lifetime_args, vec![a_id]);

    assert_eq!(func.params[1].identifier, ir_id("i"));
    assert_eq!(func.params[1].type_.rs_type.name, Some("&mut".to_string()));
    assert_eq!(func.params[1].type_.rs_type.lifetime_args, vec![b_id]);
}

fn verify_elided_lifetimes_in_default_constructor(ir: &IR) {
    let r = ir.records().next().expect("IR should contain `struct S`");
    assert_eq!(r.identifier, ir_id("S"));
    assert!(r.is_trivial_abi);

    let f = ir
        .functions()
        .find(|f| matches!(&f.name, UnqualifiedIdentifier::Constructor) && f.params.len() == 1)
        .expect("IR should contain the default constructor");
    assert_eq!(f.lifetime_params.len(), 1);

    let p = f.params.first().expect("IR should contain `__this` parameter");
    assert_eq!(p.identifier, ir_id("__this"));

    let t = &p.type_.rs_type;
    assert_eq!(t.lifetime_args.len(), 1);
    assert_eq!(t.lifetime_args[0], f.lifetime_params[0].id);
    assert_eq!(t.name, Some("&mut".to_string()));
}

#[test]
fn test_operator_names() {
    let ir = ir_from_cc(
        r#"
        // TOOD(b/208377928): Use #include <stddef.h> instead of declaring `size_t` ourselves...
        using size_t = unsigned long;
        #pragma clang lifetime_elision
        struct SomeStruct {
          // There is an implicit/default `oparator=` hidden here as well.
          void* operator new(size_t size);
          void* operator new[](size_t size);
          bool operator==(const SomeStruct& other) const;
        };"#,
    )
    .unwrap();
    let operator_names: HashSet<&str> = ir
        .functions()
        .filter(|f| {
            // Only SomeStruct member functions (excluding stddef.h stuff).
            f.member_func_metadata
                .as_ref()
                .map(|m| m.find_record(&ir).unwrap().identifier.identifier == "SomeStruct")
                .unwrap_or_default()
        })
        .flat_map(|f| match &f.name {
            UnqualifiedIdentifier::Operator(op) => Some(op.name.as_ref()),
            _ => None,
        })
        .collect();
    assert!(operator_names.contains("="));
    assert!(operator_names.contains("new"));
    assert!(operator_names.contains("new[]"));
    assert!(operator_names.contains("=="));
}

#[test]
fn test_elided_lifetimes_in_default_constructor_with_implicit_default() {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct S {
          int i;
        };"#,
    )
    .unwrap();
    verify_elided_lifetimes_in_default_constructor(&ir);
}

#[test]
fn test_elided_lifetimes_in_default_constructor_with_explicit_default() {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct S {
          S() = default;
          int i;
        };"#,
    )
    .unwrap();
    verify_elided_lifetimes_in_default_constructor(&ir);
}

#[test]
fn test_no_aligned_attr() {
    let ir = ir_from_cc("struct SomeStruct {};").unwrap();

    assert_ir_matches! {ir, quote! {
      Record {
        ... identifier: "SomeStruct" ...
        ... override_alignment: false ...
      }}
    };
}

#[test]
fn test_aligned_attr() {
    let ir = ir_from_cc("struct SomeStruct {} __attribute__((aligned(64)));").unwrap();

    assert_ir_matches! {ir, quote! {
      Record {
        ... identifier: "SomeStruct" ...
        ... override_alignment: true ...
      }}
    };
}

#[test]
fn test_volatile_is_unsupported() {
    let ir = ir_from_cc("volatile int* foo();").unwrap();
    let f = ir.unsupported_items().find(|i| i.message.contains("volatile")).unwrap();
    assert_eq!("foo", f.name);
}

#[test]
fn test_unnamed_enum_unsupported() {
    let ir = ir_from_cc("enum { kFoo = 1, kBar = 2 };").unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            UnsupportedItem {
                name: "(anonymous)",
                message: "Unnamed enums are not supported yet" ...
            }
        }
    );
}
