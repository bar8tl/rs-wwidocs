// symbols.rs - Symbolic constants for IDOC file conversion to JSON format
// (2021-07-01 bar8tl)

// Symbols
pub const OKAY    : &str = "00";
pub const RC01    : &str = "01";
pub const INP     : &str = "inp";
pub const OUT     : &str = "out";
pub const EDI_DC40: &str = "EDI_DC40";
pub const CONTROL : &str = "CONTROL";
pub const DATA    : &str = "DATA";
pub const SDATA   : &str = "SDATA";
pub const SEGNAM  : &str = "SEGNAM";
pub const SGM     : &str = "SGM";
pub const QUAL    : &str = "QUAL";
pub const SAME    : &str = "SAME";
pub const LOWER   : &str = "LOWER";
pub const UPPER   : &str = "UPPER";

// Flags
pub const OUTCTRL : bool = false;
pub const OUTDATA : bool = false;
pub const OUTSEGM : bool = true;
