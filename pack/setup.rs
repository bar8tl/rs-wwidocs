//**********************************************************************************
// setup.rs : Setup section and segment dump (2017-05-24 bar8tl)
//**********************************************************************************
use crate::pack::{HstrucTp, ConvertTp, EDIDC, EDIDD, EDIDS};
use crate::pack::dump::{dump_controlline, dump_segmentline};
use rusqlite::Connection;
use std::fs::File;

// Function to setup measures to take for each data section. Each new section
// causes dumping data from previous one
pub fn setup_section(cnn: &Connection, c: &mut ConvertTp, lctrl: &mut [char;  524],
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
pub fn setup_segment(cnn: &Connection, c: &mut ConvertTp, lsegm: &mut [char; 1063],
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
      for _ in 1..nstep {
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
