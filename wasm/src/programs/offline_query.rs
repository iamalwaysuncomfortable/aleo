// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use crate::types::native::{CurrentNetwork, Field, Network};
use snarkvm_console::program::StatePath;
use snarkvm_ledger_query::QueryTrait;

use anyhow::anyhow;
use async_trait::async_trait;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use std::str::FromStr;

/// An offline query object used to insert the global state root and state paths needed to create
/// a valid inclusion proof offline.
#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OfflineQuery {
    state_paths: IndexMap<Field<CurrentNetwork>, StatePath<CurrentNetwork>>,
    state_root: <CurrentNetwork as Network>::StateRoot,
}

#[wasm_bindgen]
impl OfflineQuery {
    /// Creates a new offline query object. The state root is required to be passed in as a string
    #[wasm_bindgen(constructor)]
    pub fn new(state_root: &str) -> Result<OfflineQuery, String> {
        let state_root = <CurrentNetwork as Network>::StateRoot::from_str(state_root).map_err(|e| e.to_string())?;
        Ok(Self { state_paths: IndexMap::new(), state_root })
    }

    /// Add a new state path to the offline query object.
    ///
    /// @param {string} commitment: The commitment corresponding to a record inpout
    /// @param {string} state_path: The state path corresponding to the commitment
    #[wasm_bindgen(js_name = "addStatePath")]
    pub fn add_state_path(&mut self, commitment: &str, state_path: &str) -> Result<(), String> {
        let commitment = Field::from_str(commitment).map_err(|e| e.to_string())?;
        let state_path = StatePath::from_str(state_path).map_err(|e| e.to_string())?;
        self.state_paths.insert(commitment, state_path);
        Ok(())
    }

    /// Get a json string representation of the offline query object
    #[wasm_bindgen(js_name = "toString")]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Create an offline query object from a json string representation
    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(s: &str) -> Result<OfflineQuery, String> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }
}

#[async_trait(?Send)]
impl QueryTrait<CurrentNetwork> for OfflineQuery {
    fn current_state_root(&self) -> anyhow::Result<<CurrentNetwork as Network>::StateRoot> {
        Ok(self.state_root)
    }

    async fn current_state_root_async(&self) -> anyhow::Result<<CurrentNetwork as Network>::StateRoot> {
        Ok(self.state_root)
    }

    fn get_state_path_for_commitment(
        &self,
        commitment: &Field<CurrentNetwork>,
    ) -> anyhow::Result<StatePath<CurrentNetwork>> {
        self.state_paths.get(commitment).cloned().ok_or(anyhow!("State path not found for commitment"))
    }

    async fn get_state_path_for_commitment_async(
        &self,
        commitment: &Field<CurrentNetwork>,
    ) -> anyhow::Result<StatePath<CurrentNetwork>> {
        self.state_paths.get(commitment).cloned().ok_or(anyhow!("State path not found for commitment"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::RecordPlaintext;

    use wasm_bindgen_test::*;

    const OFFLINE_QUERY: &str =
        r#"{"state_paths":{},"state_root":"sr1wjueje6hy86yw9j4lhl7jwvhjxwunw34paj4k3cn2wm5h5r2syfqd83yw4"}"#;
    const STATE_ROOT: &str = "sr1wjueje6hy86yw9j4lhl7jwvhjxwunw34paj4k3cn2wm5h5r2syfqd83yw4";

    #[wasm_bindgen_test]
    fn test_to_string_and_from_string() {
        let offline_query = OfflineQuery::new(STATE_ROOT).unwrap();
        assert_eq!(offline_query.to_string(), OFFLINE_QUERY);

        let offline_query_from_str = OfflineQuery::from_string(OFFLINE_QUERY).unwrap();
        assert_eq!(offline_query_from_str, offline_query);
    }
}
