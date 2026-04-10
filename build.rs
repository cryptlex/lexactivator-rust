use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::fs;
use std::io;
use ureq;
use zip;
/// Since the build.rs is supposed to run on the host machine, cross-compiling this package on a single host machine will lead to unexpected results.

/// Targets = x86_64-apple-darwin, aarch64-apple-darwin, aarch64-unknown-linux-gnu, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu,
/// x86_64-unknown-linux-musl, aarch64-unknown-linux-musl

fn download_and_setup_libs() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let libs_dir = PathBuf::from(&out_dir).join("libs");
    
    
    // Create libs directory if it doesn't exist
    if !libs_dir.exists() {
        fs::create_dir_all(&libs_dir)?;
    }

    let version = "v3.41.0";
    let base_url = "https://dl.cryptlex.com/downloads";
    
    // Determine which library to download based on target
    #[cfg(all(target_arch="x86_64", target_os="macos"))]
    let (zip_filename, lib_path) = ("LexActivator-Static-Mac.zip", "clang/x86_64/libLexActivator.a");
    
    #[cfg(all(target_arch="aarch64", target_os="macos"))]
    let (zip_filename, lib_path) = ("LexActivator-Static-Mac.zip", "clang/arm64/libLexActivator.a");
    
    #[cfg(all(target_arch="aarch64", target_os="linux", target_env="gnu"))]
    let (zip_filename, lib_path) = ("LexActivator-Static-Linux.zip", "gcc-6/arm64/libLexActivator.a");
    
    #[cfg(all(target_arch="x86_64", target_os="linux", target_env="gnu"))]
    let (zip_filename, lib_path) = ("LexActivator-Static-Linux.zip", "gcc-6/amd64/libLexActivator.a");
    
    #[cfg(all(target_arch="x86", target_os="linux", target_env="gnu"))]
    let (zip_filename, lib_path) = ("LexActivator-Static-Linux.zip", "gcc-6/i386/libLexActivator.a");
    
    #[cfg(all(target_arch="x86_64", target_os="linux", target_env="musl"))]
    let (zip_filename, lib_path) = ("LexActivator-Static-Linux.zip", "musl/amd64/libLexActivator.a");
    
    #[cfg(all(target_arch="aarch64", target_os="linux", target_env="musl"))]
    let (zip_filename, lib_path) = ("LexActivator-Static-Linux.zip", "musl/arm64/libLexActivator.a");
    
    #[cfg(all(target_arch="x86_64", target_os="windows", target_env="msvc"))]
    let (zip_filename, lib_path) = ("LexActivator-Win.zip", "vc14/x64/LexActivator.lib");
    
    #[cfg(all(target_arch="x86", target_os="windows", target_env="msvc"))]
    let (zip_filename, lib_path) = ("LexActivator-Win.zip", "vc14/x86/LexActivator.lib");

    // Create target-specific directory
    let target_dir = get_target_lib_dir(&out_dir);
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    // Check if library already exists
    #[cfg(target_os="windows")]
    let lib_file = target_dir.join("LexActivator.lib");
    
    #[cfg(not(target_os="windows"))]
    let lib_file = target_dir.join("libLexActivator.a");

    if !lib_file.exists() {
        println!("cargo:warning=Downloading LexActivator library for target...");
        
        // Download the library
        let download_url = format!("{}/{}/{}", base_url, version, zip_filename);
        let temp_dir = PathBuf::from(&out_dir).join("tmp");
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir)?;
        }

        // Download and extract
        let zip_path = temp_dir.join(zip_filename);
        download_file(&download_url, &zip_path)?;
        
        // Extract the specific library file
        extract_library(&zip_path, &lib_path, &target_dir)?;
        let _ = extract_library(&zip_path, "THIRD-PARTY-NOTICES.txt", &target_dir);
        
        // Clean up
        fs::remove_file(zip_path)?;
        fs::remove_dir_all(temp_dir)?;
    }

    Ok(())
}

