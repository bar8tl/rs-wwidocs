// prep_sectn_header.rs - Function to prepare measures to take for each data segment
// in Data Idoc being converted (2021-07-01 bar8tl)
use crate::to_fixedsize::types::{HstrucTp, ConvertTp};
use crate::to_fixedsize::write_segmt_line::write_segmt_line;
use rusqlite::Connection;
use std::fs::File;

pub fn prep_segmt_header(cnn: &Connection, c: &mut ConvertTp,
  lsegm: &mut [char; 1063], tokn: Vec<&str>, of: &mut File) {
  c.nsegm += 1;
  if c.nsegm > 1 {
    write_segmt_line(cnn, c, lsegm, of);
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
