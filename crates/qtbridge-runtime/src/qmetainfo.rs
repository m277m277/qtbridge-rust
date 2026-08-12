// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::DynamicMetaObjectBuilder;

/// Describes the Rust-side additions to a QObject's meta-object: the
/// signals, slots, and properties declared on the type.
///
/// Pure description: the realized meta-object and metatypes live on the
/// proxy pair in [`crate::QObjectHolder`], since they chain to the concrete
/// C++ proxy class.
pub trait QMetaInfo : 'static {
    /// The class_name in the Qt meta-object system
    fn class_name() -> &'static str {
        std::any::type_name::<Self>()
    }

    /// This function takes the meta_obj_builder and builds a meta object
    /// that contains all slots, signals and properties for the respective type.
    /// This function is usually implemented by a macro.
    fn build_dynamic_meta_type(meta_obj_builder: std::pin::Pin<&mut DynamicMetaObjectBuilder>);
}
