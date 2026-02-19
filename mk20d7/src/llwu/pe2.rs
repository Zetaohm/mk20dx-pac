#[doc = "Register `PE2` reader"]
pub type R = crate::R<Pe2Spec>;
#[doc = "Register `PE2` writer"]
pub type W = crate::W<Pe2Spec>;
#[doc = "Wakeup Pin Enable for LLWU_P4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe4 {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<Wupe4> for u8 {
    #[inline(always)]
    fn from(variant: Wupe4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe4 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe4 {}
#[doc = "Field `WUPE4` reader - Wakeup Pin Enable for LLWU_P4"]
pub type Wupe4R = crate::FieldReader<Wupe4>;
impl Wupe4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe4 {
        match self.bits {
            0 => Wupe4::_00,
            1 => Wupe4::_01,
            2 => Wupe4::_10,
            3 => Wupe4::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Wupe4::_00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Wupe4::_01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Wupe4::_10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Wupe4::_11
    }
}
#[doc = "Field `WUPE4` writer - Wakeup Pin Enable for LLWU_P4"]
pub type Wupe4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe4, crate::Safe>;
impl<'a, REG> Wupe4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe4::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe4::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe4::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe4::_11)
    }
}
#[doc = "Wakeup Pin Enable for LLWU_P5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe5 {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<Wupe5> for u8 {
    #[inline(always)]
    fn from(variant: Wupe5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe5 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe5 {}
#[doc = "Field `WUPE5` reader - Wakeup Pin Enable for LLWU_P5"]
pub type Wupe5R = crate::FieldReader<Wupe5>;
impl Wupe5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe5 {
        match self.bits {
            0 => Wupe5::_00,
            1 => Wupe5::_01,
            2 => Wupe5::_10,
            3 => Wupe5::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Wupe5::_00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Wupe5::_01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Wupe5::_10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Wupe5::_11
    }
}
#[doc = "Field `WUPE5` writer - Wakeup Pin Enable for LLWU_P5"]
pub type Wupe5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe5, crate::Safe>;
impl<'a, REG> Wupe5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe5::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe5::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe5::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe5::_11)
    }
}
#[doc = "Wakeup Pin Enable for LLWU_P6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe6 {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<Wupe6> for u8 {
    #[inline(always)]
    fn from(variant: Wupe6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe6 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe6 {}
#[doc = "Field `WUPE6` reader - Wakeup Pin Enable for LLWU_P6"]
pub type Wupe6R = crate::FieldReader<Wupe6>;
impl Wupe6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe6 {
        match self.bits {
            0 => Wupe6::_00,
            1 => Wupe6::_01,
            2 => Wupe6::_10,
            3 => Wupe6::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Wupe6::_00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Wupe6::_01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Wupe6::_10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Wupe6::_11
    }
}
#[doc = "Field `WUPE6` writer - Wakeup Pin Enable for LLWU_P6"]
pub type Wupe6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe6, crate::Safe>;
impl<'a, REG> Wupe6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe6::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe6::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe6::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe6::_11)
    }
}
#[doc = "Wakeup Pin Enable for LLWU_P7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe7 {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<Wupe7> for u8 {
    #[inline(always)]
    fn from(variant: Wupe7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe7 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe7 {}
#[doc = "Field `WUPE7` reader - Wakeup Pin Enable for LLWU_P7"]
pub type Wupe7R = crate::FieldReader<Wupe7>;
impl Wupe7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe7 {
        match self.bits {
            0 => Wupe7::_00,
            1 => Wupe7::_01,
            2 => Wupe7::_10,
            3 => Wupe7::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Wupe7::_00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Wupe7::_01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Wupe7::_10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Wupe7::_11
    }
}
#[doc = "Field `WUPE7` writer - Wakeup Pin Enable for LLWU_P7"]
pub type Wupe7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe7, crate::Safe>;
impl<'a, REG> Wupe7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe7::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe7::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe7::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe7::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable for LLWU_P4"]
    #[inline(always)]
    pub fn wupe4(&self) -> Wupe4R {
        Wupe4R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable for LLWU_P5"]
    #[inline(always)]
    pub fn wupe5(&self) -> Wupe5R {
        Wupe5R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable for LLWU_P6"]
    #[inline(always)]
    pub fn wupe6(&self) -> Wupe6R {
        Wupe6R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable for LLWU_P7"]
    #[inline(always)]
    pub fn wupe7(&self) -> Wupe7R {
        Wupe7R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable for LLWU_P4"]
    #[inline(always)]
    #[must_use]
    pub fn wupe4(&mut self) -> Wupe4W<Pe2Spec> {
        Wupe4W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable for LLWU_P5"]
    #[inline(always)]
    #[must_use]
    pub fn wupe5(&mut self) -> Wupe5W<Pe2Spec> {
        Wupe5W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable for LLWU_P6"]
    #[inline(always)]
    #[must_use]
    pub fn wupe6(&mut self) -> Wupe6W<Pe2Spec> {
        Wupe6W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable for LLWU_P7"]
    #[inline(always)]
    #[must_use]
    pub fn wupe7(&mut self) -> Wupe7W<Pe2Spec> {
        Wupe7W::new(self, 6)
    }
}
#[doc = "LLWU Pin Enable 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pe2Spec;
impl crate::RegisterSpec for Pe2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pe2::R`](R) reader structure"]
impl crate::Readable for Pe2Spec {}
#[doc = "`write(|w| ..)` method takes [`pe2::W`](W) writer structure"]
impl crate::Writable for Pe2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PE2 to value 0"]
impl crate::Resettable for Pe2Spec {
    const RESET_VALUE: u8 = 0;
}
