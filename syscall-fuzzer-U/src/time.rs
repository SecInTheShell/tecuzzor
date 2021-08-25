//! Fuzz syscalls related to Time/Timing System  

use super::*;
use dec_macro::{call, testcall, type_of};
use fuzzer_types::calls::*;
use fuzzer_types::utils::*;
use rand::rngs::StdRng;

pub fn time_test(gen: &mut StdRng) {
    for _ in 0..REPEAT {

        // nanosleep
        let (nanosleep, _res) = testcall!(Nanosleep, gen);
        println!("---- after {}: {}", type_of(&nanosleep), serde_json::to_string(&nanosleep).unwrap());

    }
}
