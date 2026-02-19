#[doc = "Register `IS7816` reader"]
pub type R = crate::R<Is7816Spec>;
#[doc = "Register `IS7816` writer"]
pub type W = crate::W<Is7816Spec>;
#[doc = "Receive Threshold Exceeded Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxt {
    #[doc = "0: The number of consecutive NACKS generated as a result of parity errors and buffer overruns is less than or equal to the value in ET7816\\[RXTHRESHOLD\\]."]
    _0 = 0,
    #[doc = "1: The number of consecutive NACKS generated as a result of parity errors and buffer overruns is greater than the value in ET7816\\[RXTHRESHOLD\\]."]
    _1 = 1,
}
impl From<Rxt> for bool {
    #[inline(always)]
    fn from(variant: Rxt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXT` reader - Receive Threshold Exceeded Interrupt"]
pub type RxtR = crate::BitReader<Rxt>;
impl RxtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxt {
        match self.bits {
            false => Rxt::_0,
            true => Rxt::_1,
        }
    }
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is less than or equal to the value in ET7816\\[RXTHRESHOLD\\]."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxt::_0
    }
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is greater than the value in ET7816\\[RXTHRESHOLD\\]."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxt::_1
    }
}
#[doc = "Field `RXT` writer - Receive Threshold Exceeded Interrupt"]
pub type RxtW<'a, REG> = crate::BitWriter<'a, REG, Rxt>;
impl<'a, REG> RxtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is less than or equal to the value in ET7816\\[RXTHRESHOLD\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxt::_0)
    }
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is greater than the value in ET7816\\[RXTHRESHOLD\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxt::_1)
    }
}
#[doc = "Transmit Threshold Exceeded Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txt {
    #[doc = "0: The number of retries and corresponding NACKS does not exceed the value in the ET7816\\[TXTHRESHOLD\\]
field."]
    _0 = 0,
    #[doc = "1: The number of retries and corresponding NACKS exceeds the value in the ET7816\\[TXTHRESHOLD\\]
field."]
    _1 = 1,
}
impl From<Txt> for bool {
    #[inline(always)]
    fn from(variant: Txt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXT` reader - Transmit Threshold Exceeded Interrupt"]
pub type TxtR = crate::BitReader<Txt>;
impl TxtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txt {
        match self.bits {
            false => Txt::_0,
            true => Txt::_1,
        }
    }
    #[doc = "The number of retries and corresponding NACKS does not exceed the value in the ET7816\\[TXTHRESHOLD\\]
field."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txt::_0
    }
    #[doc = "The number of retries and corresponding NACKS exceeds the value in the ET7816\\[TXTHRESHOLD\\]
field."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txt::_1
    }
}
#[doc = "Field `TXT` writer - Transmit Threshold Exceeded Interrupt"]
pub type TxtW<'a, REG> = crate::BitWriter<'a, REG, Txt>;
impl<'a, REG> TxtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of retries and corresponding NACKS does not exceed the value in the ET7816\\[TXTHRESHOLD\\]
field."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txt::_0)
    }
    #[doc = "The number of retries and corresponding NACKS exceeds the value in the ET7816\\[TXTHRESHOLD\\]
