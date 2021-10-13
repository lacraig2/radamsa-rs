use std::env::var;
use std::process::Command;
use which::which;
use std::path::Path;

// I got a lot of inspiration from this example:
// https://github.com/AFLplusplus/LibAFL/blob/main/libafl_qemu/build.rs
const REPO_URL: &str = "https://gitlab.com/akihe/radamsa/";
const REPO_REV: &str = "ba1b18bbf8f89c9f727030845c7a4e859a0069d8";

fn build_dep_check(tools: &[&str]) {
    for tool in tools {
        which(tool).unwrap_or_else(|_| panic!("Build tool {} not found", tool));
    }
}

fn main() {
    if let Ok(_) = var("DOCS_RS") {
        // no clone for building docs
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");

    let target_os =
        var("CARGO_CFG_TARGET_OS").expect("Could not find CARG_CFG_TARGET_OS");
    if target_os != "linux" {
        return;
    }

    let jobs = var("CARGO_BUILD_JOBS").unwrap_or(String::from("1"));

    build_dep_check(&["make", "git"]);

    let out = var("OUT_DIR").expect("Could not resolve OUT_DIR");
    let out_path = Path::new(&out);

    let radamsa_path = out_path.join("radamsa");

    if !radamsa_path.is_dir(){
        println!("radamsa not found. Cloning with git from {} @ {}", REPO_URL, REPO_REV);
        Command::new("git")
                    .current_dir(&out_path)
                    .arg("clone")
                    .arg(REPO_URL)
                    .status()
                    .unwrap();
        Command::new("git")
                    .current_dir(&out_path)
                    .arg("checkout")
                    .arg(REPO_REV)
                    .status()
                    .unwrap();
    }

    let out_lib = out_path.join("radamsa/lib/libradamsa.a");

    if !out_lib.is_file(){
        println!("building libradamsa.a");
        Command::new("make")
            .arg("-j")
            .arg(format!("{}",&jobs))
            .current_dir(&radamsa_path)
            .output()
            .expect("make in radamsa failed");
        Command::new("make")
            .arg("-j")
            .arg(format!("{}", &jobs))
            .arg("lib/libradamsa.a")
            .current_dir(&radamsa_path)
            .output()
            .expect("make in radamsa failed");
    }

    println!("cargo:rustc-link-search={}/lib/", radamsa_path.display());
    println!("cargo:rustc-link-lib=static=radamsa");
}
