// proc_segmt_struct.rs - Get IDOC segments structure data and start creation of
// corresponding structure records into the local database (2021-07-01 bar8tl)
use crate::idoc_definitn::symbols::{BEGIN, END, IDOC, SEGMENT, EXTENSION, FIELDS,
  QUALIFIED, QUALF, LEVEL, SEGMENTTYPE};
use crate::idoc_definitn::types::{ParslTp, InpsgmTp, KeystTp};
use crate::idoc_definitn::write_struc_indb::{clear_struc, write_struc};
use rusqlite::Connection;

pub fn proc_segmt_struct(cnn: &Connection, sline: &ParslTp, is: &mut InpsgmTp) {
  if sline.label.ident == BEGIN {
    if sline.label.recnm == IDOC {
      is.stack.push(KeystTp {
        rname: sline.label.recnm.clone(),
        dname: sline.value.clone(),
        dtype: Default::default(), dqual: Default::default(),
        level: 0, pseqn: 0, seqno: 0
      });
      is.l += 1;
      is.tnode.rname = sline.label.recnm.clone();
      is.tnode.dname = sline.value.clone();
      is.tnode.dqual = Default::default();
      is.tnode.pseqn = 0;
      is.idocn       = sline.value.clone();
      clear_struc(cnn, is.idocn.clone(), is.strtp.clone());
    } else if sline.label.recnm == SEGMENT && sline.label.rectp.len() == 0 {
      is.sseqn += 1;
      is.tnode.rname = sline.label.recnm.clone();
      is.tnode.dname = sline.value.clone();
      is.tnode.dqual = Default::default();
      is.tnode.pseqn = is.sseqn.clone();
    }
    return;
  }

  if sline.label.ident == END && is.l >= 0 {
    if sline.label.recnm == IDOC {
      is.stack = is.stack[..is.l as usize].to_vec();
      is.l -= 1;
    } else if sline.label.recnm == SEGMENT && sline.label.rectp.len() == 0 {
      if is.l == 0 {
        is.stack[is.l as usize].seqno += 1;
        is.stack.push(KeystTp {
          rname: is.tnode.rname.clone(),
          dname: is.tnode.dname.clone(),
          dtype: is.tnode.dtype.clone(),
          dqual: is.tnode.dqual.clone(),
          level: is.tnode.level.clone(),
          pseqn: is.tnode.pseqn.clone(),
          seqno: 0
        });
        is.l += 1;
      } else if is.tnode.level <= is.stack[is.l as usize].level {
        while is.tnode.level <= is.stack[is.l as usize].level {
          write_struc(cnn, is.idocn.clone(), is.strtp.clone(),
            is.stack[is.l as usize-1].clone(),
            is.stack[is.l as usize  ].clone());
          is.stack = is.stack[..is.l as usize].to_vec();
          is.l -= 1;
        }
        is.stack[is.l as usize].seqno += 1;
        is.stack.push(KeystTp {
          rname: is.tnode.rname.clone(),
          dname: is.tnode.dname.clone(),
          dtype: is.tnode.dtype.clone(),
          dqual: is.tnode.dqual.clone(),
          level: is.tnode.level.clone(),
          pseqn: is.tnode.pseqn.clone(),
          seqno: 0
        });
        is.l += 1;
      } else if is.tnode.level > is.stack[is.l as usize].level {
        is.stack[is.l as usize].seqno += 1;
        is.stack.push(KeystTp {
          rname: is.tnode.rname.clone(),
          dname: is.tnode.dname.clone(),
          dtype: is.tnode.dtype.clone(),
          dqual: is.tnode.dqual.clone(),
          level: is.tnode.level.clone(),
          pseqn: is.tnode.pseqn.clone(),
          seqno: 0
        });
        is.l += 1;
      }
    } else if sline.label.recnm == FIELDS && is.l >= 0 {
      is.fnode.rname = Default::default();
      is.fnode.dname = Default::default();
      is.fnode.dqual = Default::default();
    }
    return;
  }

  if is.tnode.rname == SEGMENT && is.tnode.dname.len() > 0 {
    if sline.label.ident == SEGMENTTYPE {
      is.tnode.dtype = sline.value.clone();
    }
    if sline.label.ident == QUALIFIED {
      is.tnode.dqual = QUALF.to_string();
    }
    if sline.label.ident == LEVEL {
      let l = sline.value.parse::<usize>().unwrap();
      is.tnode.level = l;
    }
    return;
  }

  if is.tnode.rname == IDOC {
    if sline.label.ident == EXTENSION {
      is.idocn = sline.value.clone();
      clear_struc(cnn, is.idocn.clone(), is.strtp.clone());
    }
    return;
  }
}
