// next_stage.rs - (2021-07-01 bar8tl)
use crate::to_fixedsize::symbols::{INP, OUT, OKAY};
use rblib::files_infolder::FilelistTp;
use rblib::move_file_wf::move_file_wf;
use rblib::pass_filter::pass_filter;
use rblib::rename_file_wf::rename_file_wf;

pub fn next_stage(rtncd: &String, inptp: &String, inpdr: &String, outdr: &String,
  outtp: &String, pcddr: &String, fl: &FilelistTp, ifilt: &String) {
  if ifilt.len() == 0 || (ifilt.len() > 0 && pass_filter(&ifilt, &fl.flnam)) {
    if rtncd == OKAY {
      move_file_wf(INP, inpdr, pcddr, &fl.flnam, &fl.flext);
      rename_file_wf(OUT, outdr, &fl.flnam, &fl.flext);
    }
  }
}
