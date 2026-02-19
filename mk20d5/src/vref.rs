#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    trm: Trm,
    sc: Sc,
}
impl RegisterBlock {
    #[doc = "0x00 - VREF Trim Register"]
    #[inline(always)]
    pub const fn trm(&self) -> &Trm {
        &self.trm
    }
    #[doc = "0x01 - VREF Status and Control Register"]
    #[inline(always)]
    pub const fn sc(&self) -> &Sc {
        &self.sc
    }
}
#[doc = "TRM (rw) register accessor: VREF Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trm`]
module"]
#[doc(alias = "TRM")]
pub type Trm = crate::Reg<trm::TrmSpec>;
#[doc = "VREF Trim Register"]
pub mod trm;
#[doc = "SC (rw) register accessor: VREF Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sc`]
module"]
#[doc(alias = "SC")]
pub type Sc = crate::Reg<sc::ScSpec>;
#[doc = "VREF Status and Control Register"]
pub mod sc;
