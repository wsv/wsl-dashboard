use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
use windows::core::HSTRING;

pub fn get_disk_free_space(path: &str) -> u64 {
    let mut free_bytes_available: u64 = 0;
    let mut total_number_of_bytes: u64 = 0;
    let mut total_number_of_free_bytes: u64 = 0;

    let path_hstring = HSTRING::from(path);
    unsafe {
        if GetDiskFreeSpaceExW(
            &path_hstring,
            Some(&mut free_bytes_available),
            Some(&mut total_number_of_bytes),
            Some(&mut total_number_of_free_bytes),
        ).is_ok() {
            free_bytes_available
        } else {
            0
        }
    }
}

pub fn get_c_drive_free_space() -> u64 {
    get_disk_free_space("C:\\")
}

pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    use std::process::{Command, Stdio};
    use std::io::Write;

    let mut cmd = Command::new("clip");
    cmd.stdin(Stdio::piped());
    
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = cmd.spawn()
        .map_err(|e| format!("Failed to spawn clip.exe: {}", e))?;

    let mut stdin = child.stdin.take().ok_or("Failed to open stdin for clip.exe")?;
    stdin.write_all(text.as_bytes()).map_err(|e| format!("Failed to write to clip.exe: {}", e))?;
    drop(stdin);

    let status = child.wait().map_err(|e| format!("Failed to wait for clip.exe: {}", e))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("clip.exe exited with status: {}", status))
    }
}
