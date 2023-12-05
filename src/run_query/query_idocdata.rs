// query_idocdata.rs - Perform query on individual IDOC files in JSON format
// (2021-07-01 bar8tl)
use crate::run_query::types::{RquryTp, SquryTp, QtoknTp};
use crate::to_json::symbols::OKAY;
use crate::to_json::types::{DidocTp, FieldTp};
use crate::run_query::split_querykey::split_querykey;
use serde_json::from_reader;
use std::fs::File;
use std::io::Write;

pub fn query_idocdata(reqqy: &RquryTp, inpdr: &String, inppt: &String,
  outdr: &String, idocn: &String, flide: &String, flnam: &String,
  flext: &String) -> String {
  let mut resqy: SquryTp = Default::default();
  let mut token: Vec<QtoknTp> = Default::default();
  let mut field: String = Default::default();
  for fld in &reqqy.fields {
    let tokn: Vec<&str> = fld.split('\\').collect();
    if tokn.len() == 1 {
      resqy.fields.push(FieldTp{key: fld.to_string(), val: String::new()});
      continue;
    }
/*
    if tokn.len() == 2 && tokn[0] == "CONTROL" {
      resqy.fields.push(FieldTp{key: fld, val: query_control(tokn[1]});
      continue;
    }
    for (i, t) in tokn.iter().enumerate() {
      if i < tokn.len()-1 {
        let c = split_querykey(t.to_string());
        println!("{:?}|", c);
        token.push(c);
      } else {
        field = tokn[tokn.len()-1].to_string();
        println!("{}", field);
        if token.len() == 1 {
          resqy.fields.push(FieldTp{key: fld, val: query_segment(token[0], field)});
          continue;
        }
      }
    }
*/
  }
  let mut file = File::create(format!("{}_resp.json", flnam)).expect("error");
  let fdata = serde_json::to_string_pretty(&resqy).unwrap();
  let bdata: &[u8] = fdata.as_bytes();
  file.write_all(&bdata).unwrap();
  return OKAY.to_string();
}
