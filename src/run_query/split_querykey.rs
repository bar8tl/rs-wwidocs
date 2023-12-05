// split_querykey.rs -  Function to identify individual tokens in IDoc query key
// (2021-07-01 bar8tl)
use crate::run_query::types::QtoknTp;

pub fn split_querykey(key: String) -> QtoknTp {
  let mut q: QtoknTp = Default::default();
  let atokn: Vec<&str> = key.splitn(2, "[").collect();
  if atokn.len() == 2 {
    q.segmn = atokn[0].to_string();
    let btokn: Vec<&str> = atokn[1].splitn(2, "]").collect();
    if btokn.len() == 2 {
      q.instn = btokn[0].parse::<usize>().unwrap();
      let ctokn: Vec<&str> = btokn[1].splitn(2, ".").collect();
      if ctokn.len() == 2 {
        q.segmn = ctokn[0].to_string();
        let dtokn: Vec<&str> = ctokn[1].splitn(2, ":").collect();
        if dtokn.len() == 2 {
          q.qlkey = dtokn[0].to_string();
          q.qlval = dtokn[1].to_string();
        }
      }
    }
  } else {
    let btokn: Vec<&str> = key.splitn(2, ".").collect();
    if btokn.len() == 2 {
      q.segmn = btokn[0].to_string();
      let ctokn: Vec<&str> = btokn[1].splitn(2, ":").collect();
      if ctokn.len() == 2 {
        q.qlkey = ctokn[0].to_string();
        q.qlval = ctokn[1].to_string();
      }
    } else {
      q.segmn = key;
    }
  }
  return q;
}
