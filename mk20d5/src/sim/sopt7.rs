#[doc = "Register `SOPT7` reader"]
pub type R = crate::R<Sopt7Spec>;
#[doc = "Register `SOPT7` writer"]
pub type W = crate::W<Sopt7Spec>;
#[doc = "ADC0 trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc0trgsel {
    #[doc = "0: PDB external trigger pin input (PDB0_EXTRG)"]
    _0000 = 0,
    #[doc = "1: High speed comparator 0 output"]
    _0001 = 1,
    #[doc = "2: High speed comparator 1 output"]
    _0010 = 2,
    #[doc = "4: PIT trigger 0"]
    _0100 = 4,
    #[doc = "5: PIT trigger 1"]
    _0101 = 5,
    #[doc = "6: PIT trigger 2"]
    _0110 = 6,
    #[doc = "7: PIT trigger 3"]
    _0111 = 7,
    #[doc = "8: FTM0 trigger"]
    _1000 = 8,
    #[doc = "9: FTM1 trigger"]
    _1001 = 9,
    #[doc = "10: Unused"]
    _1010 = 10,
    #[doc = "11: Unused"]
    _1011 = 11,
    #[doc = "12: RTC alarm"]
    _1100 = 12,
    #[doc = "13: RTC seconds"]
    _1101 = 13,
    #[doc = "14: Low-power timer trigger"]
    _1110 = 14,
    #[doc = "15: Unused"]
    _1111 = 15,
}
impl From<Adc0trgsel> for u8 {
    #[inline(always)]
    fn from(variant: Adc0trgsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc0trgsel {
    type Ux = u8;
}
impl crate::IsEnum for Adc0trgsel {}
#[doc = "Field `ADC0TRGSEL` reader - ADC0 trigger select"]
pub type Adc0trgselR = crate::FieldReader<Adc0trgsel>;
impl Adc0trgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adc0trgsel> {
        match self.bits {
            0 => Some(Adc0trgsel::_0000),
            1 => Some(Adc0trgsel::_0001),
            2 => Some(Adc0trgsel::_0010),
            4 => Some(Adc0trgsel::_0100),
            5 => Some(Adc0trgsel::_0101),
            6 => Some(Adc0trgsel::_0110),
            7 => Some(Adc0trgsel::_0111),
            8 => Some(Adc0trgsel::_1000),
            9 => Some(Adc0trgsel::_1001),
            10 => Some(Adc0trgsel::_1010),
            11 => Some(Adc0trgsel::_1011),
            12 => Some(Adc0trgsel::_1100),
            13 => Some(Adc0trgsel::_1101),
            14 => Some(Adc0trgsel::_1110),
            15 => Some(Adc0trgsel::_1111),
            _ => None,
        }
    }
    #[doc = "PDB external trigger pin input (PDB0_EXTRG)"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Adc0trgsel::_0000
    }
    #[doc = "High speed comparator 0 output"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Adc0trgsel::_0001
    }
    #[doc = "High speed comparator 1 output"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Adc0trgsel::_0010
    }
    #[doc = "PIT trigger 0"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Adc0trgsel::_0100
    }
    #[doc = "PIT trigger 1"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Adc0trgsel::_0101
    }
    #[doc = "PIT trigger 2"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Adc0trgsel::_0110
    }
    #[doc = "PIT trigger 3"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == Adc0trgsel::_0111
    }
    #[doc = "FTM0 trigger"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Adc0trgsel::_1000
    }
    #[doc = "FTM1 trigger"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Adc0trgsel::_1001
    }
    #[doc = "Unused"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == Adc0trgsel::_1010
    }
    #[doc = "Unused"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == Adc0trgsel::_1011
    }
    #[doc = "RTC alarm"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == Adc0trgsel::_1100
    }
    #[doc = "RTC seconds"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == Adc0trgsel::_1101
    }
    #[doc = "Low-power timer trigger"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == Adc0trgsel::_1110
    }
    #[doc = "Unused"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == Adc0trgsel::_1111
    }
}
#[doc = "Field `ADC0TRGSEL` writer - ADC0 trigger select"]
pub type Adc0trgselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Adc0trgsel>;
impl<'a, REG> Adc0trgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PDB external trigger pin input (PDB0_EXTRG)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_0000)
    }
    #[doc = "High speed comparator 0 output"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_0001)
    }
    #[doc = "High speed comparator 1 output"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_0010)
    }
    #[doc = "PIT trigger 0"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_0100)
    }
    #[doc = "PIT trigger 1"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_0101)
    }
    #[doc = "PIT trigger 2"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_0110)
    }
    #[doc = "PIT trigger 3"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_0111)
    }
    #[doc = "FTM0 trigger"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_1000)
    }
    #[doc = "FTM1 trigger"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_1001)
    }
    #[doc = "Unused"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_1010)
    }
    #[doc = "Unused"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_1011)
    }
    #[doc = "RTC alarm"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_1100)
    }
    #[doc = "RTC seconds"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_1101)
    }
    #[doc = "Low-power timer trigger"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_1110)
    }
    #[doc = "Unused"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::_1111)
    }
}
#[doc = "ADC0 pretrigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0pretrgsel {
    #[doc = "0: Pre-trigger A"]
    _0 = 0,
    #[doc = "1: Pre-trigger B"]
    _1 = 1,
}
impl From<Adc0pretrgsel> for bool {
    #[inline(always)]
    fn from(variant: Adc0pretrgsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0PRETRGSEL` reader - ADC0 pretrigger select"]
pub type Adc0pretrgselR = crate::BitReader<Adc0pretrgsel>;
impl Adc0pretrgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0pretrgsel {
        match self.bits {
            false => Adc0pretrgsel::_0,
            true => Adc0pretrgsel::_1,
        }
    }
    #[doc = "Pre-trigger A"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adc0pretrgsel::_0
    }
    #[doc = "Pre-trigger B"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adc0pretrgsel::_1
    }
}
#[doc = "Field `ADC0PRETRGSEL` writer - ADC0 pretrigger select"]
pub type Adc0pretrgselW<'a, REG> = crate::BitWriter<'a, REG, Adc0pretrgsel>;
impl<'a, REG> Adc0pretrgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pre-trigger A"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0pretrgsel::_0)
    }
    #[doc = "Pre-trigger B"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0pretrgsel::_1)
    }
}
#[doc = "ADC0 alternate trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0alttrgen {
    #[doc = "0: PDB trigger selected for ADC0."]
    _0 = 0,
    #[doc = "1: Alternate trigger selected for ADC0."]
    _1 = 1,
}
impl From<Adc0alttrgen> for bool {
    #[inline(always)]
    fn from(variant: Adc0alttrgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0ALTTRGEN` reader - ADC0 alternate trigger enable"]
pub type Adc0alttrgenR = crate::BitReader<Adc0alttrgen>;
impl Adc0alttrgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0alttrgen {
        match self.bits {
            false => Adc0alttrgen::_0,
            true => Adc0alttrgen::_1,
        }
    }
    #[doc = "PDB trigger selected for ADC0."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adc0alttrgen::_0
    }
    #[doc = "Alternate trigger selected for ADC0."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adc0alttrgen::_1
    }
}
#[doc = "Field `ADC0ALTTRGEN` writer - ADC0 alternate trigger enable"]
pub type Adc0alttrgenW<'a, REG> = crate::BitWriter<'a, REG, Adc0alttrgen>;
impl<'a, REG> Adc0alttrgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PDB trigger selected for ADC0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0alttrgen::_0)
    }
    #[doc = "Alternate trigger selected for ADC0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0alttrgen::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC0 trigger select"]
    #[inline(always)]
    pub fn adc0trgsel(&self) -> Adc0trgselR {
        Adc0trgselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - ADC0 pretrigger select"]
    #[inline(always)]
    pub fn adc0pretrgsel(&self) -> Adc0pretrgselR {
        Adc0pretrgselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0 alternate trigger enable"]
    #[inline(always)]
    pub fn adc0alttrgen(&self) -> Adc0alttrgenR {
        Adc0alttrgenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC0 trigger select"]
    #[inline(always)]
    #[must_use]
    pub fn adc0trgsel(&mut self) -> Adc0trgselW<Sopt7Spec> {
        Adc0trgselW::new(self, 0)
    }
    #[doc = "Bit 4 - ADC0 pretrigger select"]
    #[inline(always)]
    #[must_use]
    pub fn adc0pretrgsel(&mut self) -> Adc0pretrgselW<Sopt7Spec> {
        Adc0pretrgselW::new(self, 4)
    }
    #[doc = "Bit 7 - ADC0 alternate trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0alttrgen(&mut self) -> Adc0alttrgenW<Sopt7Spec> {
        Adc0alttrgenW::new(self, 7)
    }
}
#[doc = "System Options Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sopt7Spec;
impl crate::RegisterSpec for Sopt7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sopt7::R`](R) reader structure"]
impl crate::Readable for Sopt7Spec {}
#[doc = "`write(|w| ..)` method takes [`sopt7::W`](W) writer structure"]
impl crate::Writable for Sopt7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOPT7 to value 0"]
impl crate::Resettable for Sopt7Spec {
    const RESET_VALUE: u32 = 0;
}
