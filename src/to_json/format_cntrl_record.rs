// format_cntrl_record - Read Control Record line and prepare JSON output
// (2021-07-01 bar8tl)
use crate::idoc_definitn::types::OutitmTp;
use crate::to_json::types::{DidocTp, RsegmTp, FieldTp, RctrlTp};
use crate::to_json::write_json_file::write_json_file;
use rusqlite::Connection;

pub fn format_cntrl_record(cnn: &Connection, d: &mut DidocTp, iline: &str,
  idocn: &String, rname: &str, first: &mut bool) {
  let mut f    : OutitmTp = OutitmTp{ ..Default::default() };
  let mut rctrl: RctrlTp  = RctrlTp { ..Default::default() };
  if *first {
    *first = false;
  } else {
    write_json_file(d);
  }
  d.recno  = 0; // Inits at Control Record level
  d.l      = -1;
  (d.c1, d.c2, d.c3, d.c4, d.c5, d.c6, d.c7, d.c8) = (-1,-1,-1,-1,-1,-1,-1,-1);
  d.setno += 1;
  d.recnf += 1;
  let mut stmt = cnn.prepare("SELECT dname, strps, endps FROM items WHERE idocn=?1
    and rname=?2 order by seqno;").expect("DB Err");
  let mut rows = stmt.query([idocn, &rname.to_string(),]).expect("DB Err");
  while let Some(row) = rows.next().expect("while row failed") {
    f.dname = row.get(0).unwrap();
    f.strps = row.get(1).unwrap();
    f.endps = row.get(2).unwrap();
    let cdval: String = iline[f.strps-1..f.endps].trim().to_string();
    if cdval.len() == 0 || cdval == "" {
      continue
    }
    rctrl.field.push(FieldTp { key: f.dname, val: cdval });
  }
  rctrl.instn = d.setno as usize;
  d.lctrl.rctrl.push(rctrl);
  d.rsegm = RsegmTp { segmn: idocn.to_string(), recno: 0, level: 0,
    qlkey: String::new(), qlval: String::new(), instn: 0, field: Vec::new(),
    child: Vec::new()
  };
}
