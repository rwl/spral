use std::os::raw::c_int;
use std::ptr::null_mut;

pub const NONSINGULAR: u32 = spral_sys::SPRAL_RANDOM_MATRIX_NONSINGULAR;
pub const SORT: u32 = spral_sys::SPRAL_RANDOM_MATRIX_SORT;

pub fn random_matrix_generate(
    state: &mut u32,
    matrix_type: spral_sys::spral_matrix_type,
    m: usize,
    n: usize,
    nnz: usize,
    ptr: &mut [usize],
    row: &mut [usize],
    val: Option<&mut [f64]>,
    flags: u32,
) {
    let mut state_c = *state as c_int;
    let mut ptr_c: Vec<_> = ptr.iter().map(|&p| p as c_int).collect();
    let mut row_c: Vec<_> = row.iter().map(|&r| r as c_int).collect();
    unsafe {
        spral_sys::spral_random_matrix_generate(
            &mut state_c,
            matrix_type,
            m as c_int,
            n as c_int,
            nnz as c_int,
            ptr_c.as_mut_ptr(),
            row_c.as_mut_ptr(),
            if let Some(val) = val {
                val.as_mut_ptr()
            } else {
                null_mut()
            },
            flags as c_int,
        );
    }
    *state = state_c as u32;
}
