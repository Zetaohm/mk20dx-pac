#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Channel Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: The channel is not explicitly started"]
    _0 = 0,
    #[doc = "1: The channel is explicitly started via a software initiated service request"]
    _1 = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Channel Start"]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::_0,
            true => Start::_1,
        }
    }
    #[doc = "The channel is not explicitly started"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Start::_0
    }
    #[doc = "The channel is explicitly started via a software initiated service request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Start::_1
    }
}
#[doc = "Field `START` writer - Channel Start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel is not explicitly started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::_0)
    }
    #[doc = "The channel is explicitly started via a software initiated service request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::_1)
    }
}
#[doc = "Enable an interrupt when major iteration count completes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intmajor {
    #[doc = "0: The end-of-major loop interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The end-of-major loop interrupt is enabled"]
    _1 = 1,
}
impl From<Intmajor> for bool {
    #[inline(always)]
    fn from(variant: Intmajor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTMAJOR` reader - Enable an interrupt when major iteration count completes"]
pub type IntmajorR = crate::BitReader<Intmajor>;
impl IntmajorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intmajor {
        match self.bits {
            false => Intmajor::_0,
            true => Intmajor::_1,
        }
    }
    #[doc = "The end-of-major loop interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Intmajor::_0
    }
    #[doc = "The end-of-major loop interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Intmajor::_1
    }
}
#[doc = "Field `INTMAJOR` writer - Enable an interrupt when major iteration count completes"]
pub type IntmajorW<'a, REG> = crate::BitWriter<'a, REG, Intmajor>;
impl<'a, REG> IntmajorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The end-of-major loop interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Intmajor::_0)
    }
    #[doc = "The end-of-major loop interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Intmajor::_1)
    }
}
#[doc = "Enable an interrupt when major counter is half complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inthalf {
    #[doc = "0: The half-point interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The half-point interrupt is enabled"]
    _1 = 1,
}
impl From<Inthalf> for bool {
    #[inline(always)]
    fn from(variant: Inthalf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTHALF` reader - Enable an interrupt when major counter is half complete."]
pub type InthalfR = crate::BitReader<Inthalf>;
impl InthalfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inthalf {
        match self.bits {
            false => Inthalf::_0,
            true => Inthalf::_1,
        }
    }
    #[doc = "The half-point interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Inthalf::_0
    }
    #[doc = "The half-point interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Inthalf::_1
    }
}
#[doc = "Field `INTHALF` writer - Enable an interrupt when major counter is half complete."]
pub type InthalfW<'a, REG> = crate::BitWriter<'a, REG, Inthalf>;
impl<'a, REG> InthalfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The half-point interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Inthalf::_0)
    }
    #[doc = "The half-point interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Inthalf::_1)
    }
}
#[doc = "Disable Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dreq {
    #[doc = "0: The channel's ERQ bit is not affected"]
    _0 = 0,
    #[doc = "1: The channel's ERQ bit is cleared when the major loop is complete"]
    _1 = 1,
}
impl From<Dreq> for bool {
    #[inline(always)]
    fn from(variant: Dreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DREQ` reader - Disable Request"]
pub type DreqR = crate::BitReader<Dreq>;
impl DreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dreq {
        match self.bits {
            false => Dreq::_0,
            true => Dreq::_1,
        }
    }
    #[doc = "The channel's ERQ bit is not affected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dreq::_0
    }
    #[doc = "The channel's ERQ bit is cleared when the major loop is complete"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dreq::_1
    }
}
#[doc = "Field `DREQ` writer - Disable Request"]
pub type DreqW<'a, REG> = crate::BitWriter<'a, REG, Dreq>;
impl<'a, REG> DreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel's ERQ bit is not affected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dreq::_0)
    }
    #[doc = "The channel's ERQ bit is cleared when the major loop is complete"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dreq::_1)
    }
}
#[doc = "Enable Scatter/Gather Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esg {
    #[doc = "0: The current channel's TCD is normal format."]
    _0 = 0,
    #[doc = "1: The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    _1 = 1,
}
impl From<Esg> for bool {
    #[inline(always)]
    fn from(variant: Esg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESG` reader - Enable Scatter/Gather Processing"]
pub type EsgR = crate::BitReader<Esg>;
impl EsgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Esg {
        match self.bits {
            false => Esg::_0,
            true => Esg::_1,
        }
    }
    #[doc = "The current channel's TCD is normal format."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Esg::_0
    }
    #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Esg::_1
    }
}
#[doc = "Field `ESG` writer - Enable Scatter/Gather Processing"]
pub type EsgW<'a, REG> = crate::BitWriter<'a, REG, Esg>;
impl<'a, REG> EsgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The current channel's TCD is normal format."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Esg::_0)
    }
    #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Esg::_1)
    }
}
#[doc = "Enable channel-to-channel linking on major loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Majorelink {
    #[doc = "0: The channel-to-channel linking is disabled"]
    _0 = 0,
    #[doc = "1: The channel-to-channel linking is enabled"]
    _1 = 1,
}
impl From<Majorelink> for bool {
    #[inline(always)]
    fn from(variant: Majorelink) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAJORELINK` reader - Enable channel-to-channel linking on major loop complete"]
pub type MajorelinkR = crate::BitReader<Majorelink>;
impl MajorelinkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Majorelink {
        match self.bits {
            false => Majorelink::_0,
            true => Majorelink::_1,
        }
    }
    #[doc = "The channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Majorelink::_0
    }
    #[doc = "The channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Majorelink::_1
    }
}
#[doc = "Field `MAJORELINK` writer - Enable channel-to-channel linking on major loop complete"]
pub type MajorelinkW<'a, REG> = crate::BitWriter<'a, REG, Majorelink>;
impl<'a, REG> MajorelinkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Majorelink::_0)
    }
    #[doc = "The channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Majorelink::_1)
    }
}
#[doc = "Field `ACTIVE` reader - Channel Active"]
pub type ActiveR = crate::BitReader;
#[doc = "Field `ACTIVE` writer - Channel Active"]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - Channel Done"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - Channel Done"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAJORLINKCH` reader - Link Channel Number"]
pub type MajorlinkchR = crate::FieldReader;
#[doc = "Field `MAJORLINKCH` writer - Link Channel Number"]
pub type MajorlinkchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Bandwidth Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bwc {
    #[doc = "0: No eDMA engine stalls"]
    _00 = 0,
    #[doc = "2: eDMA engine stalls for 4 cycles after each r/w"]
    _10 = 2,
    #[doc = "3: eDMA engine stalls for 8 cycles after each r/w"]
    _11 = 3,
}
impl From<Bwc> for u8 {
    #[inline(always)]
    fn from(variant: Bwc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bwc {
    type Ux = u8;
}
impl crate::IsEnum for Bwc {}
#[doc = "Field `BWC` reader - Bandwidth Control"]
pub type BwcR = crate::FieldReader<Bwc>;
impl BwcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bwc> {
        match self.bits {
            0 => Some(Bwc::_00),
            2 => Some(Bwc::_10),
            3 => Some(Bwc::_11),
            _ => None,
        }
    }
    #[doc = "No eDMA engine stalls"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Bwc::_00
    }
    #[doc = "eDMA engine stalls for 4 cycles after each r/w"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Bwc::_10
    }
    #[doc = "eDMA engine stalls for 8 cycles after each r/w"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Bwc::_11
    }
}
#[doc = "Field `BWC` writer - Bandwidth Control"]
pub type BwcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bwc>;
impl<'a, REG> BwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No eDMA engine stalls"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Bwc::_00)
    }
    #[doc = "eDMA engine stalls for 4 cycles after each r/w"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Bwc::_10)
    }
    #[doc = "eDMA engine stalls for 8 cycles after each r/w"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Bwc::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Start"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes"]
    #[inline(always)]
    pub fn intmajor(&self) -> IntmajorR {
        IntmajorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub fn inthalf(&self) -> InthalfR {
        InthalfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline(always)]
    pub fn dreq(&self) -> DreqR {
        DreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub fn esg(&self) -> EsgR {
        EsgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub fn majorelink(&self) -> MajorelinkR {
        MajorelinkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Link Channel Number"]
    #[inline(always)]
    pub fn majorlinkch(&self) -> MajorlinkchR {
        MajorlinkchR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline(always)]
    pub fn bwc(&self) -> BwcR {
        BwcR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CsrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes"]
    #[inline(always)]
    #[must_use]
    pub fn intmajor(&mut self) -> IntmajorW<CsrSpec> {
        IntmajorW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    #[must_use]
    pub fn inthalf(&mut self) -> InthalfW<CsrSpec> {
        InthalfW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline(always)]
    #[must_use]
    pub fn dreq(&mut self) -> DreqW<CsrSpec> {
        DreqW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline(always)]
    #[must_use]
    pub fn esg(&mut self) -> EsgW<CsrSpec> {
        EsgW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    #[must_use]
    pub fn majorelink(&mut self) -> MajorelinkW<CsrSpec> {
        MajorelinkW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ActiveW<CsrSpec> {
        ActiveW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<CsrSpec> {
        DoneW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Link Channel Number"]
    #[inline(always)]
    #[must_use]
    pub fn majorlinkch(&mut self) -> MajorlinkchW<CsrSpec> {
        MajorlinkchW::new(self, 8)
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline(always)]
    #[must_use]
    pub fn bwc(&mut self) -> BwcW<CsrSpec> {
        BwcW::new(self, 14)
    }
}
#[doc = "TCD Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u16 = 0;
}
