use std::ffi::CString;
use terminal_spinners::{SpinnerBuilder, DOTS};
use std::num::Wrapping;
use std::sync::Mutex;
use libc::{c_void, uintptr_t};
use mlua::{Compiler};
use obfustr::obfuscate;
use muddy::muddy;

use crate::{API, Roblox, ROBLOX, Scheduler, SCHEDULER};
use crate::lua_opcodes::{get_opcode_from_byte, LuauOpcode};

static mut SCRIPT_QUEUE: Vec<Vec<u8>> = vec![];
static mut ORIGINAL_FUNC: uintptr_t = 0;
static MUTEX: Mutex<i32> = Mutex::new(0);

pub struct Execution<'a> {
    pub(crate) scheduler: &'a Scheduler,
}

impl Execution<'_> {
    pub fn new(scheduler: &Scheduler) -> Execution<'_> {
        Execution { scheduler }
    }

    #[cfg(target_pointer_width = "32")]
    extern "fastcall" fn scheduler_cycle(waiting_scripts_job: uintptr_t, _fake_arg: i32, a2: i32) -> i32 {
        Self::scheduler_cycle_impl(waiting_scripts_job, a2)
    }

    #[cfg(target_pointer_width = "64")]
    extern "C" fn scheduler_cycle(waiting_scripts_job: uintptr_t, a2: i32) -> i32 {
        Self::scheduler_cycle_impl(waiting_scripts_job, a2)
    }

    #[inline(never)]
    fn scheduler_cycle_impl(waiting_scripts_job: uintptr_t, a2: i32) -> i32 {
        let lock = MUTEX.lock().unwrap();
        let r1 = SCHEDULER.get_global_luastate();

        unsafe {
            if !SCRIPT_QUEUE.is_empty() {
                println!("{}", obfuscate!("[Rusty] Scheduler has been stepped"));
                let mut bytecode = SCRIPT_QUEUE.pop().unwrap();

                drop(lock);

                if bytecode.get(0).unwrap().eq(&0u8) {
                    bytecode.remove(0);
                    let error_header = CString::new(muddy!("RUSTY ERROR -> ")).unwrap();
                    let error_msg = CString::from_vec_with_nul(bytecode).unwrap();

                    let error = error_header.to_str().unwrap().to_owned() + error_msg.to_str().unwrap();
                    (API.output)(1, CString::new(error).expect(obfuscate!("CString error")).as_ptr());
                } else {
                    println!("{}", muddy!("[Rusty] Script bytecode valid to execute"));

                    let bytecode_len = bytecode.len();
                    let bytecode_string = CString::from_vec_unchecked(bytecode);
                    let chunk_name = CString::new(obfuscate!("¬Rusty¬")).expect(obfuscate!("CString error"));

                    ROBLOX.deserialize(r1, chunk_name.as_ptr(), bytecode_string.as_ptr(), bytecode_len as i32);
                    println!("{}", muddy!("Deserialized"));

                    ROBLOX.other_spawn(r1);
                    println!("{}", muddy!("Deployed script"));

                    Roblox::decrement_top(r1, 1);
                }
            }
        }

        unsafe {
            #[cfg(target_pointer_width = "64")]
            {
                let function: extern "C" fn(uintptr_t, i32) -> i32 =
                    std::mem::transmute(ORIGINAL_FUNC as *const ());
                function(waiting_scripts_job, a2)
            }
            #[cfg(target_pointer_width = "32")]
            {
                let function: extern "thiscall" fn(uintptr_t, i32) -> i32 =
                    std::mem::transmute(ORIGINAL_FUNC as *const ());
                function(waiting_scripts_job, a2)
            }
        }
    }

    pub(crate) fn hook_waiting_scripts_job(&mut self) {
        #[cfg(target_pointer_width = "32")]
        let hooked_func = Self::scheduler_cycle as *mut c_void;

        #[cfg(target_pointer_width = "64")]
        let hooked_func = Self::scheduler_cycle as *mut c_void;

        unsafe {
            self.scheduler.hook_waiting_scripts_job(hooked_func, &mut ORIGINAL_FUNC, self.scheduler.task_scheduler);
            println!("{} {}", obfuscate!("[Rusty] Original function ->"), ORIGINAL_FUNC);
        }
    }

    pub(crate) fn run_script(&mut self, script: &str) {
        let spinner = SpinnerBuilder::new()
            .spinner(&DOTS)
            .text(muddy!("Compiling script..."))
            .start();

        let compiler = Compiler::new();
        let mut compiled_script = compiler.compile(script);
        let mut offset = 1;
        let num_of_constants = Self::read_var_int(&compiled_script, &mut offset);

        // Остальной код компиляции без изменений...
        for _ in 1..=num_of_constants {
            let constant_length = Self::read_var_int(&compiled_script, &mut offset);
            offset += constant_length as usize;
        }

        let num_of_functions = Self::read_var_int(&compiled_script, &mut offset);

        for _ in 1..=num_of_functions {
            offset += 4;
            let num_of_opcodes = Self::read_var_int(&compiled_script, &mut offset);
            let mut instruction_counter = offset;
            let mut byte_counter = 0;
            let mut double_inc = false;
            let counter = offset;

            loop {
                if instruction_counter == counter + num_of_opcodes as usize {
                    break;
                }

                if byte_counter == 0 {
                    if double_inc {
                        instruction_counter += 1;
                        double_inc = false;
                        byte_counter = 3;
                        offset += 1;
                        continue;
                    }

                    let byte = compiled_script.get_mut(offset).unwrap();
                    instruction_counter += 1;

                    if Self::get_op_length(get_opcode_from_byte(*byte).unwrap()) == 2 {
                        byte_counter = 4;
                        double_inc = true;
                        *byte = (Wrapping(*byte) * Wrapping(227)).0;
                        continue;
                    } else {
                        byte_counter = 4;
                    }

                    *byte = (Wrapping(*byte) * Wrapping(227)).0;
                }

                byte_counter -= 1;
                offset += 1;
            }

            offset += 3;
            let size_k = Self::read_var_int(&compiled_script, &mut offset);

            for _ in 0..size_k {
                let lbc = compiled_script.get(offset).unwrap();
                offset += 1;
                match *lbc {
                    1 => offset += 1,
                    2 => offset += 8,
                    3 => { Self::read_var_int(&compiled_script, &mut offset); },
                    4 => offset += 4,
                    5 => {
                        let keys = Self::read_var_int(&compiled_script, &mut offset);
                        for _ in 0..keys {
                            Self::read_var_int(&compiled_script, &mut offset);
                        }
                    },
                    6 => { Self::read_var_int(&compiled_script, &mut offset); },
                    _ => {}
                }
            }

            let size_p = Self::read_var_int(&compiled_script, &mut offset);
            for _ in 0..size_p {
                Self::read_var_int(&compiled_script, &mut offset);
            }

            Self::read_var_int(&compiled_script, &mut offset);
            Self::read_var_int(&compiled_script, &mut offset);

            let line_info = compiled_script.get(offset).unwrap();
            offset += 1;

            if *line_info == 1 {
                let line_ga_plog2 = compiled_script.get(offset).unwrap();
                offset += 1;
                for _ in 0..num_of_opcodes {
                    offset += 1;
                }

                let intervals = ((num_of_opcodes - 1) >> *line_ga_plog2 as u32) + 1;
                for _ in 0..intervals {
                    offset += 4;
                }
            }

            let debug_info = compiled_script.get(offset).unwrap();
            offset += 1;

            if *debug_info == 1 {
                let size_loc_vars = Self::read_var_int(&compiled_script, &mut offset);
                for _ in 0..size_loc_vars {
                    Self::read_var_int(&compiled_script, &mut offset);
                    Self::read_var_int(&compiled_script, &mut offset);
                    Self::read_var_int(&compiled_script, &mut offset);
                    Self::read_var_int(&compiled_script, &mut offset);
                }

                let size_upvalues = Self::read_var_int(&compiled_script, &mut offset);
                for _ in 0..size_upvalues {
                    Self::read_var_int(&compiled_script, &mut offset);
                }
            }
        }

        let _guard = MUTEX.lock().unwrap();
        unsafe { SCRIPT_QUEUE.push(compiled_script); }
        spinner.text(muddy!("Script compiled."));
        spinner.done();
    }

    pub fn set_identity(&self, identity: i8) {
        Roblox::set_identity(self.scheduler.get_global_luastate(), identity);
    }

    fn get_op_length(op: LuauOpcode) -> i32 {
        match op {
            LuauOpcode::LopGetglobal | LuauOpcode::LopSetglobal
            | LuauOpcode::LopGetimport | LuauOpcode::LopGettableks
            | LuauOpcode::LopSettableks | LuauOpcode::LopNamecall
            | LuauOpcode::LopJumpifeq | LuauOpcode::LopJumpifle
            | LuauOpcode::LopJumpiflt | LuauOpcode::LopJumpifnoteq
            | LuauOpcode::LopJumpifnotle | LuauOpcode::LopJumpifnotlt
            | LuauOpcode::LopNewtable | LuauOpcode::LopSetlist
            | LuauOpcode::LopForgloop | LuauOpcode::LopLoadkx
            | LuauOpcode::LopJumpifeqk | LuauOpcode::LopJumpifnoteqk
            | LuauOpcode::LopFastcall2 | LuauOpcode::LopFastcall2k => 2,
            _ => 1
        }
    }

    fn read_var_int(data: &Vec<u8>, offset: &mut usize) -> u32 {
        let mut result: u32 = 0;
        let mut shift = 0;
        let mut byte;

        loop {
            byte = *data.get(*offset).unwrap();
            result |= ((byte as u32 & 127) << shift) as u32;
            shift += 7;
            if (byte & 128) != 128 {
                break;
            }
            *offset += 1;
        }
        *offset += 1;
        result
    }
}