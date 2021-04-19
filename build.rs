fn main() {
    ::windows::build!(
        Windows::Win32::Direct2D::*,
        Windows::Win32::DisplayDevices::RECT,
        Windows::Win32::Gdi::HBRUSH,
        Windows::Win32::MenusAndResources::{ HCURSOR, HICON },
        Windows::Win32::SystemServices::{ GetModuleHandleW, HINSTANCE },
        Windows::Win32::WindowsAndMessaging::{
            CW_USEDEFAULT,
            CreateWindowExW,
            DefWindowProcW,
            GetClientRect,
            LPARAM,
            MSG,
            PostQuitMessage,
            RegisterClassW,
            ShowWindow,
            WINDOW_EX_STYLE,
            WINDOW_STYLE,
            WM_QUIT,
            WNDCLASSW,
            WNDCLASS_STYLES,
            WPARAM,
        },
    );
}