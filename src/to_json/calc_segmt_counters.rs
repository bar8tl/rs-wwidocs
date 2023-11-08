// calc_segmt_counters.rs - Process segment data (2021-07-01 bar8tl)
use crate::to_json::add_tostruct::add_tostruct;
use crate::to_json::symbols::{SAME, LOWER, UPPER};
use crate::to_json::types::{DidocTp, CountTp};
use rusqlite::Connection;

// proc_segmt.rs: Process Segment Data - Determines segment Qualifier and Instance
// Number
pub fn calc_segmt_counters(cnn: &Connection, d: &mut DidocTp, iline: &str,
  idocn: &String, strtp: &str, cdnam: &String, level: usize) {
  let mut instn: i32    = -1;
  let mut ident: String = String::new();
  if level == d.l as usize {
    instn = updt_counter(d, cdnam.to_string(), d.l as usize);
    ident = SAME.to_string();
  } else if level > d.l as usize || d.l < 0 {
    d.l = level as i32;
    d.count[d.l as usize].push(CountTp { segmn: cdnam.to_string(), instn: 1 });
    instn = rtrv_counter(d, cdnam.to_string(), d.l as usize);
    ident = LOWER.to_string();
  } else if level < d.l as usize {
    let goupl: usize = d.l as usize - level;
    for _ in 0..goupl {
      d.count[d.l as usize] = Default::default();
      d.l -= 1;
    }
    instn = updt_counter(d, cdnam.to_string(), d.l as usize);
    ident = UPPER.to_string();
  }
  add_tostruct(cnn, d, iline, idocn, ident, cdnam.to_string(), d.l, instn as usize);
}

// updt_counter.rs: Update counter of segment with equal segment ID in the current
// struct level
pub fn updt_counter(d: &mut DidocTp, segmn: String, l: usize) -> i32 {
  for j in 0..d.count[l].len() {
    if d.count[l][j].segmn == segmn {
      d.count[l][j].instn += 1;
      return d.count[l][j].instn as i32;
    }
  }
  d.count[l].push(CountTp{ segmn: segmn, instn: 1 });
  return 1;
}

// rtrv_counter.rs: Retrieve last counter of segment with equal segm ID in the
// current struct lvl
pub fn rtrv_counter(d: &mut DidocTp, segmn: String, l: usize) -> i32 {
  for j in 0..d.count[l].len() {
    if d.count[l][j].segmn == segmn {
      return d.count[l][j].instn as i32;
    }
  }
  return 0;
}
