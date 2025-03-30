use windows::Win32::UI::Input::XboxController::{XINPUT_CAPABILITIES, XINPUT_FLAG, XINPUT_STATE, XINPUT_VIBRATION};

#[link(name = "XInput1_3", kind = "raw-dylib")]
unsafe extern "C" {
    pub unsafe fn XInputGetState(wuserindex: u32, pstate: *mut XINPUT_STATE) -> u32;
    pub unsafe fn XInputSetState(dwuserindex: u32, pvibration: *const XINPUT_VIBRATION) -> u32;
    pub unsafe fn XInputGetCapabilities(dwuserindex: u32, dwflags: XINPUT_FLAG, pcapabilities: *mut XINPUT_CAPABILITIES) -> u32;
}
