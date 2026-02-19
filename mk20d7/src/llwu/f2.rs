#[doc = "Register `F2` reader"]
pub type R = crate::R<F2Spec>;
#[doc = "Register `F2` writer"]
pub type W = crate::W<F2Spec>;
#[doc = "Wakeup Flag for LLWU_P8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf8 {
    #[doc = "0: LLWU_P8 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P8 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf8> for bool {
    #[inline(always)]
    fn from(variant: Wuf8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF8` reader - Wakeup Flag for LLWU_P8"]
pub type Wuf8R = crate::BitReader<Wuf8>;
impl Wuf8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf8 {
        match self.bits {
            false => Wuf8::_0,
            true => Wuf8::_1,
        }
    }
    #[doc = "LLWU_P8 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf8::_0
    }
    #[doc = "LLWU_P8 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf8::_1
    }
}
#[doc = "Field `WUF8` writer - Wakeup Flag for LLWU_P8"]
pub type Wuf8W<'a, REG> = crate::BitWriter<'a, REG, Wuf8>;
impl<'a, REG> Wuf8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P8 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf8::_0)
    }
    #[doc = "LLWU_P8 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf8::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf9 {
    #[doc = "0: LLWU_P9 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P9 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf9> for bool {
    #[inline(always)]
    fn from(variant: Wuf9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF9` reader - Wakeup Flag for LLWU_P9"]
pub type Wuf9R = crate::BitReader<Wuf9>;
impl Wuf9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf9 {
        match self.bits {
            false => Wuf9::_0,
            true => Wuf9::_1,
        }
    }
    #[doc = "LLWU_P9 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf9::_0
    }
    #[doc = "LLWU_P9 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf9::_1
    }
}
#[doc = "Field `WUF9` writer - Wakeup Flag for LLWU_P9"]
pub type Wuf9W<'a, REG> = crate::BitWriter<'a, REG, Wuf9>;
impl<'a, REG> Wuf9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P9 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf9::_0)
    }
    #[doc = "LLWU_P9 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf9::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf10 {
    #[doc = "0: LLWU_P10 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P10 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf10> for bool {
    #[inline(always)]
    fn from(variant: Wuf10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF10` reader - Wakeup Flag for LLWU_P10"]
pub type Wuf10R = crate::BitReader<Wuf10>;
impl Wuf10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf10 {
        match self.bits {
            false => Wuf10::_0,
            true => Wuf10::_1,
        }
    }
    #[doc = "LLWU_P10 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf10::_0
    }
    #[doc = "LLWU_P10 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf10::_1
    }
}
#[doc = "Field `WUF10` writer - Wakeup Flag for LLWU_P10"]
pub type Wuf10W<'a, REG> = crate::BitWriter<'a, REG, Wuf10>;
impl<'a, REG> Wuf10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P10 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf10::_0)
    }
    #[doc = "LLWU_P10 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf10::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf11 {
    #[doc = "0: LLWU_P11 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P11 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf11> for bool {
    #[inline(always)]
    fn from(variant: Wuf11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF11` reader - Wakeup Flag for LLWU_P11"]
pub type Wuf11R = crate::BitReader<Wuf11>;
impl Wuf11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf11 {
        match self.bits {
            false => Wuf11::_0,
            true => Wuf11::_1,
        }
    }
    #[doc = "LLWU_P11 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf11::_0
    }
    #[doc = "LLWU_P11 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf11::_1
    }
}
#[doc = "Field `WUF11` writer - Wakeup Flag for LLWU_P11"]
pub type Wuf11W<'a, REG> = crate::BitWriter<'a, REG, Wuf11>;
impl<'a, REG> Wuf11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P11 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf11::_0)
    }
    #[doc = "LLWU_P11 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf11::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf12 {
    #[doc = "0: LLWU_P12 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P12 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf12> for bool {
    #[inline(always)]
    fn from(variant: Wuf12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF12` reader - Wakeup Flag for LLWU_P12"]
pub type Wuf12R = crate::BitReader<Wuf12>;
impl Wuf12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf12 {
        match self.bits {
            false => Wuf12::_0,
            true => Wuf12::_1,
        }
    }
    #[doc = "LLWU_P12 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf12::_0
    }
    #[doc = "LLWU_P12 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf12::_1
    }
}
#[doc = "Field `WUF12` writer - Wakeup Flag for LLWU_P12"]
pub type Wuf12W<'a, REG> = crate::BitWriter<'a, REG, Wuf12>;
impl<'a, REG> Wuf12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P12 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf12::_0)
    }
    #[doc = "LLWU_P12 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf12::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf13 {
    #[doc = "0: LLWU_P13 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P13 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf13> for bool {
    #[inline(always)]
    fn from(variant: Wuf13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF13` reader - Wakeup Flag for LLWU_P13"]
pub type Wuf13R = crate::BitReader<Wuf13>;
impl Wuf13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf13 {
        match self.bits {
            false => Wuf13::_0,
            true => Wuf13::_1,
        }
    }
    #[doc = "LLWU_P13 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf13::_0
    }
    #[doc = "LLWU_P13 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf13::_1
    }
}
#[doc = "Field `WUF13` writer - Wakeup Flag for LLWU_P13"]
pub type Wuf13W<'a, REG> = crate::BitWriter<'a, REG, Wuf13>;
impl<'a, REG> Wuf13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P13 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf13::_0)
    }
    #[doc = "LLWU_P13 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf13::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf14 {
    #[doc = "0: LLWU_P14 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P14 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf14> for bool {
    #[inline(always)]
    fn from(variant: Wuf14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF14` reader - Wakeup Flag for LLWU_P14"]
pub type Wuf14R = crate::BitReader<Wuf14>;
impl Wuf14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf14 {
        match self.bits {
            false => Wuf14::_0,
            true => Wuf14::_1,
        }
    }
    #[doc = "LLWU_P14 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf14::_0
    }
    #[doc = "LLWU_P14 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf14::_1
    }
}
#[doc = "Field `WUF14` writer - Wakeup Flag for LLWU_P14"]
pub type Wuf14W<'a, REG> = crate::BitWriter<'a, REG, Wuf14>;
impl<'a, REG> Wuf14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P14 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf14::_0)
    }
    #[doc = "LLWU_P14 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf14::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf15 {
    #[doc = "0: LLWU_P15 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P15 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf15> for bool {
    #[inline(always)]
    fn from(variant: Wuf15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF15` reader - Wakeup Flag for LLWU_P15"]
pub type Wuf15R = crate::BitReader<Wuf15>;
impl Wuf15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf15 {
        match self.bits {
            false => Wuf15::_0,
            true => Wuf15::_1,
        }
    }
    #[doc = "LLWU_P15 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf15::_0
    }
    #[doc = "LLWU_P15 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf15::_1
    }
}
#[doc = "Field `WUF15` writer - Wakeup Flag for LLWU_P15"]
pub type Wuf15W<'a, REG> = crate::BitWriter<'a, REG, Wuf15>;
impl<'a, REG> Wuf15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P15 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf15::_0)
    }
    #[doc = "LLWU_P15 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf15::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Flag for LLWU_P8"]
    #[inline(always)]
    pub fn wuf8(&self) -> Wuf8R {
        Wuf8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Flag for LLWU_P9"]
    #[inline(always)]
    pub fn wuf9(&self) -> Wuf9R {
        Wuf9R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag for LLWU_P10"]
    #[inline(always)]
    pub fn wuf10(&self) -> Wuf10R {
        Wuf10R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup Flag for LLWU_P11"]
    #[inline(always)]
    pub fn wuf11(&self) -> Wuf11R {
        Wuf11R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup Flag for LLWU_P12"]
    #[inline(always)]
    pub fn wuf12(&self) -> Wuf12R {
        Wuf12R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup Flag for LLWU_P13"]
    #[inline(always)]
    pub fn wuf13(&self) -> Wuf13R {
        Wuf13R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup Flag for LLWU_P14"]
    #[inline(always)]
    pub fn wuf14(&self) -> Wuf14R {
        Wuf14R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Flag for LLWU_P15"]
    #[inline(always)]
    pub fn wuf15(&self) -> Wuf15R {
        Wuf15R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Flag for LLWU_P8"]
    #[inline(always)]
    #[must_use]
    pub fn wuf8(&mut self) -> Wuf8W<F2Spec> {
        Wuf8W::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup Flag for LLWU_P9"]
    #[inline(always)]
    #[must_use]
    pub fn wuf9(&mut self) -> Wuf9W<F2Spec> {
        Wuf9W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup Flag for LLWU_P10"]
    #[inline(always)]
    #[must_use]
    pub fn wuf10(&mut self) -> Wuf10W<F2Spec> {
        Wuf10W::new(self, 2)
    }
    #[doc = "Bit 3 - Wakeup Flag for LLWU_P11"]
    #[inline(always)]
    #[must_use]
    pub fn wuf11(&mut self) -> Wuf11W<F2Spec> {
        Wuf11W::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup Flag for LLWU_P12"]
    #[inline(always)]
    #[must_use]
    pub fn wuf12(&mut self) -> Wuf12W<F2Spec> {
        Wuf12W::new(self, 4)
    }
    #[doc = "Bit 5 - Wakeup Flag for LLWU_P13"]
    #[inline(always)]
    #[must_use]
    pub fn wuf13(&mut self) -> Wuf13W<F2Spec> {
        Wuf13W::new(self, 5)
    }
    #[doc = "Bit 6 - Wakeup Flag for LLWU_P14"]
    #[inline(always)]
    #[must_use]
    pub fn wuf14(&mut self) -> Wuf14W<F2Spec> {
        Wuf14W::new(self, 6)
    }
    #[doc = "Bit 7 - Wakeup Flag for LLWU_P15"]
    #[inline(always)]
    #[must_use]
    pub fn wuf15(&mut self) -> Wuf15W<F2Spec> {
        Wuf15W::new(self, 7)
    }
}
#[doc = "LLWU Flag 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`f2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F2Spec;
impl crate::RegisterSpec for F2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`f2::R`](R) reader structure"]
impl crate::Readable for F2Spec {}
#[doc = "`write(|w| ..)` method takes [`f2::W`](W) writer structure"]
impl crate::Writable for F2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets F2 to value 0"]
impl crate::Resettable for F2Spec {
    const RESET_VALUE: u8 = 0;
}
