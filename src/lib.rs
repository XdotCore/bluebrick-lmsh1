use ctor::ctor;
use dlopen::raw::Library;

#[cfg(windows)]
pub mod windows;

#[ctor]
fn hello() {
    unsafe {
        if let Err(e) = Library::open("bluebrick/bluebrick.dll") {
            msgbox::create("Error Loading BlueBrick", &format!("Problem opening loader:\n{e:?}"), msgbox::IconType::Error);
        };
    }
}
