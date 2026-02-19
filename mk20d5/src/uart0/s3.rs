#[doc = "Register `S3` reader"]
pub type R = crate::R<S3Spec>;
#[doc = "Register `S3` writer"]
pub type W = crate::W<S3Spec>;
#[doc = "Transmission Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txff {
    #[doc = "0: Transmission continues normally."]
    _0 = 0,
    #[doc = "1: Transmission is failed."]
    _1 = 1,
}
impl From<Txff> for bool {
    #[inline(always)]
    fn from(variant: Txff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFF` reader - Transmission Fail Flag"]
pub type TxffR = crate::BitReader<Txff>;
impl TxffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txff {
        match self.bits {
            false => Txff::_0,
            true => Txff::_1,
        }
    }
    #[doc = "Transmission continues normally."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txff::_0
    }
    #[doc = "Transmission is failed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txff::_1
    }
}
#[doc = "Field `TXFF` writer - Transmission Fail Flag"]
pub type TxffW<'a, REG> = crate::BitWriter<'a, REG, Txff>;
impl<'a, REG> TxffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission continues normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txff::_0)
    }
    #[doc = "Transmission is failed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txff::_1)
    }
}
#[doc = "Preamble Start Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psf {
    #[doc = "0: Preamble start is not detected."]
    _0 = 0,
    #[doc = "1: Preamble start is detected."]
    _1 = 1,
}
impl From<Psf> for bool {
    #[inline(always)]
    fn from(variant: Psf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSF` reader - Preamble Start Flag"]
pub type PsfR = crate::BitReader<Psf>;
impl PsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psf {
        match self.bits {
            false => Psf::_0,
            true => Psf::_1,
        }
    }
    #[doc = "Preamble start is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psf::_0
    }
    #[doc = "Preamble start is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psf::_1
    }
}
#[doc = "Field `PSF` writer - Preamble Start Flag"]
pub type PsfW<'a, REG> = crate::BitWriter<'a, REG, Psf>;
impl<'a, REG> PsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preamble start is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psf::_0)
    }
    #[doc = "Preamble start is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psf::_1)
    }
}
#[doc = "Packet Cycle Timer Expired Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pctef {
    #[doc = "0: Packet Cycle Time is not expired."]
    _0 = 0,
    #[doc = "1: Packet cycle time is expired."]
    _1 = 1,
}
impl From<Pctef> for bool {
    #[inline(always)]
    fn from(variant: Pctef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCTEF` reader - Packet Cycle Timer Expired Flag"]
pub type PctefR = crate::BitReader<Pctef>;
impl PctefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pctef {
        match self.bits {
            false => Pctef::_0,
            true => Pctef::_1,
        }
    }
    #[doc = "Packet Cycle Time is not expired."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pctef::_0
    }
    #[doc = "Packet cycle time is expired."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pctef::_1
    }
}
#[doc = "Field `PCTEF` writer - Packet Cycle Timer Expired Flag"]
pub type PctefW<'a, REG> = crate::BitWriter<'a, REG, Pctef>;
impl<'a, REG> PctefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Packet Cycle Time is not expired."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pctef::_0)
    }
    #[doc = "Packet cycle time is expired."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pctef::_1)
    }
}
#[doc = "Packet Transmitted Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ptxf {
    #[doc = "0: Packet transmission is not complete."]
    _0 = 0,
    #[doc = "1: Packet transmission is complete."]
    _1 = 1,
}
impl From<Ptxf> for bool {
    #[inline(always)]
    fn from(variant: Ptxf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTXF` reader - Packet Transmitted Flag"]
pub type PtxfR = crate::BitReader<Ptxf>;
impl PtxfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ptxf {
        match self.bits {
            false => Ptxf::_0,
            true => Ptxf::_1,
        }
    }
    #[doc = "Packet transmission is not complete."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ptxf::_0
    }
    #[doc = "Packet transmission is complete."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ptxf::_1
    }
}
#[doc = "Field `PTXF` writer - Packet Transmitted Flag"]
pub type PtxfW<'a, REG> = crate::BitWriter<'a, REG, Ptxf>;
impl<'a, REG> PtxfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Packet transmission is not complete."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ptxf::_0)
    }
    #[doc = "Packet transmission is complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ptxf::_1)
    }
}
#[doc = "Packet Received Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prxf {
    #[doc = "0: Packet is not received."]
    _0 = 0,
    #[doc = "1: Packet is received."]
    _1 = 1,
}
impl From<Prxf> for bool {
    #[inline(always)]
    fn from(variant: Prxf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRXF` reader - Packet Received Flag"]
pub type PrxfR = crate::BitReader<Prxf>;
impl PrxfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prxf {
        match self.bits {
            false => Prxf::_0,
            true => Prxf::_1,
        }
    }
    #[doc = "Packet is not received."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prxf::_0
    }
    #[doc = "Packet is received."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prxf::_1
    }
}
#[doc = "Field `PRXF` writer - Packet Received Flag"]
pub type PrxfW<'a, REG> = crate::BitWriter<'a, REG, Prxf>;
impl<'a, REG> PrxfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Packet is not received."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Prxf::_0)
    }
    #[doc = "Packet is received."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Prxf::_1)
    }
}
#[doc = "Initial Sync Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isd {
    #[doc = "0: Initial sync is not detected."]
    _0 = 0,
    #[doc = "1: Initial sync is detected."]
    _1 = 1,
}
impl From<Isd> for bool {
    #[inline(always)]
    fn from(variant: Isd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISD` reader - Initial Sync Detect"]
pub type IsdR = crate::BitReader<Isd>;
impl IsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isd {
        match self.bits {
            false => Isd::_0,
            true => Isd::_1,
        }
    }
    #[doc = "Initial sync is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Isd::_0
    }
    #[doc = "Initial sync is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Isd::_1
    }
}
#[doc = "Wbase Expired Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wbef {
    #[doc = "0: Wbase time period is not expired."]
    _0 = 0,
    #[doc = "1: Wbase time period has been expired after beta1 time slots."]
    _1 = 1,
}
impl From<Wbef> for bool {
    #[inline(always)]
    fn from(variant: Wbef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WBEF` reader - Wbase Expired Flag"]
pub type WbefR = crate::BitReader<Wbef>;
impl WbefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wbef {
        match self.bits {
            false => Wbef::_0,
            true => Wbef::_1,
        }
    }
    #[doc = "Wbase time period is not expired."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wbef::_0
    }
    #[doc = "Wbase time period has been expired after beta1 time slots."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wbef::_1
    }
}
#[doc = "Field `WBEF` writer - Wbase Expired Flag"]
pub type WbefW<'a, REG> = crate::BitWriter<'a, REG, Wbef>;
impl<'a, REG> WbefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wbase time period is not expired."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wbef::_0)
    }
    #[doc = "Wbase time period has been expired after beta1 time slots."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wbef::_1)
    }
}
#[doc = "Preamble Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pef {
    #[doc = "0: Preamble is correct."]
    _0 = 0,
    #[doc = "1: Preamble is in error."]
    _1 = 1,
}
impl From<Pef> for bool {
    #[inline(always)]
    fn from(variant: Pef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEF` reader - Preamble Error Flag"]
pub type PefR = crate::BitReader<Pef>;
impl PefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pef {
        match self.bits {
            false => Pef::_0,
            true => Pef::_1,
        }
    }
    #[doc = "Preamble is correct."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pef::_0
    }
    #[doc = "Preamble is in error."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pef::_1
    }
}
#[doc = "Field `PEF` writer - Preamble Error Flag"]
pub type PefW<'a, REG> = crate::BitWriter<'a, REG, Pef>;
impl<'a, REG> PefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preamble is correct."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pef::_0)
    }
    #[doc = "Preamble is in error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pef::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Transmission Fail Flag"]
    #[inline(always)]
    pub fn txff(&self) -> TxffR {
        TxffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Preamble Start Flag"]
    #[inline(always)]
    pub fn psf(&self) -> PsfR {
        PsfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Packet Cycle Timer Expired Flag"]
    #[inline(always)]
    pub fn pctef(&self) -> PctefR {
        PctefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Packet Transmitted Flag"]
    #[inline(always)]
    pub fn ptxf(&self) -> PtxfR {
        PtxfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Packet Received Flag"]
    #[inline(always)]
    pub fn prxf(&self) -> PrxfR {
        PrxfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Initial Sync Detect"]
    #[inline(always)]
    pub fn isd(&self) -> IsdR {
        IsdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wbase Expired Flag"]
    #[inline(always)]
    pub fn wbef(&self) -> WbefR {
        WbefR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Preamble Error Flag"]
    #[inline(always)]
    pub fn pef(&self) -> PefR {
        PefR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission Fail Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txff(&mut self) -> TxffW<S3Spec> {
        TxffW::new(self, 0)
    }
    #[doc = "Bit 1 - Preamble Start Flag"]
    #[inline(always)]
    #[must_use]
    pub fn psf(&mut self) -> PsfW<S3Spec> {
        PsfW::new(self, 1)
    }
    #[doc = "Bit 2 - Packet Cycle Timer Expired Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pctef(&mut self) -> PctefW<S3Spec> {
        PctefW::new(self, 2)
    }
    #[doc = "Bit 3 - Packet Transmitted Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ptxf(&mut self) -> PtxfW<S3Spec> {
        PtxfW::new(self, 3)
    }
    #[doc = "Bit 4 - Packet Received Flag"]
    #[inline(always)]
    #[must_use]
    pub fn prxf(&mut self) -> PrxfW<S3Spec> {
        PrxfW::new(self, 4)
    }
    #[doc = "Bit 6 - Wbase Expired Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wbef(&mut self) -> WbefW<S3Spec> {
        WbefW::new(self, 6)
    }
    #[doc = "Bit 7 - Preamble Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pef(&mut self) -> PefW<S3Spec> {
        PefW::new(self, 7)
    }
}
#[doc = "UART CEA709.1-B Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S3Spec;
impl crate::RegisterSpec for S3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s3::R`](R) reader structure"]
impl crate::Readable for S3Spec {}
#[doc = "`write(|w| ..)` method takes [`s3::W`](W) writer structure"]
impl crate::Writable for S3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets S3 to value 0"]
impl crate::Resettable for S3Spec {
    const RESET_VALUE: u8 = 0;
}
