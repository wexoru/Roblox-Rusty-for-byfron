// API

use std::arch::asm;
use std::arch::naked_asm;
use std::os::raw::c_char;
use std::ptr::{copy_nonoverlapping, null};
use libc::{c_void, memcpy, size_t, uintptr_t};
use region::protect_with_handle;
use winapi::um::libloaderapi::GetModuleHandleA;
use muddy::{muddy};
use obfustr::obfuscate;
use std::mem::transmute;



use crate::addresses::{offsets};
use crate::addresses::addresses::*;
use super::addresses::func_defs;

pub struct Roblox {
    pub(crate) handle: uintptr_t,

    pub(crate) get_scheduler_addy: uintptr_t,
    pub(crate) get_output_addy: uintptr_t,
    pub(crate) get_state_addy: uintptr_t,
    pub(crate) pushvfstring_addy: uintptr_t,

    pub(crate) spawn_func_addy: uintptr_t,
    pub(crate) deserializer_func_addy: uintptr_t,

    pub(crate) pushcclosure_addy: uintptr_t,
    pub(crate) pushcclosure_exit_addy: uintptr_t,
    pub(crate) setglobal_addy: uintptr_t,

    pub(crate) setglobal_exit_addy: uintptr_t,
    pub(crate) setglobal_path_1_addy: uintptr_t,
    pub(crate) setglobal_path_2_addy: uintptr_t,

    pub(crate) pseudo2adr_addy: uintptr_t,
    pub(crate) fake_ret_addy: uintptr_t,

    pub(crate) callcheck_addy_data: uintptr_t,
    pub(crate) callcheck_addy_code: uintptr_t,
    pub(crate) callcheck_addy_vm: uintptr_t,
    pub(crate) xor_const: uintptr_t,


    pub(crate) patch_spot: uintptr_t,
    pub(crate) backup: i32,
    pub(crate) sec_backup: i32,
    pub(crate) old_val: i32
}

