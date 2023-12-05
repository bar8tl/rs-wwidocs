// read_config_file.rs - Upload external configuration json file with user defined
// settings (2021-07-01 bar8tl)
use serde::Deserialize;
use serde_json::from_reader;
use std::fs::File;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProgmTp {
  #[serde(default)]
  pub dbonm: String,
  #[serde(default)]
  pub dbodr: String,
  #[serde(default)]
  pub pcddr: String,
  #[serde(default)]
  pub qrytp: String,
  #[serde(default)]
  pub qrydr: String,
  #[serde(default)]
  pub deftp: String,
  #[serde(default)]
  pub defdr: String,
  #[serde(default)]
  pub strtp: String,
  #[serde(default)]
  pub strdr: String,
  #[serde(default)]
  pub fxstp: String,
  #[serde(default)]
  pub fxsdr: String,
  #[serde(default)]
  pub jsntp: String,
  #[serde(default)]
  pub jsndr: String,
  #[serde(default)]
  pub inqtp: String,
  #[serde(default)]
  pub inqdr: String,
  #[serde(default)]
  pub cntrl: String,
  #[serde(default)]
  pub clien: String,
  #[serde(default)]
  pub rcprf: String,
  #[serde(default)]
  pub ifilt: String,
  #[serde(default)]
  pub ifnam: String,
  #[serde(default)]
  pub ofnam: String
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RunTp {
  #[serde(default)]
  pub optn:  String,
  #[serde(default)]
  pub objnm: String
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct IdoctTp {
  #[serde(default)]
  pub itype: String,
  #[serde(default)]
  pub short: String,
  #[serde(default)]
  pub cntrl: String,
  #[serde(default)]
  pub clien: String,
  #[serde(default)]
  pub rcprf: String
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConfigTp {
  #[serde(default)]
  pub progm: ProgmTp,
  #[serde(default)]
  pub run  : Vec<RunTp>,
  #[serde(default)]
  pub idoct: Vec<IdoctTp>
}

pub fn read_config_file(fname: &str) -> ConfigTp {
  let mut cfg: ConfigTp = Default::default();
  match File::open(fname) {
    Ok(f)  => { cfg = from_reader(f).expect("JSON not well-formed"); },
    Err(_) => {},
  };
  return cfg;
}
