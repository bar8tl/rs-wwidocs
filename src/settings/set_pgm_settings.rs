// set_pgm_settings.rs - Program level settings definition (2021-07-01 bar8tl)
use crate::settings::read_config_file::{ConfigTp, read_config_file};
use rblib::read_cmdline_args::{ParamsTp, read_cmdline_args};
use chrono::Local;
use chrono::NaiveDateTime;
use serde_json::from_str;

const DEFAULTS: &str = include!("_defaults.json");

#[derive(Debug, Clone, Default)]
pub struct SettingsTp {
  pub prm  : ParamsTp,
  pub dfl  : ConfigTp,
  pub cfg  : ConfigTp,
  // program setttings
  pub dbonm: String,
  pub dbodr: String,
  pub inpdr: String,
  pub outdr: String,
  pub ifilt: String,
  pub ifnam: String,
  pub ofnam: String,
  // run settings
  pub objnm: String,
  pub pcddr: String,
  pub qrynm: String,
  pub qrydr: String,
  pub rcvpf: String,
  pub cntrl: String,
  pub clien: String,
  pub dbopt: String,
  pub found: i8,
  pub mitm : bool,
  pub sgrp : bool,
  pub ssgm : bool,
  pub dtsys: NaiveDateTime,
  pub dtcur: NaiveDateTime,
  pub dtnul: NaiveDateTime
}

pub fn set_pgm_settings(fname: &str) -> SettingsTp {
  let mut s = SettingsTp { ..Default::default() };
  s.prm = read_cmdline_args();
  s.dfl = from_str(DEFAULTS).unwrap();
  s.cfg = read_config_file(fname);
  let c = &s.cfg;
  s.dbonm = if c.progm.dbonm.len() > 0
    { c.progm.dbonm.clone() } else { s.dfl.progm.dbonm.to_string() };
  s.dbodr = if c.progm.dbodr.len() > 0
    { c.progm.dbodr.clone() } else { s.dfl.progm.dbodr.to_string() };
  s.inpdr = if c.progm.inpdr.len() > 0
    { c.progm.inpdr.clone() } else { s.dfl.progm.inpdr.to_string() };
  s.outdr = if c.progm.outdr.len() > 0
    { c.progm.outdr.clone() } else { s.dfl.progm.outdr.to_string() };
  s.ifilt = if c.progm.ifilt.len() > 0
    { c.progm.ifilt.clone() } else { s.dfl.progm.ifilt.to_string() };
  s.ifnam = if c.progm.ifnam.len() > 0
    { c.progm.ifnam.clone() } else { s.dfl.progm.ifnam.to_string() };
  s.ofnam = if c.progm.ofnam.len() > 0
    { c.progm.ofnam.clone() } else { s.dfl.progm.ofnam.to_string() };
  s.dbopt = format!("{}{}", s.dbodr, s.dbonm);
  s.dtsys = Local::now().naive_local();
  s.dtcur = Local::now().naive_local();
  s.dtnul = NaiveDateTime::MIN;
  return s;
}