impl Roblox {
    pub fn new() -> Roblox {
        let base;
        unsafe {
            base = GetModuleHandleA(null()) as uintptr_t;
        }

        Roblox {
            handle: base,
            get_scheduler_addy: base + RBX_GETSCHEDULER_ADDY,
            get_output_addy: base + RBX_OUTPUT_ADDY,
            get_state_addy: base + RBX_GETSTATE_ADDY,
            pushvfstring_addy: base + RBX_PUSHVFSTRING_ADDY,
            spawn_func_addy: base + SPAWN_FUNC_ADDY,
            deserializer_func_addy: base + DESERIALIZER_FUNC_ADDY,
            pushcclosure_addy: base + PUSHCCLOSURE_ADDY,
            pushcclosure_exit_addy: base + PUSHCCLOSURE_EXIT_ADDY,
            setglobal_addy: base + SETGLOBAL_ADDY,
            setglobal_exit_addy: base + SETGLOBAL_EXIT_ADDY,
            setglobal_path_1_addy: base + SETGLOBAL_PATH_1_ADDY,
            setglobal_path_2_addy: base + SETGLOBAL_PATCH_2_ADDY,
            pseudo2adr_addy: base + PSEUDO2ADR_ADDY,
            fake_ret_addy: base + FAKE_RET_ADDY,
            callcheck_addy_data: base + CALLCHECK_ADDY_DATA,
            callcheck_addy_code: base + CALLCHECK_ADDY_CODE,
            callcheck_addy_vm: base + CALLCHECK_ADDY_VM,
            xor_const: base + XOR_CONST,

            patch_spot: base + PATCH_SPOT,
            backup: 0,
            sec_backup: 0,
            old_val: 0
        }
    }
    // for spvwn
    unsafe extern "C" fn sexual3ty() {
        asm!(
        "xor eax, eax",
        "pop edi",
        "pop esi",
        "pop ebx",
        "mov esp, ebp",
        "pop ebp",
        "add esp, 12",
        "popad",
        "jmp edi",
        options(noreturn)
        );
    }
    // check for stdcall constraints
    pub(crate) unsafe fn spawn(&self, r1: uintptr_t) {
        let mut shellcode: [u8; 6] = [0x68, 0xEF, 0xBE, 0xAD, 0xDE, 0xC3];
        let mut original = [0u8; 6];

        *((shellcode.as_mut_ptr().offset(1)) as *mut usize) = Self::sexual3ty as *const () as usize;

        // Используем VirtualProtect вместо region для большей совместимости
        let mut old_protect: u32 = 0;
        winapi::um::memoryapi::VirtualProtect(
            self.patch_spot as *mut winapi::ctypes::c_void,
            6,
            winapi::um::winnt::PAGE_EXECUTE_READWRITE,
            &mut old_protect
        );

        copy_nonoverlapping(self.patch_spot as *mut c_void, original.as_mut_ptr() as *mut c_void, 6);
        copy_nonoverlapping(shellcode.as_mut_ptr() as *mut c_void, self.patch_spot as *mut c_void, 6);

        // Flush instruction cache для предотвращения проблем с кэшем
        winapi::um::processthreadsapi::FlushInstructionCache(
            winapi::um::processthreadsapi::GetCurrentProcess(),
            self.patch_spot as *const winapi::ctypes::c_void,
            6
        );

        self.spawn_internal(r1);

        copy_nonoverlapping(original.as_mut_ptr() as *mut c_void, self.patch_spot as *mut c_void, 6);

        // Восстанавливаем защиту памяти
        winapi::um::memoryapi::VirtualProtect(
            self.patch_spot as *mut winapi::ctypes::c_void,
            6,
            old_protect,
            &mut old_protect
        );
    }
    // FOR SPvWN
    #[inline(never)]
    unsafe fn spawn_internal(&self, luastate: uintptr_t) {
        #[cfg(target_pointer_width = "32")]
        {
            asm!(
            "push edi",
            "lea edi, 2f",
            "pushad",
            "push {luastate}",
            "push {luastate}",
            "push {patch_spot}",
            "mov eax, {rbx_spawn_addy}",
            "push ebp",
            "mov ebp, esp",
            "sub esp, 0x1C",
            "mov [esp], ebp",
            "jmp eax",
            "2:",
            "pop edi",
            luastate = in(reg) luastate,
            patch_spot = in(reg) self.patch_spot,
            rbx_spawn_addy = in(reg) self.spawn_func_addy
            );
        }

        #[cfg(target_pointer_width = "64")]
        {
            asm!(
            "sub rsp, 40",
            "mov rcx, {luastate}",
            "mov rdx, {luastate}",
            "mov rax, {rbx_spawn_addy}",
            "call rax",
            "add rsp, 40",
            luastate = in(reg) luastate,
            rbx_spawn_addy = in(reg) self.spawn_func_addy,
            out("rax") _,
            out("rcx") _,
            out("rdx") _,
            out("r8") _,
            out("r9") _,
            out("r10") _,
            out("r11") _,
            );
        }
    }

    pub(crate) unsafe fn other_spawn(&self, r1: uintptr_t) {
        asm!(
            "lea edi, 9f",
            "push {r1}",
            "push {fake_ret_addy}",
            "jmp {spawn_func_addy}",
        "9:",
            "add esp, 4",
        r1 = in(reg) r1,
        fake_ret_addy = in(reg) self.fake_ret_addy,
        spawn_func_addy = in(reg) self.spawn_func_addy
        );
    }

