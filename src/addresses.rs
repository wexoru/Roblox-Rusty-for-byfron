pub mod func_defs {
    use std::os::raw::c_char;
    use libc::uintptr_t;

    pub type RbxGetschedulerT = extern "C" fn() -> uintptr_t;

    #[cfg(target_pointer_width = "32")]
    pub type RbxOutputT = extern "fastcall" fn(output_type: i16, str: *const c_char) -> ();

    #[cfg(target_pointer_width = "64")]
    pub type RbxOutputT = extern "C" fn(output_type: i16, str: *const c_char) -> ();

    #[cfg(target_pointer_width = "32")]
    pub type RbxGetStateT = extern "thiscall" fn(SC: uintptr_t, state_type: *const i32) -> uintptr_t;

    #[cfg(target_pointer_width = "64")]
    pub type RbxGetStateT = extern "C" fn(SC: uintptr_t, state_type: *const i32) -> uintptr_t;

    pub type RbxPushvfstringT = extern "C" fn(rl: uintptr_t, fmt: *const char, ...) -> i32;

    #[cfg(target_pointer_width = "32")]
    pub type RbxPseudo2adrT = extern "fastcall" fn(rl: uintptr_t, idx: i32) -> *const uintptr_t;

    #[cfg(target_pointer_width = "64")]
    pub type RbxPseudo2adrT = extern "C" fn(rl: uintptr_t, idx: i32) -> *const uintptr_t;
}
pub mod addresses {
    use libc::uintptr_t;

    pub const HYPERION_BASE: uintptr_t = 0x2BC718;

    pub const LUAL_ARGERRORL: uintptr_t = 0x3736810;
    pub const LUAL_TYPEERRORL: uintptr_t = 0x2107800;
    pub const LUAL_FINDTABLE: uintptr_t = 0x373AC50;
    pub const LUAL_WHERE: uintptr_t = 0x37369B0;
    pub const LUAL_REGISTER: uintptr_t = 0x3739D70;
    pub const CURRFUNCNAME: uintptr_t = 0x3736790;

    pub const LUAG_RUNERRORL: uintptr_t = 0x3741470;
    pub const NEWPAGE: uintptr_t = 0x3789820;
    pub const NEWCLASSPAGE: uintptr_t = 0x37898D0;
    pub const FREECLASSPAGE: uintptr_t = 0x3789940;
    pub const NEWBLOCK: uintptr_t = 0x37899E0;
    pub const NEWGCOBLOCK: uintptr_t = 0x3789A70;
    pub const FREEBLOCK: uintptr_t = 0x3789B10;
    pub const LUAM_FREE: uintptr_t = 0x3789BC0;
    pub const LUAM_FREEGCO: uintptr_t = 0x3789C40;
    pub const LUAM_VISITGCO: uintptr_t = 0x3789D60;
    pub const LUAM_TOOBIG: uintptr_t = 0x37897F0;

    pub const CLOSE_STATE: uintptr_t = 0x37427F0;
    pub const F_LUAOPEN: uintptr_t = 0x3741AC0;
    pub const LUA_RAWCHECKSTACK: uintptr_t = 0x3734FB0;
    pub const LUAA_TOOBJECT: uintptr_t = 0x3734B60;
    pub const LUA_PUSHVFSTRING: uintptr_t = 0x3735AF0;
    pub const PSEUDO2ADDR: uintptr_t = 0x3734AA0;

    pub const LUAT_OBJTYPENAME: uintptr_t = 0x3797EF0;
    pub const AUXOPEN: uintptr_t = 0x376BAD0;
    pub const LUAD_THROW: uintptr_t = 0x3743DC0;
    pub const LUAD_RAWRUNPROTECTED: uintptr_t = 0x3743D90;

    pub const GET_CAPABILITIES: uintptr_t = 0x3AB1250;
    pub const FLOG_DATABANK: uintptr_t = 0x71205E8;
    pub const SCRIPT_CONTEXT_RESUME: uintptr_t = 0x126CE90;
    pub const APPDATA_INFO: uintptr_t = 0x73A6DF8;
    pub const OPCODE_LOOKUP_TABLE: uintptr_t = 0x5737000;
    pub const PRINT: uintptr_t = 0x18CDF60;

    pub const GET_GLOBAL_STATE: uintptr_t = 0xF5B090;
    pub const ENCRYPT_STATE: uintptr_t = 0x123B430;
    pub const RAW_SCHEDULER: uintptr_t = 0x778C0E8; // Это GetScheduler
    pub const TASK_SCHEDULER_TARGET_FPS: uintptr_t = 0x6B61118;

