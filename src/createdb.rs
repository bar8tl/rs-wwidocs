//**********************************************************************************
// createdb.rs : IDOC definition DB tables creation (2017-05-24 bar8tl)
//**********************************************************************************
use crate::settings::SettingsTp;
use rblib::db::{TlistTp, reset_tables};
use serde::Deserialize;
use serde_json;

const ITABLES: &str = r#"{
  "sqlst": [
    {"activ": true, "table": "items", "sqlst": "CREATE TABLE IF NOT EXISTS items (idocn TEXT, rname TEXT, dname TEXT, rclas TEXT, rtype TEXT, dtype TEXT, dtext TEXT, level INTEGER, stats TEXT, minlp INTEGER, maxlp INTEGER, lngth INTEGER, seqno INTEGER, strps INTEGER, endps INTEGER, PRIMARY KEY (idocn, rname, dname));"},
    {"activ": true, "table": "struc", "sqlst": "CREATE TABLE IF NOT EXISTS struc (idocn TEXT, strtp TEXT, level INTEGER, prnam TEXT, pseqn INTEGER, pdnam TEXT, pdtyp TEXT, pdqlf TEXT, crnam TEXT, cseqn INTEGER, cdnam TEXT, cdtyp TEXT, cdqlf TEXT, PRIMARY KEY (idocn, strtp, prnam, pseqn, pdnam, crnam, cseqn, cdnam));"},
    {"activ": true, "table": "segma", "sqlst": "CREATE TABLE IF NOT EXISTS segma (idocn TEXT, segtp TEXT, segdf TEXT, PRIMARY KEY (idocn, segtp, segdf));"}
  ]
}"#;

#[derive(Debug, Clone, Default, Deserialize)]
struct SqlstTp {
  activ: bool,
  table: String,
  sqlst: String
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ItablesTp {
  sqlst: Vec<SqlstTp>
}

pub fn crt_tables(s: SettingsTp) {
  let it: ItablesTp = serde_json::from_str(ITABLES).unwrap();
  let mut tlist: Vec<TlistTp> = Vec::with_capacity(it.sqlst.len());
  for cdb in &s.cfd.cdb {
    for sql in &it.sqlst {
      if cdb.table == sql.table && cdb.cr && sql.activ {
        tlist.push(TlistTp {table: sql.table.clone(), sqlst: sql.sqlst.clone()});
        break;
      }
    }
  }
  reset_tables(&s.dbopt, &tlist);
}
