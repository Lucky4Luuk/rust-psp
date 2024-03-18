psp_extern! {
    #![name = "sceUsbGps"]
    #![flags = 0x4009]
    #![version = (0x00, 0x00)]

    #[psp(0x268F95CA)]
    pub fn sceUsbGpsSetInitDataLocation(addr: u32);

    #[psp(0x54D26AA4)]
    pub fn sceUsbGpsGetInitDataLocation(addr: *mut u32);

    #[psp(0x9F267D34)]
    pub fn sceUsbGpsOpen();

    #[psp(0x7C16AC3A)]
    pub fn sceUsbGpsGetState(gps_state: *mut u32) -> i32;
}
