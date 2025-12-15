use std::ffi::c_char;


unsafe extern "C"
{
    pub unsafe fn print_error(origin: i32, crit_type: i32, message: *const c_char, debug_only: bool, condition: bool);
    pub unsafe fn set_error_pipe(pipe: *const c_char);
    pub unsafe fn enable_debug_mode();
}

pub const NEB_ERROR_CRITICAL: i32 = 0;
pub const NEB_ERROR_WARNING: i32 = 1;
pub const NEB_ERROR_INFO: i32 = 2;

#[macro_export]
macro_rules! neberror {
    ($t:expr, $m:expr) => {
        let message: CString = CString::new($m)
            .expect("[RS][Neblang] CRITICAL - Error converting message during rust handoff.");
        print_error(1, $t, message.as_ptr(), false, true);
    };
}
#[macro_export]
macro_rules! neberror_debug {
    ($t:expr, $m:expr) => {
        let message: CString = CString::new($m)
            .expect("[RS][Neblang] CRITICAL - Error converting message during rust handoff.");
        print_error(1, $t, message.as_ptr(), true, true);
    };
}
#[macro_export]
macro_rules! neberror_assertion {
    ($c:expr, $t:expr, $m:expr) => {
        let message: CString = CString::new($m)
            .expect("[RS][Neblang] CRITICAL - Error converting message during rust handoff.");
        print_error(1, $t, message.as_ptr(), true, $c);
    };
}
#[macro_export]
macro_rules! set_error_pipe_message {
    ($p:expr) => {
        let pipe: CString = CString::new($p)
            .expect("[RS][Neblang] CRITICAL - Error converting pipe message during rust handoff.");
        set_error_pipe(pipe.as_ptr());
    };
}