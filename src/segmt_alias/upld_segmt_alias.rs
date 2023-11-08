// upld_segmt_alias: Upload segment alias names to IDOC definition database
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::from_reader;
use std::fs::File;

#[derive(Debug, Clone, Default, Deserialize)]
struct SgmaTp {
  tipe: String,
  defn: String
}

#[derive(Debug, Clone, Default, Deserialize)]
struct SegmaTp {
  idoc: String,
  segm: Vec<SgmaTp>
}

#[derive(Debug, Clone, Default, Deserialize)]
struct SgmaliasTp {
  sgmal: Vec<SegmaTp>
}

pub fn upld_segmt_alias(dbopt: String, inpdr: String, objnm: String) {
  let f = File::open(format!("{}{}", inpdr, objnm)).unwrap();
  let ua: SgmaliasTp = from_reader(f).unwrap();
  let cnn = Connection::open(&dbopt).expect("Error opening DB");
  cnn.execute("DELETE FROM segma;", []).expect("Error clearing SEGMA table");
  for sa in &ua.sgmal {
    for ss in &sa.segm {
      cnn.execute(
        "INSERT INTO segma VALUES(?1,?2,?3);", (&sa.idoc, &ss.tipe, &ss.defn,))
      .expect("Error writing on SEGMA table");
    }
  }
  println!("SEGMA table written okay");
}
