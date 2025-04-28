fn main() {
    match find_section("ABCDEFGH") {
        Ok(Some(bytes)) => println!("Data: {:?}", String::from_utf8_lossy(bytes)),
        Ok(None) => println!("Not found."),
        Err(err) => println!("ERR: {:?}", err),
    }
}

use std::ffi::CString;
use windows_sys::Win32::System::LibraryLoader::{
    FindResourceA, LoadResource, LockResource, SizeofResource,
};

pub fn find_section(section_name: &str) -> std::io::Result<Option<&[u8]>> {
    let section_name = section_name.to_uppercase();
    let section_name = CString::new(section_name)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;

    unsafe {
        let current_process_hmod = 0;
        let resource_handle = FindResourceA(
            current_process_hmod,
            section_name.as_ptr() as _,
            10 as *const _,
        );
        eprintln!("RESOURCE HANDLE: {:?}", resource_handle);
        if resource_handle == 0 {
            return Ok(None);
        }

        let resource_data = LoadResource(current_process_hmod, resource_handle);
        if resource_data == 0 {
            return Err(std::io::Error::last_os_error());
        }

        let resource_size = SizeofResource(current_process_hmod, resource_handle);
        if resource_size == 0 {
            return Ok(Some(&[]));
        }

        let resource_ptr = LockResource(resource_data);
        Ok(Some(std::slice::from_raw_parts(
            resource_ptr as *const u8,
            resource_size as usize,
        )))
    }
}
