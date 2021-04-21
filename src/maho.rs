mod bindings {
    ::windows::include_bindings!();
}

use bindings::Windows::Win32::{
    Direct2D, DisplayDevices, Gdi, MenusAndResources, SystemServices, WindowsAndMessaging,
};

use windows::{Abi, Interface};

#[allow(dead_code)]
pub struct Context {
    brush: Option<Direct2D::ID2D1SolidColorBrush>,
    factory: Option<Direct2D::ID2D1Factory1>,
    hwnd: WindowsAndMessaging::HWND,
    hwnd_render_target: Option<Direct2D::ID2D1HwndRenderTarget>,
}

pub enum Event {
    Key(usize),
    Quit,
    None,
}

impl Context {
    pub fn create(title: &str, width: i32, height: i32) -> Self {
        unsafe {
            let mut context = Self {
                brush: None,
                factory: None,
                hwnd: WindowsAndMessaging::HWND(0),
                hwnd_render_target: None,
            };

            // obtain instance
            let h_instance = SystemServices::GetModuleHandleW(SystemServices::PWSTR::NULL);
            let h_instance = SystemServices::HINSTANCE(h_instance);
            let class_name = SystemServices::PWSTR::from(title);

            debug_assert!(h_instance.0 != 0);

            // create window class
            let wnd_class = WindowsAndMessaging::WNDCLASSW {
                style: WindowsAndMessaging::WNDCLASS_STYLES(0),
                lpfnWndProc: Some(Context::wnd_proc),
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
            WindowsAndMessaging::RegisterClassW(&wnd_class);

            // create window
            context.hwnd = WindowsAndMessaging::CreateWindowExW(
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
                &mut context as *mut Context as *mut libc::c_void, // pointer to additional context data
            );

            // show created window
            WindowsAndMessaging::ShowWindow(
                context.hwnd,
                WindowsAndMessaging::SHOW_WINDOW_CMD::SW_SHOWDEFAULT,
            );

            // create factory
            let mut options = Direct2D::D2D1_FACTORY_OPTIONS::default();
            options.debugLevel = Direct2D::D2D1_DEBUG_LEVEL::D2D1_DEBUG_LEVEL_INFORMATION;

            Direct2D::D2D1CreateFactory(
                Direct2D::D2D1_FACTORY_TYPE::D2D1_FACTORY_TYPE_SINGLE_THREADED,
                &Direct2D::ID2D1Factory1::IID,
                &options,
                context.factory.set_abi(),
            )
            .unwrap();

            context
        }
    }

    fn create_graphic_resources(&mut self) {
        if self.hwnd_render_target.is_none() {
            unsafe {
                // get client rect
                let mut rect = DisplayDevices::RECT::default();
                WindowsAndMessaging::GetClientRect(self.hwnd, &mut rect);

                let size = Direct2D::D2D_SIZE_U {
                    width: (rect.right - rect.left) as u32,
                    height: (rect.bottom - rect.top) as u32,
                };

                let render_target_properties = Direct2D::D2D1_RENDER_TARGET_PROPERTIES::default();
                let hwnd_render_target_properties = Direct2D::D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    hwnd: self.hwnd,
                    pixelSize: size,
                    presentOptions: Direct2D::D2D1_PRESENT_OPTIONS::default(),
                };

                self.factory.as_ref()
                    .unwrap()
                    .CreateHwndRenderTarget(
                        &render_target_properties,
                        &hwnd_render_target_properties,
                        &mut self.hwnd_render_target,
                    )
                    .unwrap();

                let brush_colour = Direct2D::D2D1_COLOR_F {
                    r: 1.0,
                    g: 0.0,
                    b: 1.0,
                    a: 1.0,
                };

                let brush_properties = Direct2D::D2D1_BRUSH_PROPERTIES {
                    opacity: 1.0,
                    transform: bindings::Windows::Foundation::Numerics::Matrix3x2::identity(),
                };

                self.hwnd_render_target
                    .as_ref()
                    .unwrap()
                    .CreateSolidColorBrush(&brush_colour, &brush_properties, &mut self.brush)
                    .unwrap();
            }
        }
    }

