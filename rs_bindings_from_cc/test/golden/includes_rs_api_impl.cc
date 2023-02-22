// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:includes_cc

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/includes.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

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

#pragma clang diagnostic pop
