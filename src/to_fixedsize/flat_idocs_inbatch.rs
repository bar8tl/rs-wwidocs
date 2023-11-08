// flat_idocs_inbatch.rs - Start batch process to convert IDOC files from structured
// hierarchical (parser file) format to fixed-size (flat) format (2021-07-01 bar8tl)
use crate::to_fixedsize::flat_idocdata::flat_idocdata;
use crate::to_fixedsize::symbols::{INP, OUT, OKAY};
use rblib::pass_filter::pass_filter;
use rblib::rename_file_wf::rename_file_wf;
use rblib::move_file_wf::move_file_wf;
use std::fs;
use std::path::Path;

pub fn flat_idocs_inbatch(dbopt: String, inpdr: String, outdr: String, 
  pcddr: String, ifilt: String, objnm: String, rcvpf: String, cntrl: String, 
  clien: String) {
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
    let objnm = objnm.to_uppercase();
    if ifilt.len() == 0 || (ifilt.len() > 0 && pass_filter(&ifilt, &flnam)) {
      let rtncd: String = flat_idocdata(&dbopt, &inpdr, &inppt, &outdr, &objnm,
        &rcvpf, &cntrl, &clien, &flnam, &flext);
      if rtncd == OKAY {
        move_file_wf(INP, &inpdr, &pcddr, &flnam, &flext);
        rename_file_wf(OUT, &outdr, &flnam, &flext);
      }
    }
  }
}
