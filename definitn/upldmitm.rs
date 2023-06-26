//**********************************************************************************
// upld_mitmdata.rs: Get IDoc item data (records, groups, segments and fields) and *
// Create corresponding item records in the database                               *
//**********************************************************************************
use crate::definitn::rdparser::*;
use crate::definitn::ldtables::*;
use rusqlite::Connection;

pub const IDOC           : &str = "IDOC";
pub const EXTENSION      : &str = "EXTENSION";
pub const RECORD         : &str = "RECORD";
pub const GROUP          : &str = "GROUP";
pub const SEGMENTTYPE    : &str = "SEGMENTTYPE";
pub const SEGMENT        : &str = "SEGMENT";
pub const FIELDS         : &str = "FIELDS";
pub const BEGIN          : &str = "BEGIN";
pub const END            : &str = "END";
pub const LEVEL          : &str = "LEVEL";
pub const LOOPMIN        : &str = "LOOPMIN";
pub const LOOPMAX        : &str = "LOOPMAX";
pub const QUALIFIED      : &str = "QUALIFIED";
pub const STATUS         : &str = "STATUS";
pub const NAME           : &str = "NAME";
pub const TEXT           : &str = "TEXT";
pub const TYPE           : &str = "TYPE";
pub const LENGTH         : &str = "LENGTH";
pub const FIELD_POS      : &str = "FIELD_POS";
pub const CHARACTER_FIRST: &str = "CHARACTER_FIRST";
pub const CHARACTER_LAST : &str = "CHARACTER_LAST";
pub const QUALF          : &str = "QUALF";

#[derive(Debug, Clone, Default)]
pub struct IdcdfTp {
  name:  String,
  typi:  String,
  cols: [String; 2] // Name, Extn
}

#[derive(Debug, Clone, Default)]
pub struct GrpdfTp {
  pub name:  String,
  pub typi:  String,
  pub seqn:  usize,
  pub cols: [String; 5] // Numb, Levl, Stat, Mnlp, Mxlp
}

#[derive(Debug, Clone, Default)]
pub struct SegdfTp {
  pub name:  String,
  pub typi:  String,
  pub seqn:  usize,
  pub cols: [String; 7] // Name, Type, Qual, Levl, Stat, Mnlp, Mxlp
}

#[derive(Debug, Clone, Default)]
pub struct FlddfTp {
  pub name:  String,
  pub typi:  String,
  pub clas:  String,
  pub cols: [String; 7] // Name, Text, Type, Lgth, Seqn, Strp, Endp
}

#[derive(Debug, Clone, Default)]
pub struct UpldmitmTp<'a> {
  pub icol :  Vec<&'a str>, // idoc    columns
  pub gcol :  Vec<&'a str>, // group   columns
  pub scol :  Vec<&'a str>, // segment columns
  pub fcol :  Vec<&'a str>, // Field   columns
  pub stack:  Vec<ParslTp>, // List of ParslTp: Levels stack
  pub lidoc:  Vec<IdcdfTp>, // List of IdcdfTp: Idoc
  pub lgrup:  Vec<GrpdfTp>, // List of GrpdfTp: Grup
  pub lsegm:  Vec<SegdfTp>, // List of SegdfTp: Segm
  pub lfild:  Vec<FlddfTp>, // List of FlddfTp: Fild
  pub lrecd:  Vec<FlddfTp>, // List of FlddfTp: Fild
  pub colsi: [String; 2],   // Name, Extn
  pub colsg: [String; 5],   // Numb, Levl, Stat, Mnlp, Mxlp
  pub colss: [String; 7],   // Name, Type, Qual, Levl, Stat, Mnlp, Mxlp
  pub colsf: [String; 7],   // Name, Text, Type, Lgth, Seqn, Strp, Endp
  pub colsr: [String; 7],   // Name, Text, Type, Lgth, Seqn, Strp, Endp
  pub l    :  usize,        // Stack level
  pub gseqn:  usize,        // Group   counter
  pub sseqn:  usize         // Segment counter
}

pub fn init_upldmitm(ui: &mut UpldmitmTp) {
  ui.icol = vec![EXTENSION];
  ui.gcol = vec![LEVEL, STATUS, LOOPMIN, LOOPMAX];
  ui.scol = vec![SEGMENTTYPE, QUALIFIED, LEVEL, STATUS, LOOPMIN, LOOPMAX];
  ui.fcol = vec![NAME, TEXT, TYPE, LENGTH, FIELD_POS, CHARACTER_FIRST, CHARACTER_LAST];
  ui.stack.push(ParslTp { ..Default::default() });
  ui.l = 0;
}

