// upload_definitn.rs - Read SAP IDoc parser file, and upload IDoc definition
// detail and structure into an internal database (2021-07-01 bar8tl)
use crate::idoc_definitn::prep_items_output::prep_items_output;
use crate::idoc_definitn::proc_items_master::proc_items_master;
use crate::idoc_definitn::proc_group_struct::proc_group_struct;
use crate::idoc_definitn::proc_segmt_struct::proc_segmt_struct;
use crate::idoc_definitn::scan_parserfile_line::scan_parserfile_line;
use crate::idoc_definitn::symbols::{EXTENSION, SEGMENTTYPE, LEVEL, LOOPMIN, LOOPMAX,
  QUALIFIED, STATUS, NAME, TEXT, TYPE, LENGTH, FIELD_POS,   CHARACTER_FIRST,
  CHARACTER_LAST, GRP, SGM};
use crate::idoc_definitn::types::{InpitmTp, InpgrpTp, InpsgmTp};
use rusqlite::Connection;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Main logic to process: 1) Master data for IDoc Items, 2) Structure for groups,
// and 3) Structure for segments
pub fn upld_definitn(dbopt: String, inpdr: String, objnm: String) {
  let cnn = Connection::open(&dbopt).expect("DB Open Error");
  let mut ii = InpitmTp { ..Default::default() };
  let mut ig = InpgrpTp { ..Default::default() };
  let mut is = InpsgmTp { ..Default::default() };
  init_items_master(&mut ii);
  init_group_struct(&mut ig);
  init_segmt_struct(&mut is);
  let ifile = File::open(format!("{}{}", inpdr, objnm)).unwrap();
  let rdr = BufReader::new(ifile);
  for wline in rdr.lines() {
    let wline = wline.unwrap();
    let line  = wline.trim();
    if line.len() > 0 {
      let sline = scan_parserfile_line(line);
      proc_items_master(      &sline, &mut ii);
      proc_group_struct(&cnn, &sline, &mut ig);
      proc_segmt_struct(&cnn, &sline, &mut is);
    }
  }
  prep_items_output(&cnn, &mut ii);
}

fn init_items_master(ii: &mut InpitmTp) {
  ii.icol = vec![EXTENSION];
  ii.gcol = vec![LEVEL, STATUS, LOOPMIN, LOOPMAX];
  ii.scol = vec![SEGMENTTYPE, QUALIFIED, LEVEL, STATUS, LOOPMIN, LOOPMAX];
  ii.fcol = vec![NAME, TEXT, TYPE, LENGTH, FIELD_POS, CHARACTER_FIRST,
    CHARACTER_LAST];
  ii.l = -1;
}

fn init_group_struct(ig: &mut InpgrpTp) {
  ig.strtp = GRP.to_uppercase();
  ig.l = -1;
}

fn init_segmt_struct(is: &mut InpsgmTp) {
  is.strtp = SGM.to_uppercase();
  is.l = -1;
}
