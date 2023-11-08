// write_json_file.rs - Write JSON output for Control, Data and Segment structures
// (2021-07-01 bar8tl)
use crate::to_json::symbols::{OUTCTRL, OUTDATA, OUTSEGM};
use crate::to_json::types::{DidocTp, SdataTp, SsegmTp};
use serde_json;
use std::fs::File;
use std::io::{BufWriter, Write};

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
    let file = File::create(format!("{}-control.json", ofnam)).expect("error");
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &d.lctrl).expect("error");
    writer.flush().expect("error");
  }
  if OUTDATA {
    let file = File::create(format!("{}-data.json", ofnam)).expect("error");
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, 
      &serde_json::to_string_pretty(&d.ldata).unwrap()).expect("error");
    writer.flush().expect("error");
  }
  if OUTSEGM {
    let file = File::create(format!("{}-segment.json", ofnam)).expect("error");
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, 
      &serde_json::to_string_pretty(&d.lsegm).unwrap()).expect("error");
    writer.flush().expect("error");
  }
  d.sdata.rdata = Default::default();
  d.ldata.sdata = Default::default();
  d.ssegm.rsegm = Default::default();
  d.lsegm.ssegm = Default::default();
}
