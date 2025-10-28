#[macro_use]
extern crate lazy_static;

extern crate core;

use std::{thread};
use std::ffi::{CString};
use std::thread::sleep;
use std::time::Duration;
use libc::c_int;
use crate::api::{Api, Roblox};
use crate::console::Console;
use crate::execution::Execution;
use crate::scheduler::Scheduler;
use obfustr::obfuscate;
use muddy::muddy;

mod addresses;
mod api;
mod console;
mod execution;
mod scheduler;
mod lua_opcodes;

lazy_static! {
    static ref ROBLOX: Roblox = Roblox::new();
    static ref API: Api = Api::new(&ROBLOX);
    static ref SCHEDULER: Scheduler = unsafe { Scheduler::new() };
}

unsafe fn main_fn() {
    let _console = Console::new(CString::new(muddy!("Rusty")).expect(obfuscate!("CString::new failed")).as_ptr());
    (API.output)(0, CString::new(muddy!("Initializing rusty..")).expect(obfuscate!("CSTRING FAILED")).as_ptr());
    println!("{}", muddy!("No console Rusty!"));
    println!("{} {:#01x}", obfuscate!("[Rusty] Task scheduler ->"), SCHEDULER.task_scheduler);
    println!("{} {:#01x}", obfuscate!("[Rusty] Datamodel ->"), SCHEDULER.datamodel);
    println!("{} {:#01x}", obfuscate!("[Rusty] Script Context ->"), SCHEDULER.script_context);
    println!("{} {:#01x}", obfuscate!("[Rusty] Lua state ->"), SCHEDULER.get_global_luastate());

    let mut execution = Execution::new(&SCHEDULER);
    println!("{} {:p}", obfuscate!("[Rusty] EXECUTION STRUCT ID ->"), &execution);
    println!("{}", muddy!("[Rusty] Hooking.."));

    sleep(Duration::from_millis(1000));
    execution.hook_waiting_scripts_job();

    println!("{}", muddy!("[Rusty] Setting identity to 7.."));
    execution.set_identity(7);

    sleep(Duration::from_millis(1000));

    let path = std::path::Path::new(r"D:\yield.txt");
    let contents = std::fs::read_to_string(path).expect(obfuscate!("Unable to read file"));
    execution.run_script(contents.as_str());
}

#[no_mangle]
#[link_section = ".text"]
pub extern "system" fn DllMain(
    hinst_dll: winapi::shared::minwindef::HINSTANCE,
    fwd_reason: winapi::shared::minwindef::DWORD,
    _lpl_reserved: winapi::shared::minwindef::LPVOID
) -> i32 {
    const DLL_PROCESS_ATTACH: u32 = 1;
    const DLL_PROCESS_DETACH: u32 = 0;

    match fwd_reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                // Отключаем уведомления о потоках для уменьшения следов
                winapi::um::libloaderapi::DisableThreadLibraryCalls(hinst_dll);

                // Создаем поток с задержкой для обхода ранних проверок Byfron
                let thread_handle = winapi::um::processthreadsapi::CreateThread(
                    std::ptr::null_mut(),
                    0,
                    Some(thread_proc),
                    std::ptr::null_mut(),
                    winapi::um::winbase::CREATE_SUSPENDED, // Создаем приостановленный
                    std::ptr::null_mut()
                );

                if !thread_handle.is_null() {
                    winapi::um::processthreadsapi::SetThreadPriority(
                        thread_handle,
                        winapi::um::winbase::THREAD_PRIORITY_BELOW_NORMAL as c_int
                    );
                    
                    winapi::um::processthreadsapi::ResumeThread(thread_handle);
                    winapi::um::handleapi::CloseHandle(thread_handle);
                }
            }
        }
        DLL_PROCESS_DETACH => {}
        _ => {}
    }
    1
}

unsafe extern "system" fn thread_proc(_: *mut winapi::ctypes::c_void) -> u32 {
    // Увеличенная задержка для обхода начальных проверок Byfron
    sleep(Duration::from_millis(2000));

    // Проверка на отладчик (дополнительная мера предосторожности)
    if winapi::um::debugapi::IsDebuggerPresent() == 0 {
        main_fn();
    }

    0
}