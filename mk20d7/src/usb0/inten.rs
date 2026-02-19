#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "USBRST Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrsten {
    #[doc = "0: The USBRST interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The USBRST interrupt is enabled."]
    _1 = 1,
}
impl From<Usbrsten> for bool {
    #[inline(always)]
    fn from(variant: Usbrsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRSTEN` reader - USBRST Interrupt Enable"]
pub type UsbrstenR = crate::BitReader<Usbrsten>;
impl UsbrstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbrsten {
        match self.bits {
            false => Usbrsten::_0,
            true => Usbrsten::_1,
        }
    }
    #[doc = "The USBRST interrupt is not enabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbrsten::_0
    }
    #[doc = "The USBRST interrupt is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbrsten::_1
    }
}
#[doc = "Field `USBRSTEN` writer - USBRST Interrupt Enable"]
pub type UsbrstenW<'a, REG> = crate::BitWriter<'a, REG, Usbrsten>;
impl<'a, REG> UsbrstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The USBRST interrupt is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrsten::_0)
    }
    #[doc = "The USBRST interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrsten::_1)
    }
}
#[doc = "ERROR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erroren {
    #[doc = "0: The ERROR interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The ERROR interrupt is enabled."]
    _1 = 1,
}
impl From<Erroren> for bool {
    #[inline(always)]
    fn from(variant: Erroren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROREN` reader - ERROR Interrupt Enable"]
pub type ErrorenR = crate::BitReader<Erroren>;
impl ErrorenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erroren {
        match self.bits {
            false => Erroren::_0,
            true => Erroren::_1,
        }
    }
    #[doc = "The ERROR interrupt is not enabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Erroren::_0
    }
    #[doc = "The ERROR interrupt is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Erroren::_1
    }
}
#[doc = "Field `ERROREN` writer - ERROR Interrupt Enable"]
pub type ErrorenW<'a, REG> = crate::BitWriter<'a, REG, Erroren>;
impl<'a, REG> ErrorenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The ERROR interrupt is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Erroren::_0)
    }
    #[doc = "The ERROR interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Erroren::_1)
    }
}
#[doc = "SOFTOK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Softoken {
    #[doc = "0: The SOFTOK interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The SOFTOK interrupt is enabled."]
    _1 = 1,
}
impl From<Softoken> for bool {
    #[inline(always)]
    fn from(variant: Softoken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTOKEN` reader - SOFTOK Interrupt Enable"]
pub type SoftokenR = crate::BitReader<Softoken>;
impl SoftokenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Softoken {
        match self.bits {
            false => Softoken::_0,
            true => Softoken::_1,
        }
    }
    #[doc = "The SOFTOK interrupt is not enabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Softoken::_0
    }
    #[doc = "The SOFTOK interrupt is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Softoken::_1
    }
}
#[doc = "Field `SOFTOKEN` writer - SOFTOK Interrupt Enable"]
pub type SoftokenW<'a, REG> = crate::BitWriter<'a, REG, Softoken>;
impl<'a, REG> SoftokenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SOFTOK interrupt is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Softoken::_0)
    }
    #[doc = "The SOFTOK interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Softoken::_1)
    }
}
#[doc = "TOKDNE Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tokdneen {
    #[doc = "0: The TOKDNE interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The TOKDNE interrupt is enabled."]
    _1 = 1,
}
impl From<Tokdneen> for bool {
    #[inline(always)]
    fn from(variant: Tokdneen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOKDNEEN` reader - TOKDNE Interrupt Enable"]
pub type TokdneenR = crate::BitReader<Tokdneen>;
impl TokdneenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tokdneen {
        match self.bits {
            false => Tokdneen::_0,
            true => Tokdneen::_1,
        }
    }
    #[doc = "The TOKDNE interrupt is not enabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tokdneen::_0
    }
    #[doc = "The TOKDNE interrupt is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tokdneen::_1
    }
}
#[doc = "Field `TOKDNEEN` writer - TOKDNE Interrupt Enable"]
pub type TokdneenW<'a, REG> = crate::BitWriter<'a, REG, Tokdneen>;
impl<'a, REG> TokdneenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TOKDNE interrupt is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tokdneen::_0)
    }
    #[doc = "The TOKDNE interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tokdneen::_1)
    }
}
#[doc = "SLEEP Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleepen {
    #[doc = "0: The SLEEP interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The SLEEP interrupt is enabled."]
    _1 = 1,
}
impl From<Sleepen> for bool {
    #[inline(always)]
    fn from(variant: Sleepen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPEN` reader - SLEEP Interrupt Enable"]
pub type SleepenR = crate::BitReader<Sleepen>;
impl SleepenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleepen {
        match self.bits {
            false => Sleepen::_0,
            true => Sleepen::_1,
        }
    }
    #[doc = "The SLEEP interrupt is not enabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sleepen::_0
    }
    #[doc = "The SLEEP interrupt is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sleepen::_1
    }
}
#[doc = "Field `SLEEPEN` writer - SLEEP Interrupt Enable"]
pub type SleepenW<'a, REG> = crate::BitWriter<'a, REG, Sleepen>;
impl<'a, REG> SleepenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SLEEP interrupt is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepen::_0)
    }
    #[doc = "The SLEEP interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepen::_1)
    }
}
#[doc = "RESUME Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resumeen {
    #[doc = "0: The RESUME interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The RESUME interrupt is enabled."]
    _1 = 1,
}
impl From<Resumeen> for bool {
    #[inline(always)]
    fn from(variant: Resumeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESUMEEN` reader - RESUME Interrupt Enable"]
pub type ResumeenR = crate::BitReader<Resumeen>;
impl ResumeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resumeen {
        match self.bits {
            false => Resumeen::_0,
            true => Resumeen::_1,
        }
    }
    #[doc = "The RESUME interrupt is not enabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Resumeen::_0
    }
    #[doc = "The RESUME interrupt is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Resumeen::_1
    }
}
#[doc = "Field `RESUMEEN` writer - RESUME Interrupt Enable"]
pub type ResumeenW<'a, REG> = crate::BitWriter<'a, REG, Resumeen>;
impl<'a, REG> ResumeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RESUME interrupt is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Resumeen::_0)
    }
    #[doc = "The RESUME interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Resumeen::_1)
    }
}
#[doc = "ATTACH Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attachen {
    #[doc = "0: The ATTACH interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The ATTACH interrupt is enabled."]
    _1 = 1,
}
impl From<Attachen> for bool {
    #[inline(always)]
    fn from(variant: Attachen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATTACHEN` reader - ATTACH Interrupt Enable"]
pub type AttachenR = crate::BitReader<Attachen>;
impl AttachenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Attachen {
        match self.bits {
            false => Attachen::_0,
            true => Attachen::_1,
        }
    }
    #[doc = "The ATTACH interrupt is not enabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Attachen::_0
    }
    #[doc = "The ATTACH interrupt is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Attachen::_1
    }
}
#[doc = "Field `ATTACHEN` writer - ATTACH Interrupt Enable"]
pub type AttachenW<'a, REG> = crate::BitWriter<'a, REG, Attachen>;
impl<'a, REG> AttachenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The ATTACH interrupt is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Attachen::_0)
    }
    #[doc = "The ATTACH interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Attachen::_1)
    }
}
#[doc = "STALL Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stallen {
    #[doc = "0: The STALL interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The STALL interrupt is enabled."]
    _1 = 1,
}
impl From<Stallen> for bool {
    #[inline(always)]
    fn from(variant: Stallen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALLEN` reader - STALL Interrupt Enable"]
pub type StallenR = crate::BitReader<Stallen>;
impl StallenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stallen {
        match self.bits {
            false => Stallen::_0,
            true => Stallen::_1,
        }
    }
    #[doc = "The STALL interrupt is not enabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stallen::_0
    }
    #[doc = "The STALL interrupt is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stallen::_1
    }
}
#[doc = "Field `STALLEN` writer - STALL Interrupt Enable"]
pub type StallenW<'a, REG> = crate::BitWriter<'a, REG, Stallen>;
impl<'a, REG> StallenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The STALL interrupt is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stallen::_0)
    }
    #[doc = "The STALL interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Stallen::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USBRST Interrupt Enable"]
    #[inline(always)]
    pub fn usbrsten(&self) -> UsbrstenR {
        UsbrstenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ERROR Interrupt Enable"]
    #[inline(always)]
    pub fn erroren(&self) -> ErrorenR {
        ErrorenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SOFTOK Interrupt Enable"]
    #[inline(always)]
    pub fn softoken(&self) -> SoftokenR {
        SoftokenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TOKDNE Interrupt Enable"]
    #[inline(always)]
    pub fn tokdneen(&self) -> TokdneenR {
        TokdneenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SLEEP Interrupt Enable"]
    #[inline(always)]
    pub fn sleepen(&self) -> SleepenR {
        SleepenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RESUME Interrupt Enable"]
    #[inline(always)]
    pub fn resumeen(&self) -> ResumeenR {
        ResumeenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ATTACH Interrupt Enable"]
    #[inline(always)]
    pub fn attachen(&self) -> AttachenR {
        AttachenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - STALL Interrupt Enable"]
    #[inline(always)]
    pub fn stallen(&self) -> StallenR {
        StallenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBRST Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbrsten(&mut self) -> UsbrstenW<IntenSpec> {
        UsbrstenW::new(self, 0)
    }
    #[doc = "Bit 1 - ERROR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erroren(&mut self) -> ErrorenW<IntenSpec> {
        ErrorenW::new(self, 1)
    }
    #[doc = "Bit 2 - SOFTOK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn softoken(&mut self) -> SoftokenW<IntenSpec> {
        SoftokenW::new(self, 2)
    }
    #[doc = "Bit 3 - TOKDNE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tokdneen(&mut self) -> TokdneenW<IntenSpec> {
        TokdneenW::new(self, 3)
    }
    #[doc = "Bit 4 - SLEEP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sleepen(&mut self) -> SleepenW<IntenSpec> {
        SleepenW::new(self, 4)
    }
    #[doc = "Bit 5 - RESUME Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resumeen(&mut self) -> ResumeenW<IntenSpec> {
        ResumeenW::new(self, 5)
    }
    #[doc = "Bit 6 - ATTACH Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn attachen(&mut self) -> AttachenW<IntenSpec> {
        AttachenW::new(self, 6)
    }
    #[doc = "Bit 7 - STALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stallen(&mut self) -> StallenW<IntenSpec> {
        StallenW::new(self, 7)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u8 = 0;
}
