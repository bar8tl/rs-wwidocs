// main.rs: Starts processes to work with SAP IDOCs [20170524-BAR8TL]
mod dbcreatn;
mod flat2jsn;
mod idocdefn;
mod sap2flat;
mod settings;
mod sgmalias;

fn main() { // Starts processes for command line options
  let mut stg = settings::SettingsTp::new_settings();
  stg.set_settings("_config.json");
  let t = stg.clone();
  for parm in t.prm.cmdpr {
    let s = stg.clone();
           if parm.optn == "cdb" { // Create reference IDoc-definition database
      dbcreatn::crt_tables(parm, s);
    } else if parm.optn == "upl" { // Read and upload IDoc-definition files
      idocdefn::upld_idocdefn(parm, s);
    } else if parm.optn == "usa" { // Upload segment-definition alias names
      sgmalias::upld_segmalias(parm, s);
    } else if parm.optn == "cnv" { // Convert IDOC-data parser-fmt SAP->Flat-TXT
      sap2flat::convert_idoc2flat(parm, s);
    } else if parm.optn == "fmt" { // Convert IDOC-data Flat-TXT->Intern Struct
      flat2jsn::convert_flat2json(parm, s);
    } else {
      println!("Run option not valid");
    }
  }
}