fn get_target_lib_dir(out_dir: &str) -> PathBuf {
    let mut path = PathBuf::from(out_dir);
    path.extend(&["libs"]);
    
    cfg_if::cfg_if! {
        if #[cfg(all(target_arch="x86_64", target_os="macos"))] {
            path.extend(&["darwin-x86_64"]);
        } else if #[cfg(all(target_arch="aarch64", target_os="macos"))] {
            path.extend(&["darwin-aarch64"]);
        } else if #[cfg(all(target_arch="aarch64", target_os="linux", target_env="gnu"))] {
            path.extend(&["linux-aarch64"]);
        } else if #[cfg(all(target_arch="x86_64", target_os="linux", target_env="gnu"))] {
            path.extend(&["linux-x86_64"]);
        } else if #[cfg(all(target_arch="x86", target_os="linux", target_env="gnu"))] {
            path.extend(&["linux-x86"]);
        } else if #[cfg(all(target_arch="x86_64", target_os="linux", target_env="musl"))] {
            path.extend(&["musl-x86_64"]);
        } else if #[cfg(all(target_arch="aarch64", target_os="linux", target_env="musl"))] {
            path.extend(&["musl-aarch64"]);
        } else if #[cfg(all(target_arch="x86_64", target_os="windows", target_env="msvc"))] {
            path.extend(&["win32-x86_64"]);
        } else if #[cfg(all(target_arch="x86", target_os="windows", target_env="msvc"))] {
            path.extend(&["win32-x86"]);
        }
    }
    path
}

fn download_file(url: &str, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let response = ureq::get(url).call()?;
    let mut file = fs::File::create(path)?;
    io::copy(&mut response.into_reader(), &mut file)?;
    Ok(())
}

fn extract_library(zip_path: &PathBuf, lib_path: &str, target_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name();
        
        if name.ends_with(lib_path) {
            let filename = std::path::Path::new(name).file_name().unwrap();
            let outpath = target_dir.join(filename);
            
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
            break;
        }
    }
    Ok(())
}

fn set_lib_search_dir() -> () {
    let out_dir: String = env::var("OUT_DIR").unwrap();

    fn print_link_search_path(base_dir: String, extension_path: &[&str]) {
        let mut path = PathBuf::from(base_dir.clone());
        path.extend(extension_path);
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    cfg_if::cfg_if! {
        if #[cfg(all(target_arch="x86_64", target_os="macos"))] {
            // Intel Macs
            print_link_search_path(out_dir, &["libs", "darwin-x86_64"]);
        } else if #[cfg(all(target_arch="aarch64", target_os="macos"))] {
            // Apple Silicon Macs
            print_link_search_path(out_dir, &["libs", "darwin-aarch64"]);
        } else if #[cfg(all(target_arch="aarch64", target_os="linux", target_env="gnu"))] {
            // ARM64 Linux GNU
            print_link_search_path(out_dir, &["libs", "linux-aarch64"]);
        } else if #[cfg(all(target_arch="x86_64", target_os="linux", target_env="gnu"))] {
            // x86_64 Linux GNU
            print_link_search_path(out_dir, &["libs", "linux-x86_64"]);
        } else if #[cfg(all(target_arch="x86", target_os="linux", target_env="gnu"))] {
            // x86 Linux GNU
            print_link_search_path(out_dir, &["libs", "linux-x86"]);
        } else if #[cfg(all(target_arch="x86_64", target_os="linux", target_env="musl"))] {
            // x86_64 Linux MUSL
            print_link_search_path(out_dir, &["libs", "musl-x86_64"]);
        } else if #[cfg(all(target_arch="aarch64", target_os="linux", target_env="musl"))] {
            // ARM64 Linux MUSL
            print_link_search_path(out_dir, &["libs", "musl-aarch64"]);
        } else if #[cfg(all(target_arch="x86_64", target_os="windows", target_env="msvc"))] {
            // x86_64 Windows
            // TODO find MSVC Version
            print_link_search_path(out_dir, &["libs", "win32-x86_64"]);
        } else if #[cfg(all(target_arch="x86", target_os="windows", target_env="msvc"))] {
            // x86 Windows
            // TODO find MSVC Version
            print_link_search_path(out_dir, &["libs", "win32-x86"]);
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

    // Add linkage instruction to LexActivator library
    println!("cargo:rustc-link-lib=LexActivator");
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
            // Static linking is not supported
            // println!("cargo:rustc-link-lib=winhttp");
            // println!("cargo:rustc-link-lib=crypt32");
            // println!("cargo:rustc-link-lib=libcurl_MD");
            // println!("cargo:rustc-link-lib=ws2_32");
            // println!("cargo:rustc-link-lib=shell32");
        }
    }
}
fn main() {
    // Download libraries if needed
    if let Err(e) = download_and_setup_libs() {
        eprintln!("Failed to download libraries: {}", e);
        std::process::exit(1);
    }
    set_lib_search_dir();
    set_libs_to_link();
}
