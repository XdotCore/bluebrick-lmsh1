use bluebrick::proxy::{load_bluebrick, Config, RequestedPlatform, RequestedRenderer};
use ctor::ctor;

#[cfg(windows)]
pub mod windows;

#[ctor]
fn hello() {
    let _ = msgbox::create("For debugging", "For debugging", msgbox::IconType::None);

    load_bluebrick(Config {
        platform: RequestedPlatform::Win32,
        renderer: RequestedRenderer::DX9
    });
}
