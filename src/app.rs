/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DSTSMApp {
    // Example stuff:
    label: String,
    create_server: bool,
    edit_server: bool,
    boolean: bool,
    //edit_server: bool,

    // this how you opt-out of serialization of a member

}

impl Default for DSTSMApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "MyDediServer".to_owned(),
            create_server: false,
            edit_server: false,
            boolean: false,
        }
    }
}

impl DSTSMApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}


impl eframe::App for DSTSMApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {label, create_server, edit_server, boolean} = self;
        //label
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save Preset...").clicked() {
                        _frame.close();
                    }
                    if ui.button("Load Preset...").clicked() {
                        _frame.close();
                    }
                    if ui.button("Quit...").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("Create Server", |ui| {
                    _frame.close();
                });
                ui.menu_button("Edit Server", |ui| {
                    _frame.close();
                });                     
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Create New Server");
            ui.horizontal(|ui| {
                ui.label("Enter name for server: ");
                ui.text_edit_singleline(label)
            });

            ui.horizontal(|ui| {
                ui.checkbox(boolean, "Mod Setup");
            });
            
            
            

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Made by ");
                    ui.hyperlink_to("Atmoist", "https://github.com/Atmosphericss");
                    ui.label(".");
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Options...");
            ui.label("Customize your server with the options below.");
            //ui.set_visible(self.mods_enabled);
            ui.group(|ui|{
                ui.heading("Mod Setup Options");
                ui.label("Mod options Go Here");
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
        
    }

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}