    // maybe needs c_char (need to test) (also check fo stdcall constraints)
    // В impl Roblox
    #[inline(never)]
    pub(crate) fn deserialize(&self, luastate: uintptr_t, chunk_name: *const c_char, bytecode: *const c_char, bytecode_len: i32) {
        unsafe {
            #[cfg(target_pointer_width = "32")]
            {
                asm!(
                "push 0",
                "push {byte_len}",
                "push {bytecode}",
                "push {fake_ret_addy}",
                "lea edi, 2f",
                "mov ecx, {r1}",
                "mov eax, {deserializer_func_addy}",
                "mov edx, {chunk_name}",
                "jmp eax",
                "2:",
                "add esp, 12",
                r1 = in(reg) luastate,
                fake_ret_addy = in(reg) self.fake_ret_addy,
                deserializer_func_addy = in(reg) self.deserializer_func_addy,
                chunk_name = in(reg) chunk_name,
                byte_len = in(reg) bytecode_len,
                bytecode = in(reg) bytecode,
                );
            }

            #[cfg(target_pointer_width = "64")]
            {
                // Windows x64 calling convention:
                // RCX = первый аргумент (luastate)
                // RDX = второй аргумент (chunk_name)
                // R8 = третий аргумент (bytecode)
                // R9 = четвертый аргумент (bytecode_len)
                // Пятый параметр (0) передается через стек
                asm!(
                "push 0",
                "sub rsp, 32",      // shadow space для Windows x64
                "mov rcx, {r1}",
                "mov rdx, {chunk_name}",
                "mov r8, {bytecode}",
                "mov r9, {byte_len}",
                "mov rax, {deserializer_func_addy}",
                "call rax",
                "add rsp, 40",      // очистка стека (32 + 8)
                r1 = in(reg) luastate,
                deserializer_func_addy = in(reg) self.deserializer_func_addy,
                chunk_name = in(reg) chunk_name,
                byte_len = in(reg) bytecode_len,
                bytecode = in(reg) bytecode,
                out("rax") _,
                out("rcx") _,
                out("rdx") _,
                out("r8") _,
                out("r9") _,
                out("r10") _,
                out("r11") _,
                );
            }
        }
    }

    pub(crate) fn decrement_top(r1: uintptr_t, amount: i32) {
        unsafe { *((r1 + offsets::luastate::TOP) as *mut uintptr_t) -= (16 * amount) as usize; }
    }

    pub(crate) fn set_identity(r1: uintptr_t, identity: i8) {
        unsafe { *((*((r1 + offsets::identity::EXTRA_SPACE) as *const uintptr_t) + offsets::identity::IDENTITY) as *mut i8) = identity;}
    }

    fn get_top(r1: uintptr_t) -> u32 {
        unsafe { *((r1 + offsets::luastate::TOP) as *const u32) - *((r1 + offsets::luastate::BASE) as *const u32) >> 4 }
    }

