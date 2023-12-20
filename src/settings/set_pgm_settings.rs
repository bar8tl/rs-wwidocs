// set_pgm_settings.rs - Program level settings definition (2021-07-01 bar8tl)
use crate::settings::read_config_file::{ConfigTp, read_config_file};
use chrono::Local;
use chrono::NaiveDateTime;
use rblib::read_cmdline_args::{ParamsTp, read_cmdline_args};
use serde_json::from_str;
use std::collections::HashMap;

const DEFAULTS: &str = include!("_defaults.json");

#[derive(Debug, Clone, Default)]
pub struct IdoctpTp {
  pub itype: String,
  pub idocf: String,
  pub short: String,
  pub cntrl: String,
  pub clien: String,
  pub rcvpf: String
}

#[derive(Debug, Clone, Default)]
pub struct SettingsTp {
  pub prm  : ParamsTp,
  pub dfl  : ConfigTp,
  pub cfg  : ConfigTp,
  // program setttings
  pub dbonm: String,
  pub dbodr: String,
  pub inptp: String,
  pub inpdr: String,
  pub outtp: String,
  pub outdr: String,
  pub reftp: String,
  pub refdr: String,
  pub wkflw: String,
  pub pcddr: String,
  pub cntrl: String,
  pub clien: String,
  pub rcvpf: String,
  pub ifilt: String,
  pub ifnam: String,
  pub ofnam: String,
  pub idt  : HashMap<String, IdoctpTp>,
  // run settings
  pub optn : String,
  pub objtp: String,
  pub objnm: String,
  pub short: String,
  // other settings
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
  let dfl = s.dfl.clone();
  let cfg = s.cfg.clone();
  set_progmstgs(&mut s, dfl);
  set_progmstgs(&mut s, cfg);
  s.dbopt = format!("{}{}", s.dbodr, s.dbonm);
  s.dtsys = Local::now().naive_local();
  s.dtcur = Local::now().naive_local();
  s.dtnul = NaiveDateTime::MIN;
  return s;
}

fn set_progmstgs(s: &mut SettingsTp, c: ConfigTp) {
  if c.progm.dbonm.len() > 0 { s.dbonm = c.progm.dbonm.clone(); }
  if c.progm.dbodr.len() > 0 { s.dbodr = c.progm.dbodr.clone(); }
  if c.progm.inptp.len() > 0 { s.inptp = c.progm.inptp.clone(); }
  if c.progm.inpdr.len() > 0 { s.inpdr = c.progm.inpdr.clone(); }
  if c.progm.outtp.len() > 0 { s.outtp = c.progm.outtp.clone(); }
  if c.progm.outdr.len() > 0 { s.outdr = c.progm.outdr.clone(); }
  if c.progm.reftp.len() > 0 { s.reftp = c.progm.reftp.clone(); }
  if c.progm.refdr.len() > 0 { s.refdr = c.progm.refdr.clone(); }
  if c.progm.wkflw.len() > 0 { s.wkflw = c.progm.wkflw.clone(); }
  if c.progm.pcddr.len() > 0 { s.pcddr = c.progm.pcddr.clone(); }
  if c.progm.cntrl.len() > 0 { s.cntrl = c.progm.cntrl.clone(); }
  if c.progm.clien.len() > 0 { s.clien = c.progm.clien.clone(); }
  if c.progm.rcvpf.len() > 0 { s.rcvpf = c.progm.rcvpf.clone(); }
  if c.progm.ifilt.len() > 0 { s.ifilt = c.progm.ifilt.clone(); }
  if c.progm.ifnam.len() > 0 { s.ifnam = c.progm.ifnam.clone(); }
  if c.progm.ofnam.len() > 0 { s.ofnam = c.progm.ofnam.clone(); }
  let mut idte: IdoctpTp = Default::default();
  for itab in c.idoct.clone() {
    idte.itype = itab.itype.to_uppercase();
    idte.idocf = itab.itype.to_uppercase().replace("/", "_-");
    idte.short = itab.short.clone();
    idte.cntrl = if itab.cntrl.len() > 0 { itab.cntrl.clone() } else {
      c.progm.cntrl.clone() };
    idte.clien = if itab.clien.len() > 0 { itab.clien.clone() } else {
      c.progm.clien.clone() };
    idte.rcvpf = if itab.rcvpf.len() > 0 { itab.rcvpf.clone() } else {
      c.progm.rcvpf.clone() };
    s.idt.insert(idte.short.clone(), idte.clone());
  }
}
