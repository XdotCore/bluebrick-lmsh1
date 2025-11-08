use bluebrick::{
    bluebrick_library,
    imgui::Ui,
    log,
    logger::Logger,
    subbrick::{Library, SubBrick},
};

#[bluebrick_library("Lego Marvel Superheroes", "X.Core")]
struct LMSH1;

impl LMSH1 {}

impl Library for LMSH1 {}

impl SubBrick for LMSH1 {
    fn new() -> Self {
        Self {}
    }

    fn init(&mut self) {
        log!(Self::logger(), "Hello LMSH1!");
    }

    fn enable(&mut self) -> bool {
        log!(Self::logger(), "enabled");
        true
    }

    fn disable(&mut self) -> bool {
        log!(Self::logger(), "disabled");
        true
    }

    fn draw(&mut self, ui: &Ui) {
        ui.window("LMSH1").build(|| {
            ui.text_colored([1.0, 0.0, 0.0, 1.0], "Hello World!");
        });
    }
}
