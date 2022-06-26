use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = Path::new("./target");
    let build_profile = env::var_os("PROFILE").unwrap();
    let dest_path = Path::new(&out_dir).join(build_profile).join("resources");
    fs::create_dir_all(&dest_path).expect(
        format!(
            "Failed to create resources directory {}",
            dest_path.display()
        )
        .as_str(),
    );
    let resource_files = fs::read_dir("./resources").unwrap();
    for resource_file in resource_files {
        if let Some(file) = resource_file.ok() {
            if file
                .metadata()
                .map(|metadata| metadata.is_file())
                .unwrap_or(false)
            {
                let dest_file = dest_path.join(file.file_name());
                eprintln!("{}", dest_file.display());
                fs::copy(file.path(), dest_file.as_path()).expect(
                    format!("Failed to copy resource file {}", file.path().display()).as_str(),
                );
                println!("Copied resource file {} to {}", file.path().display(), dest_file.display());
            }
        }
    }
    println!("cargo:rerun-if-changed=build.rs");
}
