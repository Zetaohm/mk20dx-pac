#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bdh: Bdh,
    bdl: Bdl,
    c1: C1,
    c2: C2,
    s1: S1,
    s2: S2,
    c3: C3,
    d: D,
    ma1: Ma1,
    ma2: Ma2,
    c4: C4,
    c5: C5,
    ed: Ed,
    modem: Modem,
    ir: Ir,
    _reserved15: [u8; 0x01],
    pfifo: Pfifo,
    cfifo: Cfifo,
    sfifo: Sfifo,
    twfifo: Twfifo,
    tcfifo: Tcfifo,
    rwfifo: Rwfifo,
    rcfifo: Rcfifo,
    _reserved22: [u8; 0x01],
    c7816: C7816,
    ie7816: Ie7816,
    is7816: Is7816,
    _reserved_25_uart0_wp7816t: [u8; 0x01],
    wn7816: Wn7816,
    wf7816: Wf7816,
    et7816: Et7816,
    tl7816: Tl7816,
    _reserved30: [u8; 0x01],
    c6: C6,
    pcth: Pcth,
    pctl: Pctl,
    b1t: B1t,
    sdth: Sdth,
    sdtl: Sdtl,
    pre: Pre,
    tpl: Tpl,
    ie: Ie,
    wb: Wb,
    s3: S3,
    s4: S4,
    rpl: Rpl,
    rprel: Rprel,
    cpw: Cpw,
    ridt: Ridt,
    tidt: Tidt,
}
impl RegisterBlock {
    #[doc = "0x00 - UART Baud Rate Registers:High"]
    #[inline(always)]
    pub const fn bdh(&self) -> &Bdh {
        &self.bdh
    }
    #[doc = "0x01 - UART Baud Rate Registers: Low"]
    #[inline(always)]
    pub const fn bdl(&self) -> &Bdl {
        &self.bdl
    }
    #[doc = "0x02 - UART Control Register 1"]
    #[inline(always)]
    pub const fn c1(&self) -> &C1 {
        &self.c1
    }
    #[doc = "0x03 - UART Control Register 2"]
    #[inline(always)]
    pub const fn c2(&self) -> &C2 {
        &self.c2
    }
    #[doc = "0x04 - UART Status Register 1"]
    #[inline(always)]
    pub const fn s1(&self) -> &S1 {
        &self.s1
    }
    #[doc = "0x05 - UART Status Register 2"]
    #[inline(always)]
    pub const fn s2(&self) -> &S2 {
        &self.s2
    }
    #[doc = "0x06 - UART Control Register 3"]
    #[inline(always)]
    pub const fn c3(&self) -> &C3 {
        &self.c3
    }
    #[doc = "0x07 - UART Data Register"]
    #[inline(always)]
    pub const fn d(&self) -> &D {
        &self.d
    }
    #[doc = "0x08 - UART Match Address Registers 1"]
    #[inline(always)]
    pub const fn ma1(&self) -> &Ma1 {
        &self.ma1
    }
    #[doc = "0x09 - UART Match Address Registers 2"]
    #[inline(always)]
    pub const fn ma2(&self) -> &Ma2 {
        &self.ma2
    }
    #[doc = "0x0a - UART Control Register 4"]
    #[inline(always)]
    pub const fn c4(&self) -> &C4 {
        &self.c4
    }
    #[doc = "0x0b - UART Control Register 5"]
    #[inline(always)]
    pub const fn c5(&self) -> &C5 {
        &self.c5
    }
    #[doc = "0x0c - UART Extended Data Register"]
    #[inline(always)]
    pub const fn ed(&self) -> &Ed {
        &self.ed
    }
    #[doc = "0x0d - UART Modem Register"]
    #[inline(always)]
    pub const fn modem(&self) -> &Modem {
        &self.modem
    }
    #[doc = "0x0e - UART Infrared Register"]
    #[inline(always)]
    pub const fn ir(&self) -> &Ir {
        &self.ir
    }
    #[doc = "0x10 - UART FIFO Parameters"]
    #[inline(always)]
    pub const fn pfifo(&self) -> &Pfifo {
        &self.pfifo
    }
    #[doc = "0x11 - UART FIFO Control Register"]
    #[inline(always)]
    pub const fn cfifo(&self) -> &Cfifo {
        &self.cfifo
    }
    #[doc = "0x12 - UART FIFO Status Register"]
    #[inline(always)]
    pub const fn sfifo(&self) -> &Sfifo {
        &self.sfifo
    }
    #[doc = "0x13 - UART FIFO Transmit Watermark"]
    #[inline(always)]
    pub const fn twfifo(&self) -> &Twfifo {
        &self.twfifo
    }
    #[doc = "0x14 - UART FIFO Transmit Count"]
    #[inline(always)]
    pub const fn tcfifo(&self) -> &Tcfifo {
        &self.tcfifo
    }
    #[doc = "0x15 - UART FIFO Receive Watermark"]
    #[inline(always)]
    pub const fn rwfifo(&self) -> &Rwfifo {
        &self.rwfifo
    }
    #[doc = "0x16 - UART FIFO Receive Count"]
    #[inline(always)]
    pub const fn rcfifo(&self) -> &Rcfifo {
        &self.rcfifo
    }
    #[doc = "0x18 - UART 7816 Control Register"]
    #[inline(always)]
    pub const fn c7816(&self) -> &C7816 {
        &self.c7816
    }
    #[doc = "0x19 - UART 7816 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie7816(&self) -> &Ie7816 {
        &self.ie7816
    }
    #[doc = "0x1a - UART 7816 Interrupt Status Register"]
    #[inline(always)]
    pub const fn is7816(&self) -> &Is7816 {
        &self.is7816
    }
    #[doc = "0x1b - UART 7816 Wait Parameter Register"]
    #[inline(always)]
    pub const fn uart0_wp7816t1(&self) -> &Uart0Wp7816t1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(27).cast() }
    }
    #[doc = "0x1b - UART 7816 Wait Parameter Register"]
    #[inline(always)]
    pub const fn uart0_wp7816t0(&self) -> &Uart0Wp7816t0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(27).cast() }
    }
    #[doc = "0x1c - UART 7816 Wait N Register"]
    #[inline(always)]
    pub const fn wn7816(&self) -> &Wn7816 {
        &self.wn7816
    }
    #[doc = "0x1d - UART 7816 Wait FD Register"]
    #[inline(always)]
    pub const fn wf7816(&self) -> &Wf7816 {
        &self.wf7816
    }
    #[doc = "0x1e - UART 7816 Error Threshold Register"]
    #[inline(always)]
    pub const fn et7816(&self) -> &Et7816 {
        &self.et7816
    }
    #[doc = "0x1f - UART 7816 Transmit Length Register"]
    #[inline(always)]
    pub const fn tl7816(&self) -> &Tl7816 {
        &self.tl7816
    }
    #[doc = "0x21 - UART CEA709.1-B Control Register 6"]
    #[inline(always)]
    pub const fn c6(&self) -> &C6 {
        &self.c6
    }
    #[doc = "0x22 - UART CEA709.1-B Packet Cycle Time Counter High"]
    #[inline(always)]
    pub const fn pcth(&self) -> &Pcth {
        &self.pcth
    }
    #[doc = "0x23 - UART CEA709.1-B Packet Cycle Time Counter Low"]
    #[inline(always)]
    pub const fn pctl(&self) -> &Pctl {
        &self.pctl
    }
    #[doc = "0x24 - UART CEA709.1-B Beta1 Timer"]
    #[inline(always)]
    pub const fn b1t(&self) -> &B1t {
        &self.b1t
    }
    #[doc = "0x25 - UART CEA709.1-B Secondary Delay Timer High"]
    #[inline(always)]
    pub const fn sdth(&self) -> &Sdth {
        &self.sdth
    }
    #[doc = "0x26 - UART CEA709.1-B Secondary Delay Timer Low"]
    #[inline(always)]
    pub const fn sdtl(&self) -> &Sdtl {
        &self.sdtl
    }
    #[doc = "0x27 - UART CEA709.1-B Preamble"]
    #[inline(always)]
    pub const fn pre(&self) -> &Pre {
        &self.pre
    }
    #[doc = "0x28 - UART CEA709.1-B Transmit Packet Length"]
    #[inline(always)]
    pub const fn tpl(&self) -> &Tpl {
        &self.tpl
    }
    #[doc = "0x29 - UART CEA709.1-B Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x2a - UART CEA709.1-B WBASE"]
    #[inline(always)]
    pub const fn wb(&self) -> &Wb {
        &self.wb
    }
    #[doc = "0x2b - UART CEA709.1-B Status Register"]
    #[inline(always)]
    pub const fn s3(&self) -> &S3 {
        &self.s3
    }
    #[doc = "0x2c - UART CEA709.1-B Status Register"]
    #[inline(always)]
    pub const fn s4(&self) -> &S4 {
        &self.s4
    }
    #[doc = "0x2d - UART CEA709.1-B Received Packet Length"]
    #[inline(always)]
    pub const fn rpl(&self) -> &Rpl {
        &self.rpl
    }
    #[doc = "0x2e - UART CEA709.1-B Received Preamble Length"]
    #[inline(always)]
    pub const fn rprel(&self) -> &Rprel {
        &self.rprel
    }
    #[doc = "0x2f - UART CEA709.1-B Collision Pulse Width"]
    #[inline(always)]
    pub const fn cpw(&self) -> &Cpw {
        &self.cpw
    }
    #[doc = "0x30 - UART CEA709.1-B Receive Indeterminate Time"]
    #[inline(always)]
    pub const fn ridt(&self) -> &Ridt {
        &self.ridt
    }
    #[doc = "0x31 - UART CEA709.1-B Transmit Indeterminate Time"]
    #[inline(always)]
    pub const fn tidt(&self) -> &Tidt {
        &self.tidt
    }
}
#[doc = "BDH (rw) register accessor: UART Baud Rate Registers:High\n\nYou can [`read`](crate::Reg::read) this register and get [`bdh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdh`]
module"]
#[doc(alias = "BDH")]
pub type Bdh = crate::Reg<bdh::BdhSpec>;
#[doc = "UART Baud Rate Registers:High"]
pub mod bdh;
#[doc = "BDL (rw) register accessor: UART Baud Rate Registers: Low\n\nYou can [`read`](crate::Reg::read) this register and get [`bdl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdl`]
module"]
#[doc(alias = "BDL")]
pub type Bdl = crate::Reg<bdl::BdlSpec>;
#[doc = "UART Baud Rate Registers: Low"]
pub mod bdl;
#[doc = "C1 (rw) register accessor: UART Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1`]
module"]
pub type C1 = crate::Reg<c1::C1Spec>;
#[doc = "UART Control Register 1"]
pub mod c1;
#[doc = "C2 (rw) register accessor: UART Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2`]
module"]
pub type C2 = crate::Reg<c2::C2Spec>;
#[doc = "UART Control Register 2"]
pub mod c2;
#[doc = "S1 (r) register accessor: UART Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`s1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1`]
module"]
pub type S1 = crate::Reg<s1::S1Spec>;
#[doc = "UART Status Register 1"]
pub mod s1;
#[doc = "S2 (rw) register accessor: UART Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`s2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2`]
module"]
pub type S2 = crate::Reg<s2::S2Spec>;
#[doc = "UART Status Register 2"]
pub mod s2;
#[doc = "C3 (rw) register accessor: UART Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3`]
module"]
pub type C3 = crate::Reg<c3::C3Spec>;
#[doc = "UART Control Register 3"]
pub mod c3;
#[doc = "D (rw) register accessor: UART Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d`]
module"]
pub type D = crate::Reg<d::DSpec>;
#[doc = "UART Data Register"]
pub mod d;
#[doc = "MA1 (rw) register accessor: UART Match Address Registers 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ma1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ma1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ma1`]
module"]
#[doc(alias = "MA1")]
pub type Ma1 = crate::Reg<ma1::Ma1Spec>;
#[doc = "UART Match Address Registers 1"]
pub mod ma1;
#[doc = "MA2 (rw) register accessor: UART Match Address Registers 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ma2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ma2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ma2`]
module"]
#[doc(alias = "MA2")]
pub type Ma2 = crate::Reg<ma2::Ma2Spec>;
#[doc = "UART Match Address Registers 2"]
pub mod ma2;
#[doc = "C4 (rw) register accessor: UART Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4`]
module"]
pub type C4 = crate::Reg<c4::C4Spec>;
#[doc = "UART Control Register 4"]
pub mod c4;
#[doc = "C5 (rw) register accessor: UART Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`c5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5`]
module"]
pub type C5 = crate::Reg<c5::C5Spec>;
#[doc = "UART Control Register 5"]
pub mod c5;
#[doc = "ED (r) register accessor: UART Extended Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ed::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ed`]
module"]
#[doc(alias = "ED")]
pub type Ed = crate::Reg<ed::EdSpec>;
#[doc = "UART Extended Data Register"]
pub mod ed;
#[doc = "MODEM (rw) register accessor: UART Modem Register\n\nYou can [`read`](crate::Reg::read) this register and get [`modem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem`]
module"]
#[doc(alias = "MODEM")]
pub type Modem = crate::Reg<modem::ModemSpec>;
#[doc = "UART Modem Register"]
pub mod modem;
#[doc = "IR (rw) register accessor: UART Infrared Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
#[doc(alias = "IR")]
pub type Ir = crate::Reg<ir::IrSpec>;
#[doc = "UART Infrared Register"]
pub mod ir;
#[doc = "PFIFO (rw) register accessor: UART FIFO Parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`pfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfifo`]
module"]
#[doc(alias = "PFIFO")]
pub type Pfifo = crate::Reg<pfifo::PfifoSpec>;
#[doc = "UART FIFO Parameters"]
pub mod pfifo;
#[doc = "CFIFO (rw) register accessor: UART FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfifo`]
module"]
#[doc(alias = "CFIFO")]
pub type Cfifo = crate::Reg<cfifo::CfifoSpec>;
#[doc = "UART FIFO Control Register"]
pub mod cfifo;
#[doc = "SFIFO (rw) register accessor: UART FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfifo`]
module"]
#[doc(alias = "SFIFO")]
pub type Sfifo = crate::Reg<sfifo::SfifoSpec>;
#[doc = "UART FIFO Status Register"]
pub mod sfifo;
#[doc = "TWFIFO (rw) register accessor: UART FIFO Transmit Watermark\n\nYou can [`read`](crate::Reg::read) this register and get [`twfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twfifo`]
module"]
#[doc(alias = "TWFIFO")]
pub type Twfifo = crate::Reg<twfifo::TwfifoSpec>;
#[doc = "UART FIFO Transmit Watermark"]
pub mod twfifo;
#[doc = "TCFIFO (r) register accessor: UART FIFO Transmit Count\n\nYou can [`read`](crate::Reg::read) this register and get [`tcfifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcfifo`]
module"]
#[doc(alias = "TCFIFO")]
pub type Tcfifo = crate::Reg<tcfifo::TcfifoSpec>;
#[doc = "UART FIFO Transmit Count"]
pub mod tcfifo;
#[doc = "RWFIFO (rw) register accessor: UART FIFO Receive Watermark\n\nYou can [`read`](crate::Reg::read) this register and get [`rwfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwfifo`]
module"]
#[doc(alias = "RWFIFO")]
pub type Rwfifo = crate::Reg<rwfifo::RwfifoSpec>;
#[doc = "UART FIFO Receive Watermark"]
pub mod rwfifo;
#[doc = "RCFIFO (r) register accessor: UART FIFO Receive Count\n\nYou can [`read`](crate::Reg::read) this register and get [`rcfifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcfifo`]
module"]
#[doc(alias = "RCFIFO")]
pub type Rcfifo = crate::Reg<rcfifo::RcfifoSpec>;
#[doc = "UART FIFO Receive Count"]
pub mod rcfifo;
#[doc = "C7816 (rw) register accessor: UART 7816 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7816`]
module"]
pub type C7816 = crate::Reg<c7816::C7816Spec>;
#[doc = "UART 7816 Control Register"]
pub mod c7816;
#[doc = "IE7816 (rw) register accessor: UART 7816 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie7816`]
module"]
#[doc(alias = "IE7816")]
pub type Ie7816 = crate::Reg<ie7816::Ie7816Spec>;
#[doc = "UART 7816 Interrupt Enable Register"]
pub mod ie7816;
#[doc = "IS7816 (rw) register accessor: UART 7816 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`is7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`is7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is7816`]
module"]
#[doc(alias = "IS7816")]
pub type Is7816 = crate::Reg<is7816::Is7816Spec>;
#[doc = "UART 7816 Interrupt Status Register"]
pub mod is7816;
#[doc = "UART0_WP7816T0 (rw) register accessor: UART 7816 Wait Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_wp7816t0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_wp7816t0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_wp7816t0`]
module"]
#[doc(alias = "UART0_WP7816T0")]
pub type Uart0Wp7816t0 = crate::Reg<uart0_wp7816t0::Uart0Wp7816t0Spec>;
#[doc = "UART 7816 Wait Parameter Register"]
pub mod uart0_wp7816t0;
#[doc = "UART0_WP7816T1 (rw) register accessor: UART 7816 Wait Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_wp7816t1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_wp7816t1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_wp7816t1`]
module"]
#[doc(alias = "UART0_WP7816T1")]
pub type Uart0Wp7816t1 = crate::Reg<uart0_wp7816t1::Uart0Wp7816t1Spec>;
#[doc = "UART 7816 Wait Parameter Register"]
pub mod uart0_wp7816t1;
#[doc = "WN7816 (rw) register accessor: UART 7816 Wait N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wn7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wn7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wn7816`]
module"]
#[doc(alias = "WN7816")]
pub type Wn7816 = crate::Reg<wn7816::Wn7816Spec>;
#[doc = "UART 7816 Wait N Register"]
pub mod wn7816;
#[doc = "WF7816 (rw) register accessor: UART 7816 Wait FD Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wf7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf7816`]
module"]
#[doc(alias = "WF7816")]
pub type Wf7816 = crate::Reg<wf7816::Wf7816Spec>;
#[doc = "UART 7816 Wait FD Register"]
pub mod wf7816;
#[doc = "ET7816 (rw) register accessor: UART 7816 Error Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`et7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`et7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@et7816`]
module"]
#[doc(alias = "ET7816")]
pub type Et7816 = crate::Reg<et7816::Et7816Spec>;
#[doc = "UART 7816 Error Threshold Register"]
pub mod et7816;
#[doc = "TL7816 (rw) register accessor: UART 7816 Transmit Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tl7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tl7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tl7816`]
module"]
#[doc(alias = "TL7816")]
pub type Tl7816 = crate::Reg<tl7816::Tl7816Spec>;
#[doc = "UART 7816 Transmit Length Register"]
pub mod tl7816;
#[doc = "C6 (rw) register accessor: UART CEA709.1-B Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`c6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6`]
module"]
pub type C6 = crate::Reg<c6::C6Spec>;
#[doc = "UART CEA709.1-B Control Register 6"]
pub mod c6;
#[doc = "PCTH (rw) register accessor: UART CEA709.1-B Packet Cycle Time Counter High\n\nYou can [`read`](crate::Reg::read) this register and get [`pcth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcth`]
module"]
#[doc(alias = "PCTH")]
pub type Pcth = crate::Reg<pcth::PcthSpec>;
#[doc = "UART CEA709.1-B Packet Cycle Time Counter High"]
pub mod pcth;
#[doc = "PCTL (rw) register accessor: UART CEA709.1-B Packet Cycle Time Counter Low\n\nYou can [`read`](crate::Reg::read) this register and get [`pctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pctl`]
module"]
#[doc(alias = "PCTL")]
pub type Pctl = crate::Reg<pctl::PctlSpec>;
#[doc = "UART CEA709.1-B Packet Cycle Time Counter Low"]
pub mod pctl;
#[doc = "B1T (rw) register accessor: UART CEA709.1-B Beta1 Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`b1t::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b1t::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b1t`]
module"]
#[doc(alias = "B1T")]
pub type B1t = crate::Reg<b1t::B1tSpec>;
#[doc = "UART CEA709.1-B Beta1 Timer"]
pub mod b1t;
#[doc = "SDTH (rw) register accessor: UART CEA709.1-B Secondary Delay Timer High\n\nYou can [`read`](crate::Reg::read) this register and get [`sdth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdth`]
module"]
#[doc(alias = "SDTH")]
pub type Sdth = crate::Reg<sdth::SdthSpec>;
#[doc = "UART CEA709.1-B Secondary Delay Timer High"]
pub mod sdth;
#[doc = "SDTL (rw) register accessor: UART CEA709.1-B Secondary Delay Timer Low\n\nYou can [`read`](crate::Reg::read) this register and get [`sdtl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdtl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdtl`]
module"]
#[doc(alias = "SDTL")]
pub type Sdtl = crate::Reg<sdtl::SdtlSpec>;
#[doc = "UART CEA709.1-B Secondary Delay Timer Low"]
pub mod sdtl;
#[doc = "PRE (rw) register accessor: UART CEA709.1-B Preamble\n\nYou can [`read`](crate::Reg::read) this register and get [`pre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pre`]
module"]
#[doc(alias = "PRE")]
pub type Pre = crate::Reg<pre::PreSpec>;
#[doc = "UART CEA709.1-B Preamble"]
pub mod pre;
#[doc = "TPL (rw) register accessor: UART CEA709.1-B Transmit Packet Length\n\nYou can [`read`](crate::Reg::read) this register and get [`tpl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpl`]
module"]
#[doc(alias = "TPL")]
pub type Tpl = crate::Reg<tpl::TplSpec>;
#[doc = "UART CEA709.1-B Transmit Packet Length"]
pub mod tpl;
#[doc = "IE (rw) register accessor: UART CEA709.1-B Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
#[doc(alias = "IE")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "UART CEA709.1-B Interrupt Enable Register"]
pub mod ie;
#[doc = "WB (rw) register accessor: UART CEA709.1-B WBASE\n\nYou can [`read`](crate::Reg::read) this register and get [`wb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wb`]
module"]
#[doc(alias = "WB")]
pub type Wb = crate::Reg<wb::WbSpec>;
#[doc = "UART CEA709.1-B WBASE"]
pub mod wb;
#[doc = "S3 (rw) register accessor: UART CEA709.1-B Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3`]
module"]
pub type S3 = crate::Reg<s3::S3Spec>;
#[doc = "UART CEA709.1-B Status Register"]
pub mod s3;
#[doc = "S4 (rw) register accessor: UART CEA709.1-B Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4`]
module"]
pub type S4 = crate::Reg<s4::S4Spec>;
#[doc = "UART CEA709.1-B Status Register"]
pub mod s4;
#[doc = "RPL (r) register accessor: UART CEA709.1-B Received Packet Length\n\nYou can [`read`](crate::Reg::read) this register and get [`rpl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpl`]
module"]
#[doc(alias = "RPL")]
pub type Rpl = crate::Reg<rpl::RplSpec>;
#[doc = "UART CEA709.1-B Received Packet Length"]
pub mod rpl;
#[doc = "RPREL (r) register accessor: UART CEA709.1-B Received Preamble Length\n\nYou can [`read`](crate::Reg::read) this register and get [`rprel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rprel`]
module"]
#[doc(alias = "RPREL")]
pub type Rprel = crate::Reg<rprel::RprelSpec>;
#[doc = "UART CEA709.1-B Received Preamble Length"]
pub mod rprel;
#[doc = "CPW (rw) register accessor: UART CEA709.1-B Collision Pulse Width\n\nYou can [`read`](crate::Reg::read) this register and get [`cpw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpw`]
module"]
#[doc(alias = "CPW")]
pub type Cpw = crate::Reg<cpw::CpwSpec>;
#[doc = "UART CEA709.1-B Collision Pulse Width"]
pub mod cpw;
#[doc = "RIDT (rw) register accessor: UART CEA709.1-B Receive Indeterminate Time\n\nYou can [`read`](crate::Reg::read) this register and get [`ridt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ridt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ridt`]
module"]
#[doc(alias = "RIDT")]
pub type Ridt = crate::Reg<ridt::RidtSpec>;
#[doc = "UART CEA709.1-B Receive Indeterminate Time"]
pub mod ridt;
#[doc = "TIDT (rw) register accessor: UART CEA709.1-B Transmit Indeterminate Time\n\nYou can [`read`](crate::Reg::read) this register and get [`tidt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tidt`]
module"]
#[doc(alias = "TIDT")]
pub type Tidt = crate::Reg<tidt::TidtSpec>;
#[doc = "UART CEA709.1-B Transmit Indeterminate Time"]
pub mod tidt;
