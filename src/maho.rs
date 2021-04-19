mod bindings {
    ::windows::include_bindings!();
}

use bindings::Windows::Win32::{
    Direct2D, DisplayDevices, Gdi, MenusAndResources, SystemServices, WindowsAndMessaging,
};

use windows::{Abi, IUnknown};

pub struct Context {
    hwnd: WindowsAndMessaging::HWND,
}

pub enum Event {
    Key(usize),
    Quit,
    None,
}

impl Context {
    pub fn create(title: &str, width: i32, height: i32) -> Self {
        unsafe {
            // obtain instance
            let h_instance = SystemServices::GetModuleHandleW(SystemServices::PWSTR::NULL);
            let h_instance = SystemServices::HINSTANCE(h_instance);
            let class_name = SystemServices::PWSTR::from(title);

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
                title,
                title,
                WindowsAndMessaging::WINDOW_STYLE::WS_OVERLAPPEDWINDOW,
                WindowsAndMessaging::CW_USEDEFAULT,
                WindowsAndMessaging::CW_USEDEFAULT,
                width,
                height,
                WindowsAndMessaging::HWND::NULL,
                MenusAndResources::HMENU::NULL,
                h_instance,
                ::std::ptr::null_mut(),
            );

            // show created window
            WindowsAndMessaging::ShowWindow(
                hwnd,
                WindowsAndMessaging::SHOW_WINDOW_CMD::SW_SHOWDEFAULT,
            );

            Gdi::UpdateWindow(hwnd);

            // get client rect
            let mut rect = DisplayDevices::RECT::default();
            let client_rect = WindowsAndMessaging::GetClientRect(hwnd, &mut rect);

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
            Self { hwnd }
        }
    }

    pub fn get_event(&self) -> Event {
        let mut msg = WindowsAndMessaging::MSG::default();

        unsafe {
            if WindowsAndMessaging::PeekMessageW(
                &mut msg,
                WindowsAndMessaging::HWND::NULL,
                0,
                0,
                WindowsAndMessaging::PeekMessage_wRemoveMsg::PM_REMOVE,
            )
            .as_bool()
            {
                let event: Event = match msg.message {
                    WindowsAndMessaging::WM_KEYDOWN => Event::Key(msg.wParam.0),
                    WindowsAndMessaging::WM_QUIT => Event::Quit,
                    _ => Event::None,
                };

                // dispatch message calls the window procedure
                // this would be wnd_proc in our case
                WindowsAndMessaging::TranslateMessage(&msg);
                WindowsAndMessaging::DispatchMessageW(&msg);

                return event;
            }
        }

        Event::None
    }
}

#[no_mangle]
extern "system" fn wnd_proc(
    hwnd: WindowsAndMessaging::HWND,
    msg: u32,
    wparam: WindowsAndMessaging::WPARAM,
    lparam: WindowsAndMessaging::LPARAM,
) -> SystemServices::LRESULT {
    unsafe {
        match msg {
            WindowsAndMessaging::WM_PAINT => {
                // todo bitmap rendering here
                SystemServices::LRESULT(0)
            }
            WindowsAndMessaging::WM_DESTROY => {
                WindowsAndMessaging::PostQuitMessage(0);
                SystemServices::LRESULT(0)
            }
            _ => WindowsAndMessaging::DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

impl SystemServices::PWSTR {
    fn from(text: &str) -> Self {
        Self(
            text.encode_utf16()
                .chain(::std::iter::once(0))
                .collect::<Vec<u16>>()
                .as_mut_ptr(),
        )
    }
}
