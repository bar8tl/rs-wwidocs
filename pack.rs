//**********************************************************************************
// pack.rs : Convert IDoc classic hierarchical format to flat text file format
// (2017-05-24 bar8tl)
//**********************************************************************************
#![allow(non_snake_case)]

mod build;
mod dump;
mod setup;

use crate::settings::SettingsTp;
use setup::{setup_section, setup_segment};
use build::{proc_edidc, proc_edidd, proc_edids};
use rusqlite::Connection;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const EDIDC : &str = "EDIDC";
const EDIDD : &str = "EDIDD";
const EDIDS : &str = "EDIDS";
const SEGNUM: &str = "SEGNUM";
const SEGNAM: &str = "SEGNAM";

#[derive(Debug, Clone, Default)]
pub struct HstrucTp {
  pub sgnum: String,
  pub sgnam: String,
  pub sglvl: String
}

#[derive(Debug, Clone, Default)]
pub struct ConvertTp {
  pub cntrl: String,
  pub clien: String,
  pub inpdr: String,
  pub outdr: String,
  pub rcvpf: String,
  pub flide: String,
  pub flnam: String,
  pub flext: String,
  pub idocx: String,
  pub idocn: String,
  pub idocb: String,
  pub sectn: String,
  pub secnb: String,
  pub sgnum: String,
  pub sgnam: String,
  pub sgdsc: String,
  pub sgnbk: String,
  pub sghnb: String,
  pub sglvl: String,
  pub serie: String,
  pub nsegm: usize,
  pub dirty: bool,
  pub parnt: Vec<HstrucTp>,
  pub l    : usize
}

pub fn conv_idoc2flat(s: SettingsTp) {
  let mut c = ConvertTp { ..Default::default() };
  let cnn = Connection::open(&s.dbopt).expect("DB Error");
  c.cntrl = s.cntrl.clone();
  c.clien = s.clien.clone();
  c.inpdr = s.inpdr.clone();
  c.outdr = s.outdr.clone();
  c.rcvpf = s.rcvpf.clone();
  c.idocx = s.objnm.to_uppercase();
  for entry in fs::read_dir(&s.inpdr).unwrap() {
    let entry = entry.unwrap().path();
    if entry.is_dir() {
      continue;
    }
    let filid = Path::new(&entry).file_name().unwrap();
    let filnm = Path::new(&filid).file_stem().unwrap();
    let extsn = Path::new(&filid).extension().unwrap();
    c.flide = filid.to_str().unwrap().to_string();
    c.flnam = filnm.to_str().unwrap().to_string();
    c.flext = extsn.to_str().unwrap().to_string();
    let inppt = format!("{}{}", &s.inpdr, c.flide);
    if s.ifilt.len() == 0 || (s.ifilt.len() > 0 &&
       pass_filter(&s.ifilt, &c.flnam)) {
      proc_file(&cnn, &mut c, inppt);
    }
  }
}

// Function to process IDoc data files, reading line by line and determining
// measures for format conversion
fn proc_file(cnn: &Connection, c: &mut ConvertTp, inppt: String) {
  c.idocb = determ_idocprops(cnn, c.idocx.clone());
  c.parnt.push(HstrucTp { .. Default::default() });
  let mut lctrl = [' ';  524];
  let mut lsegm = [' '; 1063];
  let mut lstat = [' ';  562];
  let mut OF = File::create(format!("{}{}.{}", c.outdr, c.flnam, c.flext))
    .expect("creation failed");
  let ifile = File::open(inppt).unwrap();
  let rdr = BufReader::new(ifile);
  for wlin in rdr.lines() {
    let wlin = wlin.unwrap();
    let line = wlin.trim();
    let tokn: Vec<&str> = line.split('\t').collect();
    if line.len() == 0 { // ignores lines in blank
      continue;
    }
    // Gets IDoc number
    if c.idocn.len() == 0 && tokn.len() == 1 &&
       line[0..11] == "IDoc Number".to_string() {
      let idtkn: Vec<&str> = line.split(" : ").collect();
      c.idocn = idtkn[1].trim().to_string();
      continue;
    }

    // Ignores lines no containing tabulators (after to have gotten IDoc number)
    if tokn.len() <= 1 {
      continue
    }

    // Determines data section to analyze
    if tokn[0] == EDIDC || tokn[0] == EDIDD || tokn[0] == EDIDS {
      setup_section(cnn, c, &mut lctrl, &mut lsegm, &mut lstat, tokn, &mut OF);
      continue;
    }

    // Checks in segment number to analize
    if tokn[0] == SEGNUM && tokn.len() == 3 {
      c.sgnbk = c.sgnum.clone();
      c.sgnum = tokn[2].to_string();
      continue;
    }

    // Checks in segment name to analize
    if tokn[0] == SEGNAM && tokn.len() == 3 {
      setup_segment(cnn, c, &mut lsegm, tokn, &mut OF);
      continue;
    }

    // Process fields of each data section
    if c.sectn == EDIDC {
      proc_edidc(cnn, c, &mut lctrl, tokn);
    } else if c.sectn == EDIDD {
      proc_edidd(cnn, c, &mut lsegm, tokn);
    } else if c.sectn == EDIDS {
      proc_edids();
    }

    println!("{}", c.idocn);
  }
  ren_file("inp", &c.inpdr, &c.flnam, &c.flext);
  ren_file("out", &c.outdr, &c.flnam, &c.flext);
}

fn determ_idocprops(cnn: &Connection, idocx: String) -> String {
  let mut idocb: String = Default::default();
  cnn.query_row("SELECT dname FROM items WHERE idocn=?1 and rname=\"IDOC\";",
    [idocx.to_uppercase()], |row| { Ok(idocb = row.get(0).unwrap()) })
    .expect("Error: Idoc type not found in definition DB");
  return idocb;
}

// Rename files
pub fn ren_file(mode: &str, curdr: &str, fnm: &str, fex: &str) {
  let oldnm = format!("{}{}.{}", curdr, fnm, fex);
  let mut newnm = oldnm.clone();
  if mode == "inp" {
    newnm = format!("{}inp_{}_processed.{}", curdr, fnm, fex);
  } else if mode == "out" {
    newnm = format!("{}out_{}.{}", curdr, fnm, fex);
  }
  fs::rename(oldnm, newnm).expect("File rename failure");
}

// Indicates if a char string matches one pattern
pub fn pass_filter(ifilt: &String, filen: &str) -> bool {
  if filen == ifilt {
    return true;
  }
  true
}
