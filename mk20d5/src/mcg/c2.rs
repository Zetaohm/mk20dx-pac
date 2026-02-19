#[doc = "Register `C2` reader"]
pub type R = crate::R<C2Spec>;
#[doc = "Register `C2` writer"]
pub type W = crate::W<C2Spec>;
#[doc = "Internal Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ircs {
    #[doc = "0: Slow internal reference clock selected."]
    _0 = 0,
    #[doc = "1: Fast internal reference clock selected."]
    _1 = 1,
}
impl From<Ircs> for bool {
    #[inline(always)]
    fn from(variant: Ircs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCS` reader - Internal Reference Clock Select"]
pub type IrcsR = crate::BitReader<Ircs>;
impl IrcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ircs {
        match self.bits {
            false => Ircs::_0,
            true => Ircs::_1,
        }
    }
    #[doc = "Slow internal reference clock selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ircs::_0
    }
    #[doc = "Fast internal reference clock selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ircs::_1
    }
}
#[doc = "Field `IRCS` writer - Internal Reference Clock Select"]
pub type IrcsW<'a, REG> = crate::BitWriter<'a, REG, Ircs>;
impl<'a, REG> IrcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slow internal reference clock selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ircs::_0)
    }
    #[doc = "Fast internal reference clock selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ircs::_1)
    }
}
#[doc = "Low Power Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lp {
    #[doc = "0: FLL (or PLL) is not disabled in bypass modes."]
    _0 = 0,
    #[doc = "1: FLL (or PLL) is disabled in bypass modes (lower power)"]
    _1 = 1,
}
impl From<Lp> for bool {
    #[inline(always)]
    fn from(variant: Lp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP` reader - Low Power Select"]
pub type LpR = crate::BitReader<Lp>;
impl LpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lp {
        match self.bits {
            false => Lp::_0,
            true => Lp::_1,
        }
    }
    #[doc = "FLL (or PLL) is not disabled in bypass modes."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lp::_0
    }
    #[doc = "FLL (or PLL) is disabled in bypass modes (lower power)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lp::_1
    }
}
#[doc = "Field `LP` writer - Low Power Select"]
pub type LpW<'a, REG> = crate::BitWriter<'a, REG, Lp>;
impl<'a, REG> LpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FLL (or PLL) is not disabled in bypass modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lp::_0)
    }
    #[doc = "FLL (or PLL) is disabled in bypass modes (lower power)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lp::_1)
    }
}
#[doc = "External Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erefs0 {
    #[doc = "0: External reference clock requested."]
    _0 = 0,
    #[doc = "1: Oscillator requested."]
    _1 = 1,
}
impl From<Erefs0> for bool {
    #[inline(always)]
    fn from(variant: Erefs0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EREFS0` reader - External Reference Select"]
pub type Erefs0R = crate::BitReader<Erefs0>;
impl Erefs0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erefs0 {
        match self.bits {
            false => Erefs0::_0,
            true => Erefs0::_1,
        }
    }
    #[doc = "External reference clock requested."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Erefs0::_0
    }
    #[doc = "Oscillator requested."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Erefs0::_1
    }
}
#[doc = "Field `EREFS0` writer - External Reference Select"]
pub type Erefs0W<'a, REG> = crate::BitWriter<'a, REG, Erefs0>;
impl<'a, REG> Erefs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External reference clock requested."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Erefs0::_0)
    }
    #[doc = "Oscillator requested."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Erefs0::_1)
    }
}
#[doc = "High Gain Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hgo0 {
    #[doc = "0: Configure crystal oscillator for low-power operation."]
    _0 = 0,
    #[doc = "1: Configure crystal oscillator for high-gain operation."]
    _1 = 1,
}
impl From<Hgo0> for bool {
    #[inline(always)]
    fn from(variant: Hgo0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HGO0` reader - High Gain Oscillator Select"]
pub type Hgo0R = crate::BitReader<Hgo0>;
impl Hgo0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hgo0 {
        match self.bits {
            false => Hgo0::_0,
            true => Hgo0::_1,
        }
    }
    #[doc = "Configure crystal oscillator for low-power operation."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hgo0::_0
    }
    #[doc = "Configure crystal oscillator for high-gain operation."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hgo0::_1
    }
}
#[doc = "Field `HGO0` writer - High Gain Oscillator Select"]
pub type Hgo0W<'a, REG> = crate::BitWriter<'a, REG, Hgo0>;
impl<'a, REG> Hgo0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configure crystal oscillator for low-power operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hgo0::_0)
    }
    #[doc = "Configure crystal oscillator for high-gain operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hgo0::_1)
    }
}
#[doc = "Frequency Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Range0 {
    #[doc = "0: Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    _00 = 0,
    #[doc = "1: Encoding 1 - High frequency range selected for the crystal oscillator ."]
    _01 = 1,
}
impl From<Range0> for u8 {
    #[inline(always)]
    fn from(variant: Range0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Range0 {
    type Ux = u8;
}
impl crate::IsEnum for Range0 {}
#[doc = "Field `RANGE0` reader - Frequency Range Select"]
pub type Range0R = crate::FieldReader<Range0>;
impl Range0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Range0> {
        match self.bits {
            0 => Some(Range0::_00),
            1 => Some(Range0::_01),
            _ => None,
        }
    }
    #[doc = "Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Range0::_00
    }
    #[doc = "Encoding 1 - High frequency range selected for the crystal oscillator ."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Range0::_01
    }
}
#[doc = "Field `RANGE0` writer - Frequency Range Select"]
pub type Range0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Range0>;
impl<'a, REG> Range0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Range0::_00)
    }
    #[doc = "Encoding 1 - High frequency range selected for the crystal oscillator ."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Range0::_01)
    }
}
#[doc = "Loss of Clock Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locre0 {
    #[doc = "0: Interrupt request is generated on a loss of OSC0 external reference clock."]
    _0 = 0,
    #[doc = "1: Generate a reset request on a loss of OSC0 external reference clock"]
    _1 = 1,
}
impl From<Locre0> for bool {
    #[inline(always)]
    fn from(variant: Locre0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCRE0` reader - Loss of Clock Reset Enable"]
pub type Locre0R = crate::BitReader<Locre0>;
impl Locre0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locre0 {
        match self.bits {
            false => Locre0::_0,
            true => Locre0::_1,
        }
    }
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Locre0::_0
    }
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Locre0::_1
    }
}
#[doc = "Field `LOCRE0` writer - Loss of Clock Reset Enable"]
pub type Locre0W<'a, REG> = crate::BitWriter<'a, REG, Locre0>;
impl<'a, REG> Locre0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Locre0::_0)
    }
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Locre0::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline(always)]
    pub fn ircs(&self) -> IrcsR {
        IrcsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline(always)]
    pub fn lp(&self) -> LpR {
        LpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    pub fn erefs0(&self) -> Erefs0R {
        Erefs0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo0(&self) -> Hgo0R {
        Hgo0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline(always)]
    pub fn range0(&self) -> Range0R {
        Range0R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre0(&self) -> Locre0R {
        Locre0R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn ircs(&mut self) -> IrcsW<C2Spec> {
        IrcsW::new(self, 0)
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline(always)]
    #[must_use]
    pub fn lp(&mut self) -> LpW<C2Spec> {
        LpW::new(self, 1)
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    #[must_use]
    pub fn erefs0(&mut self) -> Erefs0W<C2Spec> {
        Erefs0W::new(self, 2)
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    #[must_use]
    pub fn hgo0(&mut self) -> Hgo0W<C2Spec> {
        Hgo0W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline(always)]
    #[must_use]
    pub fn range0(&mut self) -> Range0W<C2Spec> {
        Range0W::new(self, 4)
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn locre0(&mut self) -> Locre0W<C2Spec> {
        Locre0W::new(self, 7)
    }
}
#[doc = "MCG Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2Spec;
impl crate::RegisterSpec for C2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c2::R`](R) reader structure"]
impl crate::Readable for C2Spec {}
#[doc = "`write(|w| ..)` method takes [`c2::W`](W) writer structure"]
impl crate::Writable for C2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C2 to value 0x80"]
impl crate::Resettable for C2Spec {
    const RESET_VALUE: u8 = 0x80;
}
