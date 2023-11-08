// prep_items_output.rs - Format IDOC item definition detail (idoc, group, segment
// and field) in internal database layouts and start creation of item records into
// the local database (2021-07-01 bar8tl)
use crate::idoc_definitn::types::{InpitmTp, OutitmTp};
use crate::idoc_definitn::write_items_indb::{clear_items, write_items};
use rusqlite::Connection;

pub fn prep_items_output(cnn: &Connection, ii: &mut InpitmTp) {
  clear_items(cnn, ii.lidoc[0].cols[1].clone());

  // upld_recd(cnn) - Upload IDoc records data
  // /RB04/YP3_DELVRY_RBNA|CONTROL|TABNAM|RECORD|FIELDS|CHARACTER|
  // Name of Table Structure||0|0||0|0|10|1|1|10
  let mut w: OutitmTp = OutitmTp { ..Default::default() };
  for lrecd in &ii.lrecd {
    w.idocn = ii.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
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
    write_items(cnn, w.clone());
  }

  // upld_idoc(cnn) - Upload IDoc idoc data
  // /RB04/YP3_DELVRY_RBNA|IDOC|DELVRY07|DELVRY07|IDOC|||/RB04/YP3_DELVRY_RBNA|0|
  // 0||0|0|0|0|0|0
  let mut w: OutitmTp = OutitmTp { ..Default::default() };
  for lidoc in &ii.lidoc {
    w.idocn = ii.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
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
    write_items(cnn, w.clone());
  }

  // upld_grup(cnn) - Upload IDoc groups data
  // /RB04/YP3_DELVRY_RBNA|GROUP|1|1|GROUP||||1|2|MANDATORY|1|9999|0|0|0|0
  let mut w: OutitmTp = OutitmTp { ..Default::default() };
  for lgrup in &ii.lgrup {
    ii.gseqn += 1;
    w.idocn = ii.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
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
    write_items(cnn, w.clone());
  }

  // upld_segm(cnn) - Upload IDoc segments data
  // /RB04/YP3_DELVRY_RBNA|SEGMENT|E2EDL20004|E2EDL20004|SEGMENT|E1EDL20|QUAL||
  // 0|2|MANDATORY|1|1|0|0|0|0
  let mut w: OutitmTp = OutitmTp { ..Default::default() };
  for lsegm in &ii.lsegm {
    ii.sseqn += 1;
    w.idocn = ii.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
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
    write_items(cnn, w.clone());
  }

  // upld_flds(cnn) - Upload IDoc fields data
  // /RB04/YP3_DELVRY_RBNA|E2EDL20004|VKBUR|SEGMENT|FIELDS|CHARACTER|
  // Sales Office||0|0||0|0|4|5|84|87
  let mut w: OutitmTp = OutitmTp { ..Default::default() };
  for lfild in &ii.lfild {
    w.idocn = ii.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
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
    write_items(cnn, w.clone());
  }
}
