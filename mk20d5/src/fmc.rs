#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pfapr: Pfapr,
    pfb0cr: Pfb0cr,
    _reserved2: [u8; 0xf8],
    tagvdw0s: [Tagvdw0s; 2],
    tagvdw1s: [Tagvdw1s; 2],
    tagvdw2s: [Tagvdw2s; 2],
    tagvdw3s: [Tagvdw3s; 2],
    _reserved6: [u8; 0xe0],
    dataw0s: [Dataw0s; 2],
    dataw1s: [Dataw1s; 2],
    dataw2s: [Dataw2s; 2],
    dataw3s: [Dataw3s; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - Flash Access Protection Register"]
    #[inline(always)]
    pub const fn pfapr(&self) -> &Pfapr {
        &self.pfapr
    }
    #[doc = "0x04 - Flash Control Register"]
    #[inline(always)]
    pub const fn pfb0cr(&self) -> &Pfb0cr {
        &self.pfb0cr
    }
    #[doc = "0x100..0x108 - Cache Tag Storage"]
    #[inline(always)]
    pub const fn tagvdw0s(&self, n: usize) -> &Tagvdw0s {
        &self.tagvdw0s[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x108 - Cache Tag Storage"]
    #[inline(always)]
    pub fn tagvdw0s_iter(&self) -> impl Iterator<Item = &Tagvdw0s> {
        self.tagvdw0s.iter()
    }
    #[doc = "0x108..0x110 - Cache Tag Storage"]
    #[inline(always)]
    pub const fn tagvdw1s(&self, n: usize) -> &Tagvdw1s {
        &self.tagvdw1s[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x108..0x110 - Cache Tag Storage"]
    #[inline(always)]
    pub fn tagvdw1s_iter(&self) -> impl Iterator<Item = &Tagvdw1s> {
        self.tagvdw1s.iter()
    }
    #[doc = "0x110..0x118 - Cache Tag Storage"]
    #[inline(always)]
    pub const fn tagvdw2s(&self, n: usize) -> &Tagvdw2s {
        &self.tagvdw2s[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x118 - Cache Tag Storage"]
    #[inline(always)]
    pub fn tagvdw2s_iter(&self) -> impl Iterator<Item = &Tagvdw2s> {
        self.tagvdw2s.iter()
    }
    #[doc = "0x118..0x120 - Cache Tag Storage"]
    #[inline(always)]
    pub const fn tagvdw3s(&self, n: usize) -> &Tagvdw3s {
        &self.tagvdw3s[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x118..0x120 - Cache Tag Storage"]
    #[inline(always)]
    pub fn tagvdw3s_iter(&self) -> impl Iterator<Item = &Tagvdw3s> {
        self.tagvdw3s.iter()
    }
    #[doc = "0x200..0x208 - Cache Data Storage"]
    #[inline(always)]
    pub const fn dataw0s(&self, n: usize) -> &Dataw0s {
        &self.dataw0s[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x208 - Cache Data Storage"]
    #[inline(always)]
    pub fn dataw0s_iter(&self) -> impl Iterator<Item = &Dataw0s> {
        self.dataw0s.iter()
    }
    #[doc = "0x208..0x210 - Cache Data Storage"]
    #[inline(always)]
    pub const fn dataw1s(&self, n: usize) -> &Dataw1s {
        &self.dataw1s[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x208..0x210 - Cache Data Storage"]
    #[inline(always)]
    pub fn dataw1s_iter(&self) -> impl Iterator<Item = &Dataw1s> {
        self.dataw1s.iter()
    }
    #[doc = "0x210..0x218 - Cache Data Storage"]
    #[inline(always)]
    pub const fn dataw2s(&self, n: usize) -> &Dataw2s {
        &self.dataw2s[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x210..0x218 - Cache Data Storage"]
    #[inline(always)]
    pub fn dataw2s_iter(&self) -> impl Iterator<Item = &Dataw2s> {
        self.dataw2s.iter()
    }
    #[doc = "0x218..0x220 - Cache Data Storage"]
    #[inline(always)]
    pub const fn dataw3s(&self, n: usize) -> &Dataw3s {
        &self.dataw3s[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x218..0x220 - Cache Data Storage"]
    #[inline(always)]
    pub fn dataw3s_iter(&self) -> impl Iterator<Item = &Dataw3s> {
        self.dataw3s.iter()
    }
}
#[doc = "PFAPR (rw) register accessor: Flash Access Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfapr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfapr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfapr`]
module"]
#[doc(alias = "PFAPR")]
pub type Pfapr = crate::Reg<pfapr::PfaprSpec>;
#[doc = "Flash Access Protection Register"]
pub mod pfapr;
#[doc = "PFB0CR (rw) register accessor: Flash Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfb0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfb0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfb0cr`]
module"]
#[doc(alias = "PFB0CR")]
pub type Pfb0cr = crate::Reg<pfb0cr::Pfb0crSpec>;
#[doc = "Flash Control Register"]
pub mod pfb0cr;
#[doc = "TAGVDW0S (rw) register accessor: Cache Tag Storage\n\nYou can [`read`](crate::Reg::read) this register and get [`tagvdw0s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tagvdw0s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tagvdw0s`]
module"]
#[doc(alias = "TAGVDW0S")]
pub type Tagvdw0s = crate::Reg<tagvdw0s::Tagvdw0sSpec>;
#[doc = "Cache Tag Storage"]
pub mod tagvdw0s;
#[doc = "TAGVDW1S (rw) register accessor: Cache Tag Storage\n\nYou can [`read`](crate::Reg::read) this register and get [`tagvdw1s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tagvdw1s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tagvdw1s`]
module"]
#[doc(alias = "TAGVDW1S")]
pub type Tagvdw1s = crate::Reg<tagvdw1s::Tagvdw1sSpec>;
#[doc = "Cache Tag Storage"]
pub mod tagvdw1s;
#[doc = "TAGVDW2S (rw) register accessor: Cache Tag Storage\n\nYou can [`read`](crate::Reg::read) this register and get [`tagvdw2s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tagvdw2s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tagvdw2s`]
module"]
#[doc(alias = "TAGVDW2S")]
pub type Tagvdw2s = crate::Reg<tagvdw2s::Tagvdw2sSpec>;
#[doc = "Cache Tag Storage"]
pub mod tagvdw2s;
#[doc = "TAGVDW3S (rw) register accessor: Cache Tag Storage\n\nYou can [`read`](crate::Reg::read) this register and get [`tagvdw3s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tagvdw3s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tagvdw3s`]
module"]
#[doc(alias = "TAGVDW3S")]
pub type Tagvdw3s = crate::Reg<tagvdw3s::Tagvdw3sSpec>;
#[doc = "Cache Tag Storage"]
pub mod tagvdw3s;
#[doc = "DATAW0S (rw) register accessor: Cache Data Storage\n\nYou can [`read`](crate::Reg::read) this register and get [`dataw0s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataw0s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataw0s`]
module"]
#[doc(alias = "DATAW0S")]
pub type Dataw0s = crate::Reg<dataw0s::Dataw0sSpec>;
#[doc = "Cache Data Storage"]
pub mod dataw0s;
#[doc = "DATAW1S (rw) register accessor: Cache Data Storage\n\nYou can [`read`](crate::Reg::read) this register and get [`dataw1s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataw1s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataw1s`]
module"]
#[doc(alias = "DATAW1S")]
pub type Dataw1s = crate::Reg<dataw1s::Dataw1sSpec>;
#[doc = "Cache Data Storage"]
pub mod dataw1s;
#[doc = "DATAW2S (rw) register accessor: Cache Data Storage\n\nYou can [`read`](crate::Reg::read) this register and get [`dataw2s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataw2s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataw2s`]
module"]
#[doc(alias = "DATAW2S")]
pub type Dataw2s = crate::Reg<dataw2s::Dataw2sSpec>;
#[doc = "Cache Data Storage"]
pub mod dataw2s;
#[doc = "DATAW3S (rw) register accessor: Cache Data Storage\n\nYou can [`read`](crate::Reg::read) this register and get [`dataw3s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataw3s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataw3s`]
module"]
#[doc(alias = "DATAW3S")]
pub type Dataw3s = crate::Reg<dataw3s::Dataw3sSpec>;
#[doc = "Cache Data Storage"]
pub mod dataw3s;
