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

fn main() { // Starts processes for command line wwidocs options
  let mut stg = settings::SettingsTp::new_settings();
  stg.set_settings("_config.json");
  let t = stg.clone();
  for parm in t.prm.cmdpr {
    let mut s = stg.clone();
    s.set_runvars(&parm);
           if parm.optn == "cdb" { // Create reference IDoc-definition database
      createdb::crt_tables(s);
    } else if parm.optn == "def" { // Read and upload IDoc-definition files
      definitn::upld_idocdefn(s);
    } else if parm.optn == "ali" { // Upload segment-definition alias names
      alias::upld_segmalias(s);
    } else if parm.optn == "pck" { // Convert IDOC-data parser-fmt SAP->Flat-TXT
      pack::conv_idoc2flat(s);
    } else if parm.optn == "upk" { // Convert IDOC-data Flat-TXT->Intern Struct
      unpack::conv_flat2json(s);
    } else if parm.optn == "qry" { // Perform queries over IDOC content
      query::upld_query(s);
    } else {
      println!("Run option not valid");
    }
  }
}
