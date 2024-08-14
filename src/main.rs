#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::env;
use std::path::PathBuf;
use std::process;

use eframe::egui;

mod locker;
use locker::{aes, gui};



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {  
        // No args, run with GUI
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([640.0, 640.0])
                .with_drag_and_drop(true),
            ..Default::default()
        };
        eframe::run_native("RustyLock", native_options, Box::new(
            |_cc| Ok(Box::<gui::RustyLock>::default()))).expect("Uh oh...");

    } else {  
        // CLI Code
        let mut opt: gui::Options = { 
            gui::Options {
                control: gui::Control::Encrypt,
                paths: vec![PathBuf::from("")]
            }
        };
        for (i, arg) in args.iter().enumerate() {
            if arg == "-e" {
                opt.control = gui::Control::Encrypt;
            }
            if arg == "-d" {
                opt.control = gui::Control::Decrypt;
            }
            if arg == "-p" {
                opt.paths[0] = PathBuf::from(args[i + 1].clone());
            }
            if arg == "-h" {
                println!("\nOptions:");
                println!("  '-e' sets control to \"Encrypt\"");
                println!("  '-d' sets control to \"Decrypt\"");
                println!("  '-p <file_path> designates the path perform control operation (default: \"Encrypt\"\n");
                println!("Running without command-line arguments will open the GUI.\n");

                process::exit(0);
            }
        }

        let pth      = &opt.paths[0];
        let password = rpassword::prompt_password("Enter password: ").unwrap();
        let key      = aes::gen_key(password.as_bytes());

        match opt.control {
            gui::Control::Encrypt => {
                let password_confirm = rpassword::prompt_password("Confirm password: ").unwrap();
                if password == password_confirm {
                    let enc_result = aes::encrypt(&key, &pth);
                    match enc_result {
                        Ok(_) => (),
                        Err(e) => fs::write("./rusty_error.log", e.to_string())
                            .expect("Failed to write to error log")
                    }
                } else {
                    println!("Passwords do not match!");
                    process::exit(1);
                }
            }
            gui::Control::Decrypt => {
                let dec_result = aes::decrypt(&key, &pth);
                match dec_result {
                    Ok(_) => (),
                    Err(e) => fs::write("./rusty_error.log", e.to_string())
                        .expect("Failed to write to error log")
                }
            }
        }
    }
}