use std::path::PathBuf;

const LIB_ATOMIC_OPS_DIR: &str = "vendor/libatomic_ops";
const LIB_GC_DIR: &str = "vendor/bdwgc";

#[cfg(feature = "autotools")]
fn main() {
    for dir in &[LIB_ATOMIC_OPS_DIR, LIB_GC_DIR] {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("cd {} && ./autogen.sh", dir))
            .output()
            .unwrap();
    }

    let dst = autotools::Config::new(LIB_ATOMIC_OPS_DIR)
        .cflag("-fPIC")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=atomic_ops");

    let dst = autotools::Config::new(LIB_GC_DIR)
        .cflag(format!(
            "-I{} -L/lib/x86_64-linux-gnu -lpthread -fPIC",
            dst.join("include").display()
        ))
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=gc");

    for dir in &[LIB_ATOMIC_OPS_DIR, LIB_GC_DIR] {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("cd {} && git clean -dfx", dir))
            .output()
            .unwrap();
    }
}

#[cfg(feature = "cmake")]
fn main() {
    use cmake::Config;

    #[cfg(windows)]
    {
        use std::path::Path;
        if !Path::new(LIB_GC_DIR).join("libatomic_ops").exists() {
            panic!("TODO: find a nice solution to put libatomic_ops as a subdir into bdwgc")
        }
    }

    let dst = Config::new(LIB_GC_DIR)
        .profile("Release")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=gc");
}
