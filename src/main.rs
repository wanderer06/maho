mod win32;

fn main() {
    unsafe {
        let h_instance = win32::GetModuleHandleW(std::ptr::null_mut());
        let class_name = str_to_lpcwstr("Hello world?");

        let wnd_class = win32::tagWNDCLASSW {
            style: 0,
            lpfnWndProc: None,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: std::ptr::null_mut(),
            hCursor: std::ptr::null_mut(),
            hbrBackground: std::ptr::null_mut(),
            lpszMenuName: std::ptr::null_mut(),
            lpszClassName: class_name,
        };

        println!("This is your window class: {:?}", wnd_class);
    }
}

fn str_to_lpcwstr(text: &'static str) -> win32::LPCWSTR {
    text.encode_utf16().collect::<Vec<u16>>().as_ptr()
}

#[allow(dead_code)]
fn str_to_lpwstr(text: &'static str) -> *mut u16 {
    let mut lpwstr = text.encode_utf16().collect::<Vec<u16>>();
    lpwstr.push(0);
    lpwstr.as_mut_ptr()
}
