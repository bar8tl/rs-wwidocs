// main.rs - Program to test/start functions to Work with IDOCs (2021-07-01 bar8tl)
include!("header.rs");

fn main() {
  let stg = set_pgm_settings(CONFIG_FILENAME);
  let t = stg.clone();
  for p in t.prm.cmdpr {
    let mut s = stg.clone();
    set_run_settings(&mut s, &p);
    match p.optn.as_str() {
      CDB => create_tablelist(s.dbopt),
      DEF => upld_idoc_definitn(s.dbopt, s.inpdr, s.objnm),
      FXS => flat_idocs_inbatch(s.dbopt, s.inpdr, s.outdr, s.pcddr, s.ifilt,
                                s.objnm, s.rcvpf, s.cntrl, s.clien),
      JSN => json_idocs_inbatch(s.dbopt, s.inpdr, s.outdr, s.pcddr, s.ifilt,
                                s.objnm),
      ALS => upld_segmt_alias(s.dbopt, s.inpdr, s.objnm),
        _ => println!("Run option not valid"),
    };
  }
}
