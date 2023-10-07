//**********************************************************************************
// settings.rs : Define pgm-level & run-level settings (2017-05-24 bar8tl)
//**********************************************************************************
#![allow(non_snake_case)]

use crate::settings::config::ConfigTp;
use chrono::Local;
use chrono::NaiveDateTime;
use rblib::params::{ParamsTp, ParameTp};

const CNTRL: &str = "EDI_DC40";           // Default values for: CONTROL_CODE,
const CLIEN: &str = "011";                // CLIENT_CODE,
const DBONM: &str = "idoctp.db";          // DB_NAME,
const DBODR: &str = ".\\";                // DB_DIR,
const INPDR: &str = ".\\files\\input\\";  // INPUTS_DIR,
const OUTDR: &str = ".\\files\\output\\"; // OUTPUTS_DIR,
const IFILT: &str = "!(*processed*)";     // INPUTS_FILTER,
const INPNM: &str = "dtsys+'_'+idocn+docno+docdt+'_inp_processed'"; // INPUTS_NAMING,
const OUTNM: &str = "dtsys+'_'+idocn+docno+docdt+'_out'";           // OUTPUTS_NAMNG

pub const ITM: &str = "itm"; // Public constants for sap2flat params
pub const GRP: &str = "grp";
pub const SGM: &str = "sgm";

#[derive(Debug, Clone, Default)]
pub struct SettingsTp {
  pub prm  : ParamsTp,
  pub cfd  : ConfigTp,
  pub cntrl: String,
  pub clien: String,
  pub dbonm: String,
  pub dbodr: String,
  pub dbopt: String,
  pub inpdr: String,
  pub outdr: String,
  pub qrydr: String,
  pub ifilt: String,
  pub ifnam: String,
  pub ofnam: String,
  pub objnm: String,
  pub qrynm: String,
  pub rcvpf: String,
  pub found: bool,
  pub mitm : bool,
  pub sgrp : bool,
  pub ssgm : bool,
  pub dtsys: NaiveDateTime,
  pub dtcur: NaiveDateTime,
  pub dtnul: NaiveDateTime
}

impl SettingsTp {
  pub fn new_settings() -> SettingsTp {
    let mut stg = SettingsTp { ..Default::default() };
    stg.prm = ParamsTp::new_params();
    stg.cfd = ConfigTp::new_config();
    stg.set_settings("_config.json");
    stg
  }

  pub fn set_settings(&mut self, cfnam: &str) {
    self.prm.scan_params();
    self.cfd.get_config(cfnam);
    let c = &self.cfd;
    self.cntrl = if c.konst.CNTRL.len() > 0
      { c.konst.CNTRL.clone() } else { CNTRL.to_string() };
    self.clien = if c.konst.CLIEN.len() > 0
      { c.konst.CLIEN.clone() } else { CLIEN.to_string() };
    self.dbonm = if c.progm.dbonm.len() > 0
      { c.progm.dbonm.clone() } else { DBONM.to_string() };
    self.dbodr = if c.progm.dbodr.len() > 0
      { c.progm.dbodr.clone() } else { DBODR.to_string() };
    self.inpdr = if c.progm.inpdr.len() > 0
      { c.progm.inpdr.clone() } else { INPDR.to_string() };
    self.outdr = if c.progm.outdr.len() > 0
      { c.progm.outdr.clone() } else { OUTDR.to_string() };
    self.ifilt = if c.progm.ifilt.len() > 0
      { c.progm.ifilt.clone() } else { IFILT.to_string() };
    self.ifnam = if c.progm.ifnam.len() > 0
      { c.progm.ifnam.clone() } else { INPNM.to_string() };
    self.ofnam = if c.progm.ofnam.len() > 0
      { c.progm.ofnam.clone() } else { OUTNM.to_string() };
    self.dbopt = format!("{}{}", self.dbodr, self.dbonm);
    self.dtsys = Local::now().naive_local();
    self.dtcur = Local::now().naive_local();
    self.dtnul = NaiveDateTime::MIN;
  }

  pub fn set_runvars(&mut self, p: &ParameTp) {
    if p.prm1.len() > 0 {
      self.objnm = p.prm1.clone();
    } else {
      panic!("Error: Not possible to determine Object name");
    }
    self.found = false;
    for run in &self.cfd.run {
      if p.optn == run.optn && p.prm1 == run.objnm {
        self.found = true;
        if p.optn == "cdb" || p.optn == "def" || p.optn == "pck" ||
           p.optn == "upk" || p.optn == "ali" || p.optn == "qry" {
          if run.objnm.len() > 0 { self.objnm = run.objnm.clone(); }
          if run.dbonm.len() > 0 { self.dbonm = run.dbonm.clone(); }
          if run.dbodr.len() > 0 { self.dbodr = run.dbodr.clone(); }
          self.dbopt = format!("{}{}", self.dbodr, self.dbonm);
        }
        if p.optn == "def" || p.optn == "pck" || p.optn == "upk" ||
           p.optn == "ali" || p.optn == "qry" {
          if run.inpdr.len() > 0 { self.inpdr = run.inpdr.clone(); }
          if run.outdr.len() > 0 { self.outdr = run.outdr.clone(); }
        }
        if p.optn == "pck" || p.optn == "upk" || p.optn == "ali" {
          if run.ifilt.len() > 0 { self.ifilt = run.ifilt.clone(); }
          if run.ifnam.len() > 0 { self.ifnam = run.ifnam.clone(); }
          if run.ofnam.len() > 0 { self.ofnam = run.ofnam.clone(); }
          if run.rcvpf.len() > 0 { self.rcvpf = run.rcvpf.clone(); }
        }
        if p.optn == "qry" {
          if run.qrydr.len() > 0 { self.qrydr = run.qrydr.clone(); }
          if run.qrynm.len() > 0 { self.qrynm = run.qrynm.clone(); }
        }
        break;
      }
    }
    if p.optn == "def" {
      (self.mitm, self.sgrp, self.ssgm) = (true, false, false);
      if p.prm2.len() > 0 {
        let mflds: Vec<&str> = p.prm2.split('.').collect();
        for mfld in &mflds {
          match mfld.to_lowercase().as_str() {
            ITM => self.mitm = true,
            GRP => self.sgrp = true,
            SGM => self.ssgm = true,
            _   => { (self.mitm, self.sgrp, self.ssgm) = (true, false, false) }
          }
        }
      }
    }
  }
}

//**********************************************************************************
// config.rs : Reads config file and gets run parameters (2017-05-24 bar8tl)
//**********************************************************************************
mod config {
  use serde::Deserialize;
  use serde_json::from_reader;
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
      let cfg: ConfigTp = from_reader(f).expect("JSON not well-formed");
      self.konst = cfg.konst;
      self.progm = cfg.progm;
      self.run   = cfg.run;
      self.cdb   = cfg.cdb;
    }
  }
}
