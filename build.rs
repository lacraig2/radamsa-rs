use std::env::var;
use std::process::Command;

fn main() {
    let submodule_path =
        format!("{}/radamsa", &var("CARGO_MANIFEST_DIR").unwrap());
    
    Command::new("make")
        .current_dir(&submodule_path)
        .output()
        .expect("make in radamsa failed");
    Command::new("make")
        .arg("lib/libradamsa.a")
        .current_dir(&submodule_path)
        .output()
        .expect("make in radamsa failed");
    
    println!("cargo:rustc-link-search={}/lib/", submodule_path);
    println!("cargo:rustc-link-lib=static=radamsa");


}
