#![no_std] // Indicates to the Rust compiler that the app does not depend on the standard library but is a 'standalone' application.
#![no_main] // Indicates that this application does not have a "main" function typically found in a Linux or Windows application (although it does have its own "main" function "efi_main" as declared below)
#![feature(alloc_error_handler)] // Needed for the alloc error handler function declared below since this feature is unstable.

// Externs for efi and alloc crates (alloc crate is the one that contains definitions of String and Vec etc.)
#[macro_use] extern crate efi;
#[macro_use] extern crate alloc;


// EFI entrypoint or main function. UEFI firmware will call this function to start the application.
// The signature and the name of this function must be exactly as below.
#[no_mangle]
pub extern "win64" fn efi_main(image_handle: efi::ffi::EFI_HANDLE, sys_table : *const efi::ffi::EFI_SYSTEM_TABLE) -> isize {
    efi::init_env(image_handle, sys_table); // Call to init_env must be the first thing in efi_main. Without it things like println!() won't work

    println!("Welcome to UEFI");

    // Your business logic here

    0
}

// A handler to respond to panics in the code. Required by the Rust compiler
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// A handler to respond to allocation failures. Required by the Rust compiler
#[alloc_error_handler]
fn alloc_error(_: core::alloc::Layout) -> ! {
    loop {}
}