    pub const IMPERSONATOR: uintptr_t = 0x3AB1300;
    pub const IDENTITY_PTR: uintptr_t = 0x6B9E548;

    pub const PUSH_INSTANCE: uintptr_t = 0x12BA330;
    pub const PUSH_INSTANCE2: uintptr_t = 0x12BA380;
    pub const LUAU_EXECUTE: uintptr_t = 0x374B650;
    pub const TASK_DEFER: uintptr_t = 0x14AB8E0;

    pub const LUAD_THROW_ALT: uintptr_t = 0x3743DC0;
    pub const LUAO_NILOBJECT: uintptr_t = 0x5298F48;
    pub const LUAH_DUMMYNODE: uintptr_t = 0x5298968;
    pub const KTABLE: uintptr_t = 0x6B61230;

    pub const FIRE_MOUSE_CLICK: uintptr_t = 0x1F36E50;
    pub const FIRE_RIGHT_MOUSE_CLICK: uintptr_t = 0x1F36FF0;
    pub const FIRE_MOUSE_HOVER_ENTER: uintptr_t = 0x1F383F0;
    pub const FIRE_MOUSE_HOVER_LEAVE: uintptr_t = 0x1F38590;
    pub const FIRE_TOUCH_INTEREST: uintptr_t = 0x2244B50;

    pub const GET_PROPERTY: uintptr_t = 0xDF2E30;
    pub const FIRE_PROXIMITY_PROMPT: uintptr_t = 0x1F92090;

    pub const RBX_GETSCHEDULER_ADDY: uintptr_t = RAW_SCHEDULER;
    pub const RBX_OUTPUT_ADDY: uintptr_t = PRINT;
    pub const RBX_GETSTATE_ADDY: uintptr_t = GET_GLOBAL_STATE;
    pub const RBX_PUSHVFSTRING_ADDY: uintptr_t = LUA_PUSHVFSTRING;

    pub const SPAWN_FUNC_ADDY: uintptr_t = TASK_DEFER;
    pub const DESERIALIZER_FUNC_ADDY: uintptr_t = 0x14A8C70; // luau_load

    pub const PUSHCCLOSURE_ADDY: uintptr_t = 0x12B3750;
    pub const PUSHCCLOSURE_EXIT_ADDY: uintptr_t = 0x12B39A9;

    pub const SETGLOBAL_ADDY: uintptr_t = 0x12B3E30;
    pub const SETGLOBAL_EXIT_ADDY: uintptr_t = 0x12B3FA2;
    pub const SETGLOBAL_PATH_1_ADDY: uintptr_t = 0x014A0C78;
    pub const SETGLOBAL_PATCH_2_ADDY: uintptr_t = 0x014A1010;

    pub const PSEUDO2ADR_ADDY: uintptr_t = PSEUDO2ADDR;
    pub const FAKE_RET_ADDY: uintptr_t = 0x1365AD1;

    // Callcheck bypass addresses
    pub const CALLCHECK_ADDY_DATA: uintptr_t = 0x3996ED4;
    pub const CALLCHECK_ADDY_CODE: uintptr_t = 0x2D42A7;
    pub const CALLCHECK_ADDY_VM: uintptr_t = LUA_RAWCHECKSTACK;

    pub const XOR_CONST: uintptr_t = 0x3718790;
    pub const PATCH_SPOT: uintptr_t = 0x0045AF72;
}
pub mod offsets {

    pub mod scheduler {
        use libc::uintptr_t;

        pub(crate) const JOBS_START: uintptr_t = 0x134;
        pub(crate) const JOBS_END: uintptr_t = 0x138;
        pub(crate) const FPS: uintptr_t = 0x118;
    }

    pub mod job {
        use libc::uintptr_t;

        pub(crate) const NAME: uintptr_t = 0x10;
    }

    pub mod waiting_scripts_job {
        use libc::uintptr_t;

        pub(crate) const DATAMODEL: uintptr_t = 0x28;
        pub(crate) const SCRIPT_CONTEXT: uintptr_t = 0x130;
    }

    pub mod identity {
        use libc::uintptr_t;

        pub const EXTRA_SPACE: uintptr_t = 0x48;
        pub const IDENTITY: uintptr_t = 0x18CDF60;
    }

    pub mod luastate {
        use libc::uintptr_t;

        pub const TOP: uintptr_t = 0xC;
        pub const BASE: uintptr_t = 0x10;
    }

    pub mod luafunc {
        use libc::uintptr_t;

        pub const FUNC: uintptr_t = 16;
    }
}
