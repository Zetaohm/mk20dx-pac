#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    prs: (),
    _reserved1: [u8; 0x10],
    crs: (),
    _reserved2: [u8; 0x07f0],
    mgpcr: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - Priority Registers Slave"]
    #[inline(always)]
    pub const fn prs(&self, n: usize) -> &Prs {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Priority Registers Slave"]
    #[inline(always)]
    pub fn prs_iter(&self) -> impl Iterator<Item = &Prs> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x10..0x20 - Control Register"]
    #[inline(always)]
    pub const fn crs(&self, n: usize) -> &Crs {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - Control Register"]
    #[inline(always)]
    pub fn crs_iter(&self) -> impl Iterator<Item = &Crs> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x800..0x810 - Master General Purpose Control Register"]
    #[inline(always)]
    pub const fn mgpcr(&self, n: usize) -> &Mgpcr {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(2048)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x810 - Master General Purpose Control Register"]
    #[inline(always)]
    pub fn mgpcr_iter(&self) -> impl Iterator<Item = &Mgpcr> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(2048)
                .add(256 * n)
                .cast()
        })
    }
}
#[doc = "PRS (rw) register accessor: Priority Registers Slave\n\nYou can [`read`](crate::Reg::read) this register and get [`prs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs`]
module"]
#[doc(alias = "PRS")]
pub type Prs = crate::Reg<prs::PrsSpec>;
#[doc = "Priority Registers Slave"]
pub mod prs;
#[doc = "CRS (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crs`]
module"]
#[doc(alias = "CRS")]
pub type Crs = crate::Reg<crs::CrsSpec>;
#[doc = "Control Register"]
pub mod crs;
#[doc = "MGPCR (rw) register accessor: Master General Purpose Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mgpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mgpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mgpcr`]
module"]
#[doc(alias = "MGPCR")]
pub type Mgpcr = crate::Reg<mgpcr::MgpcrSpec>;
#[doc = "Master General Purpose Control Register"]
pub mod mgpcr;
