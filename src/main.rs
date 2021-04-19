mod bindings {
    ::windows::include_bindings!();
}

use bindings::Windows::Win32::{
    Direct2D, 
    DisplayDevices,
    Gdi, 
    MenusAndResources, 
    SystemServices, 
    WindowsAndMessaging, 
};

use windows::{Abi, IUnknown};

fn main() {
    unsafe {
        // obtain instance
        const CLASS_NAME_STR: &str = "Mahō"; 

        let h_instance = SystemServices::GetModuleHandleW(SystemServices::PWSTR::NULL);
        let h_instance = SystemServices::HINSTANCE(h_instance);
        let class_name = SystemServices::PWSTR::from(CLASS_NAME_STR);
        
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
        let hwnd = WindowsAndMessaging::CreateWindowExW(
            WindowsAndMessaging::WINDOW_EX_STYLE(0), 
            CLASS_NAME_STR, 
            CLASS_NAME_STR, 
            WindowsAndMessaging::WINDOW_STYLE::WS_OVERLAPPEDWINDOW, 
            WindowsAndMessaging::CW_USEDEFAULT,
            WindowsAndMessaging::CW_USEDEFAULT,
            WindowsAndMessaging::CW_USEDEFAULT,
            WindowsAndMessaging::CW_USEDEFAULT,
            WindowsAndMessaging::HWND::NULL,
            MenusAndResources::HMENU::NULL, 
            h_instance, 
            ::std::ptr::null_mut()
        );

        // show created window
        WindowsAndMessaging::ShowWindow(hwnd, WindowsAndMessaging::SHOW_WINDOW_CMD::SW_SHOWDEFAULT);

        // get client rect
        let mut rect = DisplayDevices::RECT::default();
        let client_rect = WindowsAndMessaging::GetClientRect(hwnd, &mut rect);
        println!("This is your window rect {:?}", rect);

        // create render factory, target and bitmap
        // let mut factory: Direct2D::ID2D1Factory1 = ::std::ptr::null_mut();
        // let options = Direct2D::D2D1_FACTORY_OPTIONS::default();

        // let kkt = factory as *mut _ as *mut _;

        // Direct2D::D2D1CreateFactory(
        //     Direct2D::D2D1_FACTORY_TYPE::D2D1_FACTORY_TYPE_SINGLE_THREADED,
        //     &Direct2D::ID2D1Factory1::IID,
        //     &options,
        //     factory as *mut _ as *mut _
        // );

        // let bitmap_render_target = factory.CreateHwndRenderTarget(rendertargetproperties, hwndrendertargetproperties, hwndrendertarget);
        // let bitmap = Direct2D::ID2D1RenderTarget::CreateBitmap(size, std::ptr::null_mut(), 

        loop {}
    }
}

#[no_mangle]
extern "system" fn wnd_proc(hwnd: WindowsAndMessaging::HWND, msg: u32, wparam: WindowsAndMessaging::WPARAM, lparam: WindowsAndMessaging::LPARAM) -> SystemServices::LRESULT {
    unsafe { WindowsAndMessaging::DefWindowProcW(hwnd, msg, wparam, lparam) }
}

impl SystemServices::PWSTR {
    fn from(text: &'static str) -> Self {
        Self(text.encode_utf16().chain(::std::iter::once(0)).collect::<Vec<u16>>().as_mut_ptr())
    }
}


