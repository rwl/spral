use spral_sys::{spral_random_integer, SPRAL_RANDOM_INITIAL_SEED};
use std::os::raw::c_int;

pub const INITIAL_SEED: u32 = SPRAL_RANDOM_INITIAL_SEED;

pub fn random_integer(state: &mut u32, n: usize) -> usize {
    let mut state_c = *state as c_int;
    let rv = unsafe { spral_random_integer(&mut state_c, n as c_int) as usize };
    *state = state_c as u32;
    rv
}
