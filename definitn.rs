//**********************************************************************************
// definitn.rs: Read SAP IDoc parser file, and upload IDoc definition detail and   *
// structure into an internal database [20170524-BAR8TL]                           *
//**********************************************************************************
pub mod upldmitm;
mod ldtables;
mod rdparser;
mod upldsgrp;
mod upldssgm;

use crate::settings::{SettingsTp, GRP, SGM};
use rdparser::scan_inpline;
use upldmitm::{UpldmitmTp, init_upldmitm, get_mitmdata, isrt_mitmdata};
use upldsgrp::{UpldsgrpTp, init_upldsgrp, get_sgrpdata};
use upldssgm::{UpldssgmTp, init_upldssgm, get_ssgmdata};
use rusqlite::Connection;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Main logic to process: 1) Master data for IDoc Items, 2) Structure for groups,
// and 3) Structure for segments
pub fn upld_idocdefn(stg: SettingsTp) {
  let s = stg.clone();
  let cnn = Connection::open(&s.dbopt).expect("DB Error");
  let mut ui = UpldmitmTp { ..Default::default() };
  let mut ug = UpldsgrpTp { ..Default::default() };
  let mut us = UpldssgmTp { ..Default::default() };
  if s.mitm {
    init_upldmitm(&mut ui);
  }
  if s.sgrp {
    init_upldsgrp(&mut ug, GRP);
  }
  if s.ssgm {
    init_upldssgm(&mut us, SGM);
  }
  let ifile = File::open(format!("{}{}", s.inpdr, s.objnm)).unwrap();
  let rdr = BufReader::new(ifile);
  for wline in rdr.lines() {
    let wline = wline.unwrap();
    let line  = wline.trim();
    if line.len() > 0 {
      let sline = scan_inpline(line);
      if s.mitm {
        get_mitmdata(&sline, &mut ui);
      }
      if s.sgrp {
        get_sgrpdata(&cnn, &sline, &mut ug);
      }
      if s.ssgm {
        get_ssgmdata(&cnn, &sline, &mut us);
      }
    }
  }
  isrt_mitmdata(&cnn, &mut ui);
}
