use std::ffi::OsStr;
use std::io::{BufRead, Error};
use std::{clone, io::{self, BufReader}};
use std::path::Path;
use std::fs::{File, read_dir};
use cxx::{SharedPtr, let_cxx_string};

use crate::novascript::nova::State;

pub fn nova_load_files(mut state: SharedPtr<State>, root_path: &Path) -> io::Result<()> {
    if root_path.exists() && root_path.is_dir()
    {
        for entry_res in read_dir(root_path)? {
            let entry = entry_res?;
            let path = entry.path();
            if path.is_dir() {
                nova_load_files(state.clone(), &path);
            } else {
                nova_load_module(state.clone(), &path)?;
            }
        }
    }

    Ok(())
}

fn nova_load_module(mut state: SharedPtr<State>, module_path: &Path) -> io::Result<()> {
    // general checks to make sure the file still exists and is not a directory 
    // then get the file extention (if possible) and make sure that it is a ns file
    if module_path.exists() && 
        module_path.is_file() && 
        let Some(file_extention) = module_path.extension() &&
        file_extention == OsStr::new("ns")
    {
        let file = File::open(module_path)?;
        let module_line = BufReader::new(file).lines().next();
        match module_line {
            Some(str) => {
                let str = str?;
                // get the module tag of the file
                let module_header = str.get(..6);
                match module_header {
                    Some(header) => {
                        // make sure the tag is correct
                        if (header == "_mod: ") {
                            // get the modules name
                            let module_name = str.get(6..);
                            match module_name {
                                Some(module_name) => {
                                    let_cxx_string!(cxx_mod_name = module_name);
                                    let file_path_str = module_path.to_path_buf();
                                    let file_path_str = file_path_str.into_os_string().into_string();
                                    match file_path_str {
                                        Ok(file_path_str) => {
                                            let_cxx_string!(file_path_str = file_path_str.trim());
                                            unsafe 
                                            {
                                                // link the module
                                                state.pin_mut_unchecked().linkModule(&cxx_mod_name, &file_path_str);
                                                println!("linked module: {:?}, from path: {:?}", cxx_mod_name, file_path_str);
                                            }
                                        }
                                        Err(_) => (),
                                    }
                                }
                                None => (),
                            }
                        }
                    }
                    None => (),
                }
            }
            None => (),
        }
    }
    Ok(())
}