use windows::{
    core::s,
    Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA},
};

fn main() {
    unsafe {
        let dll = LoadLibraryA(s!("lib.dll"))
            .map_err(|e| panic!("Can't find DLL: {}", e))
            .unwrap();

        let dll_main = GetProcAddress(dll, s!("main"));

        match dll_main {
            Some(dll_main) => {
                (dll_main());
            }
            None => panic!("Can't find main function in DLL."),
        }
    }
}
