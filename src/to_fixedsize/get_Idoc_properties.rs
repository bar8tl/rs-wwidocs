// get_idoc_basicid.rs - Retrieve the basi name of the IDOC type that is being
// converted (2021-07-01 bar8tl)
use rusqlite::Connection;

pub fn get_idoc_basicid(cnn: &Connection, idocx: &String) -> String {
  let mut idocb: String = Default::default();
  cnn.query_row("SELECT dname FROM items WHERE idocn=?1 and rname=\"IDOC\";",
    [idocx.to_uppercase()], |row| { Ok(idocb = row.get(0).unwrap()) })
    .expect("Error: Idoc type not found in definition DB");
  return idocb;
}
