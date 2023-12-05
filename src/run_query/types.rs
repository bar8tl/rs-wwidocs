// types.rs - Data structures used in IDOC query from files stored in JSON format
// (2021-07-01 bar8tl)
use crate::to_json::types::FieldTp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RquryTp {
  pub fields: Vec<String>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SquryTp {
  pub fields: Vec<FieldTp>
}

#[derive(Debug, Clone, Default)]
pub struct QtoknTp {
  pub segmn: String,
  pub instn: usize,
  pub qlkey: String,
  pub qlval: String
}
