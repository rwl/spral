use std::os::raw::{c_char, c_int};

#[derive(Debug, Clone)]
pub struct EquilibOptions {
    pub array_base: usize,
    pub max_iterations: usize,
    pub tol: f64,
}

impl Default for EquilibOptions {
    fn default() -> Self {
        Self {
            array_base: 0, // C
            max_iterations: 10,
            tol: 1e-8,
        }
    }
}

impl EquilibOptions {
    fn as_raw(&self) -> spral_sys::spral_scaling_equilib_options {
        spral_sys::spral_scaling_equilib_options {
            array_base: self.array_base as c_int,
            max_iterations: self.max_iterations as c_int,
            tol: self.tol as f32,
            unused: [c_char::default(); 80],
        }
    }
}

#[derive(Debug, Clone)]
pub struct EquilibInform {
    pub flag: i32,
    pub stat: i32,
    pub iterations: usize,
}

impl EquilibInform {
    fn as_raw(&self) -> spral_sys::spral_scaling_equilib_inform {
        spral_sys::spral_scaling_equilib_inform {
            flag: self.flag,
            stat: self.stat,
            iterations: self.iterations as c_int,
            unused: [c_char::default(); 80],
        }
    }

    fn copy_raw(&mut self, raw: &spral_sys::spral_scaling_equilib_inform) {
        self.flag = raw.flag;
        self.stat = raw.stat;
        self.iterations = raw.iterations as usize;
    }
}

pub fn equilib_sym(
    n: usize,
    ptr: &[usize],
    row: &[usize],
    val: &[f64],
    scaling: &mut [f64],
    options: &EquilibOptions,
    inform: &mut EquilibInform,
) {
    let ptr_c: Vec<_> = ptr.iter().map(|&p| p as c_int).collect();
    let row_c: Vec<_> = row.iter().map(|&r| r as c_int).collect();
    let options_c = options.as_raw();
    let mut inform_c = inform.as_raw();
    unsafe {
        spral_sys::spral_scaling_equilib_sym(
            n as c_int,
            ptr_c.as_ptr(),
            row_c.as_ptr(),
            val.as_ptr(),
            scaling.as_mut_ptr(),
            &options_c,
            &mut inform_c,
        );
    }
    inform.copy_raw(&inform_c);
}

pub fn equilib_unsym(
    m: usize,
    n: usize,
    ptr: &[usize],
    row: &[usize],
    val: &[f64],
    rscaling: &mut [f64],
    cscaling: &mut [f64],
    options: &EquilibOptions,
    inform: &mut EquilibInform,
) {
    let ptr_c: Vec<_> = ptr.iter().map(|&p| p as c_int).collect();
    let row_c: Vec<_> = row.iter().map(|&r| r as c_int).collect();
    let options_c = options.as_raw();
    let mut inform_c = inform.as_raw();
    unsafe {
        spral_sys::spral_scaling_equilib_unsym(
            m as c_int,
            n as c_int,
            ptr_c.as_ptr(),
            row_c.as_ptr(),
            val.as_ptr(),
            rscaling.as_mut_ptr(),
            cscaling.as_mut_ptr(),
            &options_c,
            &mut inform_c,
        );
    }
    inform.copy_raw(&inform_c);
}
