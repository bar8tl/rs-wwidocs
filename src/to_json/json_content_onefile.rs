// json_content_onefile.rs - Convert individual IDOC file from fixed size flat
// format to JSON hierarchical format (2021-07-01 bar8tl)
use crate::to_json::format_cntrl_record::format_cntrl_record;
use crate::to_json::format_data_record::format_data_record;
use crate::to_json::symbols::{OKAY, RC01, EDI_DC40, CONTROL, DATA};
use crate::to_json::types::DidocTp;
use crate::to_json::write_json_file::write_json_file;
use crate::settings::get_itable_entry::get_itable_entry;
use crate::settings::set_pgm_settings::IdoctpTp;
use rblib::files_infolder::FilelistTp;
use rusqlite::Connection;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn json_content_onefile(inptp: &String, inpdr: &String, outtp: &String,
  outdr: &String, idt: &HashMap<String, IdoctpTp>, cnn: &Connection,
  fle: &FilelistTp) -> String {
  let idte: IdoctpTp = get_itable_entry(&fle.flnam, idt);
  let mut d = DidocTp { ..Default::default() };
  //d.dbopt = dbopt.clone();
  d.inpdr = inpdr.clone();
  d.outdr = outdr.clone();
  d.inppt = fle.flpth.clone();
  d.flide = fle.flide.clone();
  d.flnam = fle.flnam.clone();
  d.flext = fle.flext.clone();
  d.idocn = idte.itype.clone();
  d.setno = -1; // Initialize Instance of data sets in the file
  d.recnf =  0; // Initialize Number of data records in the file
  let mut cnt  : usize = 0;
  let mut first: bool  = true;
  let ifile = File::open(d.inppt.clone()).unwrap();
  let rdr = BufReader::new(ifile);
  for wline in rdr.lines() {
    let iline = wline.unwrap();
    cnt += 1;
    if cnt == 1usize {
      if &iline[0..8] == EDI_DC40 {
        format_cntrl_record(cnn, &mut d, &iline, &idte.itype, CONTROL, &mut first);
      } else {
        println!("IDOC File {} should start with Control Record", d.flide);
        return RC01.to_string()
      }
    } else {
      format_data_record(cnn, &mut d, &iline, &idte.itype, DATA);
    }
  }
  if cnt == 0usize {
    println!("Input IDOC file %s is empty: {}", d.flide);
    return RC01.to_string();
  }
  write_json_file(&mut d);
  return OKAY.to_string();
}
