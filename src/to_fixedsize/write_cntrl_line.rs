// write_cntrl_line.rs - Complete output of control record line and address it to a
// flat fixed size text file (2021-07-01 bar8tl)
use crate::to_fixedsize::append_field_tocntrl::append_field_tocntrl;
use crate::to_fixedsize::symbols::{TABNAM, MANDT, DOCNUM, RCVPFC, SERIAL};
use crate::to_fixedsize::types::ConvertTp;
use rusqlite::Connection;
use std::io::Write;
use std::fs::File;

pub fn write_cntrl_line(cnn: &Connection, c: &mut ConvertTp,
  lctrl: &mut [char; 524], of: &mut File) {
  if c.dirty {
    append_field_tocntrl(cnn, &c.idocx, &c.idocb, lctrl, TABNAM, c.cntrl.clone());
    append_field_tocntrl(cnn, &c.idocx, &c.idocb, lctrl, MANDT , c.clien.clone());
    append_field_tocntrl(cnn, &c.idocx, &c.idocb, lctrl, DOCNUM, c.idocn.clone());
    append_field_tocntrl(cnn, &c.idocx, &c.idocb, lctrl, RCVPFC, c.rcvpf.clone());
    append_field_tocntrl(cnn, &c.idocx, &c.idocb, lctrl, SERIAL, c.serie.clone());
    let oline: String = lctrl.iter().collect();
    of.write_all(format!("{}\r\n", oline).as_bytes()).expect("write failed");
    c.dirty = false;
  }
}
