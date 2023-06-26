//**********************************************************************************
// unpack.rs: Read SAP-Idoc content in standard flat TXT format and upload data    *
// into internal structures [20170524-BAR8TL]                                      *
//**********************************************************************************
mod control;
mod data;
mod outputs;

use crate::settings::SettingsTp;
use control::read_control;
use data::{DidocTp, read_data};
use outputs::write_outputs;
use rusqlite::Connection;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn conv_flat2json(s: SettingsTp) {
  let mut d = DidocTp { ..Default::default() };
  let cnn = Connection::open(&s.dbopt).expect("DB Error");
  d.outdr = s.outdr.clone();
  d.idocn = s.objnm.to_uppercase();
  for entry in fs::read_dir(&s.inpdr).unwrap() {
    let entry = entry.unwrap().path();
    if entry.is_dir() {
      continue;
    }
    let filid = Path::new(&entry).file_name().unwrap();
    let filnm = Path::new(&filid).file_stem().unwrap();
    let extsn = Path::new(&filid).extension().unwrap();
    d.flide = filid.to_str().unwrap().to_string();
    d.flnam = filnm.to_str().unwrap().to_string();
    d.flext = extsn.to_str().unwrap().to_string();
    let inppt = format!("{}{}", &s.inpdr, d.flide);
    let idocn = s.objnm.to_uppercase();
    if s.ifilt.len() == 0 || (s.ifilt.len() > 0 &&
       pass_filter(&s.ifilt, &d.flnam)) {
      proc_file(&cnn, &mut d, &inppt, &idocn);
    }
  }
}

// Process individual Input IDOC File
fn proc_file(cnn: &Connection, d: &mut DidocTp, inppt: &String, idocn: &String) {
  println!("proc_file {}", inppt);
  d.setno = Default::default(); // Initialize Instance of data sets in the file
  d.recnf = Default::default(); // Initialize Number of data records in the file
  let mut cnt: usize = 0;
  let ifile = File::open(inppt).unwrap();
  let rdr = BufReader::new(ifile);
  for wline in rdr.lines() {
    let wline = wline.unwrap();
    let iline = wline.trim();
    cnt += 1;
    if cnt == 1usize {
      if &iline[0..10] == "EDI_DC40" {
        read_control(cnn, d, iline, idocn, "CONTROL", false).expect("DB Error");
      } else {
        println!("IDOC File {} should start with Control Record", d.flide);
      }
    } else {
      read_data(cnn, d, iline, idocn, "DATA").expect("DB Error");
    }
  }
  if cnt == 0usize {
    println!("Input IDOC file %s is empty: {}", d.flide);
  }
  write_outputs(d);
}

// Indicates if a char string matches one pattern
fn pass_filter(ifilt: &String, filen: &str) -> bool {
  if filen == ifilt {
    return true;
  }
  true
}