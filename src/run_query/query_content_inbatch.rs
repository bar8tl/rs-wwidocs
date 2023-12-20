// query_content_inbatch.rs - Start batch process to perform queries into IDOC files
// stored in JSON format (2021-07-01 bar8tl)
use crate::run_query::query_content_onefile::query_content_onefile;
use crate::run_query::types::RquryTp;
use crate::settings::set_pgm_settings::IdoctpTp;
use crate::workflow::next_stage::next_stage;
use rblib::files_infolder::{FilelistTp, files_infolder};
use serde_json::from_reader;
use std::fs::File;
use std::collections::HashMap;

pub fn query_content_inbatch(inptp: &String, inpdr: &String, outtp: &String,
  outdr: &String, idt: &HashMap<String, IdoctpTp>, objtp: &String, refdr: &String,
  pcddr: &String,
  wkflw: &String, ifilt: &String) {
  let f = File::open(&objtp).expect("Query JSON file not found.");
  let reqqy: RquryTp = from_reader(f).expect("JSON not well-formed");
  let flist: Vec<FilelistTp> = files_infolder(inpdr, inptp, objtp);
  for fle in &flist {
    let rtncd = query_content_onefile(inptp, inpdr, outtp, outdr, idt, objtp, refdr,
      fle, &reqqy);
    if wkflw == "yes" {
      next_stage(&rtncd, inptp, inpdr, outtp, outdr, pcddr, fle, ifilt);
    }
  }
}
