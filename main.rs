//**********************************************************************************
// main.rs : Starts processes to work with SAP IDOCs (2017-05-24 bar8tl)
//**********************************************************************************
mod alias;
mod createdb;
mod definitn;
mod pack;
mod query;
mod settings;
mod unpack;

fn main() {
  let stg = settings::SettingsTp::new_settings();
  let t = stg.clone();
  let optns: [&str; 6] = ["cdb", "def", "ali", "pck", "upk", "qry"];
  let funcs: [fn(settings::SettingsTp); 6] = [
    createdb::crt_tables,     // Create reference IDoc-definition database
    definitn::upld_idocdefn,  // Read and upload IDoc-definition files
    alias   ::upld_segmalias, // Upload segment-definition alias names
    pack    ::conv_idoc2flat, // Convert IDOC-data parser-fmt SAP->Flat-TXT
    unpack  ::conv_flat2json, // Convert IDOC-data Flat-TXT->Intern Struct
    query   ::upld_query      // Perform queries over IDOC content
  ];
  for parm in t.prm.cmdpr {
    let mut s = stg.clone();
    s.set_runvars(&parm);
    funcs[optns.iter().position(|&x| x == parm.optn).unwrap()](s);
  }
}
