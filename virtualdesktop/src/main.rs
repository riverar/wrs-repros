use windows::core::*;
use windows::Win32::Foundation::PSTR;
use windows::Win32::System::Com::*;
use windows::Win32::UI::Shell::*;
use windows::Win32::UI::WindowsAndMessaging::*;

// https://docs.microsoft.com/windows/win32/api/shobjidl_core/nn-shobjidl_core-ivirtualdesktopmanager

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED)?;

        let manager: IVirtualDesktopManager =
            CoCreateInstance(&VirtualDesktopManager, None, CLSCTX_INPROC_SERVER)?;

        let hwnd = FindWindowA("Notepad", PSTR::default());
        if hwnd.is_invalid() {
            panic!("Failed to get Notepad window handle. Is Notepad running?");
        }

        println!(
            "Notepad lives on virtual desktop {:?}",
            manager.GetWindowDesktopId(hwnd)
        );
    }

    Ok(())
}
