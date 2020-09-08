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

    // Please don't ask me about this either
    let command = format!("eval \"cat index.html | inliner -n -m --skip-absolute-urls > {}/index.html\"", out_dir);
    

    let mut child = Command::new("bash")
        .current_dir("src/webui")
        .args(&["-c", &command])
        .spawn()
        .expect("Unable to complete web resource inlining. Make sure you installed inliner (https://github.com/remy/inliner).");

    child.wait().expect("Failed to generate HTML output.");

    println!("cargo:rerun-if-changed=src/webui/*");     
}

