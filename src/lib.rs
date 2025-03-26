use ctor::ctor;
use dlopen::raw::Library;
use std::sync::OnceLock;

#[cfg(windows)]
pub mod windows;

#[ctor]
fn hello() {
    static BLUEBRICK: OnceLock<Option<Library>> = OnceLock::new();

    BLUEBRICK.get_or_init(|| match Library::open("bluebrick/bluebrick.dll") {
        Ok(lib) => Some(lib),
        Err(e) => {
            msgbox::create("Error Loading BlueBrick", &format!("Problem opening loader:\n{e:?}"), msgbox::IconType::Error);
            None
        }
    });
}
