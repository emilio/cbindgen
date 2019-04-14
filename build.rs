use std::fs::FileType;

fn generate_tests() {
    use std::env;
    use std::path::{Path, PathBuf};
    use std::fs::{self, File};
    use std::ffi::OsStr;
    use std::io::Write;

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut dst = File::create(Path::new(&out_dir).join("tests.rs")).unwrap();

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let tests_dir = manifest_dir.join("tests").join("rust");
    let tests = fs::read_dir(&tests_dir).unwrap();

    let entries = tests.map(|t| t.expect("Couldn't read test file"));

    println!("cargo:rerun-if-changed={}", tests_dir.display());

    for entry in entries {
        if entry.file_type().unwrap().is_file() {
            // TODO(emilio): handle directories.
            match entry.path().extension().and_then(OsStr::to_str) {
                Some("rs") => {},
                _ => continue,
            };

            let identifier = entry
                .path().file_stem().unwrap().to_os_string()
                .to_str()
                .unwrap()
                .replace(|c| !char::is_alphanumeric(c), "_")
                .replace("__", "_")
                .to_lowercase();
            writeln!(
                dst,
                "test_file!(test_{}, {:?});",
                identifier,
                entry.path(),
            ).unwrap();

        } else {
            let identifier = entry
                .file_name()
                .to_str()
                .unwrap()
                .replace(|c| !char::is_alphanumeric(c), "_")
                .replace("__", "_")
                .to_lowercase();
            writeln!(
                dst,
                "test_dir!(crate_{}, {:?});",
                identifier,
                entry.path(),
            ).unwrap();
        }
    }

    dst.flush().unwrap();
}

fn main() {
    generate_tests();
}
