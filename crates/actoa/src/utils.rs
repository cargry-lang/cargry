/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::hash::Hash;

/// implement this for enum.
/// also implement `Eq` and `Hash` for it.
///
/// example:
/// ```
/// #[derive(Eq, Hash)]
/// enum RustRel {
///     Move(usize),
///     Borrow(usize),
///     MutBorrow(usize),
/// }
/// impl Relation for RustRel {}
/// ```
pub trait Relation: Eq + Hash {
    fn mapping(&self, input: String) -> String;
}

/// implement this for enum.
///
/// example:
/// ```
/// enum MyLangType {
///     Function(Box<MyLangType>, Box<MyLangType>),
///     Ref(Box<MyLangType>),
///     Super,
///     Type(String, Vec<Box<MyLangType>>),
///     Unknown,
/// }
/// impl TypeExpr for MyLangType {}
/// ```
pub trait TypeExpr {
    fn mapping(&self) -> String;
}
