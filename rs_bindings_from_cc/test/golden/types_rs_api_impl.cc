// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:types_cc

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/types.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN10SomeStructC1Ev(struct SomeStruct* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN10SomeStructC1EOS_(
    struct SomeStruct* __this, struct SomeStruct* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" struct SomeStruct* __rust_thunk___ZN10SomeStructaSERKS_(
    struct SomeStruct* __this, const struct SomeStruct* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct SomeStruct* __rust_thunk___ZN10SomeStructaSEOS_(
    struct SomeStruct* __this, struct SomeStruct* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN19FieldTypeTestStructC1EOS_(
    struct FieldTypeTestStruct* __this, struct FieldTypeTestStruct* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___Z21VoidReturningFunctionv() {
  VoidReturningFunction();
}
extern "C" crubit::type_identity_t<int&(int const&, int*)>*
__rust_thunk___Z32FunctionPointerReturningFunctionv() {
  return FunctionPointerReturningFunction();
}
extern "C" void* __rust_thunk___Z24FunctionWithVoidPointersPvPKv(
    void* __param_0, void const* __param_1) {
  return FunctionWithVoidPointers(__param_0, __param_1);
}

static_assert(sizeof(struct std::integral_constant<bool, false>) == 1);
static_assert(alignof(struct std::integral_constant<bool, false>) == 1);

static_assert(sizeof(struct std::integral_constant<bool, true>) == 1);
static_assert(alignof(struct std::integral_constant<bool, true>) == 1);

static_assert(
    sizeof(struct std::__type_list<
           std::__align_type<unsigned char>,
           std::__type_list<
               std::__align_type<unsigned short>,
               std::__type_list<
                   std::__align_type<unsigned int>,
                   std::__type_list<
                       std::__align_type<unsigned long>,
                       std::__type_list<
                           std::__align_type<unsigned long long>,
                           std::__type_list<
                               std::__align_type<double>,
                               std::__type_list<
                                   std::__align_type<long double>,
                                   std::__type_list<
                                       std::__align_type<std::__struct_double>,
                                       std::__type_list<
                                           std::__align_type<
                                               std::__struct_double4>,
                                           std::__type_list<
                                               std::__align_type<int*>,
                                               std::__nat>>>>>>>>>>) == 1);
static_assert(
    alignof(struct std::__type_list<
            std::__align_type<unsigned char>,
            std::__type_list<
                std::__align_type<unsigned short>,
                std::__type_list<
                    std::__align_type<unsigned int>,
                    std::__type_list<
                        std::__align_type<unsigned long>,
                        std::__type_list<
                            std::__align_type<unsigned long long>,
                            std::__type_list<
                                std::__align_type<double>,
                                std::__type_list<
                                    std::__align_type<long double>,
                                    std::__type_list<
                                        std::__align_type<std::__struct_double>,
                                        std::__type_list<
                                            std::__align_type<
                                                std::__struct_double4>,
                                            std::__type_list<
                                                std::__align_type<int*>,
                                                std::__nat>>>>>>>>>>) == 1);

static_assert(sizeof(struct SomeStruct) == 1);
static_assert(alignof(struct SomeStruct) == 1);

static_assert(sizeof(struct FieldTypeTestStruct) == 312);
static_assert(alignof(struct FieldTypeTestStruct) == 8);
static_assert(CRUBIT_OFFSET_OF(bool_field, struct FieldTypeTestStruct) == 0);
static_assert(CRUBIT_OFFSET_OF(char_field, struct FieldTypeTestStruct) == 1);
static_assert(CRUBIT_OFFSET_OF(unsigned_char_field,
                               struct FieldTypeTestStruct) == 2);
static_assert(CRUBIT_OFFSET_OF(signed_char_field, struct FieldTypeTestStruct) ==
              3);
static_assert(CRUBIT_OFFSET_OF(char16_t_field, struct FieldTypeTestStruct) ==
              4);
static_assert(CRUBIT_OFFSET_OF(char32_t_field, struct FieldTypeTestStruct) ==
              8);
static_assert(CRUBIT_OFFSET_OF(wchar_t_field, struct FieldTypeTestStruct) ==
              12);
static_assert(CRUBIT_OFFSET_OF(short_field, struct FieldTypeTestStruct) == 16);
static_assert(CRUBIT_OFFSET_OF(int_field, struct FieldTypeTestStruct) == 20);
static_assert(CRUBIT_OFFSET_OF(long_field, struct FieldTypeTestStruct) == 24);
static_assert(CRUBIT_OFFSET_OF(long_long_field, struct FieldTypeTestStruct) ==
              32);
static_assert(CRUBIT_OFFSET_OF(unsigned_short_field,
                               struct FieldTypeTestStruct) == 40);
static_assert(CRUBIT_OFFSET_OF(unsigned_int_field,
                               struct FieldTypeTestStruct) == 44);
static_assert(CRUBIT_OFFSET_OF(unsigned_long_field,
                               struct FieldTypeTestStruct) == 48);
static_assert(CRUBIT_OFFSET_OF(unsigned_long_long_field,
                               struct FieldTypeTestStruct) == 56);
static_assert(CRUBIT_OFFSET_OF(signed_short_field,
                               struct FieldTypeTestStruct) == 64);
static_assert(CRUBIT_OFFSET_OF(signed_int_field, struct FieldTypeTestStruct) ==
              68);
static_assert(CRUBIT_OFFSET_OF(signed_long_field, struct FieldTypeTestStruct) ==
              72);
static_assert(CRUBIT_OFFSET_OF(signed_long_long_field,
                               struct FieldTypeTestStruct) == 80);
static_assert(CRUBIT_OFFSET_OF(int8_t_field, struct FieldTypeTestStruct) == 88);
static_assert(CRUBIT_OFFSET_OF(int16_t_field, struct FieldTypeTestStruct) ==
              90);
static_assert(CRUBIT_OFFSET_OF(int32_t_field, struct FieldTypeTestStruct) ==
              92);
static_assert(CRUBIT_OFFSET_OF(int64_t_field, struct FieldTypeTestStruct) ==
              96);
static_assert(CRUBIT_OFFSET_OF(std_int8_t_field, struct FieldTypeTestStruct) ==
              104);
static_assert(CRUBIT_OFFSET_OF(std_int16_t_field, struct FieldTypeTestStruct) ==
              106);
static_assert(CRUBIT_OFFSET_OF(std_int32_t_field, struct FieldTypeTestStruct) ==
              108);
static_assert(CRUBIT_OFFSET_OF(std_int64_t_field, struct FieldTypeTestStruct) ==
              112);
static_assert(CRUBIT_OFFSET_OF(uint8_t_field, struct FieldTypeTestStruct) ==
              120);
static_assert(CRUBIT_OFFSET_OF(uint16_t_field, struct FieldTypeTestStruct) ==
              122);
static_assert(CRUBIT_OFFSET_OF(uint32_t_field, struct FieldTypeTestStruct) ==
              124);
static_assert(CRUBIT_OFFSET_OF(uint64_t_field, struct FieldTypeTestStruct) ==
              128);
static_assert(CRUBIT_OFFSET_OF(std_uint8_t_field, struct FieldTypeTestStruct) ==
              136);
static_assert(CRUBIT_OFFSET_OF(std_uint16_t_field,
                               struct FieldTypeTestStruct) == 138);
static_assert(CRUBIT_OFFSET_OF(std_uint32_t_field,
                               struct FieldTypeTestStruct) == 140);
static_assert(CRUBIT_OFFSET_OF(std_uint64_t_field,
                               struct FieldTypeTestStruct) == 144);
static_assert(CRUBIT_OFFSET_OF(ptrdiff_t_field, struct FieldTypeTestStruct) ==
              152);
static_assert(CRUBIT_OFFSET_OF(size_t_field, struct FieldTypeTestStruct) ==
              160);
static_assert(CRUBIT_OFFSET_OF(intptr_t_field, struct FieldTypeTestStruct) ==
              168);
static_assert(CRUBIT_OFFSET_OF(uintptr_t_field, struct FieldTypeTestStruct) ==
              176);
static_assert(CRUBIT_OFFSET_OF(std_ptrdiff_t_field,
                               struct FieldTypeTestStruct) == 184);
static_assert(CRUBIT_OFFSET_OF(std_size_t_field, struct FieldTypeTestStruct) ==
              192);
static_assert(CRUBIT_OFFSET_OF(std_intptr_t_field,
                               struct FieldTypeTestStruct) == 200);
static_assert(CRUBIT_OFFSET_OF(std_uintptr_t_field,
                               struct FieldTypeTestStruct) == 208);
static_assert(CRUBIT_OFFSET_OF(rs_char_field, struct FieldTypeTestStruct) ==
              216);
static_assert(CRUBIT_OFFSET_OF(float_field, struct FieldTypeTestStruct) == 220);
static_assert(CRUBIT_OFFSET_OF(double_field, struct FieldTypeTestStruct) ==
              224);
static_assert(CRUBIT_OFFSET_OF(ptr_field, struct FieldTypeTestStruct) == 232);
static_assert(CRUBIT_OFFSET_OF(void_ptr_field, struct FieldTypeTestStruct) ==
              240);
static_assert(CRUBIT_OFFSET_OF(const_void_ptr_field,
                               struct FieldTypeTestStruct) == 248);
static_assert(CRUBIT_OFFSET_OF(void_double_ptr_field,
                               struct FieldTypeTestStruct) == 256);
static_assert(CRUBIT_OFFSET_OF(struct_field, struct FieldTypeTestStruct) ==
              264);
static_assert(CRUBIT_OFFSET_OF(struct_ptr_field, struct FieldTypeTestStruct) ==
              272);
static_assert(CRUBIT_OFFSET_OF(const_struct_ptr_field,
                               struct FieldTypeTestStruct) == 280);
static_assert(CRUBIT_OFFSET_OF(struct_ref_field, struct FieldTypeTestStruct) ==
              288);
static_assert(CRUBIT_OFFSET_OF(const_struct_ref_field,
                               struct FieldTypeTestStruct) == 296);
static_assert(CRUBIT_OFFSET_OF(forward_declared_ptr_field,
                               struct FieldTypeTestStruct) == 304);

#pragma clang diagnostic pop
