// flat_content_inbatch.rs - Start batch process to convert IDOC files from
// structured hierarchical (parser file) format to fixed-size (flat) format
// (2021-07-01 bar8tl)
use crate::to_fixedsize::flat_content_onefile::flat_content_onefile;
use crate::settings::set_pgm_settings::IdoctpTp;
use crate::workflow::next_stage::next_stage;
use rblib::files_infolder::{FilelistTp, files_infolder};
use rusqlite::Connection;
use std::collections::HashMap;

pub fn flat_content_inbatch(inptp: &String, inpdr: &String, outtp: &String,
  outdr: &String, idt: &HashMap<String, IdoctpTp>, dbopt: &String, objtp: &String,
  pcddr: &String, wkflw: &String, ifilt: &String) {
  let cnn = Connection::open(dbopt).expect("DB Error");
  let flist: Vec<FilelistTp> = files_infolder(inpdr, inptp, objtp);
  for fle in &flist {
    let rtncd = flat_content_onefile(inptp, inpdr, outtp, outdr, idt, &cnn, fle);
    if wkflw == "yes" {
      next_stage(&rtncd, inptp, inpdr, outtp, outdr, pcddr, fle, ifilt);
    }
  }
}
