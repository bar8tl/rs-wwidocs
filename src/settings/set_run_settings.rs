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
              _ => { (s.mitm, s.sgrp, s.ssgm) = (true, false, false) }
        }
      }
    }
  }
}

fn set_optn_settings(s: &mut SettingsTp, p: &ParameTp, run: &RunTp) -> bool {
  let mut found = false;
  if p.optn == run.optn && p.prm1 == run.objnm {
    if p.optn == CDB || p.optn == DEF || p.optn == FXS ||
       p.optn == JSN || p.optn == QRY {
      if run.objnm.len() > 0 { s.objnm = run.objnm.clone(); }
    }
    s.found += 1;
    found = true;
  }
  return found;
}
