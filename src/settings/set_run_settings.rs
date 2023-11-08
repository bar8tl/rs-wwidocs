// set_run_settings.rs - Option Run/Execution level setting definition
// (2021-07-01 bar8tl)
use crate::settings::read_config_file::RunTp;
use crate::settings::set_pgm_settings::SettingsTp;
use rblib::read_cmdline_args::ParameTp;

// Run options
pub const CDB: &str = "cdb";
pub const DEF: &str = "def";
pub const FXS: &str = "fxs";
pub const JSN: &str = "jsn";
pub const ALS: &str = "als";
pub const QRY: &str = "qry";

pub fn set_run_settings(s: &mut SettingsTp, p: &ParameTp) {
  if p.prm1.len() > 0 {
    s.objnm = p.prm1.clone();
  } else {
    panic!("Error: Not possible to determine Object name");
  }
  s.found = 0;
  for run in s.dfl.run.clone() {
    if set_optn_settings(s, p, &run) {
      break;
    }
  }
  for run in s.cfg.run.clone() {
    if set_optn_settings(s, p, &run) {
      break;
    }
  }
  if s.found > 0 && p.optn == DEF {
    (s.mitm, s.sgrp, s.ssgm) = (true, false, false);
    if p.prm2.len() > 0 {
      let mflds: Vec<&str> = p.prm2.split('.').collect();
      for mfld in &mflds {
        match mfld.to_lowercase().as_str() {
          "itm" => s.mitm = true,
          "grp" => s.sgrp = true,
          "sgm" => s.ssgm = true,
          _     => { (s.mitm, s.sgrp, s.ssgm) = (true, false, false) }
        }
      }
    }
  }
}

fn set_optn_settings(s: &mut SettingsTp, p: &ParameTp, run: &RunTp) -> bool {
  let mut found = false;
  if p.optn == run.optn && p.prm1 == run.objnm {
    if p.optn == CDB || p.optn == DEF || p.optn == FXS ||
       p.optn == JSN || p.optn == ALS || p.optn == QRY {
      if run.objnm.len() > 0 { s.objnm = run.objnm.clone(); }
      if run.dbonm.len() > 0 { s.dbonm = run.dbonm.clone(); }
      if run.dbodr.len() > 0 { s.dbodr = run.dbodr.clone(); }
      s.dbopt = format!("{}{}", s.dbodr, s.dbonm);
    }
    if p.optn == DEF || p.optn == FXS || p.optn == JSN ||
       p.optn == ALS || p.optn == QRY {
      if run.inpdr.len() > 0 { s.inpdr = run.inpdr.clone(); }
      if run.outdr.len() > 0 { s.outdr = run.outdr.clone(); }
    }
    if p.optn == FXS || p.optn == JSN || p.optn == ALS {
      if run.pcddr.len() > 0 { s.pcddr = run.pcddr.clone(); }
      if run.ifilt.len() > 0 { s.ifilt = run.ifilt.clone(); }
      if run.ifnam.len() > 0 { s.ifnam = run.ifnam.clone(); }
      if run.ofnam.len() > 0 { s.ofnam = run.ofnam.clone(); }
      if run.cntrl.len() > 0 { s.cntrl = run.cntrl.clone(); }
      if run.clien.len() > 0 { s.clien = run.clien.clone(); }
      if run.rcvpf.len() > 0 { s.rcvpf = run.rcvpf.clone(); }
    }
    if p.optn == QRY {
      if run.qrydr.len() > 0 { s.qrydr = run.qrydr.clone(); }
      if run.qrynm.len() > 0 { s.qrynm = run.qrynm.clone(); }
    }
    s.found += 1;
    found = true;
  }
  return found;
}
