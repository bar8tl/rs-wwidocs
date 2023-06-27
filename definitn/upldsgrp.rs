//**********************************************************************************
// upldsgrp.rs: Get IDoc groups structure data and to create corresponding         *
// structure records in the database                                               *
//**********************************************************************************
use crate::definitn::ldtables::*;
use crate::definitn::rdparser::*;
use crate::definitn::upldmitm::{BEGIN, END, IDOC, GROUP, EXTENSION};
use rusqlite::Connection;

#[derive(Debug, Clone, Default)]
pub struct KeystTp { // Structure Node Attributes
//    Field:         // IDOC        GROUP      SEGMENT
//------------------------------------------------------
  pub rname: String, // 'IDOC'      'GROUP'    'SEGMENT'
  pub dname: String, // Basic-IDoc  Group#     Segm-ID
  pub dtype: String, // ''          ''         Segm-Type
  pub dqual: String, // ''          ''         'QUAL'
  pub level: usize,  // 0           Level      Level
  pub pseqn: usize,  // 0           auto-gen   auto-gen
  pub seqno: usize   // 0           Group-Seq  Segm-Seq
}

#[derive(Debug, Clone, Default)]
pub struct UpldsgrpTp {
  pub stack:  Vec<KeystTp>, // List of KeystTp: Levels stack
  pub idocn:  String,
  pub strtp:  String,
  pub l    :  i32,
  pub gseqn:  usize
}

pub fn init_upldsgrp(ug: &mut UpldsgrpTp, strtp: &str) {
  ug.strtp = strtp.to_lowercase();
  ug.l = -1;
}

pub fn get_sgrpdata(cnn: &Connection, sline: &ParslTp, ug: &mut UpldsgrpTp) {
  if sline.label.ident == BEGIN {
    if sline.label.recnm == IDOC {
      ug.stack.push(KeystTp {
        rname: sline.label.recnm.clone(),
        dname: sline.value.clone(),
        dtype: String::new(), dqual: String::new(),
        level: 0, pseqn: 0, seqno: 0
      });
      ug.l += 1;
      ug.idocn = sline.value.clone();
      clear_struc(cnn, ug.idocn.clone(), ug.strtp.clone());
    } else if sline.label.recnm == GROUP {
      ug.stack[ug.l as usize].seqno += 1;
      ug.stack.push(KeystTp {
        rname: sline.label.recnm.clone(),
        dname: sline.value.clone(),
        dtype: String::new(), dqual: String::new(),
        level: 0, pseqn: 0, seqno: 0
      });
      ug.l += 1;
    }
    return;
  }
  if sline.label.ident == END {
    if sline.label.recnm == IDOC {
      ug.stack = ug.stack[..ug.l as usize].to_vec();
      ug.l -= 1;
    } else if sline.label.recnm == GROUP {
      ug.gseqn += 1;
      ug.stack[ug.l as usize-1].pseqn = ug.gseqn;
      isrt_struc(cnn, ug.idocn.clone(), ug.strtp.clone(), 
        ug.stack[ug.l as usize-1].clone(), ug.stack[ug.l as usize].clone());
      ug.stack = ug.stack[..ug.l as usize].to_vec();
      ug.l -= 1;
    }
    return;
  }
  if ug.l >= 0 && ug.stack[ug.l as usize].rname == IDOC {
    if sline.label.ident == EXTENSION {
      ug.idocn = sline.value.clone();
      clear_struc(cnn, ug.idocn.clone(), ug.strtp.clone());
    }
    return;
  }
}
