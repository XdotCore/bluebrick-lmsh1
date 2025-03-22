use ctor::ctor;
use dlopen::raw::Library;

#[cfg(windows)]
pub mod windows;

#[ctor]
fn hello() {
    unsafe {
        static mut BLUEBRICK: Option<Library> = None;

        BLUEBRICK = match Library::open("bluebrick/bluebrick.dll") {
            Ok(lib) => Some(lib),
            Err(e) => {
                msgbox::create("Error Loading BlueBrick", &format!("Problem opening loader:\n{e:?}"), msgbox::IconType::Error);
                return;
            }
        };
    }
}
