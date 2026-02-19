#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gencs: Gencs,
    scanc: Scanc,
    pen: Pen,
    wucntr: Wucntr,
    _reserved4: [u8; 0xf0],
    cntr: [Cntr; 8],
    threshold: Threshold,
}
impl RegisterBlock {
    #[doc = "0x00 - General Control and Status Register"]
    #[inline(always)]
    pub const fn gencs(&self) -> &Gencs {
        &self.gencs
    }
    #[doc = "0x04 - SCAN Control Register"]
    #[inline(always)]
    pub const fn scanc(&self) -> &Scanc {
        &self.scanc
    }
    #[doc = "0x08 - Pin Enable Register"]
    #[inline(always)]
    pub const fn pen(&self) -> &Pen {
        &self.pen
    }
    #[doc = "0x0c - Wake-Up Channel Counter Register"]
    #[inline(always)]
    pub const fn wucntr(&self) -> &Wucntr {
        &self.wucntr
    }
    #[doc = "0x100..0x120 - Counter Register"]
    #[inline(always)]
    pub const fn cntr(&self, n: usize) -> &Cntr {
        &self.cntr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - Counter Register"]
    #[inline(always)]
    pub fn cntr_iter(&self) -> impl Iterator<Item = &Cntr> {
        self.cntr.iter()
    }
    #[doc = "0x100 - Counter Register"]
    #[inline(always)]
    pub const fn cntr1(&self) -> &Cntr {
        self.cntr(0)
    }
    #[doc = "0x104 - Counter Register"]
    #[inline(always)]
    pub const fn cntr3(&self) -> &Cntr {
        self.cntr(1)
    }
    #[doc = "0x108 - Counter Register"]
    #[inline(always)]
    pub const fn cntr5(&self) -> &Cntr {
        self.cntr(2)
    }
    #[doc = "0x10c - Counter Register"]
    #[inline(always)]
    pub const fn cntr7(&self) -> &Cntr {
        self.cntr(3)
    }
    #[doc = "0x110 - Counter Register"]
    #[inline(always)]
    pub const fn cntr9(&self) -> &Cntr {
        self.cntr(4)
    }
    #[doc = "0x114 - Counter Register"]
    #[inline(always)]
    pub const fn cntr11(&self) -> &Cntr {
        self.cntr(5)
    }
    #[doc = "0x118 - Counter Register"]
    #[inline(always)]
    pub const fn cntr13(&self) -> &Cntr {
        self.cntr(6)
    }
    #[doc = "0x11c - Counter Register"]
    #[inline(always)]
    pub const fn cntr15(&self) -> &Cntr {
        self.cntr(7)
    }
    #[doc = "0x120 - Low Power Channel Threshold Register"]
    #[inline(always)]
    pub const fn threshold(&self) -> &Threshold {
        &self.threshold
    }
}
#[doc = "GENCS (rw) register accessor: General Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gencs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gencs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gencs`]
module"]
#[doc(alias = "GENCS")]
pub type Gencs = crate::Reg<gencs::GencsSpec>;
#[doc = "General Control and Status Register"]
pub mod gencs;
#[doc = "SCANC (rw) register accessor: SCAN Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scanc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanc`]
module"]
#[doc(alias = "SCANC")]
pub type Scanc = crate::Reg<scanc::ScancSpec>;
#[doc = "SCAN Control Register"]
pub mod scanc;
#[doc = "PEN (rw) register accessor: Pin Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pen`]
module"]
#[doc(alias = "PEN")]
pub type Pen = crate::Reg<pen::PenSpec>;
#[doc = "Pin Enable Register"]
pub mod pen;
#[doc = "WUCNTR (r) register accessor: Wake-Up Channel Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wucntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wucntr`]
module"]
#[doc(alias = "WUCNTR")]
pub type Wucntr = crate::Reg<wucntr::WucntrSpec>;
#[doc = "Wake-Up Channel Counter Register"]
pub mod wucntr;
#[doc = "CNTR (r) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
#[doc(alias = "CNTR")]
pub type Cntr = crate::Reg<cntr::CntrSpec>;
#[doc = "Counter Register"]
pub mod cntr;
#[doc = "THRESHOLD (rw) register accessor: Low Power Channel Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@threshold`]
module"]
#[doc(alias = "THRESHOLD")]
pub type Threshold = crate::Reg<threshold::ThresholdSpec>;
#[doc = "Low Power Channel Threshold Register"]
pub mod threshold;
