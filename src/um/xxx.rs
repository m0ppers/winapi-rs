#![cfg(windows)]

use ctypes::c_int;
use shared::minwindef::UINT;
use shared::minwindef::DWORD;
use shared::minwindef::LPBOOL;
use shared::ntdef::LPSTR;
use shared::ntdef::LPCSTR;
use shared::ntdef::LPWSTR;
use shared::ntdef::LPCWSTR;

extern "system" {
    pub fn MultiByteToWideChar(
        CodePage: UINT, dwFlags: DWORD, lpMultiByteStr: LPCSTR, cbMultiByte: c_int,
        lpWideCharStr: LPWSTR, cchWideChar: c_int,
    ) -> c_int;
    pub fn WideCharToMultiByte(
        CodePage: UINT, dwFlags: DWORD, lpWideCharStr: LPCWSTR, cchWideChar: c_int,
        lpMultiByteStr: LPSTR, cbMultiByte: c_int, lpDefaultChar: LPCSTR, lpUsedDefaultChar: LPBOOL,
    ) -> c_int;
}