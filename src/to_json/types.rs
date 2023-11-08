// types.rs - Data structures used in IDOC file conversion to JSON format
// (2021-07-01 bar8tl)
use serde::Serialize;

// Data structures for output in JSON format
#[derive(Debug, Clone, Default, Serialize)]
pub struct FieldTp {
  pub key: String,
  pub val: String
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RdataTp {
  pub segmn: String,
  pub qualf: String,
  pub level: usize,
  pub recno: usize,
  pub field: Vec<FieldTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SdataTp {
  pub instn: usize,
  pub rdata: Vec<RdataTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LdataTp {
  pub sdata: Vec<SdataTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RsegmTp {
  pub segmn: String,
  pub recno: usize,
  pub level: usize,
  pub qlkey: String,
  pub qlval: String,
  pub instn: usize,
  pub field: Vec<FieldTp>,
  pub child: Vec<RsegmTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SsegmTp {
  pub instn: usize,
  pub cntrl: Vec<FieldTp>,
  pub rsegm: Vec<RsegmTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LsegmTp {
  pub ssegm: Vec<SsegmTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SfildTp {
  pub segmn: String,
  pub recno: usize,
  pub level: usize,
  pub qlkey: String,
  pub qlval: String,
  pub field: Vec<FieldTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RctrlTp {
  pub instn: usize,
  pub field: Vec<FieldTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LctrlTp {
  pub rctrl: Vec<RctrlTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CountTp {
  pub segmn: String,
  pub instn: usize
}

// Data structures for functions of conversin to JSON format
#[derive(Debug, Clone, Default)]
pub struct DidocTp {
  pub dbopt: String,
  pub inppt: String,
  pub inpdr: String,
  pub outdr: String,
  pub flide: String,
  pub flnam: String,
  pub flext: String,
  pub idocn: String,
  pub qutdr: String,
  pub recnf: usize,
  pub setno: i32,
  pub recno: usize,
  pub lctrl: LctrlTp, // Control list
  pub sdata: SdataTp, // Dataset
  pub ldata: LdataTp, // Dataset list
  pub rsegm: RsegmTp, // Segment record
  pub ssegm: SsegmTp, // Segmentset
  pub lsegm: LsegmTp, // Segmentset list
  pub sfild: SfildTp,
  pub count: [Vec<CountTp>; 9],
  pub l    : i32,
  pub c1   : i32,
  pub c2   : i32,
  pub c3   : i32,
  pub c4   : i32,
  pub c5   : i32,
  pub c6   : i32,
  pub c7   : i32,
  pub c8   : i32
}
