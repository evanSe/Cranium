/*
This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
If a copy of the MPL was not distributed with this file,
You can obtain one at https://mozilla.org/MPL/2.0/.
*/

//! ActionSets - bundles of Actions, typically all the affordances of a SmartObject or an NPC.

use crate::actions::ActionTemplate;
use bevy::prelude::*;

#[cfg(feature = "actionset_loader")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Reflect)]
#[cfg_attr(
    any(feature = "actionset_loader"),
    derive(Serialize, Deserialize, Asset)
)]
pub struct ActionSet {
    pub name: String,
    pub actions: crate::types::CraniumList<ActionTemplate>,
}

impl ActionSet {
    pub fn new<IS: Into<String>>(
        name: IS,
        actions: crate::types::CraniumList<ActionTemplate>,
    ) -> Self {
        Self {
            name: name.into(),
            actions: actions,
        }
    }
}
