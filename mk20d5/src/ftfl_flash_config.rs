#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    backkey3: Backkey3,
    backkey2: Backkey2,
    backkey1: Backkey1,
    backkey0: Backkey0,
    backkey7: Backkey7,
    backkey6: Backkey6,
    backkey5: Backkey5,
    backkey4: Backkey4,
    fprot3: Fprot3,
    fprot2: Fprot2,
    fprot1: Fprot1,
    fprot0: Fprot0,
    fsec: Fsec,
    fopt: Fopt,
    feprot: Feprot,
    fdprot: Fdprot,
}
impl RegisterBlock {
    #[doc = "0x00 - Backdoor Comparison Key 3."]
    #[inline(always)]
    pub const fn backkey3(&self) -> &Backkey3 {
        &self.backkey3
    }
    #[doc = "0x01 - Backdoor Comparison Key 2."]
    #[inline(always)]
    pub const fn backkey2(&self) -> &Backkey2 {
        &self.backkey2
    }
    #[doc = "0x02 - Backdoor Comparison Key 1."]
    #[inline(always)]
    pub const fn backkey1(&self) -> &Backkey1 {
        &self.backkey1
    }
    #[doc = "0x03 - Backdoor Comparison Key 0."]
    #[inline(always)]
    pub const fn backkey0(&self) -> &Backkey0 {
        &self.backkey0
    }
    #[doc = "0x04 - Backdoor Comparison Key 7."]
    #[inline(always)]
    pub const fn backkey7(&self) -> &Backkey7 {
        &self.backkey7
    }
    #[doc = "0x05 - Backdoor Comparison Key 6."]
    #[inline(always)]
    pub const fn backkey6(&self) -> &Backkey6 {
        &self.backkey6
    }
    #[doc = "0x06 - Backdoor Comparison Key 5."]
    #[inline(always)]
    pub const fn backkey5(&self) -> &Backkey5 {
        &self.backkey5
    }
    #[doc = "0x07 - Backdoor Comparison Key 4."]
    #[inline(always)]
    pub const fn backkey4(&self) -> &Backkey4 {
        &self.backkey4
    }
    #[doc = "0x08 - Non-volatile P-Flash Protection 1 - Low Register"]
    #[inline(always)]
    pub const fn fprot3(&self) -> &Fprot3 {
        &self.fprot3
    }
    #[doc = "0x09 - Non-volatile P-Flash Protection 1 - High Register"]
    #[inline(always)]
    pub const fn fprot2(&self) -> &Fprot2 {
        &self.fprot2
    }
    #[doc = "0x0a - Non-volatile P-Flash Protection 0 - Low Register"]
    #[inline(always)]
    pub const fn fprot1(&self) -> &Fprot1 {
        &self.fprot1
    }
    #[doc = "0x0b - Non-volatile P-Flash Protection 0 - High Register"]
    #[inline(always)]
    pub const fn fprot0(&self) -> &Fprot0 {
        &self.fprot0
    }
    #[doc = "0x0c - Non-volatile Flash Security Register"]
    #[inline(always)]
    pub const fn fsec(&self) -> &Fsec {
        &self.fsec
    }
    #[doc = "0x0d - Non-volatile Flash Option Register"]
    #[inline(always)]
    pub const fn fopt(&self) -> &Fopt {
        &self.fopt
    }
    #[doc = "0x0e - Non-volatile EERAM Protection Register"]
    #[inline(always)]
    pub const fn feprot(&self) -> &Feprot {
        &self.feprot
    }
    #[doc = "0x0f - Non-volatile D-Flash Protection Register"]
    #[inline(always)]
    pub const fn fdprot(&self) -> &Fdprot {
        &self.fdprot
    }
}
#[doc = "BACKKEY3 (r) register accessor: Backdoor Comparison Key 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backkey3`]
module"]
#[doc(alias = "BACKKEY3")]
pub type Backkey3 = crate::Reg<backkey3::Backkey3Spec>;
#[doc = "Backdoor Comparison Key 3."]
pub mod backkey3;
#[doc = "BACKKEY2 (r) register accessor: Backdoor Comparison Key 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backkey2`]
module"]
#[doc(alias = "BACKKEY2")]
pub type Backkey2 = crate::Reg<backkey2::Backkey2Spec>;
#[doc = "Backdoor Comparison Key 2."]
pub mod backkey2;
#[doc = "BACKKEY1 (r) register accessor: Backdoor Comparison Key 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backkey1`]
module"]
#[doc(alias = "BACKKEY1")]
pub type Backkey1 = crate::Reg<backkey1::Backkey1Spec>;
#[doc = "Backdoor Comparison Key 1."]
pub mod backkey1;
#[doc = "BACKKEY0 (r) register accessor: Backdoor Comparison Key 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backkey0`]
module"]
#[doc(alias = "BACKKEY0")]
pub type Backkey0 = crate::Reg<backkey0::Backkey0Spec>;
#[doc = "Backdoor Comparison Key 0."]
pub mod backkey0;
#[doc = "BACKKEY7 (r) register accessor: Backdoor Comparison Key 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backkey7`]
module"]
#[doc(alias = "BACKKEY7")]
pub type Backkey7 = crate::Reg<backkey7::Backkey7Spec>;
#[doc = "Backdoor Comparison Key 7."]
pub mod backkey7;
#[doc = "BACKKEY6 (r) register accessor: Backdoor Comparison Key 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backkey6`]
module"]
#[doc(alias = "BACKKEY6")]
pub type Backkey6 = crate::Reg<backkey6::Backkey6Spec>;
#[doc = "Backdoor Comparison Key 6."]
pub mod backkey6;
#[doc = "BACKKEY5 (r) register accessor: Backdoor Comparison Key 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backkey5`]
module"]
#[doc(alias = "BACKKEY5")]
pub type Backkey5 = crate::Reg<backkey5::Backkey5Spec>;
#[doc = "Backdoor Comparison Key 5."]
pub mod backkey5;
#[doc = "BACKKEY4 (r) register accessor: Backdoor Comparison Key 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backkey4`]
module"]
#[doc(alias = "BACKKEY4")]
pub type Backkey4 = crate::Reg<backkey4::Backkey4Spec>;
#[doc = "Backdoor Comparison Key 4."]
pub mod backkey4;
#[doc = "FPROT3 (r) register accessor: Non-volatile P-Flash Protection 1 - Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fprot3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fprot3`]
module"]
#[doc(alias = "FPROT3")]
pub type Fprot3 = crate::Reg<fprot3::Fprot3Spec>;
#[doc = "Non-volatile P-Flash Protection 1 - Low Register"]
pub mod fprot3;
#[doc = "FPROT2 (r) register accessor: Non-volatile P-Flash Protection 1 - High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fprot2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fprot2`]
module"]
#[doc(alias = "FPROT2")]
pub type Fprot2 = crate::Reg<fprot2::Fprot2Spec>;
#[doc = "Non-volatile P-Flash Protection 1 - High Register"]
pub mod fprot2;
#[doc = "FPROT1 (r) register accessor: Non-volatile P-Flash Protection 0 - Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fprot1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fprot1`]
module"]
#[doc(alias = "FPROT1")]
pub type Fprot1 = crate::Reg<fprot1::Fprot1Spec>;
#[doc = "Non-volatile P-Flash Protection 0 - Low Register"]
pub mod fprot1;
#[doc = "FPROT0 (r) register accessor: Non-volatile P-Flash Protection 0 - High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fprot0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fprot0`]
module"]
#[doc(alias = "FPROT0")]
pub type Fprot0 = crate::Reg<fprot0::Fprot0Spec>;
#[doc = "Non-volatile P-Flash Protection 0 - High Register"]
pub mod fprot0;
#[doc = "FSEC (r) register accessor: Non-volatile Flash Security Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsec`]
module"]
#[doc(alias = "FSEC")]
pub type Fsec = crate::Reg<fsec::FsecSpec>;
#[doc = "Non-volatile Flash Security Register"]
pub mod fsec;
#[doc = "FOPT (r) register accessor: Non-volatile Flash Option Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fopt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fopt`]
module"]
#[doc(alias = "FOPT")]
pub type Fopt = crate::Reg<fopt::FoptSpec>;
#[doc = "Non-volatile Flash Option Register"]
pub mod fopt;
#[doc = "FEPROT (r) register accessor: Non-volatile EERAM Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`feprot::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feprot`]
module"]
#[doc(alias = "FEPROT")]
pub type Feprot = crate::Reg<feprot::FeprotSpec>;
#[doc = "Non-volatile EERAM Protection Register"]
pub mod feprot;
#[doc = "FDPROT (r) register accessor: Non-volatile D-Flash Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdprot::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdprot`]
module"]
#[doc(alias = "FDPROT")]
pub type Fdprot = crate::Reg<fdprot::FdprotSpec>;
#[doc = "Non-volatile D-Flash Protection Register"]
pub mod fdprot;
