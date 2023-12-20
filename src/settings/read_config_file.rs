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
  pub inptp: String,
  #[serde(default)]
  pub inpdr: String,
  #[serde(default)]
  pub outtp: String,
  #[serde(default)]
  pub outdr: String,
  #[serde(default)]
  pub reftp: String,
  #[serde(default)]
  pub refdr: String,
  #[serde(default)]
  pub wkflw: String,
  #[serde(default)]
  pub pcddr: String,
  #[serde(default)]
  pub cntrl: String,
  #[serde(default)]
  pub clien: String,
  #[serde(default)]
  pub rcvpf: String,
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
  pub optn : String,
  #[serde(default)]
  pub objnm: String,
  #[serde(default)]
  pub inptp: String,
  #[serde(default)]
  pub inpdr: String,
  #[serde(default)]
  pub outtp: String,
  #[serde(default)]
  pub outdr: String,
  #[serde(default)]
  pub reftp: String,
  #[serde(default)]
  pub refdr: String,
  #[serde(default)]
  pub pcddr: String
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
  pub rcvpf: String
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
