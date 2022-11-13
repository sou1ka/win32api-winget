use std::borrow::BorrowMut;
use std::ffi::CString;
use std::ffi::OsString;
use windows::{
    Win32::Foundation::{BOOL, HWND, LPARAM, PWSTR, RECT},
    Win32::UI::WindowsAndMessaging::{EnumWindows, FindWindowW, GetWindowTextW, GetForegroundWindow, GetWindowRect},
};
use windows::core::IntoParam;
use windows::core::Param::Owned;

fn main() -> windows::core::Result<()> {
    unsafe { EnumWindows(Some(enum_window), LPARAM(0)).ok() }
  //  unsafe { GetForegroundWindow(Some(get_foregroundwindow), RECT(0)).ok() }
}

extern "system" fn enum_window(window: HWND, _: LPARAM) -> BOOL {
    unsafe {
        // 表示中のウィンドウ名リスト
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, PWSTR(text.as_mut_ptr()), text.len() as i32);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        if !text.is_empty() {
            println!("{}", text);
        }

        let win = GetForegroundWindow();
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(win, PWSTR(text.as_mut_ptr()), text.len() as i32);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        // 指定ウィンドウ情報取得
        pub struct rec {
            left: f32,
            top: f32,
            right: f32,
            bottom: f32,
        };
      //  let rec;
        let mut rect =  RECT { left: 0, top: 0, right: 0, bottom: 0 };
        let ret = GetWindowRect(win, &mut rect);
        println!("{}", rect.top);

        return true.into();
    }
}

extern "system" fn get_foregroundwindow(window: HWND, _: LPARAM) -> BOOL {
    unsafe {

        return true.into();
    }
}