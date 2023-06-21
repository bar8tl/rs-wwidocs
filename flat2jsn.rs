//**********************************************************************************
// flat2jsn.rs: Read SAP-Idoc content in standard flat TXT format and upload data  *
// into internal structures [20170524 BAR8TL]                                      *
//**********************************************************************************
use crate::settings::{SettingsTp};
use rusqlite::{Connection};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const OUTCTRL: bool = false;
const OUTDATA: bool = false;
const OUTSEGM: bool = true;

#[derive(Debug, Clone, Default)]
struct FieldTp {
  key: String,
  val: String
}

#[derive(Debug, Clone, Default)]
struct RctrlTp {
  instn: usize,
  field: Vec<FieldTp>
}

#[derive(Debug, Clone, Default)]
struct LctrlTp {
  rctrl: Vec<RctrlTp>
}

#[derive(Debug, Clone, Default)]
struct RdataTp {
  segmn: String,
  qualf: String,
  level: usize,
  recno: usize,
  field: Vec<FieldTp>
}

#[derive(Debug, Clone, Default)]
struct SdataTp {
  instn: usize,
  rdata: Vec<RdataTp>
}

#[derive(Debug, Clone, Default)]
struct LdataTp {
  sdata: Vec<SdataTp>
}

#[derive(Debug, Clone, Default)]
struct RsegmTp {
  segmn: String,
  recno: usize,
  level: usize,
  qlkey: String,
  qlval: String,
  instn: usize,
  field: Vec<FieldTp>,
  child: Vec<RsegmTp>
}

#[derive(Debug, Clone, Default)]
struct SsegmTp {
  instn: usize,
  cntrl: Vec<FieldTp>,
  rsegm: Vec<RsegmTp>
}

#[derive(Debug, Clone, Default)]
struct LsegmTp {
  ssegm: Vec<SsegmTp>
}

#[derive(Debug, Clone, Default)]
struct SfildTp {
  segmn: String,
  recno: usize,
  level: usize,
  qlkey: String,
  qlval: String,
  field: Vec<FieldTp>
}

#[derive(Debug, Clone, Default)]
struct CountTp {
  segmn: String,
  instn: usize
}

#[derive(Debug, Clone, Default)]
struct DidocTp {
  inpdr: String,
  outdr: String,
  flide: String,
  flnam: String,
  flext: String,
  idocn: String,
  qutdr: String,
  recnf: usize,
  setno: usize,
  recno: usize,
  lctrl: LctrlTp, // Control list
  sdata: SdataTp, // Dataset
  ldata: LdataTp, // Dataset list
  rsegm: RsegmTp, // Segment record
  ssegm: SsegmTp, // Segmentset
  lsegm: LsegmTp, // Segmentset list
  sfild: SfildTp,
  count: [Vec<CountTp>; 9],
  l    : usize,
  c1   : usize,
  c2   : usize,
  c3   : usize,
  c4   : usize,
  c5   : usize,
  c6   : usize,
  c7   : usize,
  c8   : usize
}

pub fn conv_flat2json(s: SettingsTp) {
  let mut d = DidocTp { ..Default::default() };
  let cnn = Connection::open(&s.dbopt).expect("DB Error");
  d.outdr = s.outdr.clone();
  d.idocn = s.objnm.to_uppercase();
  for entry in fs::read_dir(&s.inpdr).unwrap() {
    let entry = entry.unwrap().path();
    if entry.is_dir() {
      continue;
    }
    let filid = Path::new(&entry).file_name().unwrap();
    let filnm = Path::new(&filid).file_stem().unwrap();
    let extsn = Path::new(&filid).extension().unwrap();
    d.flide = filid.to_str().unwrap().to_string();
    d.flnam = filnm.to_str().unwrap().to_string();
    d.flext = extsn.to_str().unwrap().to_string();
    let inppt = format!("{}{}", &s.inpdr, d.flide);
    if s.ifilt.len() == 0 || (s.ifilt.len() > 0 &&
       pass_filter(&s.ifilt, &d.flnam)) {
      proc_file(&cnn, &mut d, inppt);
    }
  }
}

