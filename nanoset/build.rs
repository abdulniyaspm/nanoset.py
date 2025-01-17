extern crate built;

fn main() {
    if std::env::var("CARGO_FEATURE_EXTENSION_MODULE").is_ok() {
        let src = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("built.rs");
        let mut opts = built::Options::default();
        opts.set_dependencies(true).set_compiler(true).set_env(true);
        built::write_built_file_with_opts(&opts, src, dst)
        .expect("Failed to acquire build-time information");
    }
}
