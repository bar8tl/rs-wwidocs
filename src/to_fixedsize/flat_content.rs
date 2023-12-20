// flat_content.rs - Starts proper function to convert IDOC content from structured
// hierarchical format to fixed size format. Either from a set of files contained
// within a folder or from an specific single file (2021-07-01 bar8tl)
use crate::to_fixedsize::flat_content_inbatch::flat_content_inbatch;
use crate::to_fixedsize::flat_content_onefile::flat_content_onefile;
use crate::settings::set_pgm_settings::SettingsTp;
use rblib::files_infolder::FilelistTp;
use rusqlite::Connection;

pub fn flat_content(s: SettingsTp) {
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
    flat_content_onefile(&s.inptp, &s.inpdr, &s.outtp, &s.outdr, &s.idt, &cnn,
      &fle);
  } else {
    flat_content_inbatch(&s.inptp, &s.inpdr, &s.outtp, &s.outdr, &s.idt, &s.dbopt,
      &s.objtp, &s.pcddr, &s.wkflw, &s.ifilt);
  }
}
