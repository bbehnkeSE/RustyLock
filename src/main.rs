use std::env;
use std::path::Path;

pub mod locker;

#[derive(Debug)]
enum Control {
    Encrypt,
    Decrypt
}


#[derive(Debug)]
struct Options {
    control: Control,
    path:    String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opt: Options = { 
        Options {
            control: Control::Decrypt,
            path: "".to_string()
        }
    };
    for (i, arg) in args.iter().enumerate() {
        if arg == "-e" {
            opt.control = Control::Encrypt;
        }
        if arg == "-d" {
            opt.control = Control::Decrypt;
        }
        if arg == "-p" {
            opt.path = args[i + 1].clone();
        }
    }

    let pth      = Path::new(&opt.path);
    let password = "Password".to_string();
    let key      = locker::gen_key(password.as_bytes());

    match opt.control {
        Control::Encrypt => {
            locker::encrypt(&key, pth)
                .expect("encrypt failed.");
        }
        Control::Decrypt => {
            locker::decrypt(&key, pth)
                .expect("decrypt failed.");
        }
    }
}