// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

// cxx-qt defines init functions in an archive that is linked after the one
// referencing them, which ld.gold do not resolve. This translation unit is
// force-linked (see build.rs), so the demand always precedes that scan.
// It does not to be called, just be linked.
// If you still observe problems call the init macros in a file that lands
// in your final translation unit.
// See also
// https://kdab.github.io/cxx-qt/book/common-issues.html#cargo-linker-error-undefined-reference-to-cxx_qt_init_
extern "C" bool cxx_qt_init_crate_cxx_qt_lib();
extern "C" bool cxx_qt_init_crate_cxx_qt();

namespace {
const bool qtbridge_cxx_qt_init =
  cxx_qt_init_crate_cxx_qt_lib() && cxx_qt_init_crate_cxx_qt();
}
