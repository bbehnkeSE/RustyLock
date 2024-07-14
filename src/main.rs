// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::PathBuf;

use eframe::egui;
use locker::{encrypt, decrypt, gen_key};

mod locker;


fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 640.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native("RustyLock", native_options, Box::new(
        |_cc| Ok(Box::<RustyLock>::default()))).expect("Uh oh...");

    // // CLI Code
    // let args: Vec<String> = env::args().collect();
    // let mut opt: Options = { 
    //     Options {
    //         control: Control::Decrypt,
    //         path: "".to_string()
    //     }
    // };
    // for (i, arg) in args.iter().enumerate() {
    //     if arg == "-e" {
    //         opt.control = Control::Encrypt;
    //     }
    //     if arg == "-d" {
    //         opt.control = Control::Decrypt;
    //     }
    //     if arg == "-p" {
    //         opt.path = args[i + 1].clone();
    //     }
    // }

    // let pth      = Path::new(&opt.path);
    // let password = rpassword::prompt_password("Enter password: ").unwrap();
    // let key      = locker::gen_key(password.as_bytes());

    // match opt.control {
    //     Control::Encrypt => {
    //         locker::encrypt(&key, pth)
    //             .expect("encrypt failed.");
    //     }
    //     Control::Decrypt => {
    //         locker::decrypt(&key, pth)
    //             .expect("decrypt failed.");
    //     }
    // }
}


#[derive(Debug, Default)]
enum Control {
    #[default]
    Encrypt,
    Decrypt
}


#[derive(Debug, Default)]
struct Options {
    control: Control,
    path:    Vec<PathBuf>,
}


#[derive(Default)]
struct RustyLock {
    dropped_files:        Vec<egui::DroppedFile>,
    show_password_window: bool,
    enable_buttons:       bool,
    password:             String,
    opts:                 Options
}

impl RustyLock {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        return Self::default();
    }
}

impl eframe::App for RustyLock {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.dropped_files.is_empty() {
            self.enable_buttons = false;
        } else {
            self.enable_buttons = true;
        }

        egui::TopBottomPanel::bottom("top_bot").show(ctx, |ui| {
            ui.add_enabled_ui(self.enable_buttons, |ui| {
                if ui.button("Encrypt").clicked() {
                    self.show_password_window = true;
                    self.opts.control = Control::Encrypt;
                }
                if ui.button("Decrypt").clicked() {
                    self.show_password_window = true;
                    self.opts.control = Control::Decrypt;
                }
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drag and drop files.");

            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");
                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            if !self.opts.path.contains(&path.to_path_buf()) {
                                self.opts.path.push(path.to_path_buf());
                            }
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };

                        let mut additional_info = vec![];
                        if !file.mime.is_empty() {
                            additional_info.push(format!("type {}", file.mime));
                        }
                        if let Some(bytes) = &file.bytes {
                            additional_info.push(format!("{} bytes", bytes.len()));
                        }
                        if !additional_info.is_empty() {
                            info += &format!(" ({})", additional_info.join(", "));
                        }

                        ui.label(info);
                    }
                });
            }
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        });

        if self.show_password_window {
            egui::Window::new("Enter password")
                .show(ctx, |ui| {
                    ui.add_space(10.0);
                    ui.label("Enter a password:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.password)
                            .password(true)
                    );
                    // ui.text_edit_singleline(&mut self.password);
                    ui.vertical_centered_justified(|ui| {
                        ui.add_space(20.0);
                        if ui.button("Submit").clicked() {
                            self.show_password_window = false;

                            match self.opts.control {
                                Control::Encrypt => {
                                    for file in self.opts.path.clone() {
                                        encrypt(&gen_key(self.password.as_bytes()), &file)
                                            .expect("Encryption failed.");
                                    }
                                }
                                Control::Decrypt => {
                                    for file in self.opts.path.clone() {
                                        decrypt(&gen_key(self.password.as_bytes()), &file)
                                            .expect("Decryption failed.");
                                    }
                                }
                            }

                            self.password = "".to_string();
                            self.opts.path.clear();
                        }
                        if ui.button("Cancel").clicked() {
                            self.password = "".to_string();
                            self.show_password_window = false;
                        }
                    })
                });
        }
    }
}


fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE
        );
    }
}