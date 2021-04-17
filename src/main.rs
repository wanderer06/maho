mod bindings {
    ::windows::include_bindings!();
}

use bindings::Windows::Win32::{SystemServices, WindowsAndMessaging, MenusAndResources, Gdi};

fn main() {
    unsafe {
        // obtain instance
        let h_instance = SystemServices::GetModuleHandleW(SystemServices::PWSTR::NULL);
        let h_instance = SystemServices::HINSTANCE(h_instance);
        let class_name = SystemServices::PWSTR::from("Hello world?");
        
        // create window class
        let wnd_class = WindowsAndMessaging::WNDCLASSW {
            style: WindowsAndMessaging::WNDCLASS_STYLES(0),
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: MenusAndResources::HICON::NULL,
            hCursor: MenusAndResources::HCURSOR::NULL,
            hbrBackground: Gdi::HBRUSH::NULL,
            lpszMenuName: SystemServices::PWSTR::NULL,
            lpszClassName: class_name,
        };

        // register window class
        let id = WindowsAndMessaging::RegisterClassW(&wnd_class);

        // create window

        println!("And again, this is your window class: {:?}, id is {}", wnd_class, id);
    }
}

#[no_mangle]
extern "system" fn wnd_proc(hwnd: WindowsAndMessaging::HWND, msg: u32, wparam: WindowsAndMessaging::WPARAM, lparam: WindowsAndMessaging::LPARAM) -> SystemServices::LRESULT {
    // do some stuff here to handle msg
    unsafe { WindowsAndMessaging::DefWindowProcW(hwnd, msg, wparam, lparam) }
}

impl SystemServices::PWSTR {
    fn from(text: &'static str) -> Self {
        Self(text.encode_utf16().chain(::std::iter::once(0)).collect::<Vec<u16>>().as_mut_ptr())
    }
}


