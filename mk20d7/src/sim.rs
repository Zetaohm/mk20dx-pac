#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sopt1: Sopt1,
    sopt1cfg: Sopt1cfg,
    _reserved2: [u8; 0x0ffc],
    sopt2: Sopt2,
    _reserved3: [u8; 0x04],
    sopt4: Sopt4,
    sopt5: Sopt5,
    _reserved5: [u8; 0x04],
    sopt7: Sopt7,
    _reserved6: [u8; 0x08],
    sdid: Sdid,
    scgc1: Scgc1,
    scgc2: Scgc2,
    scgc3: Scgc3,
    scgc4: Scgc4,
    scgc5: Scgc5,
    scgc6: Scgc6,
    scgc7: Scgc7,
    clkdiv1: Clkdiv1,
    clkdiv2: Clkdiv2,
    fcfg1: Fcfg1,
    fcfg2: Fcfg2,
    uidh: Uidh,
    uidmh: Uidmh,
    uidml: Uidml,
    uidl: Uidl,
}
impl RegisterBlock {
    #[doc = "0x00 - System Options Register 1"]
    #[inline(always)]
    pub const fn sopt1(&self) -> &Sopt1 {
        &self.sopt1
    }
    #[doc = "0x04 - SOPT1 Configuration Register"]
    #[inline(always)]
    pub const fn sopt1cfg(&self) -> &Sopt1cfg {
        &self.sopt1cfg
    }
    #[doc = "0x1004 - System Options Register 2"]
    #[inline(always)]
    pub const fn sopt2(&self) -> &Sopt2 {
        &self.sopt2
    }
    #[doc = "0x100c - System Options Register 4"]
    #[inline(always)]
    pub const fn sopt4(&self) -> &Sopt4 {
        &self.sopt4
    }
    #[doc = "0x1010 - System Options Register 5"]
    #[inline(always)]
    pub const fn sopt5(&self) -> &Sopt5 {
        &self.sopt5
    }
    #[doc = "0x1018 - System Options Register 7"]
    #[inline(always)]
    pub const fn sopt7(&self) -> &Sopt7 {
        &self.sopt7
    }
    #[doc = "0x1024 - System Device Identification Register"]
    #[inline(always)]
    pub const fn sdid(&self) -> &Sdid {
        &self.sdid
    }
    #[doc = "0x1028 - System Clock Gating Control Register 1"]
    #[inline(always)]
    pub const fn scgc1(&self) -> &Scgc1 {
        &self.scgc1
    }
    #[doc = "0x102c - System Clock Gating Control Register 2"]
    #[inline(always)]
    pub const fn scgc2(&self) -> &Scgc2 {
        &self.scgc2
    }
    #[doc = "0x1030 - System Clock Gating Control Register 3"]
    #[inline(always)]
    pub const fn scgc3(&self) -> &Scgc3 {
        &self.scgc3
    }
    #[doc = "0x1034 - System Clock Gating Control Register 4"]
    #[inline(always)]
    pub const fn scgc4(&self) -> &Scgc4 {
        &self.scgc4
    }
    #[doc = "0x1038 - System Clock Gating Control Register 5"]
    #[inline(always)]
    pub const fn scgc5(&self) -> &Scgc5 {
        &self.scgc5
    }
    #[doc = "0x103c - System Clock Gating Control Register 6"]
    #[inline(always)]
    pub const fn scgc6(&self) -> &Scgc6 {
        &self.scgc6
    }
    #[doc = "0x1040 - System Clock Gating Control Register 7"]
    #[inline(always)]
    pub const fn scgc7(&self) -> &Scgc7 {
        &self.scgc7
    }
    #[doc = "0x1044 - System Clock Divider Register 1"]
    #[inline(always)]
    pub const fn clkdiv1(&self) -> &Clkdiv1 {
        &self.clkdiv1
    }
    #[doc = "0x1048 - System Clock Divider Register 2"]
    #[inline(always)]
    pub const fn clkdiv2(&self) -> &Clkdiv2 {
        &self.clkdiv2
    }
    #[doc = "0x104c - Flash Configuration Register 1"]
    #[inline(always)]
    pub const fn fcfg1(&self) -> &Fcfg1 {
        &self.fcfg1
    }
    #[doc = "0x1050 - Flash Configuration Register 2"]
    #[inline(always)]
    pub const fn fcfg2(&self) -> &Fcfg2 {
        &self.fcfg2
    }
    #[doc = "0x1054 - Unique Identification Register High"]
    #[inline(always)]
    pub const fn uidh(&self) -> &Uidh {
        &self.uidh
    }
    #[doc = "0x1058 - Unique Identification Register Mid-High"]
    #[inline(always)]
    pub const fn uidmh(&self) -> &Uidmh {
        &self.uidmh
    }
    #[doc = "0x105c - Unique Identification Register Mid Low"]
    #[inline(always)]
    pub const fn uidml(&self) -> &Uidml {
        &self.uidml
    }
    #[doc = "0x1060 - Unique Identification Register Low"]
    #[inline(always)]
    pub const fn uidl(&self) -> &Uidl {
        &self.uidl
    }
}
#[doc = "SOPT1 (rw) register accessor: System Options Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sopt1`]
module"]
#[doc(alias = "SOPT1")]
pub type Sopt1 = crate::Reg<sopt1::Sopt1Spec>;
#[doc = "System Options Register 1"]
pub mod sopt1;
#[doc = "SOPT1CFG (rw) register accessor: SOPT1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt1cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt1cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sopt1cfg`]
module"]
#[doc(alias = "SOPT1CFG")]
pub type Sopt1cfg = crate::Reg<sopt1cfg::Sopt1cfgSpec>;
#[doc = "SOPT1 Configuration Register"]
pub mod sopt1cfg;
#[doc = "SOPT2 (rw) register accessor: System Options Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sopt2`]
module"]
#[doc(alias = "SOPT2")]
pub type Sopt2 = crate::Reg<sopt2::Sopt2Spec>;
#[doc = "System Options Register 2"]
pub mod sopt2;
#[doc = "SOPT4 (rw) register accessor: System Options Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sopt4`]
module"]
#[doc(alias = "SOPT4")]
pub type Sopt4 = crate::Reg<sopt4::Sopt4Spec>;
#[doc = "System Options Register 4"]
pub mod sopt4;
#[doc = "SOPT5 (rw) register accessor: System Options Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sopt5`]
module"]
#[doc(alias = "SOPT5")]
pub type Sopt5 = crate::Reg<sopt5::Sopt5Spec>;
#[doc = "System Options Register 5"]
pub mod sopt5;
#[doc = "SOPT7 (rw) register accessor: System Options Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sopt7`]
module"]
#[doc(alias = "SOPT7")]
pub type Sopt7 = crate::Reg<sopt7::Sopt7Spec>;
#[doc = "System Options Register 7"]
pub mod sopt7;
#[doc = "SDID (r) register accessor: System Device Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdid`]
module"]
#[doc(alias = "SDID")]
pub type Sdid = crate::Reg<sdid::SdidSpec>;
#[doc = "System Device Identification Register"]
pub mod sdid;
#[doc = "SCGC1 (rw) register accessor: System Clock Gating Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgc1`]
module"]
#[doc(alias = "SCGC1")]
pub type Scgc1 = crate::Reg<scgc1::Scgc1Spec>;
#[doc = "System Clock Gating Control Register 1"]
pub mod scgc1;
#[doc = "SCGC2 (rw) register accessor: System Clock Gating Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgc2`]
module"]
#[doc(alias = "SCGC2")]
pub type Scgc2 = crate::Reg<scgc2::Scgc2Spec>;
#[doc = "System Clock Gating Control Register 2"]
pub mod scgc2;
#[doc = "SCGC3 (rw) register accessor: System Clock Gating Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgc3`]
module"]
#[doc(alias = "SCGC3")]
pub type Scgc3 = crate::Reg<scgc3::Scgc3Spec>;
#[doc = "System Clock Gating Control Register 3"]
pub mod scgc3;
#[doc = "SCGC4 (rw) register accessor: System Clock Gating Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgc4`]
module"]
#[doc(alias = "SCGC4")]
pub type Scgc4 = crate::Reg<scgc4::Scgc4Spec>;
#[doc = "System Clock Gating Control Register 4"]
pub mod scgc4;
#[doc = "SCGC5 (rw) register accessor: System Clock Gating Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgc5`]
module"]
#[doc(alias = "SCGC5")]
pub type Scgc5 = crate::Reg<scgc5::Scgc5Spec>;
#[doc = "System Clock Gating Control Register 5"]
pub mod scgc5;
#[doc = "SCGC6 (rw) register accessor: System Clock Gating Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgc6`]
module"]
#[doc(alias = "SCGC6")]
pub type Scgc6 = crate::Reg<scgc6::Scgc6Spec>;
#[doc = "System Clock Gating Control Register 6"]
pub mod scgc6;
#[doc = "SCGC7 (rw) register accessor: System Clock Gating Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgc7`]
module"]
#[doc(alias = "SCGC7")]
pub type Scgc7 = crate::Reg<scgc7::Scgc7Spec>;
#[doc = "System Clock Gating Control Register 7"]
pub mod scgc7;
#[doc = "CLKDIV1 (rw) register accessor: System Clock Divider Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv1`]
module"]
#[doc(alias = "CLKDIV1")]
pub type Clkdiv1 = crate::Reg<clkdiv1::Clkdiv1Spec>;
#[doc = "System Clock Divider Register 1"]
pub mod clkdiv1;
#[doc = "CLKDIV2 (rw) register accessor: System Clock Divider Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv2`]
module"]
#[doc(alias = "CLKDIV2")]
pub type Clkdiv2 = crate::Reg<clkdiv2::Clkdiv2Spec>;
#[doc = "System Clock Divider Register 2"]
pub mod clkdiv2;
#[doc = "FCFG1 (rw) register accessor: Flash Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg1`]
module"]
#[doc(alias = "FCFG1")]
pub type Fcfg1 = crate::Reg<fcfg1::Fcfg1Spec>;
#[doc = "Flash Configuration Register 1"]
pub mod fcfg1;
#[doc = "FCFG2 (r) register accessor: Flash Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fcfg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg2`]
module"]
#[doc(alias = "FCFG2")]
pub type Fcfg2 = crate::Reg<fcfg2::Fcfg2Spec>;
#[doc = "Flash Configuration Register 2"]
pub mod fcfg2;
#[doc = "UIDH (r) register accessor: Unique Identification Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`uidh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uidh`]
module"]
#[doc(alias = "UIDH")]
pub type Uidh = crate::Reg<uidh::UidhSpec>;
#[doc = "Unique Identification Register High"]
pub mod uidh;
#[doc = "UIDMH (r) register accessor: Unique Identification Register Mid-High\n\nYou can [`read`](crate::Reg::read) this register and get [`uidmh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uidmh`]
module"]
#[doc(alias = "UIDMH")]
pub type Uidmh = crate::Reg<uidmh::UidmhSpec>;
#[doc = "Unique Identification Register Mid-High"]
pub mod uidmh;
#[doc = "UIDML (r) register accessor: Unique Identification Register Mid Low\n\nYou can [`read`](crate::Reg::read) this register and get [`uidml::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uidml`]
module"]
#[doc(alias = "UIDML")]
pub type Uidml = crate::Reg<uidml::UidmlSpec>;
#[doc = "Unique Identification Register Mid Low"]
pub mod uidml;
#[doc = "UIDL (r) register accessor: Unique Identification Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`uidl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uidl`]
module"]
#[doc(alias = "UIDL")]
pub type Uidl = crate::Reg<uidl::UidlSpec>;
#[doc = "Unique Identification Register Low"]
pub mod uidl;
