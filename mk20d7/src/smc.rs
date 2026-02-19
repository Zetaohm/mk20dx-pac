#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pmprot: Pmprot,
    pmctrl: Pmctrl,
    vllsctrl: Vllsctrl,
    pmstat: Pmstat,
}
impl RegisterBlock {
    #[doc = "0x00 - Power Mode Protection Register"]
    #[inline(always)]
    pub const fn pmprot(&self) -> &Pmprot {
        &self.pmprot
    }
    #[doc = "0x01 - Power Mode Control Register"]
    #[inline(always)]
    pub const fn pmctrl(&self) -> &Pmctrl {
        &self.pmctrl
    }
    #[doc = "0x02 - VLLS Control Register"]
    #[inline(always)]
    pub const fn vllsctrl(&self) -> &Vllsctrl {
        &self.vllsctrl
    }
    #[doc = "0x03 - Power Mode Status Register"]
    #[inline(always)]
    pub const fn pmstat(&self) -> &Pmstat {
        &self.pmstat
    }
}
#[doc = "PMPROT (rw) register accessor: Power Mode Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmprot`]
module"]
#[doc(alias = "PMPROT")]
pub type Pmprot = crate::Reg<pmprot::PmprotSpec>;
#[doc = "Power Mode Protection Register"]
pub mod pmprot;
#[doc = "PMCTRL (rw) register accessor: Power Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmctrl`]
module"]
#[doc(alias = "PMCTRL")]
pub type Pmctrl = crate::Reg<pmctrl::PmctrlSpec>;
#[doc = "Power Mode Control Register"]
pub mod pmctrl;
#[doc = "VLLSCTRL (rw) register accessor: VLLS Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vllsctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vllsctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vllsctrl`]
module"]
#[doc(alias = "VLLSCTRL")]
pub type Vllsctrl = crate::Reg<vllsctrl::VllsctrlSpec>;
#[doc = "VLLS Control Register"]
pub mod vllsctrl;
#[doc = "PMSTAT (r) register accessor: Power Mode Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmstat`]
module"]
#[doc(alias = "PMSTAT")]
pub type Pmstat = crate::Reg<pmstat::PmstatSpec>;
#[doc = "Power Mode Status Register"]
pub mod pmstat;
