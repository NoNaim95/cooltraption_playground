use crate::asset_bundle::Asset;
use std::collections::BTreeMap;

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
