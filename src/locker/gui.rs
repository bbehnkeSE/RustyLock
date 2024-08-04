use std::path::PathBuf;
use eframe::egui::{self, Color32};
use std::fs;

use crate::aes;

#[derive(Default)]
pub struct RustyLock {
    dropped_files:        Vec<egui::DroppedFile>,
    show_password_window: bool,
    enable_buttons:       bool,
    password:             String,
    password_confirm:     String,
    opts:                 Options
}

#[derive(Debug, Default)]
pub enum Control {
    #[default]
    Encrypt,
    Decrypt
}


#[derive(Debug, Default)]
pub struct Options {
    pub control: Control,
    pub paths:   Vec<PathBuf>,
}

// impl RustyLock {
//     fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
//         // Restore app state using cc.storage (requires the "persistence" feature).
//         // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
//         // for e.g. egui::PaintCallback.
//         return Self::default();
//     }
// }

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
                            if !self.opts.paths.contains(&path.to_path_buf()) {
                                self.opts.paths.push(path.to_path_buf());
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
            match self.opts.control {
                Control::Encrypt => {
                    egui::Window::new("Enter password")
                        .show(ctx, |ui| {
                            ui.add_space(10.0);
                            ui.label("Enter a password:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.password)
                                    .password(true)
                            );
                            ui.add_space(10.0);
                            ui.label("Confirm password:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.password_confirm)
                                    .password(true)
                            );

                            let passwords_match: bool = self.password == self.password_confirm && self.password.len() > 0;
                            let label_text = if passwords_match {
                                "Passwords match!"
                            } else {
                                "Passwords do not match"
                            };
                            let label_color = if passwords_match {
                                Color32::GREEN
                            } else {
                                Color32::RED
                            };

                            ui.colored_label(label_color, label_text);

                            ui.vertical_centered_justified(|ui| {
                                ui.add_enabled_ui(passwords_match, |ui| {
                                    ui.add_space(20.0);
                                    if ui.button("Submit").clicked() {
                                        self.show_password_window = false;
                                        for path in self.opts.paths.clone() {
                                            let enc_result = aes::encrypt(
                                                &aes::gen_key(&self.password.as_bytes()), &path
                                            );
                                            
                                            match enc_result {
                                                Ok(_) => (),
                                                Err(e) => fs::write("./rusty_error.log", e.to_string())
                                                    .expect("Failed to write to error log")
                                            }
                                        }
                                        self.password = String::from("");
                                        self.password_confirm = String::from("");
                                        self.opts.paths.clear();
                                    }
                                    if ui.button("Cancel").clicked() {
                                        self.password = String::from("");
                                        self.show_password_window = false;
                                    }
                                })
                            })
                        });
                }
                Control::Decrypt => {
                    egui::Window::new("Enter password")
                        .show(ctx, |ui| {
                            ui.add_space(10.0);
                            ui.label("Enter a password:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.password)
                                    .password(true)
                            );
                            ui.vertical_centered_justified(|ui| {
                                ui.add_space(20.0);
                                if ui.button("Submit").clicked() {
                                    self.show_password_window = false;

                                    for path in self.opts.paths.clone() {
                                        let dec_result = aes::decrypt(
                                            &aes::gen_key(&self.password.as_bytes()), &path
                                        );
                                        match dec_result {
                                            Ok(_) => (),
                                            Err(e) => fs::write("./rusty_error.log", e.to_string())
                                                .expect("Failed to write to error log")
                                        }
                                    }
                                    self.password = String::from("");
                                    self.opts.paths.clear();
                                }
                                if ui.button("Cancel").clicked() {
                                    self.password = String::from("");
                                    self.show_password_window = false;
                                }
                            })
                        });
                }
            }

            // egui::Window::new("Enter password")
            //     .show(ctx, |ui| {
            //         ui.add_space(10.0);
            //         ui.label("Enter a password:");
            //         ui.add(
            //             egui::TextEdit::singleline(&mut self.password)
            //                 .password(true)
            //         );
            //         // ui.text_edit_singleline(&mut self.password);
            //         ui.vertical_centered_justified(|ui| {
            //             ui.add_space(20.0);
            //             if ui.button("Submit").clicked() {
            //                 self.show_password_window = false;

            //                 match self.opts.control {
            //                     Control::Encrypt => {
            //                         for path in self.opts.paths.clone() {
            //                             let enc_result = aes::encrypt(&aes::gen_key(self.password.as_bytes()), &path);
            //                             match enc_result {
            //                                 Ok(_) => (),
            //                                 Err(e) => fs::write("./rusty_error.log", e.to_string())
            //                                     .expect("Failed to write to error log")
            //                             }
            //                         }
            //                     }
            //                     Control::Decrypt => {
            //                         for path in self.opts.paths.clone() {
            //                             let dec_result = aes::decrypt(&aes::gen_key(self.password.as_bytes()), &path);
            //                             match dec_result {
            //                                 Ok(_) => (),
            //                                 Err(e) => fs::write("./rusty_error.log", e.to_string())
            //                                     .expect("Failed to write to error log")
            //                             }
            //                         }
            //                     }
            //                 }
            //                 self.password = "".to_string();
            //                 self.opts.paths.clear();
            //             }
            //             if ui.button("Cancel").clicked() {
            //                 self.password = "".to_string();
            //                 self.show_password_window = false;
            //             }
            //         })
            //     });
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