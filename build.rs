fn main() {
    ::windows::build!(
        Windows::Win32::Gdi::HBRUSH,
        Windows::Win32::MenusAndResources::{ HCURSOR, HICON },
        Windows::Win32::SystemServices::{ GetModuleHandleW, HINSTANCE },
        Windows::Win32::WindowsAndMessaging::{
            CW_USEDEFAULT,
            CreateWindowExW,
            DefWindowProcW,
            LPARAM,
            RegisterClassW,
            ShowWindow,
            WINDOW_EX_STYLE,
            WINDOW_STYLE,
            WNDCLASSW,
            WNDCLASS_STYLES,
            WPARAM,
        },
    );
}