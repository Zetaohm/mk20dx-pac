#[doc = "Register `PE4` reader"]
pub type R = crate::R<Pe4Spec>;
#[doc = "Register `PE4` writer"]
pub type W = crate::W<Pe4Spec>;
#[doc = "Wakeup Pin Enable for LLWU_P12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe12 {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<Wupe12> for u8 {
    #[inline(always)]
    fn from(variant: Wupe12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe12 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe12 {}
#[doc = "Field `WUPE12` reader - Wakeup Pin Enable for LLWU_P12"]
pub type Wupe12R = crate::FieldReader<Wupe12>;
impl Wupe12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe12 {
        match self.bits {
            0 => Wupe12::_00,
            1 => Wupe12::_01,
            2 => Wupe12::_10,
            3 => Wupe12::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Wupe12::_00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Wupe12::_01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Wupe12::_10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Wupe12::_11
    }
}
#[doc = "Field `WUPE12` writer - Wakeup Pin Enable for LLWU_P12"]
pub type Wupe12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe12, crate::Safe>;
impl<'a, REG> Wupe12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe12::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe12::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe12::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe12::_11)
    }
}
#[doc = "Wakeup Pin Enable for LLWU_P13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe13 {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<Wupe13> for u8 {
    #[inline(always)]
    fn from(variant: Wupe13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe13 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe13 {}
#[doc = "Field `WUPE13` reader - Wakeup Pin Enable for LLWU_P13"]
pub type Wupe13R = crate::FieldReader<Wupe13>;
impl Wupe13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe13 {
        match self.bits {
            0 => Wupe13::_00,
            1 => Wupe13::_01,
            2 => Wupe13::_10,
            3 => Wupe13::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Wupe13::_00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Wupe13::_01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Wupe13::_10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Wupe13::_11
    }
}
#[doc = "Field `WUPE13` writer - Wakeup Pin Enable for LLWU_P13"]
pub type Wupe13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe13, crate::Safe>;
impl<'a, REG> Wupe13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe13::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe13::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe13::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe13::_11)
    }
}
#[doc = "Wakeup Pin Enable for LLWU_P14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe14 {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<Wupe14> for u8 {
    #[inline(always)]
    fn from(variant: Wupe14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe14 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe14 {}
#[doc = "Field `WUPE14` reader - Wakeup Pin Enable for LLWU_P14"]
pub type Wupe14R = crate::FieldReader<Wupe14>;
impl Wupe14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe14 {
        match self.bits {
            0 => Wupe14::_00,
            1 => Wupe14::_01,
            2 => Wupe14::_10,
            3 => Wupe14::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Wupe14::_00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Wupe14::_01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Wupe14::_10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Wupe14::_11
    }
}
#[doc = "Field `WUPE14` writer - Wakeup Pin Enable for LLWU_P14"]
pub type Wupe14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe14, crate::Safe>;
impl<'a, REG> Wupe14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe14::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe14::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe14::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe14::_11)
    }
}
#[doc = "Wakeup Pin Enable for LLWU_P15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe15 {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<Wupe15> for u8 {
    #[inline(always)]
    fn from(variant: Wupe15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe15 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe15 {}
#[doc = "Field `WUPE15` reader - Wakeup Pin Enable for LLWU_P15"]
pub type Wupe15R = crate::FieldReader<Wupe15>;
impl Wupe15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe15 {
        match self.bits {
            0 => Wupe15::_00,
            1 => Wupe15::_01,
            2 => Wupe15::_10,
            3 => Wupe15::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Wupe15::_00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Wupe15::_01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Wupe15::_10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Wupe15::_11
    }
}
#[doc = "Field `WUPE15` writer - Wakeup Pin Enable for LLWU_P15"]
pub type Wupe15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe15, crate::Safe>;
impl<'a, REG> Wupe15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe15::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe15::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe15::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe15::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable for LLWU_P12"]
    #[inline(always)]
    pub fn wupe12(&self) -> Wupe12R {
        Wupe12R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable for LLWU_P13"]
    #[inline(always)]
    pub fn wupe13(&self) -> Wupe13R {
        Wupe13R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable for LLWU_P14"]
    #[inline(always)]
    pub fn wupe14(&self) -> Wupe14R {
        Wupe14R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable for LLWU_P15"]
    #[inline(always)]
    pub fn wupe15(&self) -> Wupe15R {
        Wupe15R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable for LLWU_P12"]
    #[inline(always)]
    #[must_use]
    pub fn wupe12(&mut self) -> Wupe12W<Pe4Spec> {
        Wupe12W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable for LLWU_P13"]
    #[inline(always)]
    #[must_use]
    pub fn wupe13(&mut self) -> Wupe13W<Pe4Spec> {
        Wupe13W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable for LLWU_P14"]
    #[inline(always)]
    #[must_use]
    pub fn wupe14(&mut self) -> Wupe14W<Pe4Spec> {
        Wupe14W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable for LLWU_P15"]
    #[inline(always)]
    #[must_use]
    pub fn wupe15(&mut self) -> Wupe15W<Pe4Spec> {
        Wupe15W::new(self, 6)
    }
}
#[doc = "LLWU Pin Enable 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pe4Spec;
impl crate::RegisterSpec for Pe4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pe4::R`](R) reader structure"]
impl crate::Readable for Pe4Spec {}
#[doc = "`write(|w| ..)` method takes [`pe4::W`](W) writer structure"]
impl crate::Writable for Pe4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PE4 to value 0"]
impl crate::Resettable for Pe4Spec {
    const RESET_VALUE: u8 = 0;
}
