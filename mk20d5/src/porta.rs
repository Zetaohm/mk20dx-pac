#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcr: [Pcr; 32],
    gpclr: Gpclr,
    gpchr: Gpchr,
    _reserved3: [u8; 0x18],
    isfr: Isfr,
}
impl RegisterBlock {
    #[doc = "0x00..0x80 - Pin Control Register n"]
    #[inline(always)]
    pub const fn pcr(&self, n: usize) -> &Pcr {
        &self.pcr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - Pin Control Register n"]
    #[inline(always)]
    pub fn pcr_iter(&self) -> impl Iterator<Item = &Pcr> {
        self.pcr.iter()
    }
    #[doc = "0x80 - Global Pin Control Low Register"]
    #[inline(always)]
    pub const fn gpclr(&self) -> &Gpclr {
        &self.gpclr
    }
    #[doc = "0x84 - Global Pin Control High Register"]
    #[inline(always)]
    pub const fn gpchr(&self) -> &Gpchr {
        &self.gpchr
    }
    #[doc = "0xa0 - Interrupt Status Flag Register"]
    #[inline(always)]
    pub const fn isfr(&self) -> &Isfr {
        &self.isfr
    }
}
#[doc = "PCR (rw) register accessor: Pin Control Register n\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`]
module"]
#[doc(alias = "PCR")]
pub type Pcr = crate::Reg<pcr::PcrSpec>;
#[doc = "Pin Control Register n"]
pub mod pcr;
#[doc = "GPCLR (w) register accessor: Global Pin Control Low Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpclr`]
module"]
#[doc(alias = "GPCLR")]
pub type Gpclr = crate::Reg<gpclr::GpclrSpec>;
#[doc = "Global Pin Control Low Register"]
pub mod gpclr;
#[doc = "GPCHR (w) register accessor: Global Pin Control High Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpchr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpchr`]
module"]
#[doc(alias = "GPCHR")]
pub type Gpchr = crate::Reg<gpchr::GpchrSpec>;
#[doc = "Global Pin Control High Register"]
pub mod gpchr;
#[doc = "ISFR (rw) register accessor: Interrupt Status Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isfr`]
module"]
#[doc(alias = "ISFR")]
pub type Isfr = crate::Reg<isfr::IsfrSpec>;
#[doc = "Interrupt Status Flag Register"]
pub mod isfr;
