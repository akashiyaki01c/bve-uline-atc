#![allow(non_snake_case)]
#![feature(lazy_cell)]

mod atc;
mod ato;
mod tims;
mod settings;

use std::path::PathBuf;
use std::sync::LazyLock;
use winapi::shared::minwindef::{HINSTANCE, DWORD, BOOL, TRUE};
use winapi::um::winnt::DLL_PROCESS_ATTACH;

use ::bveats_rs::*;

ats_main!(crate::atc::uline_atc::ULineATC);

static DLL_PATH: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
    let mut buffer = vec![0u8; 260];
    let len = unsafe {
        winapi::um::libloaderapi::GetModuleFileNameA(DLL_HANDLE, buffer.as_mut_ptr() as *mut i8, buffer.len() as u32)
    } as usize;

    if len == 0 {
        None
    } else {
        let path = PathBuf::from(String::from_utf8_lossy(&buffer[..len]).to_string());
        path.parent().map(|parent| parent.to_path_buf())
    }
});

static mut DLL_HANDLE: HINSTANCE = std::ptr::null_mut();

#[no_mangle]
extern "system" fn DllMain(hinst_dll: HINSTANCE, fdw_reason: DWORD, _: *mut std::ffi::c_void) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => unsafe {
            DLL_HANDLE = hinst_dll;
        },
        _ => {}
    }
    TRUE
}
