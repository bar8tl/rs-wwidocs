// write_items_indb.rs - Functions to clear/write IDOC item detail records (idoc,
// group, segment and field) into the local DB (2021-07-01 bar8tl)
use crate::idoc_definitn::types::OutitmTp;
use rusqlite::Connection;

pub fn write_items(cnn: &Connection, w: OutitmTp) {
  cnn.execute(
    "INSERT INTO items VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)",
    (w.idocn, w.rname, w.dname, w.rclas, w.rtype, w.dtype, w.dtext, w.level,
     w.stats, w.minlp, w.maxlp, w.lngth, w.seqno, w.strps, w.endps,))
    .expect("Items insertion error");
}

pub fn clear_items(cnn: &Connection, idocn: String) {
  cnn.execute("DELETE FROM items WHERE idocn=?1", (idocn,))
    .expect("Items clearing error");
}
