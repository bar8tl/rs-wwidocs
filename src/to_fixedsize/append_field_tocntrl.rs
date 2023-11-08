// append_field_tocntrl.rs - Append a new field value to the output control record
// line (2021-07-01 bar8tl)
use crate::to_fixedsize::symbols::{IDOCTYP, CIMTYP};
use rusqlite::Connection;

pub fn append_field_tocntrl(cnn: &Connection, idocx: &String, idocb: &String,
  lctrl: &mut [char; 524], flkey: &str, mut flval: String) {
  let mut strps: usize = 0;
  cnn.query_row("SELECT strps FROM items WHERE idocn=?1 and rname=\"CONTROL\"
    and dname=?2;", [idocx.to_string(), flkey.to_string()], |row| {
      Ok(strps = row.get(0).unwrap()) })
    .expect("Error: Idoc type not found in definition DB");
  if flkey == IDOCTYP && flval == "14" {
    flval = idocb.to_string();
  }
  if flkey == CIMTYP  && flval == "14" {
    flval = idocx.to_string();
  }
  let mut k: usize = strps - 1;
  let temp: Vec<char> = flval.chars().collect();
  for i in 0..temp.len() {
    lctrl[k] = temp[i];
    k += 1;
  }
}
