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
  pub qrytp: String,
  pub qrydr: String,
  pub pcddr: String,
  pub deftp: String,
  pub defdr: String,
  pub strtp: String,
  pub strdr: String,
  pub fxstp: String,
  pub fxsdr: String,
  pub jsntp: String,
  pub jsndr: String,
  pub inqtp: String,
  pub inqdr: String,
  pub cntrl: String,
  pub clien: String,
  pub rcprf: String,
  pub ifilt: String,
  pub ifnam: String,
  pub ofnam: String,
  // run settings
  pub objnm: String,
  // internal table entry settings
  pub itype: String,
  pub short: String,
  pub idocf: String,
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
  if c.progm.pcddr.len() > 0 { s.pcddr = c.progm.pcddr.clone(); }
  if c.progm.qrytp.len() > 0 { s.qrytp = c.progm.qrytp.clone(); }
  if c.progm.qrydr.len() > 0 { s.qrydr = c.progm.qrydr.clone(); }
  if c.progm.deftp.len() > 0 { s.deftp = c.progm.deftp.clone(); }
  if c.progm.defdr.len() > 0 { s.defdr = c.progm.defdr.clone(); }
  if c.progm.strtp.len() > 0 { s.strtp = c.progm.strtp.clone(); }
  if c.progm.strdr.len() > 0 { s.strdr = c.progm.strdr.clone(); }
  if c.progm.fxstp.len() > 0 { s.fxstp = c.progm.fxstp.clone(); }
  if c.progm.fxsdr.len() > 0 { s.fxsdr = c.progm.fxsdr.clone(); }
  if c.progm.jsntp.len() > 0 { s.jsntp = c.progm.jsntp.clone(); }
  if c.progm.jsndr.len() > 0 { s.jsndr = c.progm.jsndr.clone(); }
  if c.progm.inqtp.len() > 0 { s.inqtp = c.progm.inqtp.clone(); }
  if c.progm.inqdr.len() > 0 { s.inqdr = c.progm.inqdr.clone(); }
  if c.progm.cntrl.len() > 0 { s.cntrl = c.progm.cntrl.clone(); }
  if c.progm.clien.len() > 0 { s.clien = c.progm.clien.clone(); }
  if c.progm.rcprf.len() > 0 { s.rcprf = c.progm.rcprf.clone(); }
  if c.progm.ifilt.len() > 0 { s.ifilt = c.progm.ifilt.clone(); }
  if c.progm.ifnam.len() > 0 { s.ifnam = c.progm.ifnam.clone(); }
  if c.progm.ofnam.len() > 0 { s.ofnam = c.progm.ofnam.clone(); }
  for itab in c.idoct.clone() {
    if s.objnm == itab.itype {
      if itab.itype.len() > 0 { s.itype = itab.itype.clone(); }
      if itab.short.len() > 0 { s.short = itab.short.clone(); }
      if itab.cntrl.len() > 0 { s.cntrl = itab.cntrl.clone(); }
      if itab.clien.len() > 0 { s.clien = itab.clien.clone(); }
      if itab.rcprf.len() > 0 { s.rcprf = itab.rcprf.clone(); }
      s.idocf = s.itype.replace("/", "_-");
      break;
    }
  }
}
