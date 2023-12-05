// query_idocs_inbatch.rs - Start batch process to perform queries into IDOC files
// stored in JSON format (2021-07-01 bar8tl)
use crate::run_query::query_idocdata::query_idocdata;
use crate::run_query::types::RquryTp;
use crate::to_json::symbols::{OKAY, INP, OUT};
use rblib::pass_filter::pass_filter;
use rblib::rename_file_wf::rename_file_wf;
use rblib::move_file_wf::move_file_wf;
use serde_json::from_reader;
use std::fs::{File, read_dir};
use std::path::Path;

pub fn query_idocs_inbatch(inpdr: String, outdr: String, pcddr: String,
  ifilt: String, objnm: String) {
  let f = File::open(&objnm).expect("Query JSON file not found.");
  let reqqy: RquryTp = from_reader(f).expect("JSON not well-formed");
  for entry in read_dir(&inpdr).unwrap() {
    let entry = entry.unwrap().path();
    if entry.is_dir() {
      continue;
    }
    let filid = Path::new(&entry).file_name().unwrap();
    let filnm = Path::new(&filid).file_stem().unwrap();
    let extsn = Path::new(&filid).extension().unwrap();
    let flide = filid.to_str().unwrap().to_string();
    let flnam = filnm.to_str().unwrap().to_string();
    let flext = extsn.to_str().unwrap().to_string();
    let inppt = format!("{}{}", &inpdr, flide);
    let idocn = objnm.to_uppercase();
    if ifilt.len() == 0 || (ifilt.len() > 0 && pass_filter(&ifilt, &flnam)) {
      let rtncd: String = query_idocdata(&reqqy, &inpdr, &inppt, &outdr, &idocn,
        &flide, &flnam, &flext);
      if rtncd == OKAY {
        move_file_wf(INP, &inpdr, &pcddr, &flnam, &flext);
        rename_file_wf(OUT, &outdr, &flnam, &flext);
      }
    }
  }
}
