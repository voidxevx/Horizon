use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::os::raw::c_char;
use std::ffi::CString;

// neblang linakge
#[allow(unused)]
unsafe extern "C"
{
    pub unsafe fn neb_init();
    pub unsafe fn neb_link_module(name: *const c_char, path: *const c_char);
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn link_file(file_path_str: &str) {
    let file_path: &Path = Path::new(file_path_str);
    if file_path.exists() && file_path.is_file() {
        let file = File::open(file_path)
            .expect("\x1b[31m[RS][Neblang] CRITICAL - Failed to link module, unable to open file.\x1b[0m");

        let reader = BufReader::new(file);
        let header_line = reader.lines().next();
        match header_line {
            Some(line) => {
                let line = line.unwrap();
                if let Some(header_type) = line.get(..7) {
                    if header_type.trim() == "module" {
                        if let Some(header_name) = line.get(7..) {
                            let name_cstring: CString = CString::new(header_name)
                                .expect("\x1b[31m[RS][Neblang] CRITICAL - Failed to link module, unable to convert module name during rust handoff");
                            let path_cstring: CString = CString::new(file_path_str)
                                .expect("\x1b[31m[RS][Neblang] CRITICAL - Failed to link module, unable to convert file path during rust handoff");
                            neb_link_module(name_cstring.as_ptr(), path_cstring.as_ptr());
                            return;
                        }
                    }
                }
                eprintln!("\x1b[33m[RS][Neblang] WARNING - Failed to link module, syntax error for module header. Expects pattern \"module <name>\"\x1b[0m");
            },
            None => eprintln!("\x1b[33m[RS][Neblang] WARNING - Failed to link module, failed to read module header")
        }
    }
    else {
        eprintln!("\x1b[33m[RS][Neblang] WARNING - Failed to link module, linked file path does not exist or is not a file.\x1b[0m")
    }
}