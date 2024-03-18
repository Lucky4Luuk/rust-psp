#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct ScePspGpsData {
    pub year: u16,
    pub month: u16,
    pub date: u16,      // Probably day
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub garbage1: f32,  // ??
    pub hdop: f32,
    pub garbage2: f32,  // ??
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,  // TODO: Figure out the unit
    pub garbage3: f32,  // ??
    pub speed: f32,     // TODO: Figure out the unit
    pub bearing: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ScePspSatInfo {
    pub id: u8,
    pub elevation: u8,
    pub azimuth: u16,
    pub snr: u8,
    pub good: u8,
    pub garbage: u32,   // ??
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ScePspSatData {
    pub satellites_in_view: u16,
    pub garbage: u16,   // ??
    pub satinf: [ScePspSatInfo; 24],
}

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

    #[psp(0x934EC2B2)]
    pub fn sceUsbGpsGetData(gps_data: *mut ScePspGpsData, sat_data: *mut ScePspSatData) -> i32;
}
