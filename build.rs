use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();
    let sounds_target = target_dir.join("sounds");
    
    if Path::new("sounds").exists() {
        if sounds_target.exists() {
            let _ = fs::remove_dir_all(&sounds_target);
        }
        
        copy_dir_recursive("sounds", &sounds_target).unwrap_or_else(|e| {
            eprintln!("Warning: Failed to copy sounds directory: {}", e);
        });
    }
    
    println!("cargo:rerun-if-changed=sounds");
}

fn copy_dir_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_recursive(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}