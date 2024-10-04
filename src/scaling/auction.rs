use std::os::raw::{c_char, c_int};

#[derive(Debug, Clone)]
pub struct AuctionOptions {
    pub array_base: usize, // Not in Fortran type
    pub max_iterations: usize,
    pub max_unchanged: [usize; 3],
    pub min_proportion: [f64; 3],
    pub eps_initial: f64,
}

impl Default for AuctionOptions {
    fn default() -> Self {
        Self {
            array_base: 0, // C
            max_iterations: 30000,
            max_unchanged: [10, 100, 100],
            min_proportion: [0.90, 0.0, 0.0],
            eps_initial: 0.01,
        }
    }
}

impl AuctionOptions {
    fn as_raw(&self) -> spral_sys::spral_scaling_auction_options {
        spral_sys::spral_scaling_auction_options {
            array_base: self.array_base as c_int,
            max_iterations: self.max_iterations as c_int,
            max_unchanged: self.max_unchanged.map(|m| m as c_int),
            min_proportion: self.min_proportion.map(|m| m as f32),
            eps_initial: self.eps_initial as f32,
            unused: [c_char::default(); 80],
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct AuctionInform {
    pub flag: i32,
    pub stat: i32,
    pub matched: usize,
    pub iterations: usize,
    pub unmatchable: usize,
}

impl AuctionInform {
    fn as_raw(&self) -> spral_sys::spral_scaling_auction_inform {
        spral_sys::spral_scaling_auction_inform {
            flag: self.flag,
            stat: self.stat,
            matched: self.matched as c_int,
            iterations: self.iterations as c_int,
            unmatchable: self.unmatchable as c_int,
            unused: [c_char::default(); 80],
        }
    }

    fn copy_raw(&mut self, raw: &spral_sys::spral_scaling_auction_inform) {
        self.flag = raw.flag;
        self.stat = raw.stat;
        self.matched = raw.matched as usize;
        self.iterations = raw.iterations as usize;
        self.unmatchable = raw.unmatchable as usize;
    }
}

pub fn auction_sym(
    n: usize,
    ptr: &[usize],
    row: &[usize],
    val: &[f64],
    scaling: &mut [f64],
    match_: &mut [usize],
    options: &AuctionOptions,
    inform: &mut AuctionInform,
) {
    let ptr_c: Vec<_> = ptr.iter().map(|&p| p as c_int).collect();
    let row_c: Vec<_> = row.iter().map(|&r| r as c_int).collect();
    let mut match_c: Vec<_> = match_.iter().map(|&m| m as c_int).collect();
    let mut inform_c = inform.as_raw();
    unsafe {
        spral_sys::spral_scaling_auction_sym(
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

pub fn auction_unsym(
    m: usize,
    n: usize,
    ptr: &[usize],
    row: &[usize],
    val: &[f64],
    rscaling: &mut [f64],
    cscaling: &mut [f64],
    match_: &mut [usize],
    options: &AuctionOptions,
    inform: &mut AuctionInform,
) {
    let ptr_c: Vec<_> = ptr.iter().map(|&p| p as c_int).collect();
    let row_c: Vec<_> = row.iter().map(|&r| r as c_int).collect();
    let mut match_c: Vec<_> = match_.iter().map(|&m| m as c_int).collect();
    let mut inform_c = inform.as_raw();
    unsafe {
        spral_sys::spral_scaling_auction_unsym(
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
