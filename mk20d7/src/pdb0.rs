#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sc: Sc,
    mod_: Mod,
    cnt: Cnt,
    idly: Idly,
    chc1: (),
    _reserved5: [u8; 0x04],
    chs: (),
    _reserved6: [u8; 0x04],
    chdly0: (),
    _reserved7: [u8; 0x04],
    chdly1: (),
    _reserved8: [u8; 0x0134],
    dacintc: Dacintc,
    dacint: Dacint,
    _reserved10: [u8; 0x38],
    poen: Poen,
    podly: [Podly; 3],
}
impl RegisterBlock {
    #[doc = "0x00 - Status and Control Register"]
    #[inline(always)]
    pub const fn sc(&self) -> &Sc {
        &self.sc
    }
    #[doc = "0x04 - Modulus Register"]
    #[inline(always)]
    pub const fn mod_(&self) -> &Mod {
        &self.mod_
    }
    #[doc = "0x08 - Counter Register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x0c - Interrupt Delay Register"]
    #[inline(always)]
    pub const fn idly(&self) -> &Idly {
        &self.idly
    }
    #[doc = "0x10..0x18 - Channel n Control Register 1"]
    #[inline(always)]
    pub const fn chc1(&self, n: usize) -> &Chc1 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Channel n Control Register 1"]
    #[inline(always)]
    pub fn chc1_iter(&self) -> impl Iterator<Item = &Chc1> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x10 - Channel n Control Register 1"]
    #[inline(always)]
    pub const fn ch0c1(&self) -> &Chc1 {
        self.chc1(0)
    }
    #[doc = "0x38 - Channel n Control Register 1"]
    #[inline(always)]
    pub const fn ch1c1(&self) -> &Chc1 {
        self.chc1(1)
    }
    #[doc = "0x14..0x1c - Channel n Status Register"]
    #[inline(always)]
    pub const fn chs(&self, n: usize) -> &Chs {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(20)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x1c - Channel n Status Register"]
    #[inline(always)]
    pub fn chs_iter(&self) -> impl Iterator<Item = &Chs> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(20)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x14 - Channel n Status Register"]
    #[inline(always)]
    pub const fn ch0s(&self) -> &Chs {
        self.chs(0)
    }
    #[doc = "0x3c - Channel n Status Register"]
    #[inline(always)]
    pub const fn ch1s(&self) -> &Chs {
        self.chs(1)
    }
    #[doc = "0x18..0x20 - Channel n Delay 0 Register"]
    #[inline(always)]
    pub const fn chdly0(&self, n: usize) -> &Chdly0 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(24)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x20 - Channel n Delay 0 Register"]
    #[inline(always)]
    pub fn chdly0_iter(&self) -> impl Iterator<Item = &Chdly0> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(24)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x18 - Channel n Delay 0 Register"]
    #[inline(always)]
    pub const fn ch0dly0(&self) -> &Chdly0 {
        self.chdly0(0)
    }
    #[doc = "0x40 - Channel n Delay 0 Register"]
    #[inline(always)]
    pub const fn ch1dly0(&self) -> &Chdly0 {
        self.chdly0(1)
    }
    #[doc = "0x1c..0x24 - Channel n Delay 1 Register"]
    #[inline(always)]
    pub const fn chdly1(&self, n: usize) -> &Chdly1 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(28)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x24 - Channel n Delay 1 Register"]
    #[inline(always)]
    pub fn chdly1_iter(&self) -> impl Iterator<Item = &Chdly1> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(28)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x1c - Channel n Delay 1 Register"]
    #[inline(always)]
    pub const fn ch0dly1(&self) -> &Chdly1 {
        self.chdly1(0)
    }
    #[doc = "0x44 - Channel n Delay 1 Register"]
    #[inline(always)]
    pub const fn ch1dly1(&self) -> &Chdly1 {
        self.chdly1(1)
    }
    #[doc = "0x150 - DAC Interval Trigger n Control Register"]
    #[inline(always)]
    pub const fn dacintc(&self) -> &Dacintc {
        &self.dacintc
    }
    #[doc = "0x154 - DAC Interval n Register"]
    #[inline(always)]
    pub const fn dacint(&self) -> &Dacint {
        &self.dacint
    }
    #[doc = "0x190 - Pulse-Out n Enable Register"]
    #[inline(always)]
    pub const fn poen(&self) -> &Poen {
        &self.poen
    }
    #[doc = "0x194..0x1a0 - Pulse-Out n Delay Register"]
    #[inline(always)]
    pub const fn podly(&self, n: usize) -> &Podly {
        &self.podly[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x194..0x1a0 - Pulse-Out n Delay Register"]
    #[inline(always)]
    pub fn podly_iter(&self) -> impl Iterator<Item = &Podly> {
        self.podly.iter()
    }
    #[doc = "0x194 - Pulse-Out n Delay Register"]
    #[inline(always)]
    pub const fn po0dly(&self) -> &Podly {
        self.podly(0)
    }
    #[doc = "0x198 - Pulse-Out n Delay Register"]
    #[inline(always)]
    pub const fn po1dly(&self) -> &Podly {
        self.podly(1)
    }
    #[doc = "0x19c - Pulse-Out n Delay Register"]
    #[inline(always)]
    pub const fn po2dly(&self) -> &Podly {
        self.podly(2)
    }
}
#[doc = "SC (rw) register accessor: Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sc`]
module"]
#[doc(alias = "SC")]
pub type Sc = crate::Reg<sc::ScSpec>;
#[doc = "Status and Control Register"]
pub mod sc;
#[doc = "MOD (rw) register accessor: Modulus Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mod_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mod_`]
module"]
#[doc(alias = "MOD")]
pub type Mod = crate::Reg<mod_::ModSpec>;
#[doc = "Modulus Register"]
pub mod mod_;
#[doc = "CNT (r) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Counter Register"]
pub mod cnt;
#[doc = "IDLY (rw) register accessor: Interrupt Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idly`]
module"]
#[doc(alias = "IDLY")]
pub type Idly = crate::Reg<idly::IdlySpec>;
#[doc = "Interrupt Delay Register"]
pub mod idly;
#[doc = "CHC1 (rw) register accessor: Channel n Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`chc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chc1`]
module"]
#[doc(alias = "CHC1")]
pub type Chc1 = crate::Reg<chc1::Chc1Spec>;
#[doc = "Channel n Control Register 1"]
pub mod chc1;
#[doc = "CHS (rw) register accessor: Channel n Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chs`]
module"]
#[doc(alias = "CHS")]
pub type Chs = crate::Reg<chs::ChsSpec>;
#[doc = "Channel n Status Register"]
pub mod chs;
#[doc = "CHDLY0 (rw) register accessor: Channel n Delay 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chdly0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdly0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdly0`]
module"]
#[doc(alias = "CHDLY0")]
pub type Chdly0 = crate::Reg<chdly0::Chdly0Spec>;
#[doc = "Channel n Delay 0 Register"]
pub mod chdly0;
#[doc = "CHDLY1 (rw) register accessor: Channel n Delay 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chdly1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdly1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdly1`]
module"]
#[doc(alias = "CHDLY1")]
pub type Chdly1 = crate::Reg<chdly1::Chdly1Spec>;
#[doc = "Channel n Delay 1 Register"]
pub mod chdly1;
#[doc = "DACINTC (rw) register accessor: DAC Interval Trigger n Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dacintc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacintc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacintc`]
module"]
#[doc(alias = "DACINTC")]
pub type Dacintc = crate::Reg<dacintc::DacintcSpec>;
#[doc = "DAC Interval Trigger n Control Register"]
pub mod dacintc;
#[doc = "DACINT (rw) register accessor: DAC Interval n Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dacint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacint`]
module"]
#[doc(alias = "DACINT")]
pub type Dacint = crate::Reg<dacint::DacintSpec>;
#[doc = "DAC Interval n Register"]
pub mod dacint;
#[doc = "POEN (rw) register accessor: Pulse-Out n Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`poen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poen`]
module"]
#[doc(alias = "POEN")]
pub type Poen = crate::Reg<poen::PoenSpec>;
#[doc = "Pulse-Out n Enable Register"]
pub mod poen;
#[doc = "PODLY (rw) register accessor: Pulse-Out n Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`podly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@podly`]
module"]
#[doc(alias = "PODLY")]
pub type Podly = crate::Reg<podly::PodlySpec>;
#[doc = "Pulse-Out n Delay Register"]
pub mod podly;
