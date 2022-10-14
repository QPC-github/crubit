// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/method_qualifiers.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN6Inline17UnqualifiedMethodEv(
    struct Inline* __this) {
  __this->UnqualifiedMethod();
}
extern "C" void __rust_thunk___ZNR6Inline12LvalueMethodEv(
    struct Inline* __this) {
  __this->LvalueMethod();
}
extern "C" void __rust_thunk___ZNKR6Inline17LvalueMethodConstEv(
    const struct Inline* __this) {
  __this->LvalueMethodConst();
}
extern "C" void __rust_thunk___ZNO6Inline12RvalueMethodEv(
    struct Inline* __this) {
  std::move(*__this).RvalueMethod();
}
extern "C" void __rust_thunk___ZNKO6Inline17RvalueMethodConstEv(
    const struct Inline* __this) {
  std::move(*__this).RvalueMethodConst();
}

static_assert(sizeof(struct Noninline) == 1);
static_assert(alignof(struct Noninline) == 1);

static_assert(sizeof(struct Inline) == 1);
static_assert(alignof(struct Inline) == 1);

#pragma clang diagnostic pop