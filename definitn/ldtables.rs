//**********************************************************************************
// ldtables.rs : Maintain IDoc definition tables into reference DB
// (2017-05-24 bar8tl)
//**********************************************************************************
use crate::definitn::upldmitm::ItemsTp;
use crate::definitn::upldsgrp::KeystTp;
use crate::settings::GRP;
use rusqlite::Connection;

pub fn clear_items(cnn: &Connection, idocn: String) {
  cnn.execute(
    "DELETE FROM items WHERE idocn=?1", (idocn,)
  ).expect("Items clearing error");
}

pub fn isrt_items(cnn: &Connection, w: ItemsTp) {
  cnn.execute(
    "INSERT INTO items VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)",
    (
      w.idocn, w.rname, w.dname, w.rclas, w.rtype, w.dtype, w.dtext, w.level,
      w.stats, w.minlp, w.maxlp, w.lngth, w.seqno, w.strps, w.endps,
    )
  ).expect("Items insertion error");
}

pub fn clear_struc(cnn: &Connection, idocn: String, strtp: String) {
  cnn.execute(
    "DELETE FROM struc WHERE idocn=?1 and strtp=?2", (idocn, strtp,)
  ).expect("Struc clearing error");
}

pub fn isrt_struc(cnn: &Connection, idocn: String, strtp: String, pnode: KeystTp,
  cnode: KeystTp) {
  let mut pdnam = String::new();
  let mut cdnam = String::new();
  if strtp == GRP.to_uppercase() {
    let test = pnode.dname.parse::<usize>();
    match test {
      Ok(_ok) => pdnam = format!("{:02}", pnode.dname.parse::<usize>().unwrap()),
      Err(_e) => pdnam = pnode.dname.clone(),
    }
    cdnam = format!("{:02}", cnode.dname.parse::<usize>().unwrap());
  }
  cnn.execute(
    "INSERT INTO struc VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)", (
      idocn, strtp,       pnode.level, pnode.rname, pnode.pseqn,
      pdnam, pnode.dtype, pnode.dqual, cnode.rname, pnode.seqno,
      cdnam, cnode.dtype, cnode.dqual,
    )
  ).expect("Struc insertion error");
}
