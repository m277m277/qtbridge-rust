// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::QApp;

use tst_qabstractitemmodel::Backend;

fn main() {
    let backend = Rc::new(RefCell::new(Backend::default()));

    QApp::new()
        .set_initial_object("rustmodel", backend)
        .load_qml(include_bytes!("main.qml"))
        .run();
}
