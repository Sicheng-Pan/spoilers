use std::fmt::Display;

use eframe::{get_value, set_value, App, CreationContext, Frame, Storage, APP_KEY};
use egui::{
    global_dark_light_mode_switch, menu::bar, CentralPanel, ComboBox, Context, FontData,
    FontDefinitions, FontFamily, TopBottomPanel, Ui,
};
use serde::{Deserialize, Serialize};
use spoilers::{
    adapter::{Adapter, AdapterConfig, AdapterKind},
    translator::{Translator, TranslatorConfig},
};
use strum::{Display as EnumDisplay, EnumIter, IntoEnumIterator};

const CJK: &str = "Sarasa-Gothic-Regular";

#[derive(Clone, Debug, Default, Deserialize, EnumDisplay, EnumIter, Eq, PartialEq, Serialize)]
pub enum GUIMode {
    Text,
    #[default]
    Config,
}

#[derive(Default, Deserialize, Serialize)]
pub struct TranslatorGUI {
    #[serde(skip)]
    adapter: Option<Box<dyn Adapter>>,
    adapter_config: AdapterConfig,
    gui_mode: GUIMode,
    source_content: String,
    source_language: String,
    target_content: String,
    target_language: String,
    #[serde(skip)]
    translator: Option<Translator>,
    translator_config: TranslatorConfig,
}

impl TranslatorGUI {
    pub fn new(cc: &CreationContext) -> Self {
        let mut gui_font = FontDefinitions::default();
        gui_font.font_data.insert(
            CJK.into(),
            FontData::from_static(include_bytes!(env!("SARASA_GOTHIC_PATH"))),
        );
        gui_font
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, CJK.into());
        cc.egui_ctx.set_fonts(gui_font);
        let mut gui: TranslatorGUI = cc
            .storage
            .map(|s| get_value(s, APP_KEY).unwrap_or_default())
            .unwrap_or_default();
        gui.reload_adapter();
        gui.reload_translator();
        gui
    }

    pub fn reload_adapter(&mut self) {
        if let Ok(adapter) = self.adapter_config.initialize() {
            self.adapter = Some(adapter)
        }
    }

    pub fn reload_translator(&mut self) {
        if let Ok(translator) = self.translator_config.initialize() {
            self.translator = Some(translator)
        }
    }

    pub fn config_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let adapter_label = ui.label("Adapter path:");
                ui.text_edit_singleline(&mut self.adapter_config.source)
                    .labelled_by(adapter_label.id);
                ComboBox::from_id_source("adapter_kind")
                    .selected_text(format!("{:?}", self.adapter_config.kind))
                    .show_ui(ui, |ui| {
                        enum_to_selectable(ui, &mut self.adapter_config.kind, AdapterKind::iter());
                    });
                if ui.button("Reload adapter").clicked() {
                    self.reload_adapter()
                }
            });
            ui.horizontal(|ui| {
                let model_label = ui.label("Model path:");
                ui.text_edit_singleline(&mut self.translator_config.model_path)
                    .labelled_by(model_label.id);
                if ui.button("Reload translator").clicked() {
                    self.reload_translator();
                }
            });
        });
    }

    pub fn text_translate_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if let (Some(adapter), Some(translator)) =
                (self.adapter.as_ref(), self.translator.as_ref())
            {
                ui.columns(2, |columns| {
                    columns[0].horizontal(|ui| {
                        ui.label("From");
                        ComboBox::from_id_source("source_language")
                            .selected_text(&self.source_language)
                            .show_ui(ui, |ui| {
                                enum_to_selectable(
                                    ui,
                                    &mut self.source_language,
                                    adapter.available_languages().into_iter(),
                                );
                            });
                    });
                    columns[0].text_edit_multiline(&mut self.source_content);
                    columns[1].horizontal(|ui| {
                        ui.label("To");
                        ComboBox::from_id_source("target_language")
                            .selected_text(&self.target_language)
                            .show_ui(ui, |ui| {
                                enum_to_selectable(
                                    ui,
                                    &mut self.target_language,
                                    adapter.available_languages().into_iter(),
                                );
                            });
                    });
                    columns[1].label(&self.target_content);
                });
                if ui.button("Translate").clicked() {
                    self.target_content = translator
                        .translate(
                            adapter,
                            &self.source_content,
                            &self.source_language,
                            &self.target_language,
                        )
                        .unwrap_or_default();
                }
            } else {
                if self.adapter.is_none() {
                    ui.label("Please load the adapter");
                }
                if self.translator.is_none() {
                    ui.label("Please load the translator");
                }
            }
        });
    }
}

impl App for TranslatorGUI {
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self)
    }

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("menu").show(ctx, |ui| {
            bar(ui, |ui| {
                global_dark_light_mode_switch(ui);
                enum_to_selectable(ui, &mut self.gui_mode, GUIMode::iter());
            });
        });

        match self.gui_mode {
            GUIMode::Config => self.config_panel(ctx, frame),
            GUIMode::Text => self.text_translate_panel(ctx, frame),
        }
    }
}

pub fn enum_to_selectable<S>(ui: &mut Ui, state: &mut S, variants: impl Iterator<Item = S>)
where
    S: Clone + Display + Eq,
{
    variants.for_each(|item| {
        ui.selectable_value(state, item.clone(), format!("{item}"));
    })
}
