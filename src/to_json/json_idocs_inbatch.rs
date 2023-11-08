// json_idocs_inbatch.rs - Start batch process to convert IDOC files from fixed size
// (flat file) format to JSON hierarchical format (2021-07-01 bar8tl)
use crate::to_json::json_idocdata::json_idocdata;
use crate::to_json::symbols::{OKAY, INP, OUT};
use rblib::pass_filter::pass_filter;
use rblib::rename_file_wf::rename_file_wf;
use rblib::move_file_wf::move_file_wf;
use std::fs;
use std::path::Path;

pub fn json_idocs_inbatch(dbopt: String, inpdr: String, outdr: String,
  pcddr: String, ifilt: String, objnm: String) {
  for entry in fs::read_dir(&inpdr).unwrap() {
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
      let rtncd: String = json_idocdata(&dbopt, &inpdr, &inppt, &outdr, &idocn,
        &flide, &flnam, &flext);
      if rtncd == OKAY {
        move_file_wf(INP, &inpdr, &pcddr, &flnam, &flext);
        rename_file_wf(OUT, &outdr, &flnam, "json");
      }
    }
  }
}
