use std::path::PathBuf;

use extism_pdk::*;

extern "C" {
    fn current_dir() -> PathBuf;
}

#[plugin_fn]
pub fn output(input: String) -> FnResult<String> {
    unsafe {
        let dir = current_dir().as_os_str().to_string_lossy().to_string();
        Ok(format!("The current dir is {dir}"))
    }
}