    fn decrypt_func(func: uintptr_t) -> uintptr_t {
        unsafe { *((func + offsets::luafunc::FUNC) as *const uintptr_t) + (func + offsets::luafunc::FUNC)}
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse4.2")] // Intel, AMD CPUs
    unsafe fn push_number(r1: uintptr_t, num: f64) {
        let a = std::arch::x86_64::_mm_load_sd(&num);

        let b = std::arch::x86_64::_mm_load_pd(XOR_CONST as *const f64);

        let result = std::arch::x86_64::_mm_xor_pd(a, b);

        let finish = std::arch::x86_64::_mm_cvtsd_f64(result);

        *(*((r1 + offsets::luastate::TOP) as *const uintptr_t) as *mut f64) = finish;

        *((*((r1 + offsets::luastate::TOP) as *const uintptr_t) + 12) as *mut i32) = 2;

        *((r1 + offsets::luastate::TOP) as *mut uintptr_t) += 16;
    }

    unsafe extern "cdecl" fn custom_func_handler(r1: uintptr_t, pseudo2: extern "fastcall" fn(rl: uintptr_t, idx: i32) -> *const uintptr_t) -> i32 {
        let func: uintptr_t = *((pseudo2)(r1, -10003));
        let func2: extern "cdecl" fn(r1: uintptr_t) -> i32 = std::mem::transmute((Roblox::decrypt_func(func)) as *const ());
        return func2(r1);
    }

    // needs to (possibly) be naked (maybe I need to find stable equivalent)
    // I can possibly jump to the end of the assembly code and let the natural ret execute (covering the undefined behaviour)
    // only problem is prologue (need to test if current impl works)
    unsafe extern "stdcall" fn custom_func_proxy(&mut self, api: Api) {
        asm!(
        "mov {BACKUP} ,eax",
        "pop eax", // (this is the stored ebp)
        "mov {SEC_BACKUP}, eax", // second backup (goes further back due to prologue)
        "pop eax",
        "cmp eax, {callcheck_vm}",
        "push eax",
        "mov eax, {SEC_BACKUP}",
        "je <1>",
        "push {old_val}",
        "jmp <2>", // here we jump to func ret (so this is equivalent to ret)
        "<1>:",
        "push {pseudo2}", // pushing pseudo2 address on stack for custom_func_handler (cdecl cc, as defined above)
        "push {custom_func_handler}",
        "<2>:",
        "push {BACKUP}", // restore ebp
        BACKUP = inout(reg) self.backup,
        SEC_BACKUP = inout(reg) self.sec_backup,
        old_val = inout(reg) self.old_val,
        callcheck_vm = in(reg) self.callcheck_addy_vm,
        custom_func_handler = in(reg) (Roblox::custom_func_handler as *const i32 as i32),
        pseudo2 = in(reg) api.pseudo2 as *const i32 as i32
        )
    }
}

// I can do other functions later, basic execution can work with this ^^


pub struct Api { // possible creation in scheduler
    pub(crate) get_scheduler: func_defs::RbxGetschedulerT,
    pub(crate) get_state: func_defs::RbxGetStateT,
    pub(crate) output: func_defs::RbxOutputT,
    pub(crate) pushvfstring: func_defs::RbxPushvfstringT,
    pub(crate) pseudo2: func_defs::RbxPseudo2adrT,
}

impl Api {
    pub fn new(roblox: &Roblox) -> Api { // macro for this would be nice
        Api { // need to use all powerful transmute (might need to get correct memory layout tho by casting to appropriate data struct)
            get_scheduler: unsafe { std::mem::transmute(roblox.get_scheduler_addy as *const ())},
            get_state: unsafe { std::mem::transmute(roblox.get_state_addy as *const ())},
            output:  unsafe { std::mem::transmute(roblox.get_output_addy as *const ())},
            pushvfstring:  unsafe { std::mem::transmute(roblox.pushvfstring_addy as *const ())},
            pseudo2:  unsafe { std::mem::transmute(roblox.pseudo2adr_addy as *const ())}
        }
    }
}

pub struct ReplacerT {
    addy: uintptr_t,
    stolen_len: Option<size_t>, // Maybe use usize
    stolen: Option<Vec<u8>> //byte ( might put this as option)
}

impl ReplacerT {
    fn new(addy: uintptr_t) -> ReplacerT {
        ReplacerT {
            addy,
            stolen_len: None,
            stolen: None
        }
    }
    // consider transmute casting (unsafe tho)
    unsafe fn write(&mut self, mem: *const c_void, size: size_t ) {
        self.stolen_len = Option::from(size);
        self.stolen = Option::from(vec![0 as u8; size]);

        let stolen_bytes = self.stolen.as_mut().unwrap().as_mut_ptr();
        let address = self.addy.to_be_bytes().as_mut_ptr(); // maybe theres something wrong here (possibly fixed now as a pointer)

        let _guard = protect_with_handle(self.addy as *mut i32, size, region::Protection::READ_WRITE_EXECUTE);

        copy_nonoverlapping(address, stolen_bytes, size);
        memcpy(self.addy as *mut c_void, mem, size); // might change to copy_nooverlapping too
    }

    unsafe fn revert(&mut self) {

        let stolen_bytes = self.stolen.as_ref().unwrap().as_ptr();
        let address = self.addy.to_be_bytes().as_mut_ptr();

        let _guard = protect_with_handle(self.addy as *mut i32, self.stolen_len.unwrap(), region::Protection::READ_WRITE_EXECUTE);

        copy_nonoverlapping(stolen_bytes,  address, self.stolen_len.unwrap());
    }
}

impl Drop for ReplacerT { // destructor
    fn drop(&mut self) {
        drop(self.stolen.as_ref());
    }
}
