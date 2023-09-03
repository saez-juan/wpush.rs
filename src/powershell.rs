use std::fs;
use std::process::Command;
use std::{fs::File, path::PathBuf};

pub struct PowerShell;
impl PowerShell {
    pub fn execute(file: String) -> bool {
        let filepath = PathBuf::from("/tmp/wpush.ps1");

        let windows_path = PowerShell::create_executable_file(filepath, file);
        if let None = windows_path {
            return false;
        }
        let windows_path = windows_path.unwrap();

        let cmd_result = Command::new("powershell.exe")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-File")
            .arg(&windows_path)
            .spawn();

        if let Err(e) = cmd_result {
            dbg!(e);
            return false;
        }

        return true;
    }

    fn create_executable_file(filepath: PathBuf, content: String) -> Option<String> {
        let tmp_file = File::create(&filepath);
        if let Err(e) = tmp_file {
            dbg!(e);
            return None;
        }

        let windows_filepath = format!(
            "\\\\wsl.localhost\\Ubuntu{}",
            filepath.to_str().unwrap().replace("/", "\\")
        );

        let bytes = content.as_bytes();
        let bom_bytes: &[u8] = &[0xef, 0xbb, 0xbf];

        let all_bytes = [bom_bytes, bytes].concat().clone();
        let all_bytes = all_bytes.as_slice();
        if let Err(e) = fs::write(filepath, all_bytes) {
            dbg!(e);
            return None;
        }

        return Some(windows_filepath);
    }
}
