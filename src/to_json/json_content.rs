// json_content.rs - Starts proper function to convert IDOC content from fixed size
// format to json format. Either from a set of files contained within a folder or
// from an specific single file (2021-07-01 bar8tl)
use crate::to_json::json_content_inbatch::json_content_inbatch;
use crate::to_json::json_content_onefile::json_content_onefile;
use crate::settings::set_pgm_settings::SettingsTp;
use rblib::files_infolder::FilelistTp;
use rusqlite::Connection;

pub fn json_content(s: SettingsTp) {
  if s.objnm.len() > 0 {
    let cnn = Connection::open(&s.dbopt).expect("DB Error");
    let atokn: Vec<&str> = s.objnm.rsplitn(2, ".").collect();
    let fle = FilelistTp {
      flpth: format!("{}{}", s.inpdr, s.objnm),
      fldir: s.inpdr.clone(),
      flide: s.objnm.clone(),
      flnam: atokn[1].to_string(),
      flext: atokn[0].to_string()
    };
    json_content_onefile(&s.inptp, &s.inpdr, &s.outtp, &s.outdr, &s.idt, &cnn,
      &fle);
  } else {
    json_content_inbatch(&s.inptp, &s.inpdr, &s.outtp, &s.outdr, &s.idt, &s.dbopt,
      &s.objtp, &s.pcddr, &s.wkflw, &s.ifilt);
  }
}
