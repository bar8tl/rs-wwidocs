// mod.rs - Function modules declaration for IDOC file conversion to flat fixed
// size format (2021-07-01 bar8tl)
pub mod flat_idocs_inbatch;
mod append_field_tocntrl;
mod append_field_tosegmt;
mod build_edidc_line;
mod build_edidd_line;
mod build_edids_line;
mod flat_idocdata;
mod get_idoc_properties;
mod prep_sectn_header;
mod prep_segmt_header;
mod symbols;
mod types;
mod write_cntrl_line;
mod write_segmt_line;
