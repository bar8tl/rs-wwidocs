// proc_items_master.rs - Get IDOC item detail (records, groups, segments and
// fields) and start creation of corresponding item records into the local database
// (2021-07-01 bar8tl)
use crate::idoc_definitn::symbols::{BEGIN, END, IDOC, RECORD, GROUP, SEGMENT,
  FIELDS, QUALIFIED, QUALF};
use crate::idoc_definitn::types::{ParslTp, ReclbTp, IdcdfTp, GrpdfTp, SgmdfTp,
  FlddfTp, InpitmTp};

// Scan SAP parser file to identify IDoc elements
pub fn proc_items_master(sline: &ParslTp, ii: &mut InpitmTp) {
  if sline.label.ident == BEGIN {
    ii.l += 1;
    ii.stack.push(ParslTp { label: ReclbTp { ident: sline.label.ident.clone(),
      recnm: sline.label.recnm.clone(), rectp: sline.label.rectp.clone() },
      value: sline.value.clone() });
    if sline.value != "" {
      if sline.label.recnm == IDOC {
        ii.colsi[0] = sline.value.clone();
        ii.colsi[1] = sline.value.clone();
        ii.lidoc.push(IdcdfTp {
          name: ii.colsi[0].clone(),
          typi: ii.stack[ii.l as usize].label.recnm.clone(),
          cols: ii.colsi.clone()
        });
      } else if sline.label.recnm == GROUP   {
        ii.colsg[0] = sline.value.clone();
      } else if sline.label.recnm == SEGMENT {
        ii.colss[0] = sline.value.clone();
        ii.colss[2] = String::new();
      }
    }
    return;
  }

  if sline.label.ident == END {
    ii.l -= 1;
    if ii.l < 0 {
      ii.stack = Default::default();
    } else {
      ii.stack = ii.stack[..ii.l as usize+1].to_vec();
    }
    return;
  }

  if ii.stack[ii.l as usize].label.recnm == IDOC {
    for i in 0..ii.icol.len() {
      if sline.label.ident == ii.icol[i] {
        ii.colsi[i+1] = sline.value.clone();
        if i == ii.icol.len() - 1 {
          ii.lidoc[0].cols[1] = ii.colsi[i+1].clone();
        }
        break;
      }
    }
  }

  if ii.stack[ii.l as usize].label.recnm == GROUP {
    for i in 0..ii.gcol.len() {
      if sline.label.ident == ii.gcol[i] {
        ii.colsg[i+1] = sline.value.clone();
        if i == ii.gcol.len() - 1 {
          ii.gseqn += 1;
          ii.lgrup.push(GrpdfTp {
            name: ii.colsg[0].clone(),
            typi: ii.stack[ii.l as usize].label.recnm.clone(),
            seqn: ii.gseqn.clone(),
            cols: ii.colsg.clone()
          });
        }
        break;
      }
    }
  }

  if ii.stack[ii.l as usize].label.recnm == SEGMENT {
    for i in 0..ii.scol.len() {
      if sline.label.ident == ii.scol[i] {
        if sline.label.ident == QUALIFIED {
          ii.colss[i+1] = QUALF.to_string();
        } else {
          ii.colss[i+1] = sline.value.clone();
        }
        if i == ii.scol.len() - 1 {
          ii.sseqn += 1;
          ii.lsegm.push(SgmdfTp {
            name: ii.colss[0].clone(),
            typi: ii.stack[ii.l as usize].label.recnm.clone(),
            seqn: ii.sseqn.clone(),
            cols: ii.colss.clone()
          });
        }
        break;
      }
    }
  }

  if ii.stack[ii.l as usize].label.recnm == FIELDS {
    let mut mtch = false;
    for i in 0..ii.fcol.len() {
      if sline.label.ident == ii.fcol[i] {
        ii.colsf[i] = sline.value.clone();
        mtch = true;
      }
      if i == ii.fcol.len()-1 {
        if ii.stack[ii.l as usize-1].label.rectp == RECORD {
          ii.lrecd.push(FlddfTp {
            name: ii.stack[ii.l as usize-1].label.recnm.clone(),
            typi: ii.stack[ii.l as usize  ].label.recnm.clone(),
            clas: ii.stack[ii.l as usize-1].label.rectp.clone(),
            cols: ii.colsf.clone()
          });
        } else if ii.stack[ii.l as usize-1].label.recnm == SEGMENT {
          ii.lfild.push(FlddfTp{
            name: ii.colss[0].clone(),
            typi: ii.stack[ii.l as usize  ].label.recnm.clone(),
            clas: ii.stack[ii.l as usize-1].label.recnm.clone(),
            cols: ii.colsf.clone()
          });
        }
      }
      if mtch {
        break;
      }
    }
  }
}
