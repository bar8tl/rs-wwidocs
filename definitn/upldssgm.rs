//**********************************************************************************
// upld_ssgmdata.rs: Get IDoc segments structure data and to create corresponding  *
// structure records in ref database                                               *
//**********************************************************************************
use crate::definitn::rdparser::*;
use crate::definitn::upldsgrp::KeystTp;
use crate::definitn::upldmitm::{BEGIN, END, IDOC, SEGMENT, EXTENSION, FIELDS,
  QUALIFIED, QUALF, LEVEL};
use crate::definitn::ldtables::*;
use rusqlite::Connection;

const SEGMENTTYPE: &str = "SEGMENTTYPE";

#[derive(Debug, Clone, Default)]
pub struct UpldssgmTp {
  pub stack:  Vec<KeystTp>,
  pub tnode:  KeystTp,
  pub fnode:  KeystTp,
  pub snode:  KeystTp,
  pub idocn:  String,
  pub strtp:  String,
  pub l    :  i32,
  pub sseqn:  usize
}

pub fn init_upldssgm(us: &mut UpldssgmTp, strtp: &str) {
  us.strtp = strtp.to_lowercase();
  us.l = -1;
}

pub fn get_ssgmdata(cnn: &Connection, sline: &ParslTp, us: &mut UpldssgmTp) {
  if sline.label.ident == BEGIN {
    if sline.label.recnm == IDOC {
      us.stack.push(KeystTp {
        rname: sline.label.recnm.clone(),
        dname: sline.value.clone(),
        dtype: Default::default(), dqual: Default::default(),
        level: 0, pseqn: 0, seqno: 0
      });
      us.l += 1;
      us.tnode.rname = sline.label.recnm.clone();
      us.tnode.dname = sline.value.clone();
      us.tnode.dqual = Default::default();
      us.tnode.pseqn = 0;
      us.idocn       = sline.value.clone();
      clear_struc(cnn, us.idocn.clone(), us.strtp.clone());
    } else if sline.label.recnm == SEGMENT && sline.label.rectp.len() == 0 {
      us.sseqn += 1;
      us.tnode.rname = sline.label.recnm.clone();
      us.tnode.dname = sline.value.clone();
      us.tnode.dqual = Default::default();
      us.tnode.pseqn = us.sseqn.clone();
    }
    return;
  }

  if sline.label.ident == END && us.l >= 0 {
    if sline.label.recnm == IDOC {
      us.stack = us.stack[..us.l as usize].to_vec();
      us.l -= 1;
    } else if sline.label.recnm == SEGMENT && sline.label.rectp.len() == 0 {
      if us.l == 0 {
        us.stack[us.l as usize].seqno += 1;
        us.stack.push(KeystTp {
          rname: us.tnode.rname.clone(),
          dname: us.tnode.dname.clone(),
          dtype: us.tnode.dtype.clone(),
          dqual: us.tnode.dqual.clone(),
          level: us.tnode.level.clone(),
          pseqn: us.tnode.pseqn.clone(),
          seqno: 0
        });
        us.l += 1;
      } else if us.tnode.level <= us.stack[us.l as usize].level {
        while us.tnode.level <= us.stack[us.l as usize].level {
          isrt_struc(cnn, us.idocn.clone(), us.strtp.clone(),
            us.stack[us.l as usize-1].clone(),
            us.stack[us.l as usize  ].clone());
          us.stack = us.stack[..us.l as usize].to_vec();
          us.l -= 1;
        }
        us.stack[us.l as usize].seqno += 1;
        us.stack.push(KeystTp {
          rname: us.tnode.rname.clone(),
          dname: us.tnode.dname.clone(),
          dtype: us.tnode.dtype.clone(),
          dqual: us.tnode.dqual.clone(),
          level: us.tnode.level.clone(),
          pseqn: us.tnode.pseqn.clone(),
          seqno: 0
        });
        us.l += 1;
      } else if us.tnode.level > us.stack[us.l as usize].level {
        us.stack[us.l as usize].seqno += 1;
        us.stack.push(KeystTp {
          rname: us.tnode.rname.clone(),
          dname: us.tnode.dname.clone(),
          dtype: us.tnode.dtype.clone(),
          dqual: us.tnode.dqual.clone(),
          level: us.tnode.level.clone(),
          pseqn: us.tnode.pseqn.clone(),
          seqno: 0
        });
        us.l += 1;
      }
    } else if sline.label.recnm == FIELDS && us.l >= 0 {
      us.fnode.rname = Default::default();
      us.fnode.dname = Default::default();
      us.fnode.dqual = Default::default();
    }
    return;
  }

  if us.tnode.rname == SEGMENT && us.tnode.dname.len() > 0 {
    if sline.label.ident == SEGMENTTYPE {
      us.tnode.dtype = sline.value.clone();
    }
    if sline.label.ident == QUALIFIED {
      us.tnode.dqual = QUALF.to_string();
    }
    if sline.label.ident == LEVEL {
      let l = sline.value.parse::<usize>().unwrap();
      us.tnode.level = l;
    }
    return;
  }

  if us.tnode.rname == IDOC {
    if sline.label.ident == EXTENSION {
      us.idocn = sline.value.clone();
      clear_struc(cnn, us.idocn.clone(), us.strtp.clone());
    }
    return;
  }
}
