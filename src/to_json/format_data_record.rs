// read_data_record.rs - Read Data record line and prepare JSON output for pure
// segment metadata portion (2021-07-01 bar8tl)
use crate::idoc_definitn::types::OutitmTp;
use crate::to_json::calc_segmt_counters::calc_segmt_counters;
use crate::to_json::symbols::{SDATA, SEGNAM};
use crate::to_json::types::{DidocTp, RdataTp, FieldTp};
use rusqlite::Connection;

pub fn format_data_record(cnn: &Connection, d: &mut DidocTp, iline: &str,
  idocn: &String, rname: &str) {
  let mut f    : OutitmTp = OutitmTp { ..Default::default() };
  let mut g    : OutitmTp = OutitmTp { ..Default::default() };
  let mut rdata: RdataTp  = RdataTp  { ..Default::default() };
  d.recnf += 1;
  d.recno += 1;
  let mut stmt = cnn.prepare("SELECT dname, strps, endps FROM items WHERE idocn=?1
    AND rname=?2 order by seqno;").expect("DB Err");
  let mut rows = stmt.query([idocn, &rname.to_string(),]).expect("DB Err");
  while let Some(row) = rows.next().expect("while row failed") {
    f.dname = row.get(0).unwrap();
    f.strps = row.get(1).unwrap();
    f.endps = row.get(2).unwrap();
    if f.endps >= iline.len() {
      f.endps = iline.len();
    }
    let cdval: String = iline[f.strps-1..f.endps].trim().to_string();
    if cdval.len() == 0 || cdval == "" {
      continue
    }
    if f.dname == SEGNAM {
      cnn.query_row("SELECT dname, dtype, dtext, level FROM items WHERE idocn=?1
        AND dname=?2 AND rname=\"SEGMENT\";", [idocn, &cdval,], |row| {
        Ok({
          g.dname = row.get(0).unwrap();
          g.dtype = row.get(1).unwrap();
          g.dtext = row.get(2).unwrap();
          g.level = row.get(3).unwrap();
        })
      }).expect("DB Err");
      rdata.segmn = g.dtype.clone();
      rdata.qualf = g.dtext.clone();
      rdata.level = g.level.clone();
      rdata.recno = d.recno.clone();
    }
    if f.dname == SDATA {
      calc_segmt_counters(cnn, d, iline, idocn, &g.dname, rdata.level);
      continue;
    }
    rdata.field.push(FieldTp{ key: f.dname, val: cdval });
  }
  d.sdata.rdata.push(rdata);
}
