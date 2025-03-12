use std::{env, fs, path::Path, process::Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated");

    fs::create_dir_all(&dest_path).unwrap();

    let shdc_path = match which::which("sokol-shdc") {
        Ok(path) => path,
        Err(_) => panic!("sokol-shdc not found in PATH. please install and rebuild."),
    };

    let input_shader = "shaders/basic.glsl";
    let output_file = dest_path.join("shader.rs");

    let status = Command::new(shdc_path)
        .args(&[
            "-i",
            input_shader,
            "-o",
            output_file.to_str().unwrap(),
            "-l",
            "glsl430:metal_macos:hlsl5",
            "-f",
            "sokol_rust",
        ])
        .status()
        .expect("failed to execute sokol-shdc");

    if !status.success() {
        panic!("shader compilation failed");
    }

    println!("cargo:rerun-if-changed={}", input_shader);
}
