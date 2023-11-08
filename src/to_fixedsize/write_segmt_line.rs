// write_segmt_line.rs - Complete output of data segment lines and address it to a
// flat fixed size text file (2021-07-01 bar8tl)
use crate::to_fixedsize::append_field_tosegmt::append_field_tosegmt;
use crate::to_fixedsize::symbols::{SEGNUM, SEGNAM, DATA, MANDT, DOCNUM, PSGNUM,
  HLEVEL};
use crate::to_fixedsize::types::ConvertTp;
use rusqlite::Connection;
use std::io::Write;
use std::fs::File;

pub fn write_segmt_line(cnn: &Connection, c: &mut ConvertTp,
  lsegm: &mut [char; 1063], of: &mut File) {
  if c.dirty {
    append_field_tosegmt(cnn, &c.idocx, lsegm, DATA, SEGNAM, c.sgdsc.clone());
    append_field_tosegmt(cnn, &c.idocx, lsegm, DATA, MANDT , c.clien.clone());
    append_field_tosegmt(cnn, &c.idocx, lsegm, DATA, DOCNUM, c.idocn.clone());
    append_field_tosegmt(cnn, &c.idocx, lsegm, DATA, SEGNUM, c.sgnbk.clone());
    append_field_tosegmt(cnn, &c.idocx, lsegm, DATA, PSGNUM, c.sghnb.clone());
    append_field_tosegmt(cnn, &c.idocx, lsegm, DATA, HLEVEL, c.sglvl.clone());
    let oline: String = lsegm.iter().collect();
    of.write_all(format!("{}\r\n", oline).as_bytes()).expect("write failed");
    c.dirty = false;
  }
}
