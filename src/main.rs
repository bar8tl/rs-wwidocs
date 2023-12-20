// main.rs - Program to test/start functions to Work with IDOCs (2021-07-01 bar8tl)
include!("header.rs");

fn main() {
  let stg = set_pgm_settings(CONFIG_FILENAME);
  let t = stg.clone();
  for p in t.prm.cmdpr {
    let mut s = stg.clone();
    set_run_settings(&mut s, &p);
    match p.optn.as_str() {
      CDB => crea_tablelist(s.dbopt),
      DEF => upld_definitn (s.dbopt, s.inpdr, s.objnm),
      FXS => flat_content  (s),
      JSN => json_content  (s),
      QRY => query_content (s),
        _ => println!("Run option not valid"),
    };
  }
}
