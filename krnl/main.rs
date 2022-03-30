#![no-std]
#![no-main]

#[no-mangle]
pub fn _start() {
    let slice = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };
    slice[0] = b'a';
    slice[1] = 0x02;
    slice[2] = b'm';
    slice[3] = 0x02;
    slice[4] = b'e';
    slice[5] = 0x02;
    slice[6] = b't';
    slice[7] = 0x02;
    slice[8] = b'h';
    slice[9] = 0x02;
    slice[10] = b'y';
    slice[11] = 0x02;
    slice[12] = b's';
    slice[13] = 0x02;
    slice[14] = b't';
    slice[15] = 0x02;
    slice[16] = b'-';
    slice[17] = 0x02;
    slice[18] = b'o';
    slice[19] = 0x02;
    slice[20] = b's';
    slice[21] = 0x02;
    loop {
        asm!("hlt");
    }
}