use std::env;
use std::path::PathBuf;
use std::process::Command;
/// Since the build.rs is supposed to run on the host machine, cross-compiling this package on a single host machine will lead to unexpected results.

/// Targets = x86_64-apple-darwin, aarch64-apple-darwin, aarch64-unknown-linux-gnu, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu,
/// x86_64-unknown-linux-musl, aarch64-unknown-linux-musl
fn set_lib_search_dir() -> () {
    let cargo_manifest_dir: String = env::var("CARGO_MANIFEST_DIR").unwrap();

    fn print_link_search_path(base_dir: String, extension_path: &[&str]) {
        let mut path = PathBuf::from(base_dir.clone());
        path.extend(extension_path);
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    cfg_if::cfg_if! {
        if #[cfg(all(target_arch="x86_64", target_os="macos"))] {
            // Intel Macs
            print_link_search_path (cargo_manifest_dir, &["libs", "darwin-x86-64"]);
        } else if #[cfg(all(target_arch="aarch64", target_os="macos"))] {
            // Apple Silicon Macs
            print_link_search_path(cargo_manifest_dir, &["libs", "darwin-aarch64"]);
        } else if #[cfg(all(target_arch="aarch64", target_os="linux"))] {
            // ARM64 Linux GNU
            print_link_search_path(cargo_manifest_dir, &["libs", "linux-aarch64"]);
        } else if #[cfg(all(target_arch="x86_64", target_os="linux", target_env="gnu"))] {
            // x86_64 Linux GNU
            print_link_search_path(cargo_manifest_dir, &["libs", "linux-x86-64"]);
        } else if #[cfg(all(target_arch="x86", target_os="linux", target_env="gnu"))] {
            // x86 Linux GNU
            print_link_search_path(cargo_manifest_dir, &["libs", "linux-x86"]);
            // /libs/linux/gcc-6/i386
        } else if #[cfg(all(target_arch="x86_64", target_os="linux", target_env="musl"))] {
            // x86_64 Linux MUSL
            print_link_search_path(cargo_manifest_dir, &["libs", "musl-x86-64"]);
        } else if #[cfg(all(target_arch="aarch64", target_os="linux", target_env="musl"))] {
            // ARM64 Linux MUSL
            print_link_search_path(cargo_manifest_dir, &["libs", "musl-aarch64"]);
        }  else if #[cfg(all(target_arch="x86_64", target_os="windows", target_env="msvc"))] {
            // x86_64 Windows
            // TODO find MSVC Version
            print_link_search_path(cargo_manifest_dir, &["libs", "win32-x86-64"]);
        } else if #[cfg(all(target_arch="x86", target_os="windows", target_env="msvc"))] {
            // x86 Windows
            // TODO find MSVC Version
            print_link_search_path(cargo_manifest_dir, &["libs", "win32-x86"]);
        } else {
            // TODO make this message more verbose
            // const TARGET_OS = env::var("CARGO_CFG_TARGET_OS").ok();
            // const TARGET_ARCH = env::var("CARGO_CFG_TARGET_ARCH").ok();
            // const TARGET_VENDOR = env::var("CARGO_CFG_TARGET_VENDOR").ok();
            // const TARGET_ENV = env::var("CARGO_CFG_TARGET_ENV").ok();

            // Unsupported target
            compile_error!("The build target is not supported by LexActivator.");
        }
    }

    // Add linkage instruction to static library
    println!("cargo:rustc-link-lib=static=LexActivator");
}

fn set_libs_to_link() {
    cfg_if::cfg_if! {
        if #[cfg(target_os="macos")] {
            println!("cargo:rustc-link-lib=dylib=c++");
            
            // Find correct directory for clang_rt.osx (using clang search dirs)
            let output = Command::new("sh")
                .arg("-c")
                .arg("clang --print-search-dirs | grep -E 'libraries' | cut -d'=' -f2")
                .output()
                .expect("LexActivator failed to find clang libraries. Please ensure that clang is installed.");

            let path = String::from_utf8_lossy(&output.stdout);
            let clang_libs_path = format!("{}/lib/darwin/", path.trim());
            println!("cargo:rustc-link-search=native={}", clang_libs_path);
            
            // Link clang_rt.osx
            println!("cargo:rustc-link-lib=static=clang_rt.osx");
            
            println!("cargo:rustc-link-lib=framework=Security");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=SystemConfiguration");
        } else if #[cfg(target_os="linux")] {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        } else if #[cfg(target_os="windows")] {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }
    }
}
fn main() {
    set_lib_search_dir();
    set_libs_to_link();
}
