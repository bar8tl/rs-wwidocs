//**********************************************************************************
// sap2flat.rs: Convert IDoc classic hierarchical format to flat text file format  *
// [20170524-BAR8TL]                                                               *
//**********************************************************************************
use crate::settings::SettingsTp;
use rusqlite::Connection;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const EDIDC  : &str = "EDIDC";
const EDIDD  : &str = "EDIDD";
const EDIDS  : &str = "EDIDS";
const DATA   : &str = "DATA";
const SEGNUM : &str = "SEGNUM";
const SEGNAM : &str = "SEGNAM";
const RVCPRN : &str = "RVCPRN";
const RCVPRN : &str = "RCVPRN";
const CREDAT : &str = "CREDAT";
const CRETIM : &str = "CRETIM";
const TABNAM : &str = "TABNAM";
const MANDT  : &str = "MANDT";
const DOCNUM : &str = "DOCNUM";
const RCVPFC : &str = "RCVPFC";
const SERIAL : &str = "SERIAL";
const PSGNUM : &str = "PSGNUM";
const HLEVEL : &str = "HLEVEL";
const IDOCTYP: &str = "IDOCTYP";
const CIMTYP : &str = "CIMTYP";

#[derive(Debug, Clone, Default)]
struct HstrucTp {
  sgnum: String,
  sgnam: String,
  sglvl: String
}

#[derive(Debug, Clone, Default)]
struct ConvertTp {
  cntrl: String,
  clien: String,
  inpdr: String,
  outdr: String,
  rcvpf: String,
  flide: String,
  flnam: String,
  flext: String,
  idocx: String,
  idocn: String,
  idocb: String,
  sectn: String,
  secnb: String,
  sgnum: String,
  sgnam: String,
  sgdsc: String,
  sgnbk: String,
  sghnb: String,
  sglvl: String,
  serie: String,
  nsegm: usize,
  dirty: bool,
  parnt: Vec<HstrucTp>,
  l    : usize
}

