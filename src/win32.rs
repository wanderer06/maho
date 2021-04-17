// We should keep the naming convention as the original code
#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub type HBRUSH = *mut ::std::os::raw::c_void;
pub type HINSTANCE = *mut ::std::os::raw::c_void;
pub type HICON = *mut ::std::os::raw::c_void;
pub type HCURSOR = HICON;
pub type HMODULE = HINSTANCE;

pub type WCHAR = ::libc::wchar_t;
pub type UINT = ::std::os::raw::c_uint;

pub type UINT_PTR = ::std::os::raw::c_ulonglong;
pub type LONG_PTR = ::std::os::raw::c_longlong;

pub type LPCWSTR = *const WCHAR;
pub type WPARAM = UINT_PTR;

pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;


#[derive(Debug, Copy, Clone)]
pub struct HWND__ {
    pub unused: ::std::os::raw::c_int,
}

pub type HWND = *mut HWND__;

pub type WNDPROC = ::std::option::Option<
    unsafe extern "C" fn(arg1: HWND, arg2: UINT, arg3: WPARAM, arg4: LPARAM) -> LRESULT,
>;

extern "C" {
    #[link(name = "kernel32")]
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagWNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: ::std::os::raw::c_int,
    pub cbWndExtra: ::std::os::raw::c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
}

pub struct ExtendedWindowStyles {}

// For descriptions 
// See https://docs.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles
impl ExtendedWindowStyles {
    const WS_EX_ACCEPTFILES: ::std::os::raw::c_ulong = 0x00000010;
    const WS_EX_APPWINDOW: ::std::os::raw::c_ulong = 0x00040000;
    const WS_EX_CLIENTEDGE: ::std::os::raw::c_ulong = 0x00000200;
    const WS_EX_COMPOSITED: ::std::os::raw::c_ulong = 0x02000000;
    const WS_EX_CONTEXTHELP: ::std::os::raw::c_ulong = 0x00000400;
    const WS_EX_CONTROLPARENT: ::std::os::raw::c_ulong = 0x00010000;
    const WS_EX_DLGMODALFRAME: ::std::os::raw::c_ulong = 0x00000001;
    const WS_EX_LAYERED: ::std::os::raw::c_ulong = 0x00080000;
    const WS_EX_LAYOUTRTL: ::std::os::raw::c_ulong = 0x00400000;
    const WS_EX_LEFT: ::std::os::raw::c_ulong = 0x00000000;
    const WS_EX_LEFTSCROLLBAR: ::std::os::raw::c_ulong = 0x00004000;
    const WS_EX_LTRREADING: ::std::os::raw::c_ulong = 0x00000000;
    const WS_EX_MDICHILD: ::std::os::raw::c_ulong = 0x00000040;
    const WS_EX_NOACTIVATE: ::std::os::raw::c_ulong = 0x08000000;
    const WS_EX_NOINHERITLAYOUT: ::std::os::raw::c_ulong = 0x00100000;
    const WS_EX_NOPARENTNOTIFY: ::std::os::raw::c_ulong = 0x00000004;
    const WS_EX_NOREDIRECTIONBITMAP: ::std::os::raw::c_ulong = 0x00200000;
    const WS_EX_OVERLAPPEDWINDOW: ::std::os::raw::c_ulong = (0x00000100 | 0x00000200); // (WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE);
    const WS_EX_PALETTEWINDOW: ::std::os::raw::c_ulong = (0x00000100 | 0x00000080 | 0x00000008); // (WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST);
    const WS_EX_RIGHT: ::std::os::raw::c_ulong = 0x00001000;
    const WS_EX_RIGHTSCROLLBAR: ::std::os::raw::c_ulong = 0x00000000;
    const WS_EX_RTLREADING: ::std::os::raw::c_ulong = 0x00002000;
    const WS_EX_STATICEDGE: ::std::os::raw::c_ulong = 0x00020000;
    const WS_EX_TOOLWINDOW: ::std::os::raw::c_ulong = 0x00000080;
    const WS_EX_TOPMOST: ::std::os::raw::c_ulong = 0x00000008;
    const WS_EX_TRANSPARENT: ::std::os::raw::c_ulong = 0x00000020;
    const WS_EX_WINDOWEDGE: ::std::os::raw::c_ulong = 0x00000100;
}