// Generate outputs
use crate::unpack::data::{DidocTp, SdataTp, SsegmTp};
use serde_json;
use std::fs::File;
use std::io::{BufWriter, Write};

const OUTCTRL: bool = false;
const OUTDATA: bool = false;
const OUTSEGM: bool = true;

pub fn write_outputs(d: &mut DidocTp) {
  d.ldata.sdata.push(SdataTp {
    instn: d.setno,
    rdata: d.sdata.rdata.clone()
  });
  d.ssegm.rsegm.push(
    d.rsegm.clone()
  );
  d.lsegm.ssegm.push(SsegmTp {
    instn: d.setno,
    cntrl: d.lctrl.rctrl[d.setno].field.clone(),
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
    serde_json::to_writer(&mut writer, &d.ldata).expect("error");
    writer.flush().expect("error");
  }
  if OUTSEGM {
    let file = File::create(format!("{}-segment.json", ofnam)).expect("error");
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &d.lsegm).expect("error");
    writer.flush().expect("error");
  }
  d.sdata.rdata = Default::default();
  d.ldata.sdata = Default::default();
  d.ssegm.rsegm = Default::default();
  d.lsegm.ssegm = Default::default();
}
