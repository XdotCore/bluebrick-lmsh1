use bluebrick_proxy_base::{RequestedPlatform, RequestedRenderer, load_bluebrick};
use ctor::ctor;

#[cfg(windows)]
pub mod windows;

#[ctor]
fn hello() {
    let _ = msgbox::create("d", "d", msgbox::IconType::None);

    load_bluebrick(RequestedPlatform::Win32, RequestedRenderer::DX9);
}
