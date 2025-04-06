use bluebrick::{log, logger::Logger, subbrick::{Library, SubBrick}};
use bluebrick_proc_macros::bluebrick_library;

#[bluebrick_library("Lego Marvel Superheroes", "X.Core")]
struct LMSH1;

impl LMSH1 {
}

impl Library for LMSH1 {}

impl SubBrick for LMSH1 {
    fn init() {
        log!(Self::logger(), "Hello LMSH1!");
    }

    fn enable() -> bool {
        log!(Self::logger(), "enabled");
        true
    }
    
    fn disable() -> bool {
        log!(Self::logger(), "disabled");
        true
    }
}