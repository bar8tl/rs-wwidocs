// json_idocdata.rs - Convert individual IDOC file from fixed size flat format to
// JSON hierarchical format (2021-07-01 bar8tl)
use crate::to_json::format_cntrl_record::format_cntrl_record;
use crate::to_json::format_data_record::format_data_record;
use crate::to_json::symbols::{OKAY, RC01, EDI_DC40, CONTROL, DATA};
use crate::to_json::types::DidocTp;
use crate::to_json::write_json_file::write_json_file;
use rusqlite::Connection;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn json_idocdata(dbopt: &String, inpdr: &String, inppt: &String, outdr: &String,
  idocn: &String, flide: &String, flnam: &String, flext: &String) -> String {
  let mut d = DidocTp { ..Default::default() };
  let cnn = Connection::open(&dbopt).expect("DB Error");
  d.dbopt = dbopt.clone();
  d.inpdr = inpdr.clone();
  d.inppt = inppt.clone();
  d.outdr = outdr.clone();
  d.idocn = idocn.clone();
  d.flnam = flnam.clone();
  d.flext = flext.clone();
  d.setno = -1; // Initialize Instance of data sets in the file
  d.recnf =  0; // Initialize Number of data records in the file
  d.flide = flide.clone();
  let mut cnt  : usize = 0;
  let mut first: bool  = true;
  let ifile = File::open(inppt).unwrap();
  let rdr = BufReader::new(ifile);
  for wline in rdr.lines() {
    let iline = wline.unwrap();
    cnt += 1;
    if cnt == 1usize {
      if &iline[0..8] == EDI_DC40 {
        format_cntrl_record(&cnn, &mut d, &iline, idocn, CONTROL, &mut first);
      } else {
        println!("IDOC File {} should start with Control Record", d.flide);
        return RC01.to_string()
      }
    } else {
      format_data_record(&cnn, &mut d, &iline, idocn, DATA);
    }
  }
  if cnt == 0usize {
    println!("Input IDOC file %s is empty: {}", d.flide);
    return RC01.to_string();
  }
  write_json_file(&mut d);
  return OKAY.to_string();
}
