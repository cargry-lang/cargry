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
/// #[derive(Clone, Eq, Hash)]
/// enum RustRel {
///     Move(usize, usize),
///     Borrow(usize, usize),
///     MutBorrow(usize, usize),
/// }
/// impl Relation for RustRel {
///     fn mapping(&self, input: String) -> (String, String) {
///         match self {
///             Move(_, _) => (input.clone(), input),
///             Borrow(_, _) => (input.clone(), "&" + input),
///             MutBorrow(_, _) => (input.clone(), "&mut " + input),
///         }
///     }
///     fn first(&self) -> usize {
///         match self {
///             Move(fst, _) => fst,
///             Borrow(fst, _) => fst,
///             MutBorrow(fst, _) => fst,
///         }
///     }
///     fn second(&self) -> usize {
///         match self {
///             Move(_, snd) => snd,
///             Borrow(_, snd) => snd,
///             MutBorrow(_, snd) => snd,
///         }
///     }
/// }
/// ```
pub trait Relation: Clone + Eq + Hash {
    fn mapping(&self, input: String) -> (String, String);
    fn first(&self) -> usize;
    fn second(&self) -> usize;
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
