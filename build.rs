use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("logos.rs");

    let mut match_arms = String::new();
    let ascii_dir = Path::new("ascii");

    if ascii_dir.exists() {
        for entry in fs::read_dir(ascii_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            
            if path.is_file() && path.extension().unwrap_or_default() == "txt" {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();

                let raw_logo = fs::read_to_string(&path).unwrap_or_default();
                let clean_logo = raw_logo
                    .replace("$1", "")
                    .replace("$2", "")
                    .replace("$3", "")
                    .replace("$4", "")
                    .replace("$5", "")
                    .replace("$6", "")
                    .replace("$7", "")
                    .replace("$8", "")
                    .replace("$9", "");

                // Upgraded to 15 hashes to prevent ASCII art from prematurely closing the string!
                match_arms.push_str(&format!(
                    "        \"{file_stem}\" => r###############\"\n{clean_logo}\"###############,\n"
                ));
            }
        }
    }

    // Upgraded the fallback logo's hashes here too
    let generated_code = format!(
        "pub fn get_logo(os_id: &str) -> &'static str {{\n    match os_id {{\n{match_arms}        _ => r###############\"\n    .--.\n   |o_o |\n   |:_/ |\n  //   \\ \\\n (|     | )\n/'\\_   _/`\\\n\\___)=(___/\"###############,\n    }}\n}}"
    );

    fs::write(&dest_path, generated_code).unwrap();
    println!("cargo:rerun-if-changed=ascii");
}
