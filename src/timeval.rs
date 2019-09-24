// ATSPI FFI needs a timeval implementation with Debug implemented
//

extern crate nix;

pub type timeval = nix::sys::time::TimeVal;
