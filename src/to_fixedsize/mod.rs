// mod.rs - Function modules declaration for IDOC file conversion to flat fixed
// size format (2021-07-01 bar8tl)
pub mod flat_content;
pub mod symbols;
mod append_field_tocntrl;
mod append_field_tosegmt;
mod build_edidc_line;
mod build_edidd_line;
mod build_edids_line;
mod flat_content_inbatch;
mod flat_content_onefile;
mod get_idoc_properties;
mod prep_sectn_header;
mod prep_segmt_header;
mod types;
mod write_cntrl_line;
mod write_segmt_line;
