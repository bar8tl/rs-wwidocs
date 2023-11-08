// prep_sectn_header.rs - Function to prepare measures to take for each data
// section. Each new section causes dumping data from previous one
// (2021-07-01 bar8tl)
use crate::to_fixedsize::symbols::{EDIDC, EDIDD, EDIDS};
use crate::to_fixedsize::types::ConvertTp;
use crate::to_fixedsize::write_cntrl_line::write_cntrl_line;
use crate::to_fixedsize::write_segmt_line::write_segmt_line;
use rusqlite::Connection;
use std::fs::File;

pub fn prep_sectn_header(cnn: &Connection, c: &mut ConvertTp,
   lctrl: &mut [char;  524], lsegm: &mut [char; 1063], lstat: &mut [char;  562],
   tokn: Vec<&str>, of: &mut File) {
  c.sectn = tokn[0].to_string();
  if c.sectn == EDIDC {
    *lctrl = [' '; 524];
  }
  if c.sectn == EDIDD {
    write_cntrl_line(cnn, c, lctrl, of);
  }
  if c.sectn == EDIDS {
    c.sgnbk = c.sgnum.clone();
    write_segmt_line(cnn, c, lsegm, of);
    *lstat = [' '; 562];
    if tokn.len() == 3 {
      c.secnb = tokn[2].to_string();
    }
  }
}
