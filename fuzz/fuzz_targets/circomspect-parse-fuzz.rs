#![no_main]
use libfuzzer_sys::fuzz_target;
use std::str;
use circomspect_parser::*;

fuzz_target!(|data: &[u8]| {
    match str::from_utf8(data) {
        Ok(in_string)=>{
            parse_definition(in_string);
        },
        Err(..)=>()
    }
});