// Scan SAP parser file to identify IDoc elements
pub fn get_mitmdata(sline: &ParslTp, ui: &mut UpldmitmTp) {
  if sline.label.ident == BEGIN {
    ui.l += 1;
    let reclb: ReclbTp = ReclbTp {
      ident: sline.label.ident.clone(),
      recnm: sline.label.recnm.clone(),
      rectp: sline.label.rectp.clone()
    };
    ui.stack.push(ParslTp { label: reclb, value: sline.value.clone() });
    if sline.value != "" {
      if sline.label.recnm == IDOC {
        ui.colsi[0] = sline.value.clone();
        ui.colsi[1] = sline.value.clone();
        ui.lidoc.push(IdcdfTp {
          name: ui.colsi[0].clone(),
          typi: ui.stack[ui.l].label.recnm.clone(),
          cols: ui.colsi.clone()
        });
      } else if sline.label.recnm == GROUP   {
        ui.colsg[0] = sline.value.clone();
      } else if sline.label.recnm == SEGMENT {
        ui.colss[0] = sline.value.clone();
        ui.colss[2] = Default::default();
      }
    }
    return;
  }

  if sline.label.ident == END {
    ui.l -= 1;
    ui.stack = ui.stack[..ui.l+1].to_vec();
    return;
  }

  if ui.stack[ui.l].label.recnm == IDOC {
    for i in 0..ui.icol.len() {
      if sline.label.ident == ui.icol[i] {
        ui.colsi[i+1] = sline.value.clone();
        if i == ui.icol.len() - 1 {
          ui.lidoc[0].cols[1] = ui.colsi[i+1].clone();
        }
        break;
      }
    }
  }

  if ui.stack[ui.l].label.recnm == GROUP {
    for i in 0..ui.gcol.len() {
      if sline.label.ident == ui.gcol[i] {
        ui.colsg[i+1] = sline.value.clone();
        if i == ui.gcol.len() - 1 {
          ui.gseqn += 1;
          ui.lgrup.push(GrpdfTp {
            name: ui.colsg[0].clone(),
            typi: ui.stack[ui.l].label.recnm.clone(),
            seqn: ui.gseqn.clone(),
            cols: ui.colsg.clone()
          });
        }
        break;
      }
    }
  }

  if ui.stack[ui.l].label.recnm == SEGMENT {
    for i in 0..ui.scol.len() {
      if sline.label.ident == ui.scol[i] {
        if sline.label.ident == QUALIFIED {
          ui.colss[i+1] = QUALF.to_string();
        } else {
          ui.colss[i+1] = sline.value.clone();
        }
        if i == ui.scol.len() - 1 {
          ui.sseqn += 1;
          ui.lsegm.push(SegdfTp {
            name: ui.colss[0].clone(),
            typi: ui.stack[ui.l].label.recnm.clone(),
            seqn: ui.sseqn.clone(),
            cols: ui.colss.clone()
          });
        }
        break;
      }
    }
  }

  if ui.stack[ui.l].label.recnm == FIELDS {
    let mut mtch = false;
    for i in 0..ui.fcol.len() {
      if sline.label.ident == ui.fcol[i] {
        ui.colsf[i] = sline.value.clone();
        mtch = true;
      }
      if i == ui.fcol.len()-1 {
        if ui.stack[ui.l-1].label.rectp == RECORD {
          ui.lrecd.push(FlddfTp {
            name: ui.stack[ui.l-1].label.recnm.clone(),
            typi: ui.stack[ui.l  ].label.recnm.clone(),
            clas: ui.stack[ui.l-1].label.rectp.clone(),
            cols: ui.colsf.clone()
          });
        } else if ui.stack[ui.l-1].label.recnm == SEGMENT {
          ui.lfild.push(FlddfTp{
            name: ui.colss[0].clone(),
            typi: ui.stack[ui.l  ].label.recnm.clone(),
            clas: ui.stack[ui.l-1].label.recnm.clone(),
            cols: ui.colsf.clone()
          });
        }
      }
      if mtch {
        break;
      }
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct ItemsTp { // ITEMS fields description (*=key field in DB record)
//    Field:         //  IDOC        GROUP       SEGMENT     SGM-FIELD   RECRD-FIELD
//-----------------------------------------------------------------------------------
  pub idocn: String, //* Ex/Ba-Name  Ex/Ba-Name  Ex/Ba-Name  Ex/Ba-Name  Ex/Ba-Name
  pub rname: String, //* 'IDOC'      'GROUP'     'SEGMENT'   Segm-ID     'CONTROL'...
  pub dname: String, //* Basic-IDoc  Group#      Segm-ID     Field-Name  Field-Name
  pub rclas: String, //  Basic-IDoc  Group#      Segm-ID     'SEGMENT'   'RECORD'
  pub rtype: String, //  'IDOC'      'GROUP'     'SEGMENT'   'FIELDS'    'FIELDS'
  pub dtype: String, //  ''          ''          Segm-Type    Data-Type   Data-Type
  pub dtext: String, //  Extsn-name  Group#      Qualified   Field-Desc  Field-Desc
  pub level: usize,  //  0           Level       Level       0           0
  pub stats: String, //  ''          Status      Status      ''          ''
  pub minlp: usize,  //  0           Loop-Min    Loop-Min    0           0
  pub maxlp: usize,  //  0           Loop-Max    Loop-Max    0           0
  pub lngth: usize,  //  0           0           0           Length      Length
  pub seqno: usize,  //  0           auto-gen    Auto-gen    Field-Seqn  Field-Seqn
  pub strps: usize,  //  0           0           0           Start-Pos   Start-Pos
  pub endps: usize   //  0           0           0           End-Pos     End-Pos
}

#[derive(Debug, Clone, Default)]
pub struct StrucTp { // IDoc-Structure Descr (*=key field in DB record)
//    Field:         //  GROUP                   SEGMENT
//-----------------------------------------------------------------------
  pub idocn: String, //* Ex/Ba-Name              Ex/Ba-Name
  pub strtp: String, //* 'GRP'                   'SGM'
  pub level: usize,  //  auto-gen                auto-gen
  // PARENT
  pub prnam: String, //* p.rname='IDOC'/'GROUP'  p.rname='SEGMENT'
  pub pseqn: usize,  //* p.pseqn=autogen         p.pseqn=autogen
  pub pdnam: String, //* p.dname=Group#          p.dname=Segm-ID
  pub pdtyp: String, //  ''                      p.dtype=Segm-Type
  pub pdqlf: String, //  ''                      'QUAL'
  // CHILD
  pub crnam: String, //* c.rname='GROUP          c.rname*=Segm-ID
  pub cseqn: usize,  //* p.seqno=Group-Seq       p.seqno*=Seqno
  pub cdnam: String, //* c.dname=Group#          c.dname*=Segm/Field-Name
  pub cdtyp: String, //  ''                      c.dtype =Segm/Field-Type
  pub cdqlf: String //  ''                      'QUAL'
}

pub fn isrt_mitmdata(cnn: &Connection, ui: &mut UpldmitmTp) {
  clear_items(cnn, ui.lidoc[0].cols[1].clone());

  // upld_recd(cnn) - Upload IDoc records data
  // /RB04/YP3_DELVRY_RBNA|CONTROL|TABNAM|RECORD|FIELDS|CHARACTER|
  // Name of Table Structure||0|0||0|0|10|1|1|10
  let mut w: ItemsTp = ItemsTp { ..Default::default() };
  for lrecd in &ui.lrecd {
    w.idocn = ui.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
    w.rname = lrecd.name.clone();    // B…_CONTROL_R…   CONTROL
    w.dname = lrecd.cols[0].clone(); // NAME            TABNAM
    w.rclas = lrecd.clas.clone();    // B…_C…_RECORD    RECORD
    w.rtype = lrecd.typi.clone();    // B…_FIELDS       FIELDS
    w.dtype = lrecd.cols[2].clone(); // TYPE            CHARACTER
    w.dtext = lrecd.cols[1].clone(); // TEXT            Name of Table Stru…
    w.level = 0;
    w.stats = Default::default();
    w.minlp = 0;
    w.maxlp = 0;
    w.lngth = lrecd.cols[3].parse::<usize>().unwrap(); // LENGTH          000010
    w.seqno = lrecd.cols[4].parse::<usize>().unwrap(); // FIELD_POS       0001
    w.strps = lrecd.cols[5].parse::<usize>().unwrap(); // CHARACTER_FIRST 000001
    w.endps = lrecd.cols[6].parse::<usize>().unwrap(); // CHARACTER_LAST  000010
    isrt_items(cnn, w.clone());
  }

  // upld_idoc(cnn) - Upload IDoc idoc data
  // /RB04/YP3_DELVRY_RBNA|IDOC|DELVRY07|DELVRY07|IDOC|||/RB04/YP3_DELVRY_RBNA|0|
  // 0||0|0|0|0|0|0
  let mut w: ItemsTp = ItemsTp { ..Default::default() };
  for lidoc in &ui.lidoc {
    w.idocn = ui.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
    w.rname = lidoc.typi.clone();    // B…_IDOC         IDOC
    w.dname = lidoc.cols[0].clone(); // BEGIN_IDOC      DELVRY07
    w.rclas = lidoc.name.clone();    // BEGIN_IDOC      DELVRY07
    w.rtype = lidoc.typi.clone();    // B…_IDOC         IDOC
    w.dtype = Default::default();
    w.dtext = lidoc.cols[1].clone(); // EXTENSION       /RB04/YP3_DELVRY_RBNA
    w.level = 0;
    w.stats = Default::default();
    w.minlp = 0;
    w.maxlp = 0;
    w.lngth = 0;
    w.seqno = 0;
    w.strps = 0;
    w.endps = 0;
    isrt_items(cnn, w.clone());
  }

  // upld_grup(cnn) - Upload IDoc groups data
  // /RB04/YP3_DELVRY_RBNA|GROUP|1|1|GROUP||||1|2|MANDATORY|1|9999|0|0|0|0
  let mut w: ItemsTp = ItemsTp { ..Default::default() };
  for lgrup in &ui.lgrup {
    ui.gseqn += 1;
    w.idocn = ui.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
    w.rname = lgrup.typi.clone();    // B…_GROUP        GROUP
    w.dname = lgrup.cols[0].clone(); // BEGIN_GROUP     1
    w.rclas = lgrup.name.clone();    // BEGIN_GROUP     1
    w.rtype = lgrup.typi.clone();    // B…_GROUP        GROUP
    w.dtype = Default::default();
    w.dtext = lgrup.cols[0].clone(); // BEGIN_GROUP     1
    w.level = lgrup.cols[1].parse::<usize>().unwrap(); // LEVEL       02
    w.stats = lgrup.cols[2].clone();                   // STATUS      MANDATORY
    w.minlp = lgrup.cols[3].parse::<usize>().unwrap(); // LOOPMIN     0000000001
    w.maxlp = lgrup.cols[4].parse::<usize>().unwrap(); // LOOPMAX     0000009999
    w.lngth = 0;
    w.seqno = lgrup.seqn.clone();
    w.strps = 0;
    w.endps = 0;
    isrt_items(cnn, w.clone());
  }

  // upld_segm(cnn) - Upload IDoc segments data
  // /RB04/YP3_DELVRY_RBNA|SEGMENT|E2EDL20004|E2EDL20004|SEGMENT|E1EDL20|QUAL||
  // 0|2|MANDATORY|1|1|0|0|0|0
  let mut w: ItemsTp = ItemsTp { ..Default::default() };
  for lsegm in &ui.lsegm {
    ui.sseqn += 1;
    w.idocn = ui.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
    w.rname = lsegm.typi.clone();    // B…_SEGMENT      SEGMENT
    w.dname = lsegm.cols[0].clone(); // BEGIN_SEGMENT   E2EDL20004
    w.rclas = lsegm.name.clone();    // BEGIN_SEGMENT   E2EDL20004
    w.rtype = lsegm.typi.clone();    // B…_SEGMENT      SEGMENT
    w.dtype = lsegm.cols[1].clone(); // SEGMENTTYPE     E1EDL20
    w.dtext = lsegm.cols[2].clone(); // QUALIFIED       QUAL
    w.level = lsegm.cols[3].parse::<usize>().unwrap(); // LEVEL       02
    w.stats = lsegm.cols[4].clone();                   // STATUS      MANDATORY
    w.minlp = lsegm.cols[5].parse::<usize>().unwrap(); // LOOPMIN     0000000001
    w.maxlp = lsegm.cols[6].parse::<usize>().unwrap(); // LOOPMAX     0000000001
    w.lngth = 0;
    w.seqno = lsegm.seqn.clone();
    w.strps = 0;
    w.endps = 0;
    isrt_items(cnn, w.clone());
  }

  // upld_flds(cnn) - Upload IDoc fields data
  // /RB04/YP3_DELVRY_RBNA|E2EDL20004|VKBUR|SEGMENT|FIELDS|CHARACTER|
  // Sales Office||0|0||0|0|4|5|84|87
  let mut w: ItemsTp = ItemsTp { ..Default::default() };
  for lfild in &ui.lfild {
    w.idocn = ui.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
    w.rname = lfild.name.clone();    // BEGIN_SEGMENT   E2EDL20004
    w.dname = lfild.cols[0].clone(); // NAME            VKBUR
    w.rclas = lfild.clas.clone();    // B…_SEGMENT      SEGMENT
    w.rtype = lfild.typi.clone();    // B…_FIELDS       FIELDS
    w.dtype = lfild.cols[2].clone(); // TYPE            CHARACTER
    w.dtext = lfild.cols[1].clone(); // TEXT            Sales Office
    w.level = 0;
    w.stats = Default::default();
    w.minlp = 0;
    w.maxlp = 0;
    w.lngth = lfild.cols[3].parse::<usize>().unwrap(); // LENGTH          000004
    w.seqno = lfild.cols[4].parse::<usize>().unwrap(); // FIELD_POS       0005
    w.strps = lfild.cols[5].parse::<usize>().unwrap(); // CHARACTER_FIRST 000084
    w.endps = lfild.cols[6].parse::<usize>().unwrap(); // CHARACTER_LAST  000087
    isrt_items(cnn, w.clone());
  }
}
