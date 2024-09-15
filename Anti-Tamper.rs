
extern crate winapi;

use std::ptr::null_mut;
use std::ffi::CStr;
use std::process::exit;
use winapi::um::libloaderapi::GetModuleFileNameExA;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::psapi::{EnumProcessModules, GetModuleBaseNameA};
use winapi::um::libloaderapi::FreeLibrary;
use winapi::um::wintrust::{WinVerifyTrust, WINTRUST_FILE_INFO, WINTRUST_DATA, WTD_STATEACTION_VERIFY, WTD_UI_NONE};
use winapi::shared::minwindef::{HINSTANCE, DWORD};
use winapi::um::handleapi::CloseHandle;
use winapi::shared::winerror::{TRUST_E_NOSIGNATURE, ERROR_SUCCESS};

fn is_signed_dll(dll_path: &str) -> bool {
    unsafe {
        let mut wtd: WINTRUST_DATA = std::mem::zeroed();
        let mut wtf: WINTRUST_FILE_INFO = std::mem::zeroed();

        wtf.cbStruct = std::mem::size_of::<WINTRUST_FILE_INFO>() as DWORD;
        wtf.pcwszFilePath = std::ptr::null(); //Need to convert the string to wide chars if used

        wtd.cbStruct = std::mem::size_of::<WINTRUST_DATA>() as DWORD;
        wtd.dwUIChoice = WTD_UI_NONE;
        wtd.fdwRevocationChecks = 0;
        wtd.dwUnionChoice = 1; // WTD_CHOICE_FILE
        wtd.dwStateAction = WTD_STATEACTION_VERIFY;
        wtd.pFile = &mut wtf as *mut _;

        let result = WinVerifyTrust(null_mut(), &mut std::mem::zeroed(), &mut wtd);

        CloseHandle(wtd.hWVTStateData);

        result == ERROR_SUCCESS
    }
}

fn main() {
    unsafe {
        let h_process = GetCurrentProcess();
        let mut h_modules: [HINSTANCE; 1024] = [null_mut(); 1024];
        let mut cb_needed: DWORD = 0;
        if EnumProcessModules(h_process, h_modules.as_mut_ptr(), std::mem::size_of_val(&h_modules) as DWORD, &mut cb_needed) != 0 {
            let module_count = cb_needed as usize / std::mem::size_of::<HINSTANCE>();
            for i in 0..module_count {
                let mut module_name: [i8; 255] = [0; 255];
                if GetModuleBaseNameA(h_process, h_modules[i], module_name.as_mut_ptr(), module_name.len() as DWORD) > 0 {
                    let dll_name = CStr::from_ptr(module_name.as_ptr()).to_str().unwrap();

                    //Grab full pth and get sig
                    let mut dll_path: [i8; 260] = [0; 260];
                    GetModuleFileNameExA(h_process, h_modules[i], dll_path.as_mut_ptr(), dll_path.len() as DWORD);

                    let path_str = CStr::from_ptr(dll_path.as_ptr()).to_str().unwrap();

                    if !is_signed_dll(path_str) {
                        println!("{} Detected", dll_name);

                        FreeLibrary(h_modules[i]);
                        exit(1);
                    } else {
                        println!("{} Signed", dll_name);
                    }
                }
            }
        }
    }
}