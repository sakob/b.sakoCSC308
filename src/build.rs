use std::path::PathBuf;

fn main() {
    // Print the OUT_DIR environment variable, set by Cargo for build outputs
    println!("std::env::var_os('OUT_DIR') = {:?}", std::env::var_os("OUT_DIR").unwrap());

    // Optional: If you want to print all environment variables
    // for (key, value) in std::env::vars_os() {
    //     println!("{key:?}: {value:?}");
    // }

    // Get the output directory where Cargo places build artifacts
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // Get the path to the kernel file (created with bootloader)
    let kernel = PathBuf::from(std::env::var_os("CARGO_BIN_FILE_KERNEL_WITH_BOOTLOADER").unwrap());

    // Create a UEFI disk image (optional)
    let uefi_path = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(&uefi_path)
        .unwrap();

    // Create a BIOS disk image
    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .unwrap();

    // Pass the disk image paths as environment variables to main.rs
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}
