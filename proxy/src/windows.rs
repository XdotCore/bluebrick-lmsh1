use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use windows::Win32::UI::Input::XboxController::{XINPUT_CAPABILITIES, XINPUT_FLAG, XINPUT_STATE, XINPUT_VIBRATION};
use std::sync::OnceLock;

#[derive(WrapperApi)]
struct XInput1_3API {
    XInputGetState: extern "C" fn(wuserindex: u32, pstate: *mut XINPUT_STATE) -> u32,
    XInputSetState: extern "C" fn(dwuserindex: u32, pvibration: *const XINPUT_VIBRATION) -> u32,
    XInputGetCapabilities: extern "C" fn(dwuserindex: u32, dwflags: XINPUT_FLAG, pcapabilities: *mut XINPUT_CAPABILITIES) -> u32,
}

fn xinput() -> Container<XInput1_3API> {
    let once = OnceLock::<Container<XInput1_3API>>::new();
    once.get_or_init(|| {
        match unsafe { Container::<BBApi>::load("XInput1_3") } {
            Ok(xinput) => xinput,
            Err(e) => {
                let _ = msgbox::create("Error Loading BlueBrick", &format!("Problem opening original for proxy (XInput1_3.dll):\n{e:?}"), msgbox::IconType::Error);
                return;
            }
        };
    })
}

#[unsafe(no_mangle)]
pub unsafe extern fn XInputGetState(wuserindex: u32, pstate: *mut XINPUT_STATE) -> u32 {
    xinput().XInputGetState(wuserindex, pstate)
}

#[unsafe(no_mangle)]
pub unsafe extern fn XInputSetState(dwuserindex: u32, pvibration: *const XINPUT_VIBRATION) -> u32 {
    xinput().XInputSetState(dwuserindex, pvibration)
}

#[unsafe(no_mangle)]
pub unsafe extern fn XInputGetCapabilities(dwuserindex: u32, dwflags: XINPUT_FLAG, pcapabilities: *mut XINPUT_CAPABILITIES) -> u32 {
    xinput().XInputGetCapabilities(dwuserindex, dwflags, pcapabilities)
}
