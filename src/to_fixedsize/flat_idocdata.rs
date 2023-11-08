// flat_idocdata.rs - Convert individual IDOC file from classic hierarchical format
// to flat text file format (2021-07-01 bar8tl)
use crate::to_fixedsize::build_edidc_line::build_edidc_line;
use crate::to_fixedsize::build_edidd_line::build_edidd_line;
use crate::to_fixedsize::build_edids_line::build_edids_line;
use crate::to_fixedsize::get_idoc_properties::get_idoc_basicid;
use crate::to_fixedsize::prep_sectn_header::prep_sectn_header;
use crate::to_fixedsize::prep_segmt_header::prep_segmt_header;
use crate::to_fixedsize::symbols::{OKAY, EDIDC, EDIDD, EDIDS, SEGNUM, SEGNAM};
use crate::to_fixedsize::types::{HstrucTp, ConvertTp};
use rusqlite::Connection;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Function to process IDoc data files, reading line by line and determining
// measures for format conversion
pub fn flat_idocdata(dbopt: &String, inpdr: &String, inppt: &String, outdr: &String,
  objnm: &String, rcvpf: &String, cntrl: &String, clien: &String, flnam: &String,
  flext: &String) -> String {
  let mut c = ConvertTp { ..Default::default() };
  let cnn = Connection::open(&dbopt).expect("DB Error");
  c.cntrl = cntrl.clone();
  c.clien = clien.clone();
  c.inpdr = inpdr.clone();
  c.outdr = outdr.clone();
  c.rcvpf = rcvpf.clone();
  c.idocx = objnm.to_uppercase();
  c.idocb = get_idoc_basicid(&cnn, &objnm);
  c.parnt.push(HstrucTp { .. Default::default() });
  let mut lctrl = [' ';  524];
  let mut lsegm = [' '; 1063];
  let mut lstat = [' ';  562];
  let mut of = File::create(format!("{}{}.{}", outdr, flnam, flext))
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
      prep_sectn_header(&cnn, &mut c, &mut lctrl, &mut lsegm, &mut lstat, tokn,
        &mut of);
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
      prep_segmt_header(&cnn, &mut c, &mut lsegm, tokn, &mut of);
      continue;
    }

    // Process fields of each data section
    if c.sectn == EDIDC {
      build_edidc_line(&cnn, &mut c, &mut lctrl, tokn);
    } else if c.sectn == EDIDD {
      build_edidd_line(&cnn, &mut c, &mut lsegm, tokn);
    } else if c.sectn == EDIDS {
      build_edids_line();
    }
  }
  return OKAY.to_string();
}
