// proc_group_struct.rs - Get IDOC groups structure detail and start creation of
// corresponding structure records into the local database (2021-07-01 bar8tl)
use crate::idoc_definitn::symbols::{BEGIN, END, IDOC, GROUP, EXTENSION};
use crate::idoc_definitn::types::{ParslTp, InpgrpTp, KeystTp};
use crate::idoc_definitn::write_struc_indb::{clear_struc, write_struc};
use rusqlite::Connection;

pub fn proc_group_struct(cnn: &Connection, sline: &ParslTp, ig: &mut InpgrpTp) {
  if sline.label.ident == BEGIN {
    if sline.label.recnm == IDOC {
      ig.stack.push(KeystTp {
        rname: sline.label.recnm.clone(),
        dname: sline.value.clone(),
        dtype: String::new(), dqual: String::new(),
        level: 0, pseqn: 0, seqno: 0
      });
      ig.l += 1;
      ig.idocn = sline.value.clone();
      clear_struc(cnn, ig.idocn.clone(), ig.strtp.clone());
    } else if sline.label.recnm == GROUP {
      ig.stack[ig.l as usize].seqno += 1;
      ig.stack.push(KeystTp {
        rname: sline.label.recnm.clone(),
        dname: sline.value.clone(),
        dtype: String::new(), dqual: String::new(),
        level: 0, pseqn: 0, seqno: 0
      });
      ig.l += 1;
    }
    return;
  }
  if sline.label.ident == END {
    if sline.label.recnm == IDOC {
      ig.stack = ig.stack[..ig.l as usize].to_vec();
      ig.l -= 1;
    } else if sline.label.recnm == GROUP {
      ig.gseqn += 1;
      ig.stack[ig.l as usize-1].pseqn = ig.gseqn;
      write_struc(cnn, ig.idocn.clone(), ig.strtp.clone(),
        ig.stack[ig.l as usize-1].clone(), ig.stack[ig.l as usize].clone());
      ig.stack = ig.stack[..ig.l as usize].to_vec();
      ig.l -= 1;
    }
    return;
  }
  if ig.l >= 0 && ig.stack[ig.l as usize].rname == IDOC {
    if sline.label.ident == EXTENSION {
      ig.idocn = sline.value.clone();
      clear_struc(cnn, ig.idocn.clone(), ig.strtp.clone());
    }
    return;
  }
}
