#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sc1: [Sc1; 2],
    cfg1: Cfg1,
    cfg2: Cfg2,
    r: [R; 2],
    cv: [Cv; 2],
    sc2: Sc2,
    sc3: Sc3,
    ofs: Ofs,
    pg: Pg,
    mg: Mg,
    clpd: Clpd,
    clps: Clps,
    clp4: Clp4,
    clp3: Clp3,
    clp2: Clp2,
    clp1: Clp1,
    clp0: Clp0,
    pga: Pga,
    clmd: Clmd,
    clms: Clms,
    clm4: Clm4,
    clm3: Clm3,
    clm2: Clm2,
    clm1: Clm1,
    clm0: Clm0,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - ADC status and control registers 1"]
    #[inline(always)]
    pub const fn sc1(&self, n: usize) -> &Sc1 {
        &self.sc1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - ADC status and control registers 1"]
    #[inline(always)]
    pub fn sc1_iter(&self) -> impl Iterator<Item = &Sc1> {
        self.sc1.iter()
    }
    #[doc = "0x00 - ADC status and control registers 1"]
    #[inline(always)]
    pub const fn sc1a(&self) -> &Sc1 {
        self.sc1(0)
    }
    #[doc = "0x04 - ADC status and control registers 1"]
    #[inline(always)]
    pub const fn sc1b(&self) -> &Sc1 {
        self.sc1(1)
    }
    #[doc = "0x08 - ADC configuration register 1"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x0c - Configuration register 2"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        &self.cfg2
    }
    #[doc = "0x10..0x18 - ADC data result register"]
    #[inline(always)]
    pub const fn r(&self, n: usize) -> &R {
        &self.r[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - ADC data result register"]
    #[inline(always)]
    pub fn r_iter(&self) -> impl Iterator<Item = &R> {
        self.r.iter()
    }
    #[doc = "0x10 - ADC data result register"]
    #[inline(always)]
    pub const fn ra(&self) -> &R {
        self.r(0)
    }
    #[doc = "0x14 - ADC data result register"]
    #[inline(always)]
    pub const fn rb(&self) -> &R {
        self.r(1)
    }
    #[doc = "0x18..0x20 - Compare value registers"]
    #[inline(always)]
    pub const fn cv(&self, n: usize) -> &Cv {
        &self.cv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x20 - Compare value registers"]
    #[inline(always)]
    pub fn cv_iter(&self) -> impl Iterator<Item = &Cv> {
        self.cv.iter()
    }
    #[doc = "0x18 - Compare value registers"]
    #[inline(always)]
    pub const fn cv1(&self) -> &Cv {
        self.cv(0)
    }
    #[doc = "0x1c - Compare value registers"]
    #[inline(always)]
    pub const fn cv2(&self) -> &Cv {
        self.cv(1)
    }
    #[doc = "0x20 - Status and control register 2"]
    #[inline(always)]
    pub const fn sc2(&self) -> &Sc2 {
        &self.sc2
    }
    #[doc = "0x24 - Status and control register 3"]
    #[inline(always)]
    pub const fn sc3(&self) -> &Sc3 {
        &self.sc3
    }
    #[doc = "0x28 - ADC offset correction register"]
    #[inline(always)]
    pub const fn ofs(&self) -> &Ofs {
        &self.ofs
    }
    #[doc = "0x2c - ADC plus-side gain register"]
    #[inline(always)]
    pub const fn pg(&self) -> &Pg {
        &self.pg
    }
    #[doc = "0x30 - ADC minus-side gain register"]
    #[inline(always)]
    pub const fn mg(&self) -> &Mg {
        &self.mg
    }
    #[doc = "0x34 - ADC plus-side general calibration value register"]
    #[inline(always)]
    pub const fn clpd(&self) -> &Clpd {
        &self.clpd
    }
    #[doc = "0x38 - ADC plus-side general calibration value register"]
    #[inline(always)]
    pub const fn clps(&self) -> &Clps {
        &self.clps
    }
    #[doc = "0x3c - ADC plus-side general calibration value register"]
    #[inline(always)]
    pub const fn clp4(&self) -> &Clp4 {
        &self.clp4
    }
    #[doc = "0x40 - ADC plus-side general calibration value register"]
    #[inline(always)]
    pub const fn clp3(&self) -> &Clp3 {
        &self.clp3
    }
    #[doc = "0x44 - ADC plus-side general calibration value register"]
    #[inline(always)]
    pub const fn clp2(&self) -> &Clp2 {
        &self.clp2
    }
    #[doc = "0x48 - ADC plus-side general calibration value register"]
    #[inline(always)]
    pub const fn clp1(&self) -> &Clp1 {
        &self.clp1
    }
    #[doc = "0x4c - ADC plus-side general calibration value register"]
    #[inline(always)]
    pub const fn clp0(&self) -> &Clp0 {
        &self.clp0
    }
    #[doc = "0x50 - ADC PGA register"]
    #[inline(always)]
    pub const fn pga(&self) -> &Pga {
        &self.pga
    }
    #[doc = "0x54 - ADC minus-side general calibration value register"]
    #[inline(always)]
    pub const fn clmd(&self) -> &Clmd {
        &self.clmd
    }
    #[doc = "0x58 - ADC minus-side general calibration value register"]
    #[inline(always)]
    pub const fn clms(&self) -> &Clms {
        &self.clms
    }
    #[doc = "0x5c - ADC minus-side general calibration value register"]
    #[inline(always)]
    pub const fn clm4(&self) -> &Clm4 {
        &self.clm4
    }
    #[doc = "0x60 - ADC minus-side general calibration value register"]
    #[inline(always)]
    pub const fn clm3(&self) -> &Clm3 {
        &self.clm3
    }
    #[doc = "0x64 - ADC minus-side general calibration value register"]
    #[inline(always)]
    pub const fn clm2(&self) -> &Clm2 {
        &self.clm2
    }
    #[doc = "0x68 - ADC minus-side general calibration value register"]
    #[inline(always)]
    pub const fn clm1(&self) -> &Clm1 {
        &self.clm1
    }
    #[doc = "0x6c - ADC minus-side general calibration value register"]
    #[inline(always)]
    pub const fn clm0(&self) -> &Clm0 {
        &self.clm0
    }
}
#[doc = "SC1 (rw) register accessor: ADC status and control registers 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sc1`]
module"]
#[doc(alias = "SC1")]
pub type Sc1 = crate::Reg<sc1::Sc1Spec>;
#[doc = "ADC status and control registers 1"]
pub mod sc1;
#[doc = "CFG1 (rw) register accessor: ADC configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "ADC configuration register 1"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: Configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
#[doc(alias = "CFG2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "Configuration register 2"]
pub mod cfg2;
#[doc = "R (r) register accessor: ADC data result register\n\nYou can [`read`](crate::Reg::read) this register and get [`r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r`]
module"]
pub type R = crate::Reg<r::RSpec>;
#[doc = "ADC data result register"]
pub mod r;
#[doc = "CV (rw) register accessor: Compare value registers\n\nYou can [`read`](crate::Reg::read) this register and get [`cv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cv`]
module"]
#[doc(alias = "CV")]
pub type Cv = crate::Reg<cv::CvSpec>;
#[doc = "Compare value registers"]
pub mod cv;
#[doc = "SC2 (rw) register accessor: Status and control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sc2`]
module"]
#[doc(alias = "SC2")]
pub type Sc2 = crate::Reg<sc2::Sc2Spec>;
#[doc = "Status and control register 2"]
pub mod sc2;
#[doc = "SC3 (rw) register accessor: Status and control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sc3`]
module"]
#[doc(alias = "SC3")]
pub type Sc3 = crate::Reg<sc3::Sc3Spec>;
#[doc = "Status and control register 3"]
pub mod sc3;
#[doc = "OFS (rw) register accessor: ADC offset correction register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofs`]
module"]
#[doc(alias = "OFS")]
pub type Ofs = crate::Reg<ofs::OfsSpec>;
#[doc = "ADC offset correction register"]
pub mod ofs;
#[doc = "PG (rw) register accessor: ADC plus-side gain register\n\nYou can [`read`](crate::Reg::read) this register and get [`pg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg`]
module"]
#[doc(alias = "PG")]
pub type Pg = crate::Reg<pg::PgSpec>;
#[doc = "ADC plus-side gain register"]
pub mod pg;
#[doc = "MG (rw) register accessor: ADC minus-side gain register\n\nYou can [`read`](crate::Reg::read) this register and get [`mg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mg`]
module"]
#[doc(alias = "MG")]
pub type Mg = crate::Reg<mg::MgSpec>;
#[doc = "ADC minus-side gain register"]
pub mod mg;
#[doc = "CLPD (rw) register accessor: ADC plus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clpd`]
module"]
#[doc(alias = "CLPD")]
pub type Clpd = crate::Reg<clpd::ClpdSpec>;
#[doc = "ADC plus-side general calibration value register"]
pub mod clpd;
#[doc = "CLPS (rw) register accessor: ADC plus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clps`]
module"]
#[doc(alias = "CLPS")]
pub type Clps = crate::Reg<clps::ClpsSpec>;
#[doc = "ADC plus-side general calibration value register"]
pub mod clps;
#[doc = "CLP4 (rw) register accessor: ADC plus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clp4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clp4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clp4`]
module"]
#[doc(alias = "CLP4")]
pub type Clp4 = crate::Reg<clp4::Clp4Spec>;
#[doc = "ADC plus-side general calibration value register"]
pub mod clp4;
#[doc = "CLP3 (rw) register accessor: ADC plus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clp3`]
module"]
#[doc(alias = "CLP3")]
pub type Clp3 = crate::Reg<clp3::Clp3Spec>;
#[doc = "ADC plus-side general calibration value register"]
pub mod clp3;
#[doc = "CLP2 (rw) register accessor: ADC plus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clp2`]
module"]
#[doc(alias = "CLP2")]
pub type Clp2 = crate::Reg<clp2::Clp2Spec>;
#[doc = "ADC plus-side general calibration value register"]
pub mod clp2;
#[doc = "CLP1 (rw) register accessor: ADC plus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clp1`]
module"]
#[doc(alias = "CLP1")]
pub type Clp1 = crate::Reg<clp1::Clp1Spec>;
#[doc = "ADC plus-side general calibration value register"]
pub mod clp1;
#[doc = "CLP0 (rw) register accessor: ADC plus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clp0`]
module"]
#[doc(alias = "CLP0")]
pub type Clp0 = crate::Reg<clp0::Clp0Spec>;
#[doc = "ADC plus-side general calibration value register"]
pub mod clp0;
#[doc = "PGA (rw) register accessor: ADC PGA register\n\nYou can [`read`](crate::Reg::read) this register and get [`pga::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pga::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pga`]
module"]
#[doc(alias = "PGA")]
pub type Pga = crate::Reg<pga::PgaSpec>;
#[doc = "ADC PGA register"]
pub mod pga;
#[doc = "CLMD (rw) register accessor: ADC minus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clmd`]
module"]
#[doc(alias = "CLMD")]
pub type Clmd = crate::Reg<clmd::ClmdSpec>;
#[doc = "ADC minus-side general calibration value register"]
pub mod clmd;
#[doc = "CLMS (rw) register accessor: ADC minus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clms`]
module"]
#[doc(alias = "CLMS")]
pub type Clms = crate::Reg<clms::ClmsSpec>;
#[doc = "ADC minus-side general calibration value register"]
pub mod clms;
#[doc = "CLM4 (rw) register accessor: ADC minus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clm4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clm4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clm4`]
module"]
#[doc(alias = "CLM4")]
pub type Clm4 = crate::Reg<clm4::Clm4Spec>;
#[doc = "ADC minus-side general calibration value register"]
pub mod clm4;
#[doc = "CLM3 (rw) register accessor: ADC minus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clm3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clm3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clm3`]
module"]
#[doc(alias = "CLM3")]
pub type Clm3 = crate::Reg<clm3::Clm3Spec>;
#[doc = "ADC minus-side general calibration value register"]
pub mod clm3;
#[doc = "CLM2 (rw) register accessor: ADC minus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clm2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clm2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clm2`]
module"]
#[doc(alias = "CLM2")]
pub type Clm2 = crate::Reg<clm2::Clm2Spec>;
#[doc = "ADC minus-side general calibration value register"]
pub mod clm2;
#[doc = "CLM1 (rw) register accessor: ADC minus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clm1`]
module"]
#[doc(alias = "CLM1")]
pub type Clm1 = crate::Reg<clm1::Clm1Spec>;
#[doc = "ADC minus-side general calibration value register"]
pub mod clm1;
#[doc = "CLM0 (rw) register accessor: ADC minus-side general calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`clm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clm0`]
module"]
#[doc(alias = "CLM0")]
pub type Clm0 = crate::Reg<clm0::Clm0Spec>;
#[doc = "ADC minus-side general calibration value register"]
pub mod clm0;
