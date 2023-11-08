// write_struc_indb.rs - Functions to clear/write IDOC structure records (idoc,
// group and segment levels) into the local DB (2021-07-01 bar8tl)
use crate::idoc_definitn::types::KeystTp;
use crate::idoc_definitn::symbols::GRP;
use rusqlite::Connection;

pub fn write_struc(cnn: &Connection, idocn: String, strtp: String, pnode: KeystTp,
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
    "INSERT INTO struc VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
    (idocn, strtp,       pnode.level, pnode.rname, pnode.pseqn,
     pdnam, pnode.dname, pnode.dqual, cnode.rname, pnode.seqno,
     cdnam, cnode.dname, cnode.dqual,))
    .expect("Struc insertion error");
}

pub fn clear_struc(cnn: &Connection, idocn: String, strtp: String) {
  cnn.execute("DELETE FROM struc WHERE idocn=?1 and strtp=?2", (idocn, strtp,))
    .expect("Struc clearing error");
}
