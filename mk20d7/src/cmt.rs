#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cgh1: Cgh1,
    cgl1: Cgl1,
    cgh2: Cgh2,
    cgl2: Cgl2,
    oc: Oc,
    msc: Msc,
    cmd1: Cmd1,
    cmd2: Cmd2,
    cmd3: Cmd3,
    cmd4: Cmd4,
    pps: Pps,
    dma: Dma,
}
impl RegisterBlock {
    #[doc = "0x00 - CMT Carrier Generator High Data Register 1"]
    #[inline(always)]
    pub const fn cgh1(&self) -> &Cgh1 {
        &self.cgh1
    }
    #[doc = "0x01 - CMT Carrier Generator Low Data Register 1"]
    #[inline(always)]
    pub const fn cgl1(&self) -> &Cgl1 {
        &self.cgl1
    }
    #[doc = "0x02 - CMT Carrier Generator High Data Register 2"]
    #[inline(always)]
    pub const fn cgh2(&self) -> &Cgh2 {
        &self.cgh2
    }
    #[doc = "0x03 - CMT Carrier Generator Low Data Register 2"]
    #[inline(always)]
    pub const fn cgl2(&self) -> &Cgl2 {
        &self.cgl2
    }
    #[doc = "0x04 - CMT Output Control Register"]
    #[inline(always)]
    pub const fn oc(&self) -> &Oc {
        &self.oc
    }
    #[doc = "0x05 - CMT Modulator Status and Control Register"]
    #[inline(always)]
    pub const fn msc(&self) -> &Msc {
        &self.msc
    }
    #[doc = "0x06 - CMT Modulator Data Register Mark High"]
    #[inline(always)]
    pub const fn cmd1(&self) -> &Cmd1 {
        &self.cmd1
    }
    #[doc = "0x07 - CMT Modulator Data Register Mark Low"]
    #[inline(always)]
    pub const fn cmd2(&self) -> &Cmd2 {
        &self.cmd2
    }
    #[doc = "0x08 - CMT Modulator Data Register Space High"]
    #[inline(always)]
    pub const fn cmd3(&self) -> &Cmd3 {
        &self.cmd3
    }
    #[doc = "0x09 - CMT Modulator Data Register Space Low"]
    #[inline(always)]
    pub const fn cmd4(&self) -> &Cmd4 {
        &self.cmd4
    }
    #[doc = "0x0a - CMT Primary Prescaler Register"]
    #[inline(always)]
    pub const fn pps(&self) -> &Pps {
        &self.pps
    }
    #[doc = "0x0b - CMT Direct Memory Access"]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
}
#[doc = "CGH1 (rw) register accessor: CMT Carrier Generator High Data Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cgh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgh1`]
module"]
#[doc(alias = "CGH1")]
pub type Cgh1 = crate::Reg<cgh1::Cgh1Spec>;
#[doc = "CMT Carrier Generator High Data Register 1"]
pub mod cgh1;
#[doc = "CGL1 (rw) register accessor: CMT Carrier Generator Low Data Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cgl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgl1`]
module"]
#[doc(alias = "CGL1")]
pub type Cgl1 = crate::Reg<cgl1::Cgl1Spec>;
#[doc = "CMT Carrier Generator Low Data Register 1"]
pub mod cgl1;
#[doc = "CGH2 (rw) register accessor: CMT Carrier Generator High Data Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cgh2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgh2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgh2`]
module"]
#[doc(alias = "CGH2")]
pub type Cgh2 = crate::Reg<cgh2::Cgh2Spec>;
#[doc = "CMT Carrier Generator High Data Register 2"]
pub mod cgh2;
#[doc = "CGL2 (rw) register accessor: CMT Carrier Generator Low Data Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cgl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgl2`]
module"]
#[doc(alias = "CGL2")]
pub type Cgl2 = crate::Reg<cgl2::Cgl2Spec>;
#[doc = "CMT Carrier Generator Low Data Register 2"]
pub mod cgl2;
#[doc = "OC (rw) register accessor: CMT Output Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oc`]
module"]
#[doc(alias = "OC")]
pub type Oc = crate::Reg<oc::OcSpec>;
#[doc = "CMT Output Control Register"]
pub mod oc;
#[doc = "MSC (rw) register accessor: CMT Modulator Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msc`]
module"]
#[doc(alias = "MSC")]
pub type Msc = crate::Reg<msc::MscSpec>;
#[doc = "CMT Modulator Status and Control Register"]
pub mod msc;
#[doc = "CMD1 (rw) register accessor: CMT Modulator Data Register Mark High\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd1`]
module"]
#[doc(alias = "CMD1")]
pub type Cmd1 = crate::Reg<cmd1::Cmd1Spec>;
#[doc = "CMT Modulator Data Register Mark High"]
pub mod cmd1;
#[doc = "CMD2 (rw) register accessor: CMT Modulator Data Register Mark Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd2`]
module"]
#[doc(alias = "CMD2")]
pub type Cmd2 = crate::Reg<cmd2::Cmd2Spec>;
#[doc = "CMT Modulator Data Register Mark Low"]
pub mod cmd2;
#[doc = "CMD3 (rw) register accessor: CMT Modulator Data Register Space High\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd3`]
module"]
#[doc(alias = "CMD3")]
pub type Cmd3 = crate::Reg<cmd3::Cmd3Spec>;
#[doc = "CMT Modulator Data Register Space High"]
pub mod cmd3;
#[doc = "CMD4 (rw) register accessor: CMT Modulator Data Register Space Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd4`]
module"]
#[doc(alias = "CMD4")]
pub type Cmd4 = crate::Reg<cmd4::Cmd4Spec>;
#[doc = "CMT Modulator Data Register Space Low"]
pub mod cmd4;
#[doc = "PPS (rw) register accessor: CMT Primary Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps`]
module"]
#[doc(alias = "PPS")]
pub type Pps = crate::Reg<pps::PpsSpec>;
#[doc = "CMT Primary Prescaler Register"]
pub mod pps;
#[doc = "DMA (rw) register accessor: CMT Direct Memory Access\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
#[doc(alias = "DMA")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "CMT Direct Memory Access"]
pub mod dma;
