// Process Data Record                                                             *
use crate::definitn::upldmitm::{ItemsTp, StrucTp};
use crate::unpack::control::LctrlTp;
use serde::Serialize;
use rusqlite::{Connection, Result};

#[derive(Debug, Clone, Default, Serialize)]
pub struct FieldTp {
  pub key: String,
  pub val: String
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RdataTp {
  pub segmn: String,
  pub qualf: String,
  pub level: usize,
  pub recno: usize,
  pub field: Vec<FieldTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SdataTp {
  pub instn: usize,
  pub rdata: Vec<RdataTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LdataTp {
  pub sdata: Vec<SdataTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RsegmTp {
  pub segmn: String,
  pub recno: usize,
  pub level: usize,
  pub qlkey: String,
  pub qlval: String,
  pub instn: usize,
  pub field: Vec<FieldTp>,
  pub child: Vec<RsegmTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SsegmTp {
  pub instn: usize,
  pub cntrl: Vec<FieldTp>,
  pub rsegm: Vec<RsegmTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LsegmTp {
  pub ssegm: Vec<SsegmTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SfildTp {
  pub segmn: String,
  pub recno: usize,
  pub level: usize,
  pub qlkey: String,
  pub qlval: String,
  pub field: Vec<FieldTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CountTp {
  pub segmn: String,
  pub instn: usize
}

#[derive(Debug, Clone, Default)]
pub struct DidocTp {
  pub inpdr: String,
  pub outdr: String,
  pub flide: String,
  pub flnam: String,
  pub flext: String,
  pub idocn: String,
  pub qutdr: String,
  pub recnf: usize,
  pub setno: usize,
  pub recno: usize,
  pub lctrl: LctrlTp, // Control list
  pub sdata: SdataTp, // Dataset
  pub ldata: LdataTp, // Dataset list
  pub rsegm: RsegmTp, // Segment record
  pub ssegm: SsegmTp, // Segmentset
  pub lsegm: LsegmTp, // Segmentset list
  pub sfild: SfildTp,
  pub count: [Vec<CountTp>; 9],
  pub l    : usize,
  pub c1   : usize,
  pub c2   : usize,
  pub c3   : usize,
  pub c4   : usize,
  pub c5   : usize,
  pub c6   : usize,
  pub c7   : usize,
  pub c8   : usize
}

pub fn read_data(cnn: &Connection, d: &mut DidocTp, iline: &str, idocn: &String,
   rname: &str) -> Result<()> {
  let mut f    : ItemsTp = ItemsTp { ..Default::default() };
  let mut g    : ItemsTp = ItemsTp { ..Default::default() };
  let mut rdata: RdataTp = RdataTp { ..Default::default() };
  let mut cdval: String  = Default::default();
  d.recnf += 1;
  d.recno += 1;
  let mut stmt = cnn.prepare("SELECT dname, strps, endps FROM items WHERE idocn=?1
    and rname=?2 order by seqno;")?;
  let mut rows = stmt.query([idocn, &rname.to_string(),])?;
  while let Some(row) = rows.next().expect("while row failed") {
    f.dname = row.get(0)?;
    f.strps = row.get(1)?;
    f.endps = row.get(2)?;
    cdval = iline[f.strps-1..f.endps].trim().to_string();
    if cdval.len() == 0 || cdval == "" {
      continue
    }
    if f.dname == "SEGNAM" {
      let mut dname: String = String::new();
      cnn.query_row("select segtp from segma where idocn=?1 and segdf=?2;",
        [idocn, &cdval,], |row| { Ok({ dname = row.get(0).unwrap(); })})?;
      cnn.query_row("SELECT dname, dtype, dtext, level FROM items WHERE idocn=?1
        and dname=?2 and rname=\"SEGMENT\";", [idocn, &dname,], |row| {
        Ok({
          g.dname = row.get(0).unwrap();
          g.dtype = row.get(1).unwrap();
          g.dtext = row.get(2).unwrap();
          g.level = row.get(3).unwrap();
        })
      })?;
      rdata.segmn = g.dtype.clone();
      rdata.qualf = g.dtext.clone();
      rdata.level = g.level.clone();
      rdata.recno = d.recno.clone();
    }
    if f.dname == "SDATA" {
      proc_segment(cnn, d, iline, idocn, "SGM", &g.dname, rdata.level);
      continue;
    }
    rdata.field.push(FieldTp{ key: f.dname, val: cdval });
  }
  d.sdata.rdata.push(rdata);
  Ok(())
}

//**********************************************************************************
// Process Segment Data - Determines segment Qualifier and Instance Number         *
//**********************************************************************************
fn proc_segment(cnn: &Connection, d: &mut DidocTp, iline: &str, idocn: &String,
   strtp: &str, cdnam: &String, level: usize) {
  let mut instn: usize = 0;
  let mut ident: String = String::new();
  if level == d.l {
    instn = updt_counter(d, cdnam.to_string(), d.l);
    ident = "SAME".to_string();
  } else if level > d.l {
    d.l = level;
    d.count[d.l].push(CountTp { segmn: cdnam.to_string(), instn: 1 } );
    instn = rtrv_counter(d, cdnam.to_string(), d.l);
    ident = "LOWER".to_string();
  } else if level < d.l {
    let goupl: usize = d.l - level;
    for i in 0..goupl {
      d.count[d.l] = Default::default();
      d.l -= 1;
    }
    instn = updt_counter(d, cdnam.to_string(), d.l);
    ident = "UPPER".to_string();
  }
  add_tostruct(cnn, d, iline, idocn, ident, cdnam.to_string(), d.l, instn);
}

// Update counter of segment with equal segment ID in the current struct level
fn updt_counter(d: &mut DidocTp, segmn: String, l: usize) -> usize {
  for j in 0..d.count[l].len() {
    if d.count[l][j].segmn == segmn {
      d.count[l][j].instn += 1;
      return d.count[l][j].instn;
    }
  }
  d.count[l].push(CountTp{ segmn: segmn, instn: 1 });
  return 1;
}

// Retrieve last counter of segment with equal segm ID in the current struct lvl
fn rtrv_counter(d: &mut DidocTp, segmn: String, l: usize) -> usize {
  for j in 0..d.count[l].len() {
    if d.count[l][j].segmn == segmn {
      return d.count[l][j].instn
    }
  }
  return 0;
}

// Build segment structure into an non-linked segment node
fn add_tostruct(cnn: &Connection, d: &mut DidocTp, iline: &str, idocn: &String,
   ident: String, segmn: String, l: usize, instn: usize) {
  if d.recno <= 9999 {
    d.sfild.qlkey = "".to_string();
    d.sfild.qlval = "".to_string();
    d.sfild.field = Default::default();
    get_segmdata(cnn, d, iline, idocn, "SGM".to_string(), &segmn, l).expect("err");
    if l == 1 {
      d.rsegm.child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l, qlkey: d.sfild.qlkey.clone(),
        qlval: d.sfild.qlval.clone(), instn: instn, field: d.sfild.field.clone(),
        child: Default::default() });
      (d.c2, d.c3, d.c4, d.c5, d.c6, d.c7, d.c8) = (0, 0, 0, 0, 0, 0, 0);
      d.c1 += 1;
    } else if l == 2 {
      d.rsegm.child[d.c1].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l, qlkey: d.sfild.qlkey.clone(),
        qlval: d.sfild.qlval.clone(), instn: instn, field: d.sfild.field.clone(),
        child: Default::default() });
      (d.c3, d.c4, d.c5, d.c6, d.c7, d.c8) = (0, 0, 0, 0, 0, 0);
      d.c2 += 1;
    } else if l == 3 {
      d.rsegm.child[d.c1].child[d.c2].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l, qlkey: d.sfild.qlkey.clone(),
        qlval: d.sfild.qlval.clone(), instn: instn, field: d.sfild.field.clone(),
        child: Default::default() });
      (d.c4, d.c5, d.c6, d.c7, d.c8) = (0, 0, 0, 0, 0);
      d.c3 += 1;
    } else if l == 4 {
      d.rsegm.child[d.c1].child[d.c2].child[d.c3].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l, qlkey: d.sfild.qlkey.clone(),
        qlval: d.sfild.qlval.clone(), instn: instn, field: d.sfild.field.clone(),
        child: Default::default() });
      (d.c5, d.c6, d.c7, d.c8) = (0, 0, 0, 0);
      d.c4 += 1;
    } else if l == 5 {
      d.rsegm.child[d.c1].child[d.c2].child[d.c3].child[d.c4].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l, qlkey: d.sfild.qlkey.clone(),
        qlval: d.sfild.qlval.clone(), instn: instn, field: d.sfild.field.clone(),
        child: Default::default() });
      (d.c6, d.c7, d.c8) = (0, 0, 0);
      d.c5 += 1;
    } else if l == 6 {
      d.rsegm.child[d.c1].child[d.c2].child[d.c3].child[d.c4].child[d.c5].
        child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l, qlkey: d.sfild.qlkey.clone(),
        qlval: d.sfild.qlval.clone(), instn: instn, field: d.sfild.field.clone(),
        child: Default::default() });
      (d.c7, d.c8) = (0, 0);
      d.c6 += 1;
    } else if l == 7 {
      d.rsegm.child[d.c1].child[d.c2].child[d.c3].child[d.c4].child[d.c5].
        child[d.c6].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l, qlkey: d.sfild.qlkey.clone(),
        qlval: d.sfild.qlval.clone(), instn: instn, field: d.sfild.field.clone(),
        child: Default::default() });
      (d.c8) = 0;
      d.c7 += 1;
    } else if l == 8 {
      d.rsegm.child[d.c1].child[d.c2].child[d.c3].child[d.c4].child[d.c5].
        child[d.c6].child[d.c7].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l, qlkey: d.sfild.qlkey.clone(),
        qlval: d.sfild.qlval.clone(), instn: instn, field: d.sfild.field.clone(),
        child: Default::default() });
      d.c8 += 1;
    } else if l == 9 {
      d.rsegm.child[d.c1].child[d.c2].child[d.c3].child[d.c4].child[d.c5].
        child[d.c6].child[d.c7].child[d.c8].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l, qlkey: d.sfild.qlkey.clone(),
        qlval: d.sfild.qlval.clone(), instn: instn, field: d.sfild.field.clone(),
        child: Default::default() });
    }
  }
}

