#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pe1: Pe1,
    pe2: Pe2,
    pe3: Pe3,
    pe4: Pe4,
    me: Me,
    f1: F1,
    f2: F2,
    f3: F3,
    filt1: Filt1,
    filt2: Filt2,
    rst: Rst,
}
impl RegisterBlock {
    #[doc = "0x00 - LLWU Pin Enable 1 Register"]
    #[inline(always)]
    pub const fn pe1(&self) -> &Pe1 {
        &self.pe1
    }
    #[doc = "0x01 - LLWU Pin Enable 2 Register"]
    #[inline(always)]
    pub const fn pe2(&self) -> &Pe2 {
        &self.pe2
    }
    #[doc = "0x02 - LLWU Pin Enable 3 Register"]
    #[inline(always)]
    pub const fn pe3(&self) -> &Pe3 {
        &self.pe3
    }
    #[doc = "0x03 - LLWU Pin Enable 4 Register"]
    #[inline(always)]
    pub const fn pe4(&self) -> &Pe4 {
        &self.pe4
    }
    #[doc = "0x04 - LLWU Module Enable Register"]
    #[inline(always)]
    pub const fn me(&self) -> &Me {
        &self.me
    }
    #[doc = "0x05 - LLWU Flag 1 Register"]
    #[inline(always)]
    pub const fn f1(&self) -> &F1 {
        &self.f1
    }
    #[doc = "0x06 - LLWU Flag 2 Register"]
    #[inline(always)]
    pub const fn f2(&self) -> &F2 {
        &self.f2
    }
    #[doc = "0x07 - LLWU Flag 3 Register"]
    #[inline(always)]
    pub const fn f3(&self) -> &F3 {
        &self.f3
    }
    #[doc = "0x08 - LLWU Pin Filter 1 Register"]
    #[inline(always)]
    pub const fn filt1(&self) -> &Filt1 {
        &self.filt1
    }
    #[doc = "0x09 - LLWU Pin Filter 2 Register"]
    #[inline(always)]
    pub const fn filt2(&self) -> &Filt2 {
        &self.filt2
    }
    #[doc = "0x0a - LLWU Reset Enable Register"]
    #[inline(always)]
    pub const fn rst(&self) -> &Rst {
        &self.rst
    }
}
#[doc = "PE1 (rw) register accessor: LLWU Pin Enable 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe1`]
module"]
#[doc(alias = "PE1")]
pub type Pe1 = crate::Reg<pe1::Pe1Spec>;
#[doc = "LLWU Pin Enable 1 Register"]
pub mod pe1;
#[doc = "PE2 (rw) register accessor: LLWU Pin Enable 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe2`]
module"]
#[doc(alias = "PE2")]
pub type Pe2 = crate::Reg<pe2::Pe2Spec>;
#[doc = "LLWU Pin Enable 2 Register"]
pub mod pe2;
#[doc = "PE3 (rw) register accessor: LLWU Pin Enable 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe3`]
module"]
#[doc(alias = "PE3")]
pub type Pe3 = crate::Reg<pe3::Pe3Spec>;
#[doc = "LLWU Pin Enable 3 Register"]
pub mod pe3;
#[doc = "PE4 (rw) register accessor: LLWU Pin Enable 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe4`]
module"]
#[doc(alias = "PE4")]
pub type Pe4 = crate::Reg<pe4::Pe4Spec>;
#[doc = "LLWU Pin Enable 4 Register"]
pub mod pe4;
#[doc = "ME (rw) register accessor: LLWU Module Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`me::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`me::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@me`]
module"]
#[doc(alias = "ME")]
pub type Me = crate::Reg<me::MeSpec>;
#[doc = "LLWU Module Enable Register"]
pub mod me;
#[doc = "F1 (rw) register accessor: LLWU Flag 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`f1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f1`]
module"]
pub type F1 = crate::Reg<f1::F1Spec>;
#[doc = "LLWU Flag 1 Register"]
pub mod f1;
#[doc = "F2 (rw) register accessor: LLWU Flag 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`f2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2`]
module"]
pub type F2 = crate::Reg<f2::F2Spec>;
#[doc = "LLWU Flag 2 Register"]
pub mod f2;
#[doc = "F3 (r) register accessor: LLWU Flag 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`f3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f3`]
module"]
pub type F3 = crate::Reg<f3::F3Spec>;
#[doc = "LLWU Flag 3 Register"]
pub mod f3;
#[doc = "FILT1 (rw) register accessor: LLWU Pin Filter 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`filt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt1`]
module"]
#[doc(alias = "FILT1")]
pub type Filt1 = crate::Reg<filt1::Filt1Spec>;
#[doc = "LLWU Pin Filter 1 Register"]
pub mod filt1;
#[doc = "FILT2 (rw) register accessor: LLWU Pin Filter 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`filt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filt2`]
module"]
#[doc(alias = "FILT2")]
pub type Filt2 = crate::Reg<filt2::Filt2Spec>;
#[doc = "LLWU Pin Filter 2 Register"]
pub mod filt2;
#[doc = "RST (rw) register accessor: LLWU Reset Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst`]
module"]
#[doc(alias = "RST")]
pub type Rst = crate::Reg<rst::RstSpec>;
#[doc = "LLWU Reset Enable Register"]
pub mod rst;
