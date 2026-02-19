#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    clock: Clock,
    status: Status,
    _reserved3: [u8; 0x04],
    timer0: Timer0,
    timer1: Timer1,
    timer2: Timer2,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - Clock Register"]
    #[inline(always)]
    pub const fn clock(&self) -> &Clock {
        &self.clock
    }
    #[doc = "0x08 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - TIMER0 Register"]
    #[inline(always)]
    pub const fn timer0(&self) -> &Timer0 {
        &self.timer0
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn timer1(&self) -> &Timer1 {
        &self.timer1
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn timer2(&self) -> &Timer2 {
        &self.timer2
    }
}
#[doc = "CONTROL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Control Register"]
pub mod control;
#[doc = "CLOCK (rw) register accessor: Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`]
module"]
#[doc(alias = "CLOCK")]
pub type Clock = crate::Reg<clock::ClockSpec>;
#[doc = "Clock Register"]
pub mod clock;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "TIMER0 (rw) register accessor: TIMER0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0`]
module"]
#[doc(alias = "TIMER0")]
pub type Timer0 = crate::Reg<timer0::Timer0Spec>;
#[doc = "TIMER0 Register"]
pub mod timer0;
#[doc = "TIMER1 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1`]
module"]
#[doc(alias = "TIMER1")]
pub type Timer1 = crate::Reg<timer1::Timer1Spec>;
#[doc = "no description available"]
pub mod timer1;
#[doc = "TIMER2 (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2`]
module"]
#[doc(alias = "TIMER2")]
pub type Timer2 = crate::Reg<timer2::Timer2Spec>;
#[doc = "no description available"]
pub mod timer2;
