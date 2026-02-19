#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    plasc: Plasc,
    plamc: Plamc,
    cr: Cr,
}
impl RegisterBlock {
    #[doc = "0x08 - Crossbar switch (AXBS) slave configuration"]
    #[inline(always)]
    pub const fn plasc(&self) -> &Plasc {
        &self.plasc
    }
    #[doc = "0x0a - Crossbar switch (AXBS) master configuration"]
    #[inline(always)]
    pub const fn plamc(&self) -> &Plamc {
        &self.plamc
    }
    #[doc = "0x0c - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
}
#[doc = "PLASC (r) register accessor: Crossbar switch (AXBS) slave configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`plasc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plasc`]
module"]
#[doc(alias = "PLASC")]
pub type Plasc = crate::Reg<plasc::PlascSpec>;
#[doc = "Crossbar switch (AXBS) slave configuration"]
pub mod plasc;
#[doc = "PLAMC (r) register accessor: Crossbar switch (AXBS) master configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`plamc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plamc`]
module"]
#[doc(alias = "PLAMC")]
pub type Plamc = crate::Reg<plamc::PlamcSpec>;
#[doc = "Crossbar switch (AXBS) master configuration"]
pub mod plamc;
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register"]
pub mod cr;
