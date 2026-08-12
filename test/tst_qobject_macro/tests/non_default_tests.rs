// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! A `NoQmlElement` type needs no `Default`: constructor-required structs
//! are ordinary `#[qobject]` types as long as QML never instantiates them.

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::{QmlObject, qobject};
use qtbridge::qtbridge_runtime::QObjectHolder;

#[qobject(NoQmlElement)]
pub mod backend {
    pub struct Backend {
        value: i32,
    }

    impl Backend {
        pub fn new(value: i32) -> Self {
            Self { value }
        }

        #[qslot]
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
pub use backend::Backend;

fn constructor_required_type_attaches() {
    let backend = Rc::new(RefCell::new(Backend::new(7)));
    Backend::attach_qobject(&backend);
    assert!(!backend.borrow().get_qobject_ptr().is_null());
    assert_eq!(backend.borrow().value(), 7);
}

#[cfg(not(miri))]
fn main() {
    constructor_required_type_attaches();
}

#[cfg(miri)]
fn main() {}
