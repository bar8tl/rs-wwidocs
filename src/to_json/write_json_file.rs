// write_json_file.rs - Write JSON output for Control, Data and Segment structures
// (2021-07-01 bar8tl)
use crate::to_json::symbols::{OUTCTRL, OUTDATA, OUTSEGM};
use crate::to_json::types::{DidocTp, SdataTp, SsegmTp};
use serde_json;
use std::fs::File;
use std::io::Write;

pub fn write_json_file(d: &mut DidocTp) {
  d.ldata.sdata.push(SdataTp {
    instn: d.setno as usize,
    rdata: d.sdata.rdata.clone()
  });
  d.ssegm.rsegm.push(
    d.rsegm.clone()
  );
  d.lsegm.ssegm.push(SsegmTp {
    instn: d.setno as usize,
    cntrl: d.lctrl.rctrl[d.setno as usize].field.clone(),
    rsegm: d.ssegm.rsegm.clone()
  });
  let ofnam = format!("{}{}-{}", d.outdr, d.flnam, format!("{}", d.setno));
  if OUTCTRL {
    let mut file = File::create(format!("{}-control.json", ofnam)).expect("error");
    let fctrl = serde_json::to_string_pretty(&d.lctrl).unwrap();
    let bctrl: &[u8] = fctrl.as_bytes();
    file.write_all(&bctrl).unwrap();
  }
  if OUTDATA {
    let mut file = File::create(format!("{}-data.json", ofnam)).expect("error");
    let fdata = serde_json::to_string_pretty(&d.ldata).unwrap();
    let bdata: &[u8] = fdata.as_bytes();
    file.write_all(&bdata).unwrap();
  }
  if OUTSEGM {
    let mut file = File::create(format!("{}-segment.json", ofnam)).expect("error");
    let fsegm = serde_json::to_string_pretty(&d.lsegm).unwrap();
    let bsegm: &[u8] = fsegm.as_bytes();
    file.write_all(&bsegm).unwrap();
  }
  d.sdata.rdata = Default::default();
  d.ldata.sdata = Default::default();
  d.ssegm.rsegm = Default::default();
  d.lsegm.ssegm = Default::default();
}
