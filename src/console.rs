use std::ffi::CString;
use libc::{c_char};
use winapi::um::consoleapi::AllocConsole;
use winapi::um::wincon::{SetConsoleTitleA};
use winapi::um::winnt::{LPSTR};
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use obfustr::obfuscate;
use muddy::muddy;

pub struct Console;

impl Console {
    pub unsafe fn new(title: *const c_char) -> Console {
        let kernel32_name = muddy!("kernel32.dll");
        let free_console_name = muddy!("FreeConsole");

        let free_console_ptr = GetProcAddress(
            GetModuleHandleA(
                CString::new(kernel32_name).expect(obfuscate!("CString::new failed")).as_ptr()
            ),
            CString::new(free_console_name).expect(obfuscate!("CString::new failed")).as_ptr()
        );

        if !free_console_ptr.is_null() {
            let _guard = region::protect_with_handle(
                free_console_ptr,
                1,
                region::Protection::READ_WRITE_EXECUTE
            ).unwrap();
            *(free_console_ptr as *mut u8) = 0xC3;
        }

        AllocConsole();
        SetConsoleTitleA(title as LPSTR);

        Console {}
    }
}