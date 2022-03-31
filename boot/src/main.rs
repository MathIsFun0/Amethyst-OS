#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::prelude::{SystemTable, Boot, entry};
use uefi::{Status, CStr16};
use uefi::data_types::Handle;
use uefi_services::*;

pub fn load_kernel(system_table: SystemTable<Boot>) {
    
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let mut out = system_table.stdout();
    let mut buf = [0; 11];
    let cstr = CStr16::from_str_with_buf("Amethyst OS", &mut buf).unwrap();
    out.output_string(cstr);
    loop {}
    return Status::SUCCESS;
}