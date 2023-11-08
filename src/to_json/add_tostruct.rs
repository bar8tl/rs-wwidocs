// add_tostruct.rs - Build segment structure into an non-linked segment node
// (2021-07-01 bar8tl)
use crate::to_json::get_segmt_fields::get_segmt_fields;
use crate::to_json::symbols::SGM;
use crate::to_json::types::{DidocTp, RsegmTp};
use rusqlite::Connection;

pub fn add_tostruct(cnn: &Connection, d: &mut DidocTp, iline: &str, idocn: &String,
   _ident: String, segmn: String, l: i32, instn: usize) {
  if d.recno <= 9999 {
    d.sfild.qlkey = "".to_string();
    d.sfild.qlval = "".to_string();
    d.sfild.field = Default::default();
    get_segmt_fields(cnn, d, iline, idocn, SGM.to_string(), &segmn);
    if l == 1 {
      d.rsegm.child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c2, d.c3, d.c4, d.c5, d.c6, d.c7, d.c8) = (-1, -1, -1, -1, -1, -1, -1);
      d.c1 += 1;
    } else if l == 2 {
      d.rsegm.child[d.c1 as usize].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c3, d.c4, d.c5, d.c6, d.c7, d.c8) = (-1, -1, -1, -1, -1, -1);
      d.c2 += 1;
    } else if l == 3 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c4, d.c5, d.c6, d.c7, d.c8) = (-1, -1, -1, -1, -1);
      d.c3 += 1;
    } else if l == 4 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child[d.c3 as usize].
        child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c5, d.c6, d.c7, d.c8) = (-1, -1, -1, -1);
      d.c4 += 1;
    } else if l == 5 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child[d.c3 as usize].
        child[d.c4 as usize].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c6, d.c7, d.c8) = (-1, -1, -1);
      d.c5 += 1;
    } else if l == 6 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child[d.c3 as usize].
        child[d.c4 as usize].child[d.c5 as usize].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c7, d.c8) = (-1, -1);
      d.c6 += 1;
    } else if l == 7 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child[d.c3 as usize].
        child[d.c4 as usize].child[d.c5 as usize].child[d.c6 as usize].
        child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c8) = -1;
      d.c7 += 1;
    } else if l == 8 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child[d.c3 as usize].
        child[d.c4 as usize].child[d.c5 as usize].child[d.c6 as usize].
        child[d.c7 as usize].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      d.c8 += 1;
    } else if l == 9 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child[d.c3 as usize].
        child[d.c4 as usize].child[d.c5 as usize].child[d.c6 as usize].
        child[d.c7 as usize].child[d.c8 as usize].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
    }
  }
}
