// mod.rs - Function modules declaration for IDOC file conversion to JSON
// hierarchical format (2021-07-01 bar8tl)
pub mod json_idocs_inbatch;
pub mod symbols;
pub mod types;
mod add_tostruct;
mod calc_segmt_counters;
mod format_cntrl_record;
mod format_data_record;
mod get_segmt_fields;
mod json_idocdata;
mod write_json_file;
