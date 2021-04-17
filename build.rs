fn main() {
    ::windows::build!(
        Windows::Win32::Gdi::HBRUSH,
        Windows::Win32::MenusAndResources::HCURSOR,
        Windows::Win32::MenusAndResources::HICON,
        Windows::Win32::SystemServices::GetModuleHandleW,
        Windows::Win32::SystemServices::HINSTANCE,
        Windows::Win32::WindowsAndMessaging::DefWindowProcW,
        Windows::Win32::WindowsAndMessaging::LPARAM,
        Windows::Win32::WindowsAndMessaging::RegisterClassW,
        Windows::Win32::WindowsAndMessaging::WNDCLASSW,
        Windows::Win32::WindowsAndMessaging::WNDCLASS_STYLES,
        Windows::Win32::WindowsAndMessaging::WPARAM,
    );
}