extern crate embedded_hal as hal;

use hal::digital::v2::OutputPin;

pub struct DG508<EN, S1, S2, S3>
where
    EN: OutputPin,
    S1: OutputPin,
    S2: OutputPin,
    S3: OutputPin,
{
    channel: u8,
    enable: EN,
    select1: S1,
    select2: S2,
    select3: S3,
}

impl<EN, S1, S2, S3> DG508<EN, S1, S2, S3>
where
    EN: OutputPin,
    S1: OutputPin,
    S2: OutputPin,
    S3: OutputPin,
{
    pub fn new(en: EN, s1: S1, s2: S2, s3: S3) -> Self {
        DG508 {
            channel: 0,
            enable: en,
            select1: s1,
            select2: s2,
            select3: s3,
        }
    }
    pub fn enable(&mut self) {
        self.enable.set_high().ok();
    }
    pub fn disable(&mut self) {
        self.enable.set_low().ok();
    }
    pub fn set_active(&mut self, channel: u8) {
        self.channel = channel;
        fn bit_set(value: u8, pin: u8) -> bool {
            (value & (1 << pin)) > 0
        }
        match bit_set(channel, 0) {
            true => self.select1.set_high().ok(),
            false => self.select1.set_low().ok(),
        };
        match bit_set(channel, 1) {
            true => self.select2.set_high().ok(),
            false => self.select2.set_low().ok(),
        };
        match bit_set(channel, 2) {
            true => self.select3.set_high().ok(),
            false => self.select3.set_low().ok(),
        };
    }
}

//struct ActivePort(u8);
//impl ActivePort {
//    fn new(port: u8) -> Option<ActivePort> {
//        if port <= 8 {
//            Some(ActivePort(port))
//        } else {
//            None
//        }
//    }
//}
