// Process Control Record                                                          *
use crate::definitn::upldmitm::ItemsTp;
use crate::unpack::data::{DidocTp, RsegmTp, FieldTp};
use crate::unpack::outputs::write_outputs;
use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct RctrlTp {
  pub instn: usize,
  pub field: Vec<FieldTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LctrlTp {
  pub rctrl: Vec<RctrlTp>
}

pub fn read_control(cnn: &Connection, d: &mut DidocTp, iline: &str, idocn: &String,
   rname: &str, first: bool) -> Result<()> {
  let mut f    : ItemsTp = ItemsTp{ ..Default::default() };
  let mut rctrl: RctrlTp = RctrlTp{ ..Default::default() };
  let mut cdval: String  = Default::default();
  if !first {
    write_outputs(d);
  }
  d.recno  = 0; // Inits at Control Record level
  d.l      = 0;
  (d.c1, d.c2, d.c3, d.c4, d.c5, d.c6, d.c7, d.c8) = (0, 0, 0, 0, 0, 0, 0, 0);
  d.setno += 1;
  d.recnf += 1;
  let mut stmt = cnn.prepare("SELECT dname, strps, endps FROM items WHERE idocn=?1
    and rname=?2 order by seqno;")?;
  let mut rows = stmt.query([idocn, &rname.to_string(),])?;
  while let Some(row) = rows.next().expect("while row failed") {
    f.dname = row.get(0)?;
    f.strps = row.get(1)?;
    f.endps = row.get(2)?;
    cdval = iline[f.strps-1..f.endps].trim().to_string();
    if cdval.len() == 0 || cdval == "" {
      continue
    }
    rctrl.field.push(FieldTp { key: f.dname, val: cdval });
  }
  rctrl.instn = d.setno;
  d.lctrl.rctrl.push(rctrl);
  add_root(d, idocn);
  Ok(())
}

// Define root node in segment structure
fn add_root(d: &mut DidocTp, idocn: &String) {
  d.rsegm = RsegmTp {
    segmn: idocn.to_string(),
    recno: 0,
    level: 0,
    qlkey: "".to_string(),
    qlval: "".to_string(),
    instn: 0,
    field: Vec::new(),
    child: Vec::new()
  };
}
