// scan_parserfile_line.rs - Identify individual tokens in SAP IDOC data in parser
// file format (2021-07-01 bar8tl)
use crate::idoc_definitn::symbols::{BEGIN_, END_};
use crate::idoc_definitn::types::ParslTp;

pub fn scan_parserfile_line(s: &str) -> ParslTp {
  let key: String;
  let mut val: String;
  let mut p = ParslTp { ..Default::default() };
  let flds: Vec<&str> = s.split_whitespace().collect();
  if flds.len() > 0 {
    key = flds[0].to_string();
    if (key.len() >= 6 && &key[0..6] == BEGIN_) ||
       (key.len() >= 4 && &key[0..4] == END_  ) {
      let tokn: Vec<&str> = key.split('_').collect();
      if tokn.len() == 2 {
        p.label.ident = tokn[0].to_string();
        p.label.recnm = tokn[1].to_string();
        p.label.rectp = Default::default();
      } else if tokn.len() == 3 {
        p.label.ident = tokn[0].to_string();
        p.label.recnm = tokn[1].to_string();
        p.label.rectp = tokn[2].to_string();
      }
    } else {
      p.label.ident = key;
      p.label.recnm = String::new();
      p.label.rectp = String::new();
    }
  }
  if flds.len() > 1 {
    val = flds[1].to_string();
    for i in 2..flds.len() {
      val = format!("{} {}", val, flds[i]);
    }
    p.value = val;
  }
  return p;
}