pub fn conv_idoc2flat(stg: SettingsTp) {
  let mut c = ConvertTp { ..Default::default() };
  let cnn = Connection::open(&stg.dbopt).expect("DB Error");
  c.cntrl = stg.cntrl.clone();
  c.clien = stg.clien.clone();
  c.inpdr = stg.inpdr.clone();
  c.outdr = stg.outdr.clone();
  c.rcvpf = stg.rcvpf.clone();
  c.idocx = stg.objnm.to_uppercase();
  for entry in fs::read_dir(&stg.inpdr).unwrap() {
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
    let inppt = format!("{}{}", &stg.inpdr, c.flide);
    if stg.ifilt.len() == 0 || (stg.ifilt.len() > 0 &&
       pass_filter(&stg.ifilt, &c.flnam)) {
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
    if c.idocn.len() == 0 && tokn.len() == 1 && line[0..11] == "IDoc Number".
       to_string() {
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
  //ren_file("inp", &c.inpdr, &c.flnam, &c.flext);
  //ren_file("out", &c.outdr, &c.flnam, &c.flext);
}

// Function to setup measures to take for each data section. Each new section
// causes dumping data from previous one
fn setup_section(cnn: &Connection, c: &mut ConvertTp, lctrl: &mut [char;  524],
   lsegm: &mut [char; 1063], lstat: &mut [char;  562], tokn: Vec<&str>,
   OF: &mut File) {
  c.sectn = tokn[0].to_string();
  if c.sectn == EDIDC {
    *lctrl = [' '; 524];
  }
  if c.sectn == EDIDD {
    dump_controlline(cnn, c, lctrl, OF);
  }
  if c.sectn == EDIDS {
    c.sgnbk = c.sgnum.clone();
    dump_segmentline(cnn, c, lsegm, OF);
    *lstat = [' '; 562];
    if tokn.len() == 3 {
      c.secnb = tokn[2].to_string();
    }
  }
}

// Function to setup measures to take for each data segment in Data Idoc being
// converted
fn setup_segment(cnn: &Connection, c: &mut ConvertTp, lsegm: &mut [char; 1063],
   tokn: Vec<&str>, OF: &mut File) {
  c.nsegm += 1;
  if c.nsegm > 1 {
    dump_segmentline(cnn, c, lsegm, OF);
  }
  c.sgnam = tokn[2].to_string();
  *lsegm = [' '; 1063];
  let mut level: usize = 0;
  cnn.query_row("SELECT dname, level FROM items WHERE idocn=?1 and rname=\"SEGMENT\"
    and dtype=?2;", [c.idocx.clone(), c.sgnam.clone()], |row| {
      Ok({
        c.sgdsc = row.get(0).unwrap();
        level   = row.get(1).unwrap();}
      )
    }).expect("Error: Idoc type not found in definition DB"
  );
  c.sglvl = format!("{:02}", level);

  if c.nsegm == 1 {
    c.parnt.push(HstrucTp{ sgnum: c.sgnum.clone(), sgnam: c.sgnam.clone(),
      sglvl: c.sglvl.clone() });
    c.l += 1;
    c.sghnb = "000000".to_string();
  } else {
    if c.sglvl > c.parnt[c.l].sglvl {
      c.parnt.push(HstrucTp{ sgnum: c.sgnum.clone(), sgnam: c.sgnam.clone(),
       sglvl: c.sglvl.clone() });
      c.l += 1;
      c.sghnb = c.parnt[c.l-1].sgnum.clone();
    } else if c.sglvl == c.parnt[c.l].sglvl {
      c.parnt[c.l].sgnum = c.sgnum.clone();
      c.parnt[c.l].sgnam = c.sgnam.clone();
      c.parnt[c.l].sglvl = c.sglvl.clone();
      c.sghnb = c.parnt[c.l-1].sgnum.clone();
    } else {
      let prvlv = c.parnt[c.l].sglvl.parse::<usize>().unwrap();
      let curlv = c.sglvl.           parse::<usize>().unwrap();
      let nstep = prvlv - curlv;
      for i in 1..nstep {
        c.l -= 1;
        c.parnt = c.parnt[..c.l+1].to_vec();
      }
      c.parnt[c.l].sgnum = c.sgnum.clone();
      c.parnt[c.l].sgnam = c.sgnam.clone();
      c.parnt[c.l].sglvl = c.sglvl.clone();
      c.sghnb = c.parnt[c.l-1].sgnum.clone();
    }
  }
}

// Functions to process format conversion to fields in control record
fn proc_edidc(cnn: &Connection, c: &mut ConvertTp, lctrl: &mut [char; 524],
   tokn: Vec<&str>) {
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
    set_controlfield(cnn, c, lctrl, flkey, flval);
  }
}

// Functions to process format conversion to fields in data records
fn proc_edidd(cnn: &Connection, c: &mut ConvertTp, lsegm: &mut [char; 1063],
   tokn: Vec<&str>) {
  let flkey = tokn[0].clone();
  let mut flval = Default::default();
  if tokn.len() == 3 {
    let flds: Vec<&str> = tokn[2].split(" :").collect();
    flval = flds[0].to_string();
  }
  if flval.len() > 0 {
    c.dirty = true;
    let sgdsc = c.sgdsc.clone();
    set_segmentfield(cnn, c, lsegm, sgdsc.as_str(), flkey, flval);
  }
}

fn proc_edids() {}

fn dump_controlline(cnn: &Connection, c: &mut ConvertTp, lctrl: &mut [char; 524],
   OF: &mut File) {
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

fn dump_segmentline(cnn: &Connection, c: &mut ConvertTp, lsegm: &mut [char; 1063],
   OF: &mut File) {
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

fn set_controlfield(cnn: &Connection, c: &mut ConvertTp, lctrl: &mut [char; 524],
   flkey: &str, mut flval: String) {
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

fn set_segmentfield(cnn: &Connection, c: &mut ConvertTp, lsegm: &mut [char; 1063],
   sgdsc: &str, flkey: &str, flval: String) {
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

fn determ_idocprops(cnn: &Connection, idocx: String) -> String {
  let mut idocb: String = Default::default();
  cnn.query_row("SELECT dname FROM items WHERE idocn=?1 and rname=\"IDOC\";",
    [idocx], |row| { Ok(idocb = row.get(0).unwrap()) })
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
  true
}
