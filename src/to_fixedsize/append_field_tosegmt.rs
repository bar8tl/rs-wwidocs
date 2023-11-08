// append_field_tosegmt.rs - Append a new field value to the output segment line
// (2021-07-01 bar8tl)
use rusqlite::Connection;

pub fn append_field_tosegmt(cnn: &Connection, idocx: &String,
  lsegm: &mut [char; 1063], sgdsc: &str, flkey: &str, flval: String) {
  let mut strps: usize = 0;
  cnn.query_row("SELECT strps FROM items WHERE idocn=?1 and rname=?2 and dname=?3;",
    [idocx.clone(), sgdsc.to_string(), flkey.to_string()], |row| {
      Ok(strps = row.get(0).unwrap()) })
    .expect("Error: Segment type not found in definition DB");
  let mut k: usize = strps - 1;
  let temp: Vec<char> = flval.chars().collect();
  for i in 0..temp.len() {
    lsegm[k] = temp[i];
    k += 1;
  }
}
