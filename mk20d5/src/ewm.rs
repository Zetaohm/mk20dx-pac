#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    serv: Serv,
    cmpl: Cmpl,
    cmph: Cmph,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x01 - Service Register"]
    #[inline(always)]
    pub const fn serv(&self) -> &Serv {
        &self.serv
    }
    #[doc = "0x02 - Compare Low Register"]
    #[inline(always)]
    pub const fn cmpl(&self) -> &Cmpl {
        &self.cmpl
    }
    #[doc = "0x03 - Compare High Register"]
    #[inline(always)]
    pub const fn cmph(&self) -> &Cmph {
        &self.cmph
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "SERV (w) register accessor: Service Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`serv::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@serv`]
module"]
#[doc(alias = "SERV")]
pub type Serv = crate::Reg<serv::ServSpec>;
#[doc = "Service Register"]
pub mod serv;
#[doc = "CMPL (rw) register accessor: Compare Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpl`]
module"]
#[doc(alias = "CMPL")]
pub type Cmpl = crate::Reg<cmpl::CmplSpec>;
#[doc = "Compare Low Register"]
pub mod cmpl;
#[doc = "CMPH (rw) register accessor: Compare High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmph::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmph::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmph`]
module"]
#[doc(alias = "CMPH")]
pub type Cmph = crate::Reg<cmph::CmphSpec>;
#[doc = "Compare High Register"]
pub mod cmph;
