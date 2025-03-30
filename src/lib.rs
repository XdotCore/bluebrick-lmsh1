use bluebrick_proxy_base::{RequestedPlatform, RequestedRenderer, load_bluebrick, Config};
use ctor::ctor;

#[cfg(windows)]
pub mod windows;

#[ctor]
fn hello() {
    load_bluebrick(Config {
        platform: RequestedPlatform::Win32,
        renderer: RequestedRenderer::DX9
    });
}
