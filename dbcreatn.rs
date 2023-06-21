//**********************************************************************************
// dbcreatn: Selected DB tables creation [20170524-BAR8TL]                         *
//**********************************************************************************
use crate::settings::SettingsTp;
use rusqlite::{Connection, Result};
use serde::Deserialize;
use serde_json;

const ITABLES: &str = r#"
{
  "sqlst": [
    {"activ": true, "table": "items", "sqlst": "CREATE TABLE IF NOT EXISTS items (idocn TEXT, rname TEXT, dname TEXT, rclas TEXT, rtype TEXT, dtype TEXT, dtext TEXT, level INTEGER, stats TEXT, minlp INTEGER, maxlp INTEGER, lngth INTEGER, seqno INTEGER, strps INTEGER, endps INTEGER, PRIMARY KEY (idocn, rname, dname));"},
    {"activ": true, "table": "struc", "sqlst": "CREATE TABLE IF NOT EXISTS struc (idocn TEXT, strtp TEXT, level INTEGER, prnam TEXT, pseqn INTEGER, pdnam TEXT, pdtyp TEXT, pdqlf TEXT, crnam TEXT, cseqn INTEGER, cdnam TEXT, cdtyp TEXT, cdqlf TEXT, PRIMARY KEY (idocn, strtp, prnam, pseqn, pdnam, crnam, cseqn, cdnam));"},
    {"activ": true, "table": "segma", "sqlst": "CREATE TABLE IF NOT EXISTS segma (idocn TEXT, segtp TEXT, segdf TEXT, PRIMARY KEY (idocn, segtp, segdf));"}
  ]
}
"#;

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

#[derive(Debug, Clone, Default)]
struct TlistTp {
  table: String,
  sqlst: String
}

pub fn crt_tables(s: SettingsTp) {
  let it: ItablesTp = serde_json::from_str(ITABLES).unwrap();
  let mut tlist: Vec<TlistTp> = Vec::with_capacity(5);
  let cnn = Connection::open(&s.dbopt).expect("Open DB connection error");
  for cdb in &s.cfd.cdb {
    for sql in &it.sqlst {
      if cdb.table == sql.table && cdb.cr && sql.activ {
        tlist.push(TlistTp {
          table: sql.table.clone(),
          sqlst: sql.sqlst.clone()
        });
        break;
      }
    }
  }
  crt_table(&cnn, &s.dbopt, &tlist).expect("Table creation error");
}

fn crt_table(cnn: &Connection, dbopt: &String, tlist: &Vec<TlistTp>) ->
  Result<()> {
  for tabl in tlist {
    cnn.execute(format!("DROP TABLE IF EXISTS {}", tabl.table).as_str(), [])?;
    cnn.execute(tabl.sqlst.as_str(), [])?;
    println!("Table {} created...", tabl.table);
  }
  println!("Creation of dabatase {} completed.", dbopt);
  Ok(())
}
