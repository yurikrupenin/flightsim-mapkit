use std::process::Command;
use std::env;

#[cfg(target_os = "windows")]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let cmdline = format!(
        "start /wait cmd /c inliner -n -m --skip-absolute-urls src/webui/index.html ^> {}/index.html",
        out_dir);

    // Please don't ask me about the inner workings of this 
    Command::new("cmd")
        .args(&["/u", "/e:on", "/c", &cmdline])
        .spawn()
        .expect("Unable to complete web resource inlining. Make sure you installed inliner (https://github.com/remy/inliner).");


    println!("cargo:rerun-if-changed=src/webui/*"); 
}

#[cfg(target_os = "linux")]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let cmdline = format!(
        "inliner -n -m --skip-absolute-urls src/webui/index.html > {}/index.html",
        out_dir);

    Command::new("bash")
        .args(&["-c", &cmdline])
        .spawn()
        .expect("Unable to complete web resource inlining. Make sure you installed inliner (https://github.com/remy/inliner).");


    println!("cargo:rerun-if-changed=src/webui/*");     
}

