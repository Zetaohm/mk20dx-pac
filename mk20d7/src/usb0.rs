#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    perid: Perid,
    _reserved1: [u8; 0x03],
    idcomp: Idcomp,
    _reserved2: [u8; 0x03],
    rev: Rev,
    _reserved3: [u8; 0x03],
    addinfo: Addinfo,
    _reserved4: [u8; 0x03],
    otgistat: Otgistat,
    _reserved5: [u8; 0x03],
    otgicr: Otgicr,
    _reserved6: [u8; 0x03],
    otgstat: Otgstat,
    _reserved7: [u8; 0x03],
    otgctl: Otgctl,
    _reserved8: [u8; 0x63],
    istat: Istat,
    _reserved9: [u8; 0x03],
    inten: Inten,
    _reserved10: [u8; 0x03],
    errstat: Errstat,
    _reserved11: [u8; 0x03],
    erren: Erren,
    _reserved12: [u8; 0x03],
    stat: Stat,
    _reserved13: [u8; 0x03],
    ctl: Ctl,
    _reserved14: [u8; 0x03],
    addr: Addr,
    _reserved15: [u8; 0x03],
    bdtpage1: Bdtpage1,
    _reserved16: [u8; 0x03],
    frmnuml: Frmnuml,
    _reserved17: [u8; 0x03],
    frmnumh: Frmnumh,
    _reserved18: [u8; 0x03],
    token: Token,
    _reserved19: [u8; 0x03],
    softhld: Softhld,
    _reserved20: [u8; 0x03],
    bdtpage2: Bdtpage2,
    _reserved21: [u8; 0x03],
    bdtpage3: Bdtpage3,
    _reserved22: [u8; 0x0b],
    endpt: (),
    _reserved23: [u8; 0x40],
    usbctrl: Usbctrl,
    _reserved24: [u8; 0x03],
    observe: Observe,
    _reserved25: [u8; 0x03],
    control: Control,
    _reserved26: [u8; 0x03],
    usbtrc0: Usbtrc0,
    _reserved27: [u8; 0x07],
    usbfrmadjust: Usbfrmadjust,
}
impl RegisterBlock {
    #[doc = "0x00 - Peripheral ID Register"]
    #[inline(always)]
    pub const fn perid(&self) -> &Perid {
        &self.perid
    }
    #[doc = "0x04 - Peripheral ID Complement Register"]
    #[inline(always)]
    pub const fn idcomp(&self) -> &Idcomp {
        &self.idcomp
    }
    #[doc = "0x08 - Peripheral Revision Register"]
    #[inline(always)]
    pub const fn rev(&self) -> &Rev {
        &self.rev
    }
    #[doc = "0x0c - Peripheral Additional Info Register"]
    #[inline(always)]
    pub const fn addinfo(&self) -> &Addinfo {
        &self.addinfo
    }
    #[doc = "0x10 - OTG Interrupt Status Register"]
    #[inline(always)]
    pub const fn otgistat(&self) -> &Otgistat {
        &self.otgistat
    }
    #[doc = "0x14 - OTG Interrupt Control Register"]
    #[inline(always)]
    pub const fn otgicr(&self) -> &Otgicr {
        &self.otgicr
    }
    #[doc = "0x18 - OTG Status Register"]
    #[inline(always)]
    pub const fn otgstat(&self) -> &Otgstat {
        &self.otgstat
    }
    #[doc = "0x1c - OTG Control Register"]
    #[inline(always)]
    pub const fn otgctl(&self) -> &Otgctl {
        &self.otgctl
    }
    #[doc = "0x80 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn istat(&self) -> &Istat {
        &self.istat
    }
    #[doc = "0x84 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x88 - Error Interrupt Status Register"]
    #[inline(always)]
    pub const fn errstat(&self) -> &Errstat {
        &self.errstat
    }
    #[doc = "0x8c - Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn erren(&self) -> &Erren {
        &self.erren
    }
    #[doc = "0x90 - Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x94 - Control Register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x98 - Address Register"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x9c - BDT Page Register 1"]
    #[inline(always)]
    pub const fn bdtpage1(&self) -> &Bdtpage1 {
        &self.bdtpage1
    }
    #[doc = "0xa0 - Frame Number Register Low"]
    #[inline(always)]
    pub const fn frmnuml(&self) -> &Frmnuml {
        &self.frmnuml
    }
    #[doc = "0xa4 - Frame Number Register High"]
    #[inline(always)]
    pub const fn frmnumh(&self) -> &Frmnumh {
        &self.frmnumh
    }
    #[doc = "0xa8 - Token Register"]
    #[inline(always)]
    pub const fn token(&self) -> &Token {
        &self.token
    }
    #[doc = "0xac - SOF Threshold Register"]
    #[inline(always)]
    pub const fn softhld(&self) -> &Softhld {
        &self.softhld
    }
    #[doc = "0xb0 - BDT Page Register 2"]
    #[inline(always)]
    pub const fn bdtpage2(&self) -> &Bdtpage2 {
        &self.bdtpage2
    }
    #[doc = "0xb4 - BDT Page Register 3"]
    #[inline(always)]
    pub const fn bdtpage3(&self) -> &Bdtpage3 {
        &self.bdtpage3
    }
    #[doc = "0xc0..0xd0 - Endpoint Control Register"]
    #[inline(always)]
    pub const fn endpt(&self, n: usize) -> &Endpt {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(192)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0xd0 - Endpoint Control Register"]
    #[inline(always)]
    pub fn endpt_iter(&self) -> impl Iterator<Item = &Endpt> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(192)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x100 - USB Control Register"]
    #[inline(always)]
    pub const fn usbctrl(&self) -> &Usbctrl {
        &self.usbctrl
    }
    #[doc = "0x104 - USB OTG Observe Register"]
    #[inline(always)]
    pub const fn observe(&self) -> &Observe {
        &self.observe
    }
    #[doc = "0x108 - USB OTG Control Register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x10c - USB Transceiver Control Register 0"]
    #[inline(always)]
    pub const fn usbtrc0(&self) -> &Usbtrc0 {
        &self.usbtrc0
    }
    #[doc = "0x114 - Frame Adjust Register"]
    #[inline(always)]
    pub const fn usbfrmadjust(&self) -> &Usbfrmadjust {
        &self.usbfrmadjust
    }
}
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`]
module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
#[doc = "IDCOMP (r) register accessor: Peripheral ID Complement Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idcomp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcomp`]
module"]
#[doc(alias = "IDCOMP")]
pub type Idcomp = crate::Reg<idcomp::IdcompSpec>;
#[doc = "Peripheral ID Complement Register"]
pub mod idcomp;
#[doc = "REV (r) register accessor: Peripheral Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rev`]
module"]
#[doc(alias = "REV")]
pub type Rev = crate::Reg<rev::RevSpec>;
#[doc = "Peripheral Revision Register"]
pub mod rev;
#[doc = "ADDINFO (r) register accessor: Peripheral Additional Info Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addinfo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addinfo`]
module"]
#[doc(alias = "ADDINFO")]
pub type Addinfo = crate::Reg<addinfo::AddinfoSpec>;
#[doc = "Peripheral Additional Info Register"]
pub mod addinfo;
#[doc = "OTGISTAT (rw) register accessor: OTG Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otgistat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otgistat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otgistat`]
module"]
#[doc(alias = "OTGISTAT")]
pub type Otgistat = crate::Reg<otgistat::OtgistatSpec>;
#[doc = "OTG Interrupt Status Register"]
pub mod otgistat;
#[doc = "OTGICR (rw) register accessor: OTG Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otgicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otgicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otgicr`]
module"]
#[doc(alias = "OTGICR")]
pub type Otgicr = crate::Reg<otgicr::OtgicrSpec>;
#[doc = "OTG Interrupt Control Register"]
pub mod otgicr;
#[doc = "OTGSTAT (rw) register accessor: OTG Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otgstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otgstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otgstat`]
module"]
#[doc(alias = "OTGSTAT")]
pub type Otgstat = crate::Reg<otgstat::OtgstatSpec>;
#[doc = "OTG Status Register"]
pub mod otgstat;
#[doc = "OTGCTL (rw) register accessor: OTG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otgctl`]
module"]
#[doc(alias = "OTGCTL")]
pub type Otgctl = crate::Reg<otgctl::OtgctlSpec>;
#[doc = "OTG Control Register"]
pub mod otgctl;
#[doc = "ISTAT (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`istat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istat`]
module"]
#[doc(alias = "ISTAT")]
pub type Istat = crate::Reg<istat::IstatSpec>;
#[doc = "Interrupt Status Register"]
pub mod istat;
#[doc = "INTEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "ERRSTAT (rw) register accessor: Error Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errstat`]
module"]
#[doc(alias = "ERRSTAT")]
pub type Errstat = crate::Reg<errstat::ErrstatSpec>;
#[doc = "Error Interrupt Status Register"]
pub mod errstat;
#[doc = "ERREN (rw) register accessor: Error Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`erren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erren`]
module"]
#[doc(alias = "ERREN")]
pub type Erren = crate::Reg<erren::ErrenSpec>;
#[doc = "Error Interrupt Enable Register"]
pub mod erren;
#[doc = "STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "CTL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control Register"]
pub mod ctl;
#[doc = "ADDR (rw) register accessor: Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Address Register"]
pub mod addr;
#[doc = "BDTPAGE1 (rw) register accessor: BDT Page Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtpage1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtpage1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtpage1`]
module"]
#[doc(alias = "BDTPAGE1")]
pub type Bdtpage1 = crate::Reg<bdtpage1::Bdtpage1Spec>;
#[doc = "BDT Page Register 1"]
pub mod bdtpage1;
#[doc = "FRMNUML (rw) register accessor: Frame Number Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`frmnuml::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmnuml::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmnuml`]
module"]
#[doc(alias = "FRMNUML")]
pub type Frmnuml = crate::Reg<frmnuml::FrmnumlSpec>;
#[doc = "Frame Number Register Low"]
pub mod frmnuml;
#[doc = "FRMNUMH (rw) register accessor: Frame Number Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`frmnumh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmnumh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmnumh`]
module"]
#[doc(alias = "FRMNUMH")]
pub type Frmnumh = crate::Reg<frmnumh::FrmnumhSpec>;
#[doc = "Frame Number Register High"]
pub mod frmnumh;
#[doc = "TOKEN (rw) register accessor: Token Register\n\nYou can [`read`](crate::Reg::read) this register and get [`token::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`token::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@token`]
module"]
#[doc(alias = "TOKEN")]
pub type Token = crate::Reg<token::TokenSpec>;
#[doc = "Token Register"]
pub mod token;
#[doc = "SOFTHLD (rw) register accessor: SOF Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`softhld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softhld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softhld`]
module"]
#[doc(alias = "SOFTHLD")]
pub type Softhld = crate::Reg<softhld::SofthldSpec>;
#[doc = "SOF Threshold Register"]
pub mod softhld;
#[doc = "BDTPAGE2 (rw) register accessor: BDT Page Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtpage2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtpage2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtpage2`]
module"]
#[doc(alias = "BDTPAGE2")]
pub type Bdtpage2 = crate::Reg<bdtpage2::Bdtpage2Spec>;
#[doc = "BDT Page Register 2"]
pub mod bdtpage2;
#[doc = "BDTPAGE3 (rw) register accessor: BDT Page Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtpage3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtpage3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtpage3`]
module"]
#[doc(alias = "BDTPAGE3")]
pub type Bdtpage3 = crate::Reg<bdtpage3::Bdtpage3Spec>;
#[doc = "BDT Page Register 3"]
pub mod bdtpage3;
#[doc = "ENDPT (rw) register accessor: Endpoint Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`endpt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endpt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endpt`]
module"]
#[doc(alias = "ENDPT")]
pub type Endpt = crate::Reg<endpt::EndptSpec>;
#[doc = "Endpoint Control Register"]
pub mod endpt;
#[doc = "USBCTRL (rw) register accessor: USB Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbctrl`]
module"]
#[doc(alias = "USBCTRL")]
pub type Usbctrl = crate::Reg<usbctrl::UsbctrlSpec>;
#[doc = "USB Control Register"]
pub mod usbctrl;
#[doc = "OBSERVE (r) register accessor: USB OTG Observe Register\n\nYou can [`read`](crate::Reg::read) this register and get [`observe::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@observe`]
module"]
#[doc(alias = "OBSERVE")]
pub type Observe = crate::Reg<observe::ObserveSpec>;
#[doc = "USB OTG Observe Register"]
pub mod observe;
#[doc = "CONTROL (rw) register accessor: USB OTG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "USB OTG Control Register"]
pub mod control;
#[doc = "USBTRC0 (rw) register accessor: USB Transceiver Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`usbtrc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbtrc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbtrc0`]
module"]
#[doc(alias = "USBTRC0")]
pub type Usbtrc0 = crate::Reg<usbtrc0::Usbtrc0Spec>;
#[doc = "USB Transceiver Control Register 0"]
pub mod usbtrc0;
#[doc = "USBFRMADJUST (rw) register accessor: Frame Adjust Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbfrmadjust::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbfrmadjust::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbfrmadjust`]
module"]
#[doc(alias = "USBFRMADJUST")]
pub type Usbfrmadjust = crate::Reg<usbfrmadjust::UsbfrmadjustSpec>;
#[doc = "Frame Adjust Register"]
pub mod usbfrmadjust;
