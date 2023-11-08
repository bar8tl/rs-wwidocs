// build_edidc_line.rs - Build accumulatively the Control Record (EDIDC) output
// line (2021-07-01 bar8tl)
use crate::to_fixedsize::append_field_tocntrl::append_field_tocntrl;
use crate::to_fixedsize::symbols::{RCVPRN, RVCPRN, CREDAT, CRETIM};
use crate::to_fixedsize::types::ConvertTp;
use rusqlite::Connection;

pub fn build_edidc_line(cnn: &Connection, c: &mut ConvertTp,
  lctrl: &mut [char; 524], tokn: Vec<&str>) {
  let mut flkey = tokn[0].clone();
  if flkey == RVCPRN {
    flkey = RCVPRN;
  }
  let mut flval: String = Default::default();
  if tokn.len() == 3 {
    let flds: Vec<&str> = tokn[2].split(" :").collect();
    flval = flds[0].to_string();
  }
  if flkey == CREDAT {
    c.serie = flval.clone();
  }
  if flkey == CRETIM {
    c.serie = format!("{}{}", c.serie, flval);
  }
  if flval.len() > 0 {
    c.dirty = true;
    append_field_tocntrl(cnn, &c.idocx, &c.idocb, lctrl, flkey, flval);
  }
}