//Get field values of a segment into the IDOC structure
fn get_segmdata(cnn: &Connection, d: &mut DidocTp, iline: &str, idocn: &String,
   strtp: String, cdnam: &String, level: usize) -> Result<()> {
  let mut f    : ItemsTp = Default::default();
  let mut e    : StrucTp = Default::default();
  let mut cdval: String  = String::new();
  let mut fitem: bool    = true;
  let mut stmt = cnn.prepare("SELECT a.idocn, a.level, a.pseqn, a.pdnam, a.pdtyp,
    a.pdqlf, a.cseqn, a.cdnam, a.cdtyp, a.cdqlf, b.dname, b.seqno, b.strps, b.endps
    FROM struc a LEFT JOIN items b ON (a.idocn = b.idocn and a.cdnam = b.rname)
    WHERE a.idocn=?1 and a.strtp=?2 and a.cdnam=?3  ORDER BY a.idocn, a.strtp,
    a.pseqn, a.prnam, a.pdnam, b.seqno;")?;
  let mut rows = stmt.query([idocn, &strtp, &cdnam.to_string(),])?;
  while let Some(row) = rows.next().expect("while row failed") {
    e.idocn = row.get( 0)?;
    e.level = row.get( 1)?;
    e.pseqn = row.get( 2)?;
    e.pdnam = row.get( 3)?;
    e.pdtyp = row.get( 4)?;
    e.pdqlf = row.get( 5)?;
    e.cseqn = row.get( 6)?;
    e.cdnam = row.get( 7)?;
    e.cdtyp = row.get( 8)?;
    e.cdqlf = row.get( 9)?;
    f.dname = row.get(10)?;
    f.seqno = row.get(11)?;
    f.strps = row.get(12)?;
    f.endps = row.get(13)?;
    if f.endps >= iline.len() {
      break;
    }
    cdval = iline[f.strps-1..f.endps].trim().to_string();
    if cdval.len() == 0 || cdval == "" {
      continue;
    }
    if fitem {
      d.sfild.segmn = e.cdtyp;
      d.sfild.recno = d.recno;
      d.sfild.level = e.level;
      if e.cdqlf == "QUAL" {
        d.sfild.qlkey = f.dname.clone();
        d.sfild.qlval = cdval.clone();
      } else {
        d.sfild.qlkey = "".to_string();
        d.sfild.qlval = "".to_string();
      }
      fitem = false;
    }
    d.sfild.field.push(FieldTp { key: f.dname, val: cdval });
  }
  Ok(())
}
