use bluebrick_proxy_base::{Platform, Renderer, load_bluebrick};
use ctor::ctor;

#[cfg(windows)]
pub mod windows;

#[ctor]
fn hello() {
    let _ = msgbox::create("d", "d", msgbox::IconType::None);

    load_bluebrick(Platform::Win32, Renderer::DX9);
}
