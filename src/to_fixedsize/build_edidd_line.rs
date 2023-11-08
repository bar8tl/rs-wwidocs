// build_edidd_line.rs - Build accumulatively the Data Segment (EDIDD) output
// line (2021-07-01 bar8tl)
use crate::to_fixedsize::append_field_tosegmt::append_field_tosegmt;
use crate::to_fixedsize::types::ConvertTp;
use rusqlite::Connection;

pub fn build_edidd_line(cnn: &Connection, c: &mut ConvertTp,
  lsegm: &mut [char; 1063], tokn: Vec<&str>) {
  let flkey = tokn[0].clone();
  let mut flval = Default::default();
  if tokn.len() == 3 {
    let flds: Vec<&str> = tokn[2].split(" :").collect();
    flval = flds[0].to_string();
  }
  if flval.len() > 0 {
    c.dirty = true;
    let sgdsc = c.sgdsc.clone();
    append_field_tosegmt(cnn, &c.idocx, lsegm, sgdsc.as_str(), flkey, flval);
  }
}
