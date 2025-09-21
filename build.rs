use std::process::Command;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=src/icon.png");
    fs::create_dir_all("target").unwrap();
    let output = Command::new("nwlink.cmd")
        .args(&["png-nwi", "src/icon.png", "target/icon.nwi"])
        .output().expect("Failure to launch process");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}