// get_itable_entry.rs - (2021-07-01 bar8tl)
use crate::settings::set_pgm_settings::IdoctpTp;
use std::collections::HashMap;

pub fn get_itable_entry(flide: &String, idt: &HashMap<String, IdoctpTp>)
  -> IdoctpTp {
  let mut idte: IdoctpTp = Default::default();
  let atokn: Vec<&str> = flide.splitn(2, "_").collect();
  if atokn.len() == 2 {
    let key = atokn[0].to_string();
    idte = idt.get(&key).unwrap().clone();
  }
  return idte;
}
