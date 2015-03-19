extern crate "mantle-sys" as ffi;

use std::mem;
use std::ptr;

fn main() {
    let gpus = unsafe {
        let mut appinfos: ffi::GR_APPLICATION_INFO = mem::zeroed();
        appinfos.apiVersion = ffi::GR_API_VERSION;

        let mut gpus = Vec::with_capacity(ffi::GR_MAX_PHYSICAL_GPUS);
        let mut gpus_count = 2;

        let result = ffi::grInitAndEnumerateGpus(&appinfos, ptr::null(), &mut gpus_count,
                                                 gpus.as_mut_ptr());
        check_result(result).unwrap();

        gpus.set_len(gpus_count as usize);
        gpus
    };

    println!("{:?}", gpus);

    println!("Hello, world!");
}

fn check_result(value: ffi::GR_RESULT) -> Result<(), String> {
    match value {
        ffi::GR_RESULT::GR_SUCCESS => Ok(()),
        c => Err(format!("{:?}", c))
    }
}

