use std::env;
use std::process::Command;

fn main() {
    // Read environment variables
    let uefi_path = env::var("UEFI_PATH").expect("UEFI_PATH not set");
    let bios_path = env::var("BIOS_PATH").expect("BIOS_PATH not set");

    println!("UEFI Path: {}", uefi_path);
    println!("BIOS Path: {}", bios_path);

    // Define the QEMU path (Use absolute path for Windows)
    #[cfg(target_os = "windows")]
    let qemu_path = r"C:\Program Files\qemu\qemu-system-x86_64.exe"; // Adjust if needed

    #[cfg(not(target_os = "windows"))]
    let qemu_path = "qemu-system-x86_64"; // Use default QEMU command on Linux/macOS

    // Ensure BIOS/UEFI file exists before running
    if !std::path::Path::new(&bios_path).exists() {
        eprintln!("Error: BIOS file does not exist at '{}'", bios_path);
        std::process::exit(1);
    }

    if !std::path::Path::new(&uefi_path).exists() {
        eprintln!("Error: UEFI file does not exist at '{}'", uefi_path);
        std::process::exit(1);
    }

    let uefi = false; // Change to `true` to boot using UEFI

    let mut cmd = Command::new(qemu_path);

    if uefi {
        cmd.arg("-bios")
            .arg(ovmf_prebuilt::ovmf_pure_efi()) // Ensure ovmf_prebuilt is properly set up
            .arg("-drive")
            .arg(format!("format=raw,file={}", uefi_path));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={}", bios_path));
    }

    match cmd.spawn() {
        Ok(mut child) => {
            println!("QEMU started successfully!");
            child.wait().expect("Failed to wait on QEMU");
        }
        Err(e) => {
            eprintln!("Failed to start QEMU: {:?}", e);
            std::process::exit(1);
        }
    }
}
