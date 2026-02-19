#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    srs0: Srs0,
    srs1: Srs1,
    _reserved2: [u8; 0x02],
    rpfc: Rpfc,
    rpfw: Rpfw,
    _reserved4: [u8; 0x01],
    mr: Mr,
}
impl RegisterBlock {
    #[doc = "0x00 - System Reset Status Register 0"]
    #[inline(always)]
    pub const fn srs0(&self) -> &Srs0 {
        &self.srs0
    }
    #[doc = "0x01 - System Reset Status Register 1"]
    #[inline(always)]
    pub const fn srs1(&self) -> &Srs1 {
        &self.srs1
    }
    #[doc = "0x04 - Reset Pin Filter Control Register"]
    #[inline(always)]
    pub const fn rpfc(&self) -> &Rpfc {
        &self.rpfc
    }
    #[doc = "0x05 - Reset Pin Filter Width Register"]
    #[inline(always)]
    pub const fn rpfw(&self) -> &Rpfw {
        &self.rpfw
    }
    #[doc = "0x07 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
}
#[doc = "SRS0 (r) register accessor: System Reset Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`srs0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srs0`]
module"]
#[doc(alias = "SRS0")]
pub type Srs0 = crate::Reg<srs0::Srs0Spec>;
#[doc = "System Reset Status Register 0"]
pub mod srs0;
#[doc = "SRS1 (r) register accessor: System Reset Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`srs1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srs1`]
module"]
#[doc(alias = "SRS1")]
pub type Srs1 = crate::Reg<srs1::Srs1Spec>;
#[doc = "System Reset Status Register 1"]
pub mod srs1;
#[doc = "RPFC (rw) register accessor: Reset Pin Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpfc`]
module"]
#[doc(alias = "RPFC")]
pub type Rpfc = crate::Reg<rpfc::RpfcSpec>;
#[doc = "Reset Pin Filter Control Register"]
pub mod rpfc;
#[doc = "RPFW (rw) register accessor: Reset Pin Filter Width Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpfw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpfw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpfw`]
module"]
#[doc(alias = "RPFW")]
pub type Rpfw = crate::Reg<rpfw::RpfwSpec>;
#[doc = "Reset Pin Filter Width Register"]
pub mod rpfw;
#[doc = "MR (r) register accessor: Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
