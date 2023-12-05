// header.rs - References to function modules being used into the program wwidocs,
// and to crates being used in main entry point (2021-07-01 bar8tl)
pub mod settings;
pub mod to_json;
mod db_creation;
mod idoc_definitn;
mod to_fixedsize;
mod run_query;

use db_creation::create_tablelist::create_tablelist;
use idoc_definitn::upld_idoc_definitn::upld_idoc_definitn;
use settings::set_pgm_settings::set_pgm_settings;
use settings::set_run_settings::*;
use to_fixedsize::flat_idocs_inbatch::flat_idocs_inbatch;
use to_json::json_idocs_inbatch::json_idocs_inbatch;
use run_query::query_idocs_inbatch::query_idocs_inbatch;

const CONFIG_FILENAME: &str = "_config.json";