//**********************************************************************************
// Process Input IDOC File                                                         *
//**********************************************************************************
fn proc_file(cnn: &Connection, d: &mut DidocTp, inppt: String) {
  d.setno = Default::default(); // Initialize Instance of data sets in the file
  d.recnf = Default::default(); // Initialize Number of data records in the file
  let ifile = File::open(inppt).unwrap();
  let rdr = BufReader::new(ifile);
  proc_startFile();
  for wlin in rdr.lines() {
    let wlin = wlin.unwrap();
    let line = wlin.trim();
    if &line[0..10] == "EDI_DC40" {
      read_control();
    } else {
      read_data();
    }
  }
  proc_endfile();
}

// Indicates if a char string matches one pattern
fn pass_filter(ifilt: &String, filen: &str) -> bool {
  true
}

fn proc_startFile() {}
fn read_control() {}
fn read_data() {}
fn proc_endfile() {}

// Open input IDOC file and check first record is a Control Record
/*
func (d *Didoc_tp) ProcStartOfFile(rdr *bufio.Reader, fname string) {
  iline, err := rdr.ReadString(byte('\n'))
  if err == io.EOF {
    log.Fatalf("Input IDOC file %s is empty: %s\r\n", fname, err)
  }
  if err != nil {
    log.Fatalf("Input IDOC file %s cannot be read: %s\r\n", fname, err)
  }
  if strings.TrimSpace(iline[0:10]) == "EDI_DC40" {
    d.ReadControl(iline, d.Idocn, "CONTROL", true)
  } else {
    log.Fatalf("IDOC File %s should start with Control Record\r\n", fname)
  }
}
*/
/*
// Fetch last records in structure to complete data detail in memory
func (d *Didoc_tp) ProcEndOfFile(err error, outdr string) {
  if err != io.EOF && err != nil {
    log.Fatalf("Error during reading input IDOC file %s %s:\r\n", d.Flnam, err)
  }
  d.WriteOutputs()
}

// Generate outputs
func (d *Didoc_tp) WriteOutputs() {
  d.Ldata.Sdata = append(d.Ldata.Sdata, Sdata_tp{d.Setno, d.Sdata.Rdata})
  d.Ssegm.Rsegm = append(d.Ssegm.Rsegm, d.Rsegm)
  d.Lsegm.Ssegm = append(d.Lsegm.Ssegm, Ssegm_tp{d.Setno,
    d.Lctrl.Rctrl[d.Setno].Field, d.Ssegm.Rsegm})
  ofnam := d.Outdr + d.Flnam + "-" + strconv.Itoa(d.Setno)
  if OUTCTRL {
    fc, _ := json.MarshalIndent(d.Lctrl, "", " ")
    _ = ioutil.WriteFile(ofnam + "-control.json", fc, 0644)
  }
  if OUTDATA {
    fd, _ := json.MarshalIndent(d.Ldata, "", " ")
    _ = ioutil.WriteFile(ofnam + "-data.json",    fd, 0644)
  }
  if OUTSEGM {
    fs, _ := json.MarshalIndent(d.Lsegm, "", " ")
    _ = ioutil.WriteFile(ofnam + "-segment.json", fs, 0644)
  }
  d.Sdata.Rdata = nil
  d.Ldata.Sdata = nil
  d.Ssegm.Rsegm = nil
  d.Lsegm.Ssegm = nil
}
*/
//******************************************************************************
// Process Control Record
//******************************************************************************
/*
const SELITEMS = `SELECT dname, strps, endps FROM items WHERE idocn=? and
  rname=? order by seqno;`

func (d *Didoc_tp) ReadControl(iline, idocn, rname string, first bool) {
  var f     Items_tp
  var rctrl Rctrl_tp
  var cdval string
  if !first {
    d.WriteOutputs()
  }
  d.Recno = 0                             // Inits at Control Record level
  d.L = -1                                //
  d.c1, d.c2, d.c3, d.c4 = -1, -1, -1, -1 //
  d.c5, d.c6, d.c7, d.c8 = -1, -1, -1, -1 //
  d.Setno++
  d.Recnf++
  for dbo, err := d.Db.Query(SELITEMS, idocn, rname); err == nil;
    err = dbo.Next() {
    dbo.Scan(&f.Dname, &f.Strps, &f.Endps)
    cdval = strings.TrimSpace(iline[f.Strps-1:f.Endps])
    if len(cdval) == 0 || cdval == "" {
      continue
    }
    rctrl.Field = append(rctrl.Field, Field_tp{f.Dname, cdval})
  }
  rctrl.Instn = d.Setno
  d.Lctrl.Rctrl = append(d.Lctrl.Rctrl, rctrl)
  d.AddRoot(idocn)
}

// Define root node in segment structure
func (d *Didoc_tp) AddRoot(idocn string) {
  d.Rsegm = Rsegm_tp{idocn, 0, 0, "", "", 0, nil, nil}
}
*/
//******************************************************************************
// Process Data Record
//******************************************************************************
/*
const SELITEMF = `SELECT dname, dtype, dtext, level FROM items WHERE idocn=?
  and dname=? and rname=?;`

func (d *Didoc_tp) ReadData(iline, idocn, rname string) {
  var f, g  Items_tp
  var rdata Rdata_tp
  var cdval string
  d.Recnf++
  d.Recno++
  for dbo, err := d.Db.Query(SELITEMS, idocn, rname); err == nil;
    err = dbo.Next() {
    dbo.Scan(&f.Dname, &f.Strps, &f.Endps)
    if f.Endps >= len(iline) {
      f.Endps = len(iline)
    }
    cdval = strings.TrimSpace(iline[f.Strps-1:f.Endps])
    if len(cdval) == 0 || cdval == "" {
      continue
    }
    if f.Dname == "SEGNAM" {
      dbs, err := d.Db.Query(SELITEMF, idocn, d.alias(idocn, cdval), "SEGMENT")
      if err != nil {
        log.Printf("Select ITEMS table error: %v\n", err)
      }
      log.Printf("%s %s %s\r\n", SELITEMF, idocn, d.alias(idocn, cdval))
      err = dbs.Scan(&g.Dname, &g.Dtype, &g.Dtext, &g.Level)
      if err != nil {
        log.Printf("Scan ITEMS table error: %v\n", err)
      }
      rdata.Segmn = g.Dtype
      rdata.Qualf = g.Dtext
      rdata.Level = g.Level
      rdata.Recno = d.Recno
    }
    if f.Dname == "SDATA" {
      d.ProcSegment(iline, idocn, "SGM", g.Dname, rdata.Level)
      continue
    }
    rdata.Field = append(rdata.Field, Field_tp{f.Dname, cdval})
  }
  d.Sdata.Rdata = append(d.Sdata.Rdata, rdata)
}

// Get the Segment Type from a Segment Alias
const SELALIAS = `select segtp from segma where idocn=? and segdf=?;`

func (d *Didoc_tp) alias(idocn, aname string) string {
  dname := aname
  dbo, err := d.Db.Query(SELALIAS, idocn, aname)
  if err != nil {
    return dname
  }
  dbo.Scan(&dname)
  return dname
}
*/
//******************************************************************************
// Process Segment Data
//******************************************************************************
// Determines segment Qualifier and Instance Number
/*
func (d *Didoc_tp) ProcSegment(iline, idocn, strtp, cdnam string, level int) {
  instn := -1
  ident := ""
  if level == d.L {
    instn = d.UpdateCounter(cdnam, d.L)
    ident = "SAME"
  } else if level > d.L {
    d.L = level
    d.Count[d.L] = append(d.Count[d.L], Count_tp{cdnam, 1})
    instn = d.RetrieveCounter(cdnam, d.L)
    ident = "LOWER"
  } else if level < d.L {
    goupl := d.L - level
    for i := 0;  i < goupl; i++ {
      d.Count[d.L] = nil
      d.L--
    }
    instn = d.UpdateCounter(cdnam, d.L)
    ident = "UPPER"
  }
  d.AddToStruct(iline, idocn, ident, cdnam, d.L, instn)
}
*/
// Update counter of segment with equal segment ID in the current struct level
/*
func (d *Didoc_tp) UpdateCounter(segmn string, l int) int {
  for j := 0; j < len(d.Count[l]); j++ {
    if d.Count[l][j].Segmn == segmn {
      d.Count[l][j].Instn += 1
      return d.Count[l][j].Instn
    }
  }
  d.Count[l] = append(d.Count[l], Count_tp{segmn, 1})
  return 1
}
*/
// Retrieve last counter of segment with equal segm ID in the current struct lvl
/*
func (d *Didoc_tp) RetrieveCounter(segmn string, l int) int {
  for j := 0; j < len(d.Count[l]); j++ {
    if d.Count[l][j].Segmn == segmn {
      return d.Count[l][j].Instn
    }
  }
  return -1
}
*/
// Build segment structure into an non-linked segment node
/*
func (d *Didoc_tp) AddToStruct(iline, idocn, ident, segmn string, l, instn int){
  if d.Recno <= 9999 {
  d.Sfild.Qlkey = ""
  d.Sfild.Qlval = ""
  d.Sfild.Field = nil
  d.GetSegmData(iline, idocn, "SGM", segmn, l)
  if l == 1 {
    d.Rsegm.Child = append(d.Rsegm.Child, Rsegm_tp{
      segmn, d.Recno, l, d.Sfild.Qlkey, d.Sfild.Qlval, instn,
      d.Sfild.Field, nil})
    d.c2, d.c3, d.c4, d.c5, d.c6, d.c7, d.c8 = -1, -1, -1, -1, -1, -1, -1
    d.c1++
  } else if l == 2 {
    d.Rsegm.Child[d.c1].Child = append(
      d.Rsegm.Child[d.c1].Child, Rsegm_tp{
      segmn, d.Recno, l, d.Sfild.Qlkey, d.Sfild.Qlval, instn,
      d.Sfild.Field, nil})
    d.c3, d.c4, d.c5, d.c6, d.c7, d.c8 = -1, -1, -1, -1, -1, -1
    d.c2++
  } else if l == 3 {
    d.Rsegm.Child[d.c1].Child[d.c2].Child = append(
      d.Rsegm.Child[d.c1].Child[d.c2].Child, Rsegm_tp{
      segmn, d.Recno, l, d.Sfild.Qlkey, d.Sfild.Qlval, instn,
      d.Sfild.Field, nil})
    d.c4, d.c5, d.c6, d.c7, d.c8 = -1, -1, -1, -1, -1
    d.c3++
  } else if l == 4 {
    d.Rsegm.Child[d.c1].Child[d.c2].Child[d.c3].Child = append(
      d.Rsegm.Child[d.c1].Child[d.c2].Child[d.c3].Child, Rsegm_tp{
      segmn, d.Recno, l, d.Sfild.Qlkey, d.Sfild.Qlval, instn,
      d.Sfild.Field, nil})
    d.c5, d.c6, d.c7, d.c8 = -1, -1, -1, -1
    d.c4++
  } else if l == 5 {
    d.Rsegm.Child[d.c1].Child[d.c2].Child[d.c3].Child[d.c4].Child = append(
      d.Rsegm.Child[d.c1].Child[d.c2].Child[d.c3].Child[d.c4].Child, Rsegm_tp{
      segmn, d.Recno, l, d.Sfild.Qlkey, d.Sfild.Qlval, instn,
      d.Sfild.Field, nil})
    d.c6, d.c7, d.c8 = -1, -1, -1
    d.c5++
  } else if l == 6 {
    d.Rsegm.Child[d.c1].Child[d.c2].Child[d.c3].Child[d.c4].Child[d.c5].
      Child = append(
      d.Rsegm.Child[d.c1].Child[d.c2].Child[d.c3].Child[d.c4].Child[d.c5].
      Child, Rsegm_tp{
      segmn, d.Recno, l, d.Sfild.Qlkey, d.Sfild.Qlval, instn,
      d.Sfild.Field, nil})
    d.c7, d.c8 = -1, -1
    d.c6++
  } else if l == 7 {
    d.Rsegm.Child[d.c1].Child[d.c2].Child[d.c3].Child[d.c4].Child[d.c5].
      Child[d.c6].Child = append(
      d.Rsegm.Child[d.c1].Child[d.c2].Child[d.c3].Child[d.c4].Child[d.c5].
      Child[d.c6].Child, Rsegm_tp{
      segmn, d.Recno, l, d.Sfild.Qlkey, d.Sfild.Qlval, instn,
      d.Sfild.Field, nil})
    d.c8 = -1
    d.c7++
  } else if l == 8 {
    d.Rsegm.Child[d.c1].Child[d.c2].Child[d.c3].Child[d.c4].Child[d.c5].
      Child[d.c6].Child[d.c7].Child = append(
      d.Rsegm.Child[d.c1].Child[d.c2].Child[d.c3].Child[d.c4].Child[d.c5].
      Child[d.c6].Child[d.c7].Child, Rsegm_tp{
      segmn, d.Recno, l, d.Sfild.Qlkey, d.Sfild.Qlval, instn,
      d.Sfild.Field, nil})
    d.c8++
  } else if l == 9 {
    d.Rsegm.Child[d.c1].Child[d.c2].Child[d.c3].Child[d.c4].Child[d.c5].
      Child[d.c6].Child[d.c7].Child[d.c8].Child = append(
      d.Rsegm.Child[d.c1].Child[d.c2].Child[d.c3].Child[d.c4].Child[d.c5].
      Child[d.c6].Child[d.c7].Child[d.c8].Child, Rsegm_tp{
      segmn, d.Recno, l, d.Sfild.Qlkey, d.Sfild.Qlval, instn,
      d.Sfild.Field, nil})
  }
  }
}
*/
//Get field values of a segment into the IDOC structure
/*
const SELSTRUC = `SELECT a.idocn, a.level, a.pseqn, a.pdnam, a.pdtyp, a.pdqlf,
  a.cseqn, a.cdnam, a.cdtyp, a.cdqlf, b.dname, b.seqno, b.strps, b.endps
  FROM struc a LEFT JOIN items b
  ON (a.idocn = b.idocn and a.cdnam = b.rname)
  WHERE a.idocn=? and a.strtp=? and a.cdnam=?
  ORDER BY a.idocn, a.strtp, a.pseqn, a.prnam, a.pdnam, b.seqno;`

func (d *Didoc_tp) GetSegmData(iline, idocn, strtp, cdnam string, level int) {
  var f Items_tp
  var e Struc_tp
  var cdval string
  fitem := true
  for dbo, err := d.Db.Query(SELSTRUC, idocn, strtp, cdnam); err == nil;
    err = dbo.Next() {
    dbo.Scan( &e.Idocn, &e.Level, &e.Pseqn, &e.Pdnam, &e.Pdtyp, &e.Pdqlf,
      &e.Cseqn, &e.Cdnam, &e.Cdtyp, &e.Cdqlf, &f.Dname, &f.Seqno, &f.Strps,
      &f.Endps)
    if f.Endps >= len(iline) {
      break
    }
    cdval = strings.TrimSpace(iline[f.Strps-1:f.Endps])
    if len(cdval) == 0 || cdval == "" {
      continue
    }
    if fitem {
      d.Sfild.Segmn = e.Cdtyp
      d.Sfild.Recno = d.Recno
      d.Sfild.Level = e.Level
      if e.Cdqlf == "QUAL" {
        d.Sfild.Qlkey = f.Dname
        d.Sfild.Qlval = cdval
      } else {
        d.Sfild.Qlkey = ""
        d.Sfild.Qlval = ""
      }
      fitem = false
    }
    d.Sfild.Field = append(d.Sfild.Field, Field_tp{f.Dname, cdval})
  }
}
*/