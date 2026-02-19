#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    es: Es,
    _reserved2: [u8; 0x04],
    erq: Erq,
    _reserved3: [u8; 0x04],
    eei: Eei,
    ceei: Ceei,
    seei: Seei,
    cerq: Cerq,
    serq: Serq,
    cdne: Cdne,
    ssrt: Ssrt,
    cerr: Cerr,
    cint: Cint,
    _reserved12: [u8; 0x04],
    int: Int,
    _reserved13: [u8; 0x04],
    err: Err,
    _reserved14: [u8; 0x04],
    hrs: Hrs,
    _reserved15: [u8; 0xc8],
    dchpri: [Dchpri; 16],
    _reserved16: [u8; 0x0ef0],
    tcd_saddr: (),
    _reserved17: [u8; 0x04],
    tcd_soff: (),
    _reserved18: [u8; 0x02],
    tcd_attr: (),
    _reserved19: [u8; 0x02],
    dma_tcd_nbytes_mloffyes: (),
    dma_tcd_nbytes_mloffno: (),
    dma_tcd_nbytes_mlno: (),
    _reserved22: [u8; 0x04],
    tcd_slast: (),
    _reserved23: [u8; 0x04],
    tcd_daddr: (),
    _reserved24: [u8; 0x04],
    tcd_doff: (),
    _reserved25: [u8; 0x02],
    dma_tcd_citer_elinkyes: (),
    dma_tcd_citer_elinkno: (),
    _reserved27: [u8; 0x02],
    tcd_dlastsga: (),
    _reserved28: [u8; 0x04],
    tcd_csr: (),
    _reserved29: [u8; 0x02],
    dma_tcd_biter_elinkyes: (),
    dma_tcd_biter_elinkno: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Error Status Register"]
    #[inline(always)]
    pub const fn es(&self) -> &Es {
        &self.es
    }
    #[doc = "0x0c - Enable Request Register"]
    #[inline(always)]
    pub const fn erq(&self) -> &Erq {
        &self.erq
    }
    #[doc = "0x14 - Enable Error Interrupt Register"]
    #[inline(always)]
    pub const fn eei(&self) -> &Eei {
        &self.eei
    }
    #[doc = "0x18 - Clear Enable Error Interrupt Register"]
    #[inline(always)]
    pub const fn ceei(&self) -> &Ceei {
        &self.ceei
    }
    #[doc = "0x19 - Set Enable Error Interrupt Register"]
    #[inline(always)]
    pub const fn seei(&self) -> &Seei {
        &self.seei
    }
    #[doc = "0x1a - Clear Enable Request Register"]
    #[inline(always)]
    pub const fn cerq(&self) -> &Cerq {
        &self.cerq
    }
    #[doc = "0x1b - Set Enable Request Register"]
    #[inline(always)]
    pub const fn serq(&self) -> &Serq {
        &self.serq
    }
    #[doc = "0x1c - Clear DONE Status Bit Register"]
    #[inline(always)]
    pub const fn cdne(&self) -> &Cdne {
        &self.cdne
    }
    #[doc = "0x1d - Set START Bit Register"]
    #[inline(always)]
    pub const fn ssrt(&self) -> &Ssrt {
        &self.ssrt
    }
    #[doc = "0x1e - Clear Error Register"]
    #[inline(always)]
    pub const fn cerr(&self) -> &Cerr {
        &self.cerr
    }
    #[doc = "0x1f - Clear Interrupt Request Register"]
    #[inline(always)]
    pub const fn cint(&self) -> &Cint {
        &self.cint
    }
    #[doc = "0x24 - Interrupt Request Register"]
    #[inline(always)]
    pub const fn int(&self) -> &Int {
        &self.int
    }
    #[doc = "0x2c - Error Register"]
    #[inline(always)]
    pub const fn err(&self) -> &Err {
        &self.err
    }
    #[doc = "0x34 - Hardware Request Status Register"]
    #[inline(always)]
    pub const fn hrs(&self) -> &Hrs {
        &self.hrs
    }
    #[doc = "0x100..0x110 - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri(&self, n: usize) -> &Dchpri {
        &self.dchpri[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri_iter(&self) -> impl Iterator<Item = &Dchpri> {
        self.dchpri.iter()
    }
    #[doc = "0x100 - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri3(&self) -> &Dchpri {
        self.dchpri(0)
    }
    #[doc = "0x101 - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri2(&self) -> &Dchpri {
        self.dchpri(1)
    }
    #[doc = "0x102 - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri1(&self) -> &Dchpri {
        self.dchpri(2)
    }
    #[doc = "0x103 - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri0(&self) -> &Dchpri {
        self.dchpri(3)
    }
    #[doc = "0x104 - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri7(&self) -> &Dchpri {
        self.dchpri(4)
    }
    #[doc = "0x105 - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri6(&self) -> &Dchpri {
        self.dchpri(5)
    }
    #[doc = "0x106 - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri5(&self) -> &Dchpri {
        self.dchpri(6)
    }
    #[doc = "0x107 - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri4(&self) -> &Dchpri {
        self.dchpri(7)
    }
    #[doc = "0x108 - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri11(&self) -> &Dchpri {
        self.dchpri(8)
    }
    #[doc = "0x109 - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri10(&self) -> &Dchpri {
        self.dchpri(9)
    }
    #[doc = "0x10a - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri9(&self) -> &Dchpri {
        self.dchpri(10)
    }
    #[doc = "0x10b - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri8(&self) -> &Dchpri {
        self.dchpri(11)
    }
    #[doc = "0x10c - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri15(&self) -> &Dchpri {
        self.dchpri(12)
    }
    #[doc = "0x10d - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri14(&self) -> &Dchpri {
        self.dchpri(13)
    }
    #[doc = "0x10e - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri13(&self) -> &Dchpri {
        self.dchpri(14)
    }
    #[doc = "0x10f - Channel n Priority Register"]
    #[inline(always)]
    pub const fn dchpri12(&self) -> &Dchpri {
        self.dchpri(15)
    }
    #[doc = "0x1000..0x1040 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd_saddr(&self, n: usize) -> &TcdSaddr {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4096)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1040 - TCD Source Address"]
    #[inline(always)]
    pub fn tcd_saddr_iter(&self) -> impl Iterator<Item = &TcdSaddr> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4096)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x1000 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd0_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(0)
    }
    #[doc = "0x1020 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd1_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(1)
    }
    #[doc = "0x1040 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd2_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(2)
    }
    #[doc = "0x1060 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd3_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(3)
    }
    #[doc = "0x1080 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd4_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(4)
    }
    #[doc = "0x10a0 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd5_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(5)
    }
    #[doc = "0x10c0 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd6_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(6)
    }
    #[doc = "0x10e0 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd7_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(7)
    }
    #[doc = "0x1100 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd8_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(8)
    }
    #[doc = "0x1120 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd9_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(9)
    }
    #[doc = "0x1140 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd10_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(10)
    }
    #[doc = "0x1160 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd11_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(11)
    }
    #[doc = "0x1180 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd12_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(12)
    }
    #[doc = "0x11a0 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd13_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(13)
    }
    #[doc = "0x11c0 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd14_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(14)
    }
    #[doc = "0x11e0 - TCD Source Address"]
    #[inline(always)]
    pub const fn tcd15_saddr(&self) -> &TcdSaddr {
        self.tcd_saddr(15)
    }
    #[doc = "0x1004..0x1024 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd_soff(&self, n: usize) -> &TcdSoff {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4100)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1004..0x1024 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub fn tcd_soff_iter(&self) -> impl Iterator<Item = &TcdSoff> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4100)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd0_soff(&self) -> &TcdSoff {
        self.tcd_soff(0)
    }
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd1_soff(&self) -> &TcdSoff {
        self.tcd_soff(1)
    }
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd2_soff(&self) -> &TcdSoff {
        self.tcd_soff(2)
    }
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd3_soff(&self) -> &TcdSoff {
        self.tcd_soff(3)
    }
    #[doc = "0x1084 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd4_soff(&self) -> &TcdSoff {
        self.tcd_soff(4)
    }
    #[doc = "0x10a4 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd5_soff(&self) -> &TcdSoff {
        self.tcd_soff(5)
    }
    #[doc = "0x10c4 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd6_soff(&self) -> &TcdSoff {
        self.tcd_soff(6)
    }
    #[doc = "0x10e4 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd7_soff(&self) -> &TcdSoff {
        self.tcd_soff(7)
    }
    #[doc = "0x1104 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd8_soff(&self) -> &TcdSoff {
        self.tcd_soff(8)
    }
    #[doc = "0x1124 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd9_soff(&self) -> &TcdSoff {
        self.tcd_soff(9)
    }
    #[doc = "0x1144 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd10_soff(&self) -> &TcdSoff {
        self.tcd_soff(10)
    }
    #[doc = "0x1164 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd11_soff(&self) -> &TcdSoff {
        self.tcd_soff(11)
    }
    #[doc = "0x1184 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd12_soff(&self) -> &TcdSoff {
        self.tcd_soff(12)
    }
    #[doc = "0x11a4 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd13_soff(&self) -> &TcdSoff {
        self.tcd_soff(13)
    }
    #[doc = "0x11c4 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd14_soff(&self) -> &TcdSoff {
        self.tcd_soff(14)
    }
    #[doc = "0x11e4 - TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd15_soff(&self) -> &TcdSoff {
        self.tcd_soff(15)
    }
    #[doc = "0x1006..0x1026 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd_attr(&self, n: usize) -> &TcdAttr {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4102)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1006..0x1026 - TCD Transfer Attributes"]
    #[inline(always)]
    pub fn tcd_attr_iter(&self) -> impl Iterator<Item = &TcdAttr> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4102)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x1006 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd0_attr(&self) -> &TcdAttr {
        self.tcd_attr(0)
    }
    #[doc = "0x1026 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd1_attr(&self) -> &TcdAttr {
        self.tcd_attr(1)
    }
    #[doc = "0x1046 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd2_attr(&self) -> &TcdAttr {
        self.tcd_attr(2)
    }
    #[doc = "0x1066 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd3_attr(&self) -> &TcdAttr {
        self.tcd_attr(3)
    }
    #[doc = "0x1086 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd4_attr(&self) -> &TcdAttr {
        self.tcd_attr(4)
    }
    #[doc = "0x10a6 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd5_attr(&self) -> &TcdAttr {
        self.tcd_attr(5)
    }
    #[doc = "0x10c6 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd6_attr(&self) -> &TcdAttr {
        self.tcd_attr(6)
    }
    #[doc = "0x10e6 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd7_attr(&self) -> &TcdAttr {
        self.tcd_attr(7)
    }
    #[doc = "0x1106 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd8_attr(&self) -> &TcdAttr {
        self.tcd_attr(8)
    }
    #[doc = "0x1126 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd9_attr(&self) -> &TcdAttr {
        self.tcd_attr(9)
    }
    #[doc = "0x1146 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd10_attr(&self) -> &TcdAttr {
        self.tcd_attr(10)
    }
    #[doc = "0x1166 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd11_attr(&self) -> &TcdAttr {
        self.tcd_attr(11)
    }
    #[doc = "0x1186 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd12_attr(&self) -> &TcdAttr {
        self.tcd_attr(12)
    }
    #[doc = "0x11a6 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd13_attr(&self) -> &TcdAttr {
        self.tcd_attr(13)
    }
    #[doc = "0x11c6 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd14_attr(&self) -> &TcdAttr {
        self.tcd_attr(14)
    }
    #[doc = "0x11e6 - TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd15_attr(&self) -> &TcdAttr {
        self.tcd_attr(15)
    }
    #[doc = "0x1008..0x1048 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd_nbytes_mloffyes(&self, n: usize) -> &DmaTcdNbytesMloffyes {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4104)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1008..0x1048 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd_nbytes_mloffyes_iter(&self) -> impl Iterator<Item = &DmaTcdNbytesMloffyes> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4104)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(0)
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(1)
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(2)
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(3)
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(4)
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(5)
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(6)
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(7)
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(8)
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(9)
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(10)
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(11)
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(12)
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(13)
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(14)
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_nbytes_mloffyes(&self) -> &DmaTcdNbytesMloffyes {
        self.dma_tcd_nbytes_mloffyes(15)
    }
    #[doc = "0x1008..0x1048 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd_nbytes_mloffno(&self, n: usize) -> &DmaTcdNbytesMloffno {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4104)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1008..0x1048 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd_nbytes_mloffno_iter(&self) -> impl Iterator<Item = &DmaTcdNbytesMloffno> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4104)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(0)
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(1)
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(2)
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(3)
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(4)
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(5)
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(6)
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(7)
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(8)
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(9)
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(10)
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(11)
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(12)
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(13)
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(14)
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_nbytes_mloffno(&self) -> &DmaTcdNbytesMloffno {
        self.dma_tcd_nbytes_mloffno(15)
    }
    #[doc = "0x1008..0x1048 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd_nbytes_mlno(&self, n: usize) -> &DmaTcdNbytesMlno {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4104)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1008..0x1048 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub fn dma_tcd_nbytes_mlno_iter(&self) -> impl Iterator<Item = &DmaTcdNbytesMlno> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4104)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(0)
    }
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(1)
    }
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(2)
    }
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(3)
    }
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(4)
    }
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(5)
    }
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(6)
    }
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(7)
    }
    #[doc = "0x1108 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(8)
    }
    #[doc = "0x1128 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(9)
    }
    #[doc = "0x1148 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(10)
    }
    #[doc = "0x1168 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(11)
    }
    #[doc = "0x1188 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(12)
    }
    #[doc = "0x11a8 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(13)
    }
    #[doc = "0x11c8 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(14)
    }
    #[doc = "0x11e8 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_nbytes_mlno(&self) -> &DmaTcdNbytesMlno {
        self.dma_tcd_nbytes_mlno(15)
    }
    #[doc = "0x100c..0x104c - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd_slast(&self, n: usize) -> &TcdSlast {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4108)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100c..0x104c - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub fn tcd_slast_iter(&self) -> impl Iterator<Item = &TcdSlast> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4108)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd0_slast(&self) -> &TcdSlast {
        self.tcd_slast(0)
    }
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd1_slast(&self) -> &TcdSlast {
        self.tcd_slast(1)
    }
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd2_slast(&self) -> &TcdSlast {
        self.tcd_slast(2)
    }
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd3_slast(&self) -> &TcdSlast {
        self.tcd_slast(3)
    }
    #[doc = "0x108c - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd4_slast(&self) -> &TcdSlast {
        self.tcd_slast(4)
    }
    #[doc = "0x10ac - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd5_slast(&self) -> &TcdSlast {
        self.tcd_slast(5)
    }
    #[doc = "0x10cc - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd6_slast(&self) -> &TcdSlast {
        self.tcd_slast(6)
    }
    #[doc = "0x10ec - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd7_slast(&self) -> &TcdSlast {
        self.tcd_slast(7)
    }
    #[doc = "0x110c - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd8_slast(&self) -> &TcdSlast {
        self.tcd_slast(8)
    }
    #[doc = "0x112c - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd9_slast(&self) -> &TcdSlast {
        self.tcd_slast(9)
    }
    #[doc = "0x114c - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd10_slast(&self) -> &TcdSlast {
        self.tcd_slast(10)
    }
    #[doc = "0x116c - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd11_slast(&self) -> &TcdSlast {
        self.tcd_slast(11)
    }
    #[doc = "0x118c - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd12_slast(&self) -> &TcdSlast {
        self.tcd_slast(12)
    }
    #[doc = "0x11ac - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd13_slast(&self) -> &TcdSlast {
        self.tcd_slast(13)
    }
    #[doc = "0x11cc - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd14_slast(&self) -> &TcdSlast {
        self.tcd_slast(14)
    }
    #[doc = "0x11ec - TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd15_slast(&self) -> &TcdSlast {
        self.tcd_slast(15)
    }
    #[doc = "0x1010..0x1050 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd_daddr(&self, n: usize) -> &TcdDaddr {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4112)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1010..0x1050 - TCD Destination Address"]
    #[inline(always)]
    pub fn tcd_daddr_iter(&self) -> impl Iterator<Item = &TcdDaddr> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4112)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x1010 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd0_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(0)
    }
    #[doc = "0x1030 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd1_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(1)
    }
    #[doc = "0x1050 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd2_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(2)
    }
    #[doc = "0x1070 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd3_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(3)
    }
    #[doc = "0x1090 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd4_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(4)
    }
    #[doc = "0x10b0 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd5_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(5)
    }
    #[doc = "0x10d0 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd6_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(6)
    }
    #[doc = "0x10f0 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd7_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(7)
    }
    #[doc = "0x1110 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd8_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(8)
    }
    #[doc = "0x1130 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd9_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(9)
    }
    #[doc = "0x1150 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd10_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(10)
    }
    #[doc = "0x1170 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd11_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(11)
    }
    #[doc = "0x1190 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd12_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(12)
    }
    #[doc = "0x11b0 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd13_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(13)
    }
    #[doc = "0x11d0 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd14_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(14)
    }
    #[doc = "0x11f0 - TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd15_daddr(&self) -> &TcdDaddr {
        self.tcd_daddr(15)
    }
    #[doc = "0x1014..0x1034 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd_doff(&self, n: usize) -> &TcdDoff {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4116)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1014..0x1034 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub fn tcd_doff_iter(&self) -> impl Iterator<Item = &TcdDoff> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4116)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd0_doff(&self) -> &TcdDoff {
        self.tcd_doff(0)
    }
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd1_doff(&self) -> &TcdDoff {
        self.tcd_doff(1)
    }
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd2_doff(&self) -> &TcdDoff {
        self.tcd_doff(2)
    }
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd3_doff(&self) -> &TcdDoff {
        self.tcd_doff(3)
    }
    #[doc = "0x1094 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd4_doff(&self) -> &TcdDoff {
        self.tcd_doff(4)
    }
    #[doc = "0x10b4 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd5_doff(&self) -> &TcdDoff {
        self.tcd_doff(5)
    }
    #[doc = "0x10d4 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd6_doff(&self) -> &TcdDoff {
        self.tcd_doff(6)
    }
    #[doc = "0x10f4 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd7_doff(&self) -> &TcdDoff {
        self.tcd_doff(7)
    }
    #[doc = "0x1114 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd8_doff(&self) -> &TcdDoff {
        self.tcd_doff(8)
    }
    #[doc = "0x1134 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd9_doff(&self) -> &TcdDoff {
        self.tcd_doff(9)
    }
    #[doc = "0x1154 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd10_doff(&self) -> &TcdDoff {
        self.tcd_doff(10)
    }
    #[doc = "0x1174 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd11_doff(&self) -> &TcdDoff {
        self.tcd_doff(11)
    }
    #[doc = "0x1194 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd12_doff(&self) -> &TcdDoff {
        self.tcd_doff(12)
    }
    #[doc = "0x11b4 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd13_doff(&self) -> &TcdDoff {
        self.tcd_doff(13)
    }
    #[doc = "0x11d4 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd14_doff(&self) -> &TcdDoff {
        self.tcd_doff(14)
    }
    #[doc = "0x11f4 - TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd15_doff(&self) -> &TcdDoff {
        self.tcd_doff(15)
    }
    #[doc = "0x1016..0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd_citer_elinkyes(&self, n: usize) -> &DmaTcdCiterElinkyes {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4118)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1016..0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd_citer_elinkyes_iter(&self) -> impl Iterator<Item = &DmaTcdCiterElinkyes> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4118)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(0)
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(1)
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(2)
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(3)
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(4)
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(5)
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(6)
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(7)
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(8)
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(9)
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(10)
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(11)
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(12)
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(13)
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(14)
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_citer_elinkyes(&self) -> &DmaTcdCiterElinkyes {
        self.dma_tcd_citer_elinkyes(15)
    }
    #[doc = "0x1016..0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd_citer_elinkno(&self, n: usize) -> &DmaTcdCiterElinkno {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4118)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1016..0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd_citer_elinkno_iter(&self) -> impl Iterator<Item = &DmaTcdCiterElinkno> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4118)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(0)
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(1)
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(2)
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(3)
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(4)
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(5)
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(6)
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(7)
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(8)
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(9)
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(10)
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(11)
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(12)
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(13)
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(14)
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_citer_elinkno(&self) -> &DmaTcdCiterElinkno {
        self.dma_tcd_citer_elinkno(15)
    }
    #[doc = "0x1018..0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd_dlastsga(&self, n: usize) -> &TcdDlastsga {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4120)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1018..0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub fn tcd_dlastsga_iter(&self) -> impl Iterator<Item = &TcdDlastsga> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4120)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd0_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(0)
    }
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd1_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(1)
    }
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd2_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(2)
    }
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd3_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(3)
    }
    #[doc = "0x1098 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd4_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(4)
    }
    #[doc = "0x10b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd5_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(5)
    }
    #[doc = "0x10d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd6_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(6)
    }
    #[doc = "0x10f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd7_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(7)
    }
    #[doc = "0x1118 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd8_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(8)
    }
    #[doc = "0x1138 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd9_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(9)
    }
    #[doc = "0x1158 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd10_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(10)
    }
    #[doc = "0x1178 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd11_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(11)
    }
    #[doc = "0x1198 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd12_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(12)
    }
    #[doc = "0x11b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd13_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(13)
    }
    #[doc = "0x11d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd14_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(14)
    }
    #[doc = "0x11f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd15_dlastsga(&self) -> &TcdDlastsga {
        self.tcd_dlastsga(15)
    }
    #[doc = "0x101c..0x103c - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd_csr(&self, n: usize) -> &TcdCsr {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4124)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x101c..0x103c - TCD Control and Status"]
    #[inline(always)]
    pub fn tcd_csr_iter(&self) -> impl Iterator<Item = &TcdCsr> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4124)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x101c - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd0_csr(&self) -> &TcdCsr {
        self.tcd_csr(0)
    }
    #[doc = "0x103c - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd1_csr(&self) -> &TcdCsr {
        self.tcd_csr(1)
    }
    #[doc = "0x105c - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd2_csr(&self) -> &TcdCsr {
        self.tcd_csr(2)
    }
    #[doc = "0x107c - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd3_csr(&self) -> &TcdCsr {
        self.tcd_csr(3)
    }
    #[doc = "0x109c - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd4_csr(&self) -> &TcdCsr {
        self.tcd_csr(4)
    }
    #[doc = "0x10bc - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd5_csr(&self) -> &TcdCsr {
        self.tcd_csr(5)
    }
    #[doc = "0x10dc - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd6_csr(&self) -> &TcdCsr {
        self.tcd_csr(6)
    }
    #[doc = "0x10fc - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd7_csr(&self) -> &TcdCsr {
        self.tcd_csr(7)
    }
    #[doc = "0x111c - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd8_csr(&self) -> &TcdCsr {
        self.tcd_csr(8)
    }
    #[doc = "0x113c - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd9_csr(&self) -> &TcdCsr {
        self.tcd_csr(9)
    }
    #[doc = "0x115c - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd10_csr(&self) -> &TcdCsr {
        self.tcd_csr(10)
    }
    #[doc = "0x117c - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd11_csr(&self) -> &TcdCsr {
        self.tcd_csr(11)
    }
    #[doc = "0x119c - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd12_csr(&self) -> &TcdCsr {
        self.tcd_csr(12)
    }
    #[doc = "0x11bc - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd13_csr(&self) -> &TcdCsr {
        self.tcd_csr(13)
    }
    #[doc = "0x11dc - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd14_csr(&self) -> &TcdCsr {
        self.tcd_csr(14)
    }
    #[doc = "0x11fc - TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd15_csr(&self) -> &TcdCsr {
        self.tcd_csr(15)
    }
    #[doc = "0x101e..0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd_biter_elinkyes(&self, n: usize) -> &DmaTcdBiterElinkyes {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4126)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x101e..0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd_biter_elinkyes_iter(&self) -> impl Iterator<Item = &DmaTcdBiterElinkyes> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4126)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(0)
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(1)
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(2)
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(3)
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(4)
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(5)
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(6)
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(7)
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(8)
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(9)
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(10)
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(11)
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(12)
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(13)
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(14)
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_biter_elinkyes(&self) -> &DmaTcdBiterElinkyes {
        self.dma_tcd_biter_elinkyes(15)
    }
    #[doc = "0x101e..0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd_biter_elinkno(&self, n: usize) -> &DmaTcdBiterElinkno {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4126)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x101e..0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd_biter_elinkno_iter(&self) -> impl Iterator<Item = &DmaTcdBiterElinkno> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4126)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(0)
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(1)
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(2)
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(3)
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd4_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(4)
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd5_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(5)
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd6_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(6)
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd7_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(7)
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd8_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(8)
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd9_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(9)
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd10_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(10)
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd11_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(11)
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd12_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(12)
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd13_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(13)
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd14_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(14)
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd15_biter_elinkno(&self) -> &DmaTcdBiterElinkno {
        self.dma_tcd_biter_elinkno(15)
    }
}
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "ES (r) register accessor: Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`es::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@es`]
module"]
#[doc(alias = "ES")]
pub type Es = crate::Reg<es::EsSpec>;
#[doc = "Error Status Register"]
pub mod es;
#[doc = "ERQ (rw) register accessor: Enable Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`erq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erq`]
module"]
#[doc(alias = "ERQ")]
pub type Erq = crate::Reg<erq::ErqSpec>;
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "EEI (rw) register accessor: Enable Error Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eei::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eei::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eei`]
module"]
#[doc(alias = "EEI")]
pub type Eei = crate::Reg<eei::EeiSpec>;
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "CEEI (w) register accessor: Clear Enable Error Interrupt Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceei::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ceei`]
module"]
#[doc(alias = "CEEI")]
pub type Ceei = crate::Reg<ceei::CeeiSpec>;
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "SEEI (w) register accessor: Set Enable Error Interrupt Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seei::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seei`]
module"]
#[doc(alias = "SEEI")]
pub type Seei = crate::Reg<seei::SeeiSpec>;
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "CERQ (w) register accessor: Clear Enable Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cerq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cerq`]
module"]
#[doc(alias = "CERQ")]
pub type Cerq = crate::Reg<cerq::CerqSpec>;
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "SERQ (w) register accessor: Set Enable Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`serq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@serq`]
module"]
#[doc(alias = "SERQ")]
pub type Serq = crate::Reg<serq::SerqSpec>;
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "CDNE (w) register accessor: Clear DONE Status Bit Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdne::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdne`]
module"]
#[doc(alias = "CDNE")]
pub type Cdne = crate::Reg<cdne::CdneSpec>;
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "SSRT (w) register accessor: Set START Bit Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssrt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssrt`]
module"]
#[doc(alias = "SSRT")]
pub type Ssrt = crate::Reg<ssrt::SsrtSpec>;
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "CERR (w) register accessor: Clear Error Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cerr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cerr`]
module"]
#[doc(alias = "CERR")]
pub type Cerr = crate::Reg<cerr::CerrSpec>;
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "CINT (w) register accessor: Clear Interrupt Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cint::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cint`]
module"]
#[doc(alias = "CINT")]
pub type Cint = crate::Reg<cint::CintSpec>;
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "INT (rw) register accessor: Interrupt Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`]
module"]
#[doc(alias = "INT")]
pub type Int = crate::Reg<int::IntSpec>;
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "ERR (rw) register accessor: Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err`]
module"]
#[doc(alias = "ERR")]
pub type Err = crate::Reg<err::ErrSpec>;
#[doc = "Error Register"]
pub mod err;
#[doc = "HRS (rw) register accessor: Hardware Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrs`]
module"]
#[doc(alias = "HRS")]
pub type Hrs = crate::Reg<hrs::HrsSpec>;
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "DCHPRI (rw) register accessor: Channel n Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dchpri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dchpri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dchpri`]
module"]
#[doc(alias = "DCHPRI")]
pub type Dchpri = crate::Reg<dchpri::DchpriSpec>;
#[doc = "Channel n Priority Register"]
pub mod dchpri;
#[doc = "TCD_SADDR (rw) register accessor: TCD Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcd_saddr`]
module"]
#[doc(alias = "TCD_SADDR")]
pub type TcdSaddr = crate::Reg<tcd_saddr::TcdSaddrSpec>;
#[doc = "TCD Source Address"]
pub mod tcd_saddr;
#[doc = "TCD_SOFF (rw) register accessor: TCD Signed Source Address Offset\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_soff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_soff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcd_soff`]
module"]
#[doc(alias = "TCD_SOFF")]
pub type TcdSoff = crate::Reg<tcd_soff::TcdSoffSpec>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd_soff;
#[doc = "TCD_ATTR (rw) register accessor: TCD Transfer Attributes\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcd_attr`]
module"]
#[doc(alias = "TCD_ATTR")]
pub type TcdAttr = crate::Reg<tcd_attr::TcdAttrSpec>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd_attr;
#[doc = "DMA_TCD_NBYTES_MLNO (rw) register accessor: TCD Minor Byte Count (Minor Loop Disabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tcd_nbytes_mlno::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tcd_nbytes_mlno::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tcd_nbytes_mlno`]
module"]
#[doc(alias = "DMA_TCD_NBYTES_MLNO")]
pub type DmaTcdNbytesMlno = crate::Reg<dma_tcd_nbytes_mlno::DmaTcdNbytesMlnoSpec>;
#[doc = "TCD Minor Byte Count (Minor Loop Disabled)"]
pub mod dma_tcd_nbytes_mlno;
#[doc = "DMA_TCD_NBYTES_MLOFFNO (rw) register accessor: TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tcd_nbytes_mloffno::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tcd_nbytes_mloffno::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tcd_nbytes_mloffno`]
module"]
#[doc(alias = "DMA_TCD_NBYTES_MLOFFNO")]
pub type DmaTcdNbytesMloffno = crate::Reg<dma_tcd_nbytes_mloffno::DmaTcdNbytesMloffnoSpec>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
pub mod dma_tcd_nbytes_mloffno;
#[doc = "DMA_TCD_NBYTES_MLOFFYES (rw) register accessor: TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tcd_nbytes_mloffyes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tcd_nbytes_mloffyes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tcd_nbytes_mloffyes`]
module"]
#[doc(alias = "DMA_TCD_NBYTES_MLOFFYES")]
pub type DmaTcdNbytesMloffyes = crate::Reg<dma_tcd_nbytes_mloffyes::DmaTcdNbytesMloffyesSpec>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
pub mod dma_tcd_nbytes_mloffyes;
#[doc = "TCD_SLAST (rw) register accessor: TCD Last Source Address Adjustment\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_slast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_slast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcd_slast`]
module"]
#[doc(alias = "TCD_SLAST")]
pub type TcdSlast = crate::Reg<tcd_slast::TcdSlastSpec>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd_slast;
#[doc = "TCD_DADDR (rw) register accessor: TCD Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcd_daddr`]
module"]
#[doc(alias = "TCD_DADDR")]
pub type TcdDaddr = crate::Reg<tcd_daddr::TcdDaddrSpec>;
#[doc = "TCD Destination Address"]
pub mod tcd_daddr;
#[doc = "TCD_DOFF (rw) register accessor: TCD Signed Destination Address Offset\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_doff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_doff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcd_doff`]
module"]
#[doc(alias = "TCD_DOFF")]
pub type TcdDoff = crate::Reg<tcd_doff::TcdDoffSpec>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd_doff;
#[doc = "DMA_TCD_CITER_ELINKNO (rw) register accessor: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tcd_citer_elinkno::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tcd_citer_elinkno::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tcd_citer_elinkno`]
module"]
#[doc(alias = "DMA_TCD_CITER_ELINKNO")]
pub type DmaTcdCiterElinkno = crate::Reg<dma_tcd_citer_elinkno::DmaTcdCiterElinknoSpec>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd_citer_elinkno;
#[doc = "DMA_TCD_CITER_ELINKYES (rw) register accessor: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tcd_citer_elinkyes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tcd_citer_elinkyes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tcd_citer_elinkyes`]
module"]
#[doc(alias = "DMA_TCD_CITER_ELINKYES")]
pub type DmaTcdCiterElinkyes = crate::Reg<dma_tcd_citer_elinkyes::DmaTcdCiterElinkyesSpec>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd_citer_elinkyes;
#[doc = "TCD_DLASTSGA (rw) register accessor: TCD Last Destination Address Adjustment/Scatter Gather Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_dlastsga::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_dlastsga::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcd_dlastsga`]
module"]
#[doc(alias = "TCD_DLASTSGA")]
pub type TcdDlastsga = crate::Reg<tcd_dlastsga::TcdDlastsgaSpec>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd_dlastsga;
#[doc = "TCD_CSR (rw) register accessor: TCD Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcd_csr`]
module"]
#[doc(alias = "TCD_CSR")]
pub type TcdCsr = crate::Reg<tcd_csr::TcdCsrSpec>;
#[doc = "TCD Control and Status"]
pub mod tcd_csr;
#[doc = "DMA_TCD_BITER_ELINKNO (rw) register accessor: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tcd_biter_elinkno::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tcd_biter_elinkno::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tcd_biter_elinkno`]
module"]
#[doc(alias = "DMA_TCD_BITER_ELINKNO")]
pub type DmaTcdBiterElinkno = crate::Reg<dma_tcd_biter_elinkno::DmaTcdBiterElinknoSpec>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd_biter_elinkno;
#[doc = "DMA_TCD_BITER_ELINKYES (rw) register accessor: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tcd_biter_elinkyes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tcd_biter_elinkyes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tcd_biter_elinkyes`]
module"]
#[doc(alias = "DMA_TCD_BITER_ELINKYES")]
pub type DmaTcdBiterElinkyes = crate::Reg<dma_tcd_biter_elinkyes::DmaTcdBiterElinkyesSpec>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd_biter_elinkyes;
