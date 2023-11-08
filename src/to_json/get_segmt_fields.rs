// get_segmt_fields.rs - Get field values of a segment into the IDOC structure
// (2021-07-01 bar8tl)
use crate::idoc_definitn::types::{OutitmTp, OutstrTp};
use crate::to_json::symbols::QUAL;
use crate::to_json::types::{DidocTp, FieldTp};
use rusqlite::Connection;

pub fn get_segmt_fields(cnn: &Connection, d: &mut DidocTp, iline: &str,
  idocn: &String, strtp: String, cdnam: &String) {
  let mut f    : OutitmTp = Default::default();
  let mut e    : OutstrTp = Default::default();
  let mut fitem: bool     = true;
  let mut stmt = cnn.prepare("SELECT a.idocn, a.level, a.pseqn, a.pdnam, a.pdtyp,
    a.pdqlf, a.cseqn, a.cdnam, a.cdtyp, a.cdqlf, b.dname, b.seqno, b.strps, b.endps
    FROM struc a LEFT JOIN items b ON (a.idocn = b.idocn and a.cdtyp = b.rname)
    WHERE a.idocn=?1 and a.strtp=?2 and a.cdtyp=?3  ORDER BY a.idocn, a.strtp,
    a.pseqn, a.prnam, a.pdnam, b.seqno;").expect("DB Err");
  let mut rows = stmt.query([idocn, &strtp, &cdnam.to_string(),]).expect("DB Err");
  while let Some(row) = rows.next().expect("while row failed") {
    e.idocn = row.get( 0).unwrap();
    e.level = row.get( 1).unwrap();
    e.pseqn = row.get( 2).unwrap();
    e.pdnam = row.get( 3).unwrap();
    e.pdtyp = row.get( 4).unwrap();
    e.pdqlf = row.get( 5).unwrap();
    e.cseqn = row.get( 6).unwrap();
    e.cdnam = row.get( 7).unwrap();
    e.cdtyp = row.get( 8).unwrap();
    e.cdqlf = row.get( 9).unwrap();
    f.dname = row.get(10).unwrap();
    f.seqno = row.get(11).unwrap();
    f.strps = row.get(12).unwrap();
    f.endps = row.get(13).unwrap();
    if f.endps >= iline.len() {
      break;
    }
    let cdval: String = iline[f.strps-1..f.endps].trim().to_string();
    if cdval.len() == 0 || cdval == "" {
      continue;
    }
    if fitem {
      d.sfild.segmn = e.cdtyp;
      d.sfild.recno = d.recno;
      d.sfild.level = e.level;
      if e.cdqlf == QUAL {
        d.sfild.qlkey = f.dname.clone();
        d.sfild.qlval = cdval.clone();
      } else {
        d.sfild.qlkey = String::new();
        d.sfild.qlval = String::new();
      }
      fitem = false;
    }
    d.sfild.field.push(FieldTp { key: f.dname, val: cdval });
  }
}
