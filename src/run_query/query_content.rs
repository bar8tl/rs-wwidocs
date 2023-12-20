// query_content.rs - Starts proper function to perform IDOC content inquiry from
// JSON format. Either from a set of files contained within a folder or from an
// specific single file (2021-07-01 bar8tl)
use crate::run_query::query_content_inbatch::query_content_inbatch;
use crate::run_query::query_content_onefile::query_content_onefile;
use crate::run_query::types::RquryTp;
use crate::settings::set_pgm_settings::SettingsTp;
use rblib::files_infolder::FilelistTp;
use serde_json::from_reader;
use std::fs::File;

pub fn query_content(s: SettingsTp) {
  if s.objnm.len() > 0 {
    let f = File::open(format!("{}{}", &s.refdr, &s.objnm)).expect(
      "Query JSON file not found.");
    let reqqy: RquryTp = from_reader(f).expect("JSON not well-formed");
    let atokn: Vec<&str> = s.objnm.rsplitn(2, ".").collect();
    let fle = FilelistTp {
      flpth: format!("{}{}", s.inpdr, s.objnm),
      fldir: s.inpdr.clone(),
      flide: s.objnm.clone(),
      flnam: atokn[1].to_string(),
      flext: atokn[0].to_string()
    };
    query_content_onefile(&s.inptp, &s.inpdr, &s.outtp, &s.outdr, &s.idt, &s.objtp,
      &s.refdr, &fle, &reqqy);
  } else {
    query_content_inbatch(&s.inptp, &s.inpdr, &s.outtp, &s.outdr, &s.idt, &s.objtp,
      &s.refdr, &s.pcddr, &s.wkflw, &s.ifilt);
  }
}
