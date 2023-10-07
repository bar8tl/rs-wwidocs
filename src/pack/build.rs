//**********************************************************************************
// build.rs : Process control and data IDOC records to dump them in a JSON aoutput
// (2017-05-24 bar8tl)
//**********************************************************************************
use crate::pack::ConvertTp;
use crate::pack::dump::{set_controlfield, set_segmentfield};
use rusqlite::Connection;

const RCVPRN : &str = "RCVPRN";
const RVCPRN : &str = "RVCPRN";
const CREDAT : &str = "CREDAT";
const CRETIM : &str = "CRETIM";

// Functions to process format conversion to fields in control record
pub fn proc_edidc(cnn: &Connection, c: &mut ConvertTp, lctrl: &mut [char; 524],
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
pub fn proc_edidd(cnn: &Connection, c: &mut ConvertTp, lsegm: &mut [char; 1063],
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

pub fn proc_edids() {}
