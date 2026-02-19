#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    a1: A1,
    f: F,
    c1: C1,
    s: S,
    d: D,
    c2: C2,
    flt: Flt,
    ra: Ra,
    smb: Smb,
    a2: A2,
    slth: Slth,
    sltl: Sltl,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C Address Register 1"]
    #[inline(always)]
    pub const fn a1(&self) -> &A1 {
        &self.a1
    }
    #[doc = "0x01 - I2C Frequency Divider register"]
    #[inline(always)]
    pub const fn f(&self) -> &F {
        &self.f
    }
    #[doc = "0x02 - I2C Control Register 1"]
    #[inline(always)]
    pub const fn c1(&self) -> &C1 {
        &self.c1
    }
    #[doc = "0x03 - I2C Status Register"]
    #[inline(always)]
    pub const fn s(&self) -> &S {
        &self.s
    }
    #[doc = "0x04 - I2C Data I/O register"]
    #[inline(always)]
    pub const fn d(&self) -> &D {
        &self.d
    }
    #[doc = "0x05 - I2C Control Register 2"]
    #[inline(always)]
    pub const fn c2(&self) -> &C2 {
        &self.c2
    }
    #[doc = "0x06 - I2C Programmable Input Glitch Filter register"]
    #[inline(always)]
    pub const fn flt(&self) -> &Flt {
        &self.flt
    }
    #[doc = "0x07 - I2C Range Address register"]
    #[inline(always)]
    pub const fn ra(&self) -> &Ra {
        &self.ra
    }
    #[doc = "0x08 - I2C SMBus Control and Status register"]
    #[inline(always)]
    pub const fn smb(&self) -> &Smb {
        &self.smb
    }
    #[doc = "0x09 - I2C Address Register 2"]
    #[inline(always)]
    pub const fn a2(&self) -> &A2 {
        &self.a2
    }
    #[doc = "0x0a - I2C SCL Low Timeout Register High"]
    #[inline(always)]
    pub const fn slth(&self) -> &Slth {
        &self.slth
    }
    #[doc = "0x0b - I2C SCL Low Timeout Register Low"]
    #[inline(always)]
    pub const fn sltl(&self) -> &Sltl {
        &self.sltl
    }
}
#[doc = "A1 (rw) register accessor: I2C Address Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`a1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a1`]
module"]
pub type A1 = crate::Reg<a1::A1Spec>;
#[doc = "I2C Address Register 1"]
pub mod a1;
#[doc = "F (rw) register accessor: I2C Frequency Divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`f::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f`]
module"]
pub type F = crate::Reg<f::FSpec>;
#[doc = "I2C Frequency Divider register"]
pub mod f;
#[doc = "C1 (rw) register accessor: I2C Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1`]
module"]
pub type C1 = crate::Reg<c1::C1Spec>;
#[doc = "I2C Control Register 1"]
pub mod c1;
#[doc = "S (rw) register accessor: I2C Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s`]
module"]
pub type S = crate::Reg<s::SSpec>;
#[doc = "I2C Status Register"]
pub mod s;
#[doc = "D (rw) register accessor: I2C Data I/O register\n\nYou can [`read`](crate::Reg::read) this register and get [`d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d`]
module"]
pub type D = crate::Reg<d::DSpec>;
#[doc = "I2C Data I/O register"]
pub mod d;
#[doc = "C2 (rw) register accessor: I2C Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2`]
module"]
pub type C2 = crate::Reg<c2::C2Spec>;
#[doc = "I2C Control Register 2"]
pub mod c2;
#[doc = "FLT (rw) register accessor: I2C Programmable Input Glitch Filter register\n\nYou can [`read`](crate::Reg::read) this register and get [`flt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flt`]
module"]
#[doc(alias = "FLT")]
pub type Flt = crate::Reg<flt::FltSpec>;
#[doc = "I2C Programmable Input Glitch Filter register"]
pub mod flt;
#[doc = "RA (rw) register accessor: I2C Range Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra`]
module"]
#[doc(alias = "RA")]
pub type Ra = crate::Reg<ra::RaSpec>;
#[doc = "I2C Range Address register"]
pub mod ra;
#[doc = "SMB (rw) register accessor: I2C SMBus Control and Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`smb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smb`]
module"]
#[doc(alias = "SMB")]
pub type Smb = crate::Reg<smb::SmbSpec>;
#[doc = "I2C SMBus Control and Status register"]
pub mod smb;
#[doc = "A2 (rw) register accessor: I2C Address Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`a2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2`]
module"]
pub type A2 = crate::Reg<a2::A2Spec>;
#[doc = "I2C Address Register 2"]
pub mod a2;
#[doc = "SLTH (rw) register accessor: I2C SCL Low Timeout Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`slth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slth`]
module"]
#[doc(alias = "SLTH")]
pub type Slth = crate::Reg<slth::SlthSpec>;
#[doc = "I2C SCL Low Timeout Register High"]
pub mod slth;
#[doc = "SLTL (rw) register accessor: I2C SCL Low Timeout Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`sltl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sltl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sltl`]
module"]
#[doc(alias = "SLTL")]
pub type Sltl = crate::Reg<sltl::SltlSpec>;
#[doc = "I2C SCL Low Timeout Register Low"]
pub mod sltl;
