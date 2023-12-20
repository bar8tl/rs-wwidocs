// header.rs - References to function modules being used into the program wwidocs,
// and references to crates being used in main entry point (2021-07-01 bar8tl)
pub mod settings;
pub mod to_json;
pub mod workflow;
mod db_creation;
mod idoc_definitn;
mod run_query;
mod to_fixedsize;

use db_creation::crea_tablelist::crea_tablelist;
use idoc_definitn::upld_definitn::upld_definitn;
use run_query::query_content::query_content;
use settings::set_pgm_settings::set_pgm_settings;
use settings::set_run_settings::*;
use to_fixedsize::flat_content::flat_content;
use to_json::json_content::json_content;

const CONFIG_FILENAME: &str = ".\\_config.json";
