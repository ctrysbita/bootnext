#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();

    res.set_manifest_file("app.manifest");

    if let Err(e) = res.compile() {
        eprintln!("Failed to compile windows resource: {}", e);
        std::process::exit(1);
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
