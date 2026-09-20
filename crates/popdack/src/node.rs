/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// Reference to node
pub struct RefN(usize);

/// BRSNode (Bordered Reference Series Node)
///
/// Recomend using 'lewekk' crate.
pub struct BRSNode<Nodes> {
    node_buf: Vec<Nodes>,
}
