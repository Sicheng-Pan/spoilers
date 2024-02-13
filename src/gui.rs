use eframe::{
    get_value, run_native, set_value, App, CreationContext, Frame, NativeOptions, Result, Storage,
    APP_KEY,
};
use egui::{
    global_dark_light_mode_switch, menu::bar, CentralPanel, ComboBox, Context, FontData,
    FontDefinitions, FontFamily, RichText, ScrollArea, TopBottomPanel, Ui, ViewportBuilder,
};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use serde::{Deserialize, Serialize};
use spoilers::{
    adapter::{Adapter, AdapterConfig, AdapterKind},
    translator::{Translator, TranslatorConfig},
};
use std::fmt::Display;
use strum::{Display as EnumDisplay, EnumIter, IntoEnumIterator};

const CJK: &str = "Sarasa-Gothic-Regular";
const CJK_BINARY: &[u8] = include_bytes!(env!("SARASA_GOTHIC_PATH"));
const README: &str = include_str!("../README.md");

#[derive(Clone, Debug, Default, Deserialize, EnumDisplay, EnumIter, Eq, PartialEq, Serialize)]
pub enum GUIMode {
    Translate,
    Config,
    #[default]
    Readme,
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
    pub fn run_native() -> Result<()> {
        let mut native_options = NativeOptions::default();
        native_options.viewport =
            ViewportBuilder::default().with_min_inner_size([600_f32, 400_f32]);
        run_native(
            "spoilers",
            native_options,
            Box::new(|cc| Box::new(TranslatorGUI::new(cc))),
        )
    }

    pub fn new(cc: &CreationContext) -> Self {
        let mut gui_font = FontDefinitions::default();
        gui_font
            .font_data
            .insert(CJK.into(), FontData::from_static(CJK_BINARY));
        gui_font
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, CJK.into());
        cc.egui_ctx.set_fonts(gui_font);
        let mut gui: TranslatorGUI = cc
            .storage
            .map(|storage| get_value(storage, APP_KEY).unwrap_or_default())
            .unwrap_or_default();
        gui.reload_adapter();
        gui.reload_translator();
        gui
    }

    pub fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self)
    }

    pub fn reload_adapter(&mut self) {
        self.adapter = self.adapter_config.initialize().ok();
    }

    pub fn reload_translator(&mut self) {
        self.translator = self.translator_config.initialize().ok();
    }

    pub fn menu_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu").show(ctx, |ui| {
            bar(ui, |ui| {
                global_dark_light_mode_switch(ui);
                enum_to_selectable(ui, &mut self.gui_mode, GUIMode::iter());
            });
        });
    }

    pub fn status_bar(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("status").show(ctx, |ui| {
            bar(ui, |ui| {
                ui.label(match self.adapter {
                    Some(_) => "Adapter loaded",
                    None => "No adapter loaded",
                });
                ui.label(match self.translator {
                    Some(_) => "Model loaded",
                    None => "No model loaded",
                })
            });
        });
    }

    pub fn text_translate_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            if let (Some(adapter), Some(translator)) =
                (self.adapter.as_ref(), self.translator.as_ref())
            {
                ui.columns(2, |columns| {
                    columns[0].horizontal(|ui| {
                        let from_label = ui.label("From");
                        ComboBox::from_id_source(from_label.id)
                            .selected_text(&self.source_language)
                            .show_ui(ui, |ui| {
                                enum_to_selectable(
                                    ui,
                                    &mut self.source_language,
                                    adapter.available_languages().into_iter(),
                                );
                            });
                    });
                    ScrollArea::vertical()
                        .id_source("source_panel")
                        .show(&mut columns[0], |ui| {
                            ui.centered_and_justified(|ui| {
                                ui.text_edit_multiline(&mut self.source_content);
                            })
                        });
                    columns[1].horizontal(|ui| {
                        let to_label = ui.label("To");
                        ComboBox::from_id_source(to_label.id)
                            .selected_text(&self.target_language)
                            .show_ui(ui, |ui| {
                                enum_to_selectable(
                                    ui,
                                    &mut self.target_language,
                                    adapter.available_languages().into_iter(),
                                );
                            });
                        if ui.button("Translate").clicked() {
                            self.target_content = translator
                                .translate(
                                    adapter,
                                    &self.source_content,
                                    &self.source_language,
                                    &self.target_language,
                                )
                                .unwrap_or_default()
                        }
                    });
                    ScrollArea::vertical()
                        .id_source("target_panel")
                        .show(&mut columns[1], |ui| {
                            let rich_text = RichText::new(&self.target_content);
                            ui.label(rich_text);
                        });
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label(format!(
                        "Please reload the config using the {} panel",
                        GUIMode::Config
                    ));
                });
            }
        });
    }

    pub fn config_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let adapter_label = ui.label("Adapater kind:");
                ComboBox::from_id_source(adapter_label.id)
                    .selected_text(format!("{}", self.adapter_config.kind))
                    .show_ui(ui, |ui| {
                        enum_to_selectable(ui, &mut self.adapter_config.kind, AdapterKind::iter());
                    });
            });
            ui.horizontal(|ui| {
                let adapter_label = ui.label("Adapater source:");
                ui.text_edit_singleline(&mut self.adapter_config.source)
                    .labelled_by(adapter_label.id)
                    .on_hover_ui(|ui| {
                        ui.label(match self.adapter_config.kind {
                            AdapterKind::None => "Not used",
                            AdapterKind::NLLBTokenizerHub => "Identifier of model on Hugging Face (e.g. facebook/nllb-200-distilled-600M)",
                            AdapterKind::NLLBTokenizerLocal => "Path to the local tokenizer weights (e.g. tokenizer.json)",
                        });
                    });
            });
            ui.horizontal(|ui| {
                let model_label = ui.label("Model source:");
                ui.text_edit_singleline(&mut self.translator_config.model_path)
                    .labelled_by(model_label.id)
                    .on_hover_ui(|ui| {
                        ui.label("Path to the directory containing model files");
                    });
            });
            ui.horizontal(|ui| {
                if ui.button("Reload adapter").clicked() {
                    self.reload_adapter();
                }
                if ui.button("Reload translator").clicked() {
                    self.reload_translator();
                }
            })
        });
    }

    pub fn readme_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .id_source("readme_panel")
                .show(ui, |ui| {
                    CommonMarkViewer::new("readme_viewer").show(
                        ui,
                        &mut CommonMarkCache::default(),
                        README,
                    );
                })
        });
    }

    pub fn gui(&mut self, ctx: &Context) {
        self.menu_bar(ctx);
        self.status_bar(ctx);
        match self.gui_mode {
            GUIMode::Translate => self.text_translate_panel(ctx),
            GUIMode::Config => self.config_panel(ctx),
            GUIMode::Readme => self.readme_panel(ctx),
        };
    }
}

impl App for TranslatorGUI {
    fn save(&mut self, storage: &mut dyn Storage) {
        self.save(storage);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.gui(ctx);
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
