// mod.rs - Function modules declaration for IDOC file conversion to JSON
// hierarchical format (2021-07-01 bar8tl)
pub mod json_content;
pub mod symbols;
pub mod types;
mod add_tostruct;
mod calc_segmt_counters;
mod format_cntrl_record;
mod format_data_record;
mod get_segmt_fields;
mod json_content_inbatch;
mod json_content_onefile;
mod write_json_file;