field."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txt::_1)
    }
}
#[doc = "Guard Timer Violated Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gtv {
    #[doc = "0: A guard time (GT, CGT or BGT) has not been violated."]
    _0 = 0,
    #[doc = "1: A guard time (GT, CGT or BGT) has been violated."]
    _1 = 1,
}
impl From<Gtv> for bool {
    #[inline(always)]
    fn from(variant: Gtv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTV` reader - Guard Timer Violated Interrupt"]
pub type GtvR = crate::BitReader<Gtv>;
impl GtvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gtv {
        match self.bits {
            false => Gtv::_0,
            true => Gtv::_1,
        }
    }
    #[doc = "A guard time (GT, CGT or BGT) has not been violated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gtv::_0
    }
    #[doc = "A guard time (GT, CGT or BGT) has been violated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gtv::_1
    }
}
#[doc = "Field `GTV` writer - Guard Timer Violated Interrupt"]
pub type GtvW<'a, REG> = crate::BitWriter<'a, REG, Gtv>;
impl<'a, REG> GtvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A guard time (GT, CGT or BGT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gtv::_0)
    }
    #[doc = "A guard time (GT, CGT or BGT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gtv::_1)
    }
}
#[doc = "Initial Character Detected Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initd {
    #[doc = "0: A valid initial character has not been received."]
    _0 = 0,
    #[doc = "1: A valid initial character has been received."]
    _1 = 1,
}
impl From<Initd> for bool {
    #[inline(always)]
    fn from(variant: Initd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITD` reader - Initial Character Detected Interrupt"]
pub type InitdR = crate::BitReader<Initd>;
impl InitdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initd {
        match self.bits {
            false => Initd::_0,
            true => Initd::_1,
        }
    }
    #[doc = "A valid initial character has not been received."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Initd::_0
    }
    #[doc = "A valid initial character has been received."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Initd::_1
    }
}
#[doc = "Field `INITD` writer - Initial Character Detected Interrupt"]
pub type InitdW<'a, REG> = crate::BitWriter<'a, REG, Initd>;
impl<'a, REG> InitdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A valid initial character has not been received."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Initd::_0)
    }
    #[doc = "A valid initial character has been received."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Initd::_1)
    }
}
#[doc = "Block Wait Timer Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwt {
    #[doc = "0: Block wait time (BWT) has not been violated."]
    _0 = 0,
    #[doc = "1: Block wait tTime (BWT) has been violated."]
    _1 = 1,
}
impl From<Bwt> for bool {
    #[inline(always)]
    fn from(variant: Bwt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWT` reader - Block Wait Timer Interrupt"]
pub type BwtR = crate::BitReader<Bwt>;
impl BwtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwt {
        match self.bits {
            false => Bwt::_0,
            true => Bwt::_1,
        }
    }
    #[doc = "Block wait time (BWT) has not been violated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bwt::_0
    }
    #[doc = "Block wait tTime (BWT) has been violated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bwt::_1
    }
}
#[doc = "Field `BWT` writer - Block Wait Timer Interrupt"]
pub type BwtW<'a, REG> = crate::BitWriter<'a, REG, Bwt>;
impl<'a, REG> BwtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Block wait time (BWT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bwt::_0)
    }
    #[doc = "Block wait tTime (BWT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwt::_1)
    }
}
#[doc = "Character Wait Timer Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cwt {
    #[doc = "0: Character wait time (CWT) has not been violated."]
    _0 = 0,
    #[doc = "1: Character wait time (CWT) has been violated."]
    _1 = 1,
}
impl From<Cwt> for bool {
    #[inline(always)]
    fn from(variant: Cwt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWT` reader - Character Wait Timer Interrupt"]
pub type CwtR = crate::BitReader<Cwt>;
impl CwtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cwt {
        match self.bits {
            false => Cwt::_0,
            true => Cwt::_1,
        }
    }
    #[doc = "Character wait time (CWT) has not been violated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cwt::_0
    }
    #[doc = "Character wait time (CWT) has been violated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cwt::_1
    }
}
#[doc = "Field `CWT` writer - Character Wait Timer Interrupt"]
pub type CwtW<'a, REG> = crate::BitWriter<'a, REG, Cwt>;
impl<'a, REG> CwtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Character wait time (CWT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cwt::_0)
    }
    #[doc = "Character wait time (CWT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cwt::_1)
    }
}
#[doc = "Wait Timer Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wt {
    #[doc = "0: Wait time (WT) has not been violated."]
    _0 = 0,
    #[doc = "1: Wait time (WT) has been violated."]
    _1 = 1,
}
impl From<Wt> for bool {
    #[inline(always)]
    fn from(variant: Wt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WT` reader - Wait Timer Interrupt"]
pub type WtR = crate::BitReader<Wt>;
impl WtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wt {
        match self.bits {
            false => Wt::_0,
            true => Wt::_1,
        }
    }
    #[doc = "Wait time (WT) has not been violated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wt::_0
    }
    #[doc = "Wait time (WT) has been violated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wt::_1
    }
}
#[doc = "Field `WT` writer - Wait Timer Interrupt"]
pub type WtW<'a, REG> = crate::BitWriter<'a, REG, Wt>;
impl<'a, REG> WtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait time (WT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wt::_0)
    }
    #[doc = "Wait time (WT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wt::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt"]
    #[inline(always)]
    pub fn rxt(&self) -> RxtR {
        RxtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt"]
    #[inline(always)]
    pub fn txt(&self) -> TxtR {
        TxtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt"]
    #[inline(always)]
    pub fn gtv(&self) -> GtvR {
        GtvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt"]
    #[inline(always)]
    pub fn initd(&self) -> InitdR {
        InitdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt"]
    #[inline(always)]
    pub fn bwt(&self) -> BwtR {
        BwtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt"]
    #[inline(always)]
    pub fn cwt(&self) -> CwtR {
        CwtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wait Timer Interrupt"]
    #[inline(always)]
    pub fn wt(&self) -> WtR {
        WtR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxt(&mut self) -> RxtW<Is7816Spec> {
        RxtW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txt(&mut self) -> TxtW<Is7816Spec> {
        TxtW::new(self, 1)
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn gtv(&mut self) -> GtvW<Is7816Spec> {
        GtvW::new(self, 2)
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn initd(&mut self) -> InitdW<Is7816Spec> {
        InitdW::new(self, 4)
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bwt(&mut self) -> BwtW<Is7816Spec> {
        BwtW::new(self, 5)
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cwt(&mut self) -> CwtW<Is7816Spec> {
        CwtW::new(self, 6)
    }
    #[doc = "Bit 7 - Wait Timer Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wt(&mut self) -> WtW<Is7816Spec> {
        WtW::new(self, 7)
    }
}
#[doc = "UART 7816 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`is7816::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`is7816::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Is7816Spec;
impl crate::RegisterSpec for Is7816Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`is7816::R`](R) reader structure"]
impl crate::Readable for Is7816Spec {}
#[doc = "`write(|w| ..)` method takes [`is7816::W`](W) writer structure"]
impl crate::Writable for Is7816Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IS7816 to value 0"]
impl crate::Resettable for Is7816Spec {
    const RESET_VALUE: u8 = 0;
}
