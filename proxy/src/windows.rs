#![allow(non_snake_case)]

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

fn xinput() -> &'static Container<XInput1_3API> {
    static ONCE: OnceLock<Container<XInput1_3API>> = OnceLock::new();
    ONCE.get_or_init(|| {
        match unsafe { Container::<XInput1_3API>::load("XInput1_3") } {
            Ok(xinput) => xinput,
            Err(e) => {
                let e = format!("Problem opening original for proxy (XInput1_3.dll):\n{e:?}");
                let _ = msgbox::create("Error Loading BlueBrick", &e, msgbox::IconType::Error);
                panic!("{e}")
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn XInputGetState(wuserindex: u32, pstate: *mut XINPUT_STATE) -> u32 {
    println!("XInputGetState");
    xinput().XInputGetState(wuserindex, pstate)
}

#[unsafe(no_mangle)]
pub extern "C" fn XInputSetState(dwuserindex: u32, pvibration: *const XINPUT_VIBRATION) -> u32 {
    println!("XInputSetState");
    xinput().XInputSetState(dwuserindex, pvibration)
}

#[unsafe(no_mangle)]
pub extern "C" fn XInputGetCapabilities(dwuserindex: u32, dwflags: XINPUT_FLAG, pcapabilities: *mut XINPUT_CAPABILITIES) -> u32 {
    println!("XInputGetCapabilities");
    xinput().XInputGetCapabilities(dwuserindex, dwflags, pcapabilities)
}
