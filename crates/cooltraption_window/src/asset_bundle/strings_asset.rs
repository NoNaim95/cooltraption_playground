use std::collections::BTreeMap;

use crate::asset_bundle::Asset;

#[derive(Debug)]
pub struct StringsAsset {
    pub map: BTreeMap<String, String>,
}

impl Asset for StringsAsset {}

impl From<BTreeMap<String, String>> for StringsAsset {
    fn from(map: BTreeMap<String, String>) -> Self {
        Self { map }
    }
}
