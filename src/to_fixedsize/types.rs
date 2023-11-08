// types.rs - Data structures used in IDOC file conversion to fixed size format
// (2021-07-01 bar8tl)

// Data structures for format conversion to fixed size
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
