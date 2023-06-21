//**********************************************************************************
// main.rs: Starts processes to work with SAP IDOCs [20170524-BAR8TL]              *
//**********************************************************************************
mod dbcreatn;
mod flat2jsn;
mod idocdefn;
mod sap2flat;
mod settings;
mod sgmalias;
mod wrkquery;

fn main() { // Starts processes for command line wwidocs options
  let mut stg = settings::SettingsTp::new_settings();
  stg.set_settings("_config.json");
  let t = stg.clone();
  for parm in t.prm.cmdpr {
    let mut s = stg.clone();
    s.set_runvars(&parm);
           if parm.optn == "cdb" { // Create reference IDoc-definition database
      dbcreatn::crt_tables(s);
    } else if parm.optn == "upl" { // Read and upload IDoc-definition files
      idocdefn::upld_idocdefn(s);
    } else if parm.optn == "usa" { // Upload segment-definition alias names
      sgmalias::upld_segmalias(s);
    } else if parm.optn == "cnv" { // Convert IDOC-data parser-fmt SAP->Flat-TXT
      sap2flat::conv_idoc2flat(s);
    } else if parm.optn == "fmt" { // Convert IDOC-data Flat-TXT->Intern Struct
      flat2jsn::conv_flat2json(s);
    } else if parm.optn == "qry" { //
      wrkquery::upld_query(s);
    } else {
      println!("Run option not valid");
    }
  }
}
