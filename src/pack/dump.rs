//**********************************************************************************
// dump.rs : Dump IDOC content to a JSON output (2017-05-24 bar8tl)
//**********************************************************************************
use crate::pack::{ConvertTp, SEGNUM, SEGNAM};
use rusqlite::Connection;
use std::fs::File;
use std::io::Write;

const DATA   : &str = "DATA";
const IDOCTYP: &str = "IDOCTYP";
const CIMTYP : &str = "CIMTYP";
const TABNAM : &str = "TABNAM";
const PSGNUM : &str = "PSGNUM";
const RCVPFC : &str = "RCVPFC";
const MANDT  : &str = "MANDT";
const DOCNUM : &str = "DOCNUM";
const SERIAL : &str = "SERIAL";
const HLEVEL : &str = "HLEVEL";

pub fn dump_controlline(cnn: &Connection, c: &mut ConvertTp,
  lctrl: &mut [char; 524], OF: &mut File) {
  if c.dirty {
    set_controlfield(cnn, c, lctrl, TABNAM, c.cntrl.clone());
    set_controlfield(cnn, c, lctrl, MANDT , c.clien.clone());
    set_controlfield(cnn, c, lctrl, DOCNUM, c.idocn.clone());
    set_controlfield(cnn, c, lctrl, RCVPFC, c.rcvpf.clone());
    set_controlfield(cnn, c, lctrl, SERIAL, c.serie.clone());
    let oline: String = lctrl.iter().collect();
    OF.write_all(format!("{}\r\n", oline).as_bytes()).expect("write failed");
    c.dirty = false;
  }
}

pub fn set_controlfield(cnn: &Connection, c: &mut ConvertTp,
  lctrl: &mut [char; 524], flkey: &str, mut flval: String) {
  let mut strps: usize = 0;
  cnn.query_row("SELECT strps FROM items WHERE idocn=?1 and rname=\"CONTROL\"
    and dname=?2;", [c.idocx.to_string(), flkey.to_string()], |row| {
      Ok(strps = row.get(0).unwrap()) })
    .expect("Error: Idoc type not found in definition DB");
  if flkey == IDOCTYP && flval == "14" {
    flval = c.idocb.to_string();
  }
  if flkey == CIMTYP  && flval == "14" {
    flval = c.idocx.to_string();
  }
  let mut k: usize = strps - 1;
  let temp: Vec<char> = flval.chars().collect();
  for i in 0..temp.len() {
    lctrl[k] = temp[i];
    k += 1;
  }
}

pub fn dump_segmentline(cnn: &Connection, c: &mut ConvertTp,
  lsegm: &mut [char; 1063], OF: &mut File) {
  if c.dirty {
    set_segmentfield(cnn, c, lsegm, DATA, SEGNAM, c.sgdsc.clone());
    set_segmentfield(cnn, c, lsegm, DATA, MANDT , c.clien.clone());
    set_segmentfield(cnn, c, lsegm, DATA, DOCNUM, c.idocn.clone());
    set_segmentfield(cnn, c, lsegm, DATA, SEGNUM, c.sgnbk.clone());
    set_segmentfield(cnn, c, lsegm, DATA, PSGNUM, c.sghnb.clone());
    set_segmentfield(cnn, c, lsegm, DATA, HLEVEL, c.sglvl.clone());
    let oline: String = lsegm.iter().collect();
    OF.write_all(format!("{}\r\n", oline).as_bytes()).expect("write failed");
    c.dirty = false;
  }
}

pub fn set_segmentfield(cnn: &Connection, c: &mut ConvertTp,
  lsegm: &mut [char; 1063], sgdsc: &str, flkey: &str, flval: String) {
  let mut strps: usize = 0;
  cnn.query_row("SELECT strps FROM items WHERE idocn=?1 and rname=?2 and dname=?3;",
    [c.idocx.clone(), sgdsc.to_string(), flkey.to_string()], |row| {
      Ok(strps = row.get(0).unwrap()) })
    .expect("Error: Segment type not found in definition DB");
  let mut k: usize = strps - 1;
  let temp: Vec<char> = flval.chars().collect();
  for i in 0..temp.len() {
    lsegm[k] = temp[i];
    k += 1;
  }
}
