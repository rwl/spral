use std::os::raw::{c_char, c_int};

#[derive(Default, Debug, Clone)]
pub struct HungarianOptions {
    pub array_base: usize, // Not in Fortran type
    pub scale_if_singular: bool,
}

impl HungarianOptions {
    fn as_raw(&self) -> spral_sys::spral_scaling_hungarian_options {
        spral_sys::spral_scaling_hungarian_options {
            array_base: self.array_base as c_int,
            scale_if_singular: self.scale_if_singular,
            unused: [c_char::default(); 80],
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct HungarianInform {
    pub flag: i32,
    pub stat: i32,
    pub matched: usize,
}

impl HungarianInform {
    fn as_raw(&self) -> spral_sys::spral_scaling_hungarian_inform {
        spral_sys::spral_scaling_hungarian_inform {
            flag: self.flag,
            stat: self.stat,
            matched: self.matched as c_int,
            unused: [c_char::default(); 80],
        }
    }

    fn copy_raw(&mut self, raw: &spral_sys::spral_scaling_hungarian_inform) {
        self.flag = raw.flag;
        self.stat = raw.stat;
        self.matched = raw.matched as usize;
    }
}

pub fn hungarian_sym(
    n: usize,
    ptr: &[usize],
    row: &[usize],
    val: &[f64],
    scaling: &mut [f64],
    match_: &mut [usize],
    options: &HungarianOptions,
    inform: &mut HungarianInform,
) {
    let ptr_c: Vec<_> = ptr.iter().map(|&p| p as c_int).collect();
    let row_c: Vec<_> = row.iter().map(|&r| r as c_int).collect();
    let mut match_c: Vec<_> = match_.iter().map(|&m| m as c_int).collect();
    let mut inform_c = inform.as_raw();
    unsafe {
        spral_sys::spral_scaling_hungarian_sym(
            n as c_int,
            ptr_c.as_ptr(),
            row_c.as_ptr(),
            val.as_ptr(),
            scaling.as_mut_ptr(),
            match_c.as_mut_ptr(),
            &options.as_raw(),
            &mut inform_c,
        )
    }
    inform.copy_raw(&inform_c);
}

pub fn hungarian_unsym(
    m: usize,
    n: usize,
    ptr: &[usize],
    row: &[usize],
    val: &[f64],
    rscaling: &mut [f64],
    cscaling: &mut [f64],
    match_: &mut [usize],
    options: &HungarianOptions,
    inform: &mut HungarianInform,
) {
    let ptr_c: Vec<_> = ptr.iter().map(|&p| p as c_int).collect();
    let row_c: Vec<_> = row.iter().map(|&r| r as c_int).collect();
    let mut match_c: Vec<_> = match_.iter().map(|&m| m as c_int).collect();
    let mut inform_c = inform.as_raw();
    unsafe {
        spral_sys::spral_scaling_hungarian_unsym(
            m as c_int,
            n as c_int,
            ptr_c.as_ptr(),
            row_c.as_ptr(),
            val.as_ptr(),
            rscaling.as_mut_ptr(),
            cscaling.as_mut_ptr(),
            match_c.as_mut_ptr(),
            &options.as_raw(),
            &mut inform_c,
        )
    }
    inform.copy_raw(&inform_c);
}
