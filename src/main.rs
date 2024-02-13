#[cfg(feature = "app")]
mod gui;

fn main() {
    #[cfg(feature = "app")]
    gui::TranslatorGUI::run_native().unwrap();
}