    fn discard_graphic_resources(&mut self) {
        self.brush = None;
        self.hwnd_render_target = None;
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

    fn test_render(&mut self) {
        unsafe {
            self.create_graphic_resources();

            let hwnd_render_target = self.hwnd_render_target.as_ref().unwrap();
            let brush = self.brush.as_ref().unwrap();

            let clear_colour = Direct2D::D2D1_COLOR_F {
                r: 0.25,
                g: 0.25,
                b: 0.25,
                a: 1.0,
            };

            let render_target_size = hwnd_render_target.GetSize();

            let fill_rect = Direct2D::D2D_RECT_F {
                left: render_target_size.width / 2.0 - 50.0,
                top: render_target_size.height / 2.0 - 50.0,
                right: render_target_size.width / 2.0 + 50.0,
                bottom: render_target_size.height / 2.0 + 50.0,
            };

            let mut ps = Gdi::PAINTSTRUCT::default();
            Gdi::BeginPaint(&self.hwnd, &mut ps);

            let matrix_identity = bindings::Windows::Foundation::Numerics::Matrix3x2::identity();

            hwnd_render_target.BeginDraw();

            hwnd_render_target.Clear(&clear_colour);
            hwnd_render_target.SetTransform(&matrix_identity);
            hwnd_render_target.FillRectangle(&fill_rect, brush);

            let hr = hwnd_render_target.EndDraw(std::ptr::null_mut(), std::ptr::null_mut());
            if hr.is_err() {
                self.discard_graphic_resources();
            }

            Gdi::EndPaint(&self.hwnd, &ps);
        }
    }

    fn handle_wnd_proc_message(
        &mut self,
        msg: u32,
        wparam: WindowsAndMessaging::WPARAM,
        lparam: WindowsAndMessaging::LPARAM,
    ) -> SystemServices::LRESULT {
        unsafe {
            match msg {
                WindowsAndMessaging::WM_PAINT => {
                    self.test_render();
                    SystemServices::LRESULT(0)
                }
                WindowsAndMessaging::WM_DESTROY => {
                    self.discard_graphic_resources();
                    WindowsAndMessaging::PostQuitMessage(0);
                    SystemServices::LRESULT(0)
                }
                _ => WindowsAndMessaging::DefWindowProcW(self.hwnd, msg, wparam, lparam),
            }
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
            // this is a bit tricky
            // we need to pull our Self reference from the create data
            // so we can call it later from inside the window process
            if msg == WindowsAndMessaging::WM_CREATE {
                // first time the window is created
                // we have all the parameters used in CreateWindowEx
                // The window procedure of the new window receives this message after the window is created, but before the window becomes visible
                let pointer_create_struct = lparam.0 as *const WindowsAndMessaging::CREATESTRUCTW;
                let pointer_app = (*pointer_create_struct).lpCreateParams as *mut Context;
                (*pointer_app).hwnd = hwnd;

                SetWindowLong(
                    hwnd,
                    WindowsAndMessaging::WINDOW_LONG_PTR_INDEX::GWLP_USERDATA,
                    pointer_app as isize,
                );
            } else {
                // retrieve the app pointer
                let pointer_app = GetWindowLong(
                    hwnd,
                    WindowsAndMessaging::WINDOW_LONG_PTR_INDEX::GWLP_USERDATA,
                ) as *mut Context;

                if !pointer_app.is_null() {
                    // call a procedure on our app pointer
                    (*pointer_app).handle_wnd_proc_message(msg, wparam, lparam);
                }
            }

            WindowsAndMessaging::DefWindowProcW(hwnd, msg, wparam, lparam)
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

// we need to call different WindowLong functions
// since win32 will not have the definitions for win64 WindowLongPtr variants
#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn SetWindowLong(
    hwnd: WindowsAndMessaging::HWND,
    nindex: WindowsAndMessaging::WINDOW_LONG_PTR_INDEX,
    dwnewlong: isize,
) -> isize {
    WindowsAndMessaging::SetWindowLongW(hwnd, nindex, dwnewlong as isize) as isize
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn SetWindowLong(
    hwnd: WindowsAndMessaging::HWND,
    nindex: WindowsAndMessaging::WINDOW_LONG_PTR_INDEX,
    dwnewlong: isize,
) -> isize {
    WindowsAndMessaging::SetWindowLongPtrW(hwnd, nindex, dwnewlong)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn GetWindowLong(
    hwnd: WindowsAndMessaging::HWND,
    nindex: WindowsAndMessaging::WINDOW_LONG_PTR_INDEX,
) -> isize {
    WindowsAndMessaging::GetWindowLongW(hwnd, nindex) as isize
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn GetWindowLong(
    hwnd: WindowsAndMessaging::HWND,
    nindex: WindowsAndMessaging::WINDOW_LONG_PTR_INDEX,
) -> isize {
    WindowsAndMessaging::GetWindowLongPtrW(hwnd, nindex)
}
