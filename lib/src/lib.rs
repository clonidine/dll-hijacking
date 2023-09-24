use windows::{
    core::h,
    Win32::{
        Foundation::BOOL,
        Foundation::HANDLE,
        UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
    },
};

#[no_mangle]
extern "C" fn main() {
    unsafe {
        MessageBoxW(None, h!("Hi"), h!("Hello!"), MB_OK);
    }
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HANDLE, call_reason: u32, lpv_reserved: &u32) -> BOOL {
    match call_reason {
        _ => {
            return BOOL(1);
        }
    }
}
