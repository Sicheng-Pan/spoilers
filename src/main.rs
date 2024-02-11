mod gui;

use eframe::{run_native, NativeOptions, Result};
use gui::TranslatorGUI;

fn main() -> Result<()> {
    run_native(
        "spoilers",
        NativeOptions::default(),
        Box::new(|cc| Box::new(TranslatorGUI::new(cc))),
    )
}
