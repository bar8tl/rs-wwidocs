//**********************************************************************************
// config,rs: Reads config file and gets run parameters                            *
//**********************************************************************************
use serde::Deserialize;
use serde_json;
use std::env;
use std::fs::File;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct KonstTp {
  #[serde(default)]
  pub CNTRL: String,
  #[serde(default)]
  pub CLIEN: String
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProgmTp {
  #[serde(default)]
  pub dbonm: String,
  #[serde(default)]
  pub dbodr: String,
  #[serde(default)]
  pub inpdr: String,
  #[serde(default)]
  pub outdr: String,
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
  pub objnm: String,
  #[serde(default)]
  pub qrynm: String,
  #[serde(default)]
  pub dbonm: String,
  #[serde(default)]
  pub dbodr: String,
  #[serde(default)]
  pub inpdr: String,
  #[serde(default)]
  pub outdr: String,
  #[serde(default)]
  pub qrydr: String,
  #[serde(default)]
  pub ifilt: String,
  #[serde(default)]
  pub ifnam: String,
  #[serde(default)]
  pub ofnam: String,
  #[serde(default)]
  pub rcvpf: String
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CdbTp {
  #[serde(default)]
  pub id   : String,
  #[serde(default)]
  pub table: String,
  #[serde(default)]
  pub cr   : bool
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConfigTp {
  pub konst: KonstTp,
  pub progm: ProgmTp,
  pub run  : Vec<RunTp>,
  pub cdb  : Vec<CdbTp>
}

impl ConfigTp {
  pub fn new_config() -> ConfigTp {
    let cfg = ConfigTp{ ..Default::default() };
    cfg
  }

  pub fn get_config(&mut self, fname: &str) {
    let f = File::open(fname).unwrap();
    let cfg: ConfigTp = serde_json::from_reader(f)
      .expect("JSON not well-formed");
    self.konst = cfg.konst;
    self.progm = cfg.progm;
    self.run   = cfg.run;
    self.cdb   = cfg.cdb;
  }
}
