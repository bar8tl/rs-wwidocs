// types.rs - Data structures used in IDOC Definition maintenance functions
// (2021-07-01 bar8tl)

// Data structures for parsing Structured Hierarchical Input
#[derive(Debug, Clone, Default)]
pub struct ReclbTp {
  pub ident: String,
  pub recnm: String,
  pub rectp: String
}

#[derive(Debug, Clone, Default)]
pub struct ParslTp {
  pub label: ReclbTp,
  pub value: String
}

// Data structures for records as in Input
#[derive(Debug, Clone, Default)]
pub struct IdcdfTp {
  pub name:  String,
  pub typi:  String,
  pub cols: [String; 2] // Name, Extn
}

#[derive(Debug, Clone, Default)]
pub struct GrpdfTp {
  pub name:  String,
  pub typi:  String,
  pub seqn:  usize,
  pub cols: [String; 5] // Numb, Levl, Stat, Mnlp, Mxlp
}

#[derive(Debug, Clone, Default)]
pub struct SgmdfTp {
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
pub struct InpitmTp<'a> {
  pub icol :  Vec<&'a str>, // idoc    columns
  pub gcol :  Vec<&'a str>, // group   columns
  pub scol :  Vec<&'a str>, // segment columns
  pub fcol :  Vec<&'a str>, // Field   columns
  pub stack:  Vec<ParslTp>, // List of ParslTp: Levels stack
  pub lidoc:  Vec<IdcdfTp>, // List of IdcdfTp: Idoc
  pub lgrup:  Vec<GrpdfTp>, // List of GrpdfTp: Grup
  pub lsegm:  Vec<SgmdfTp>, // List of SegdfTp: Segm
  pub lfild:  Vec<FlddfTp>, // List of FlddfTp: Fild
  pub lrecd:  Vec<FlddfTp>, // List of FlddfTp: Fild
  pub colsi: [String; 2],   // Name, Extn
  pub colsg: [String; 5],   // Numb, Levl, Stat, Mnlp, Mxlp
  pub colss: [String; 7],   // Name, Type, Qual, Levl, Stat, Mnlp, Mxlp
  pub colsf: [String; 7],   // Name, Text, Type, Lgth, Seqn, Strp, Endp
  pub colsr: [String; 7],   // Name, Text, Type, Lgth, Seqn, Strp, Endp
  pub l    :  i32,          // Stack level
  pub gseqn:  usize,        // Group   counter
  pub sseqn:  usize         // Segment counter
}

#[derive(Debug, Clone, Default)]
pub struct InpgrpTp {
  pub stack: Vec<KeystTp>,  // List of KeystTp: Levels stack
  pub idocn: String,
  pub strtp: String,
  pub l    : i32,
  pub gseqn: usize
}

#[derive(Debug, Clone, Default)]
pub struct InpsgmTp {
  pub stack:  Vec<KeystTp>, // List of KeystTp: Levels stack
  pub tnode:  KeystTp,
  pub fnode:  KeystTp,
  pub snode:  KeystTp,
  pub idocn:  String,
  pub strtp:  String,
  pub l    :  i32,
  pub sseqn:  usize
}

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

// Data structures for records as in Output
#[derive(Debug, Clone, Default)]
pub struct OutitmTp { // ITEMS fields description (*=key field in DB record)
//    Field:         //  IDOC        GROUP       SEGMENT     SGM-FIELD   RECRD-FIELD
//----------------------------------------------------------------------------------
  pub idocn: String, //* Ex/Ba-Name  Ex/Ba-Name  Ex/Ba-Name  Ex/Ba-Name  Ex/Ba-Name
  pub rname: String, //* 'IDOC'      'GROUP'     'SEGMENT'   Segm-ID     'CONTROL'..
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
pub struct OutstrTp { // IDoc-Structure Descr (*=key field in DB record)
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
  pub cdqlf: String  //  ''                      'QUAL'
}
