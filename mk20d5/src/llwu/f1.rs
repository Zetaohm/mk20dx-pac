#[doc = "Register `F1` reader"]
pub type R = crate::R<F1Spec>;
#[doc = "Register `F1` writer"]
pub type W = crate::W<F1Spec>;
#[doc = "Wakeup Flag for LLWU_P0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf0 {
    #[doc = "0: LLWU_P0 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P0 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf0> for bool {
    #[inline(always)]
    fn from(variant: Wuf0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF0` reader - Wakeup Flag for LLWU_P0"]
pub type Wuf0R = crate::BitReader<Wuf0>;
impl Wuf0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf0 {
        match self.bits {
            false => Wuf0::_0,
            true => Wuf0::_1,
        }
    }
    #[doc = "LLWU_P0 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf0::_0
    }
    #[doc = "LLWU_P0 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf0::_1
    }
}
#[doc = "Field `WUF0` writer - Wakeup Flag for LLWU_P0"]
pub type Wuf0W<'a, REG> = crate::BitWriter<'a, REG, Wuf0>;
impl<'a, REG> Wuf0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P0 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf0::_0)
    }
    #[doc = "LLWU_P0 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf0::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf1 {
    #[doc = "0: LLWU_P1 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P1 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf1> for bool {
    #[inline(always)]
    fn from(variant: Wuf1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF1` reader - Wakeup Flag for LLWU_P1"]
pub type Wuf1R = crate::BitReader<Wuf1>;
impl Wuf1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf1 {
        match self.bits {
            false => Wuf1::_0,
            true => Wuf1::_1,
        }
    }
    #[doc = "LLWU_P1 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf1::_0
    }
    #[doc = "LLWU_P1 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf1::_1
    }
}
#[doc = "Field `WUF1` writer - Wakeup Flag for LLWU_P1"]
pub type Wuf1W<'a, REG> = crate::BitWriter<'a, REG, Wuf1>;
impl<'a, REG> Wuf1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P1 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf1::_0)
    }
    #[doc = "LLWU_P1 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf1::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf2 {
    #[doc = "0: LLWU_P2 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P2 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf2> for bool {
    #[inline(always)]
    fn from(variant: Wuf2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF2` reader - Wakeup Flag for LLWU_P2"]
pub type Wuf2R = crate::BitReader<Wuf2>;
impl Wuf2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf2 {
        match self.bits {
            false => Wuf2::_0,
            true => Wuf2::_1,
        }
    }
    #[doc = "LLWU_P2 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf2::_0
    }
    #[doc = "LLWU_P2 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf2::_1
    }
}
#[doc = "Field `WUF2` writer - Wakeup Flag for LLWU_P2"]
pub type Wuf2W<'a, REG> = crate::BitWriter<'a, REG, Wuf2>;
impl<'a, REG> Wuf2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P2 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf2::_0)
    }
    #[doc = "LLWU_P2 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf2::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf3 {
    #[doc = "0: LLWU_P3 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P3 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf3> for bool {
    #[inline(always)]
    fn from(variant: Wuf3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF3` reader - Wakeup Flag for LLWU_P3"]
pub type Wuf3R = crate::BitReader<Wuf3>;
impl Wuf3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf3 {
        match self.bits {
            false => Wuf3::_0,
            true => Wuf3::_1,
        }
    }
    #[doc = "LLWU_P3 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf3::_0
    }
    #[doc = "LLWU_P3 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf3::_1
    }
}
#[doc = "Field `WUF3` writer - Wakeup Flag for LLWU_P3"]
pub type Wuf3W<'a, REG> = crate::BitWriter<'a, REG, Wuf3>;
impl<'a, REG> Wuf3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P3 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf3::_0)
    }
    #[doc = "LLWU_P3 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf3::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf4 {
    #[doc = "0: LLWU_P4 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P4 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf4> for bool {
    #[inline(always)]
    fn from(variant: Wuf4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF4` reader - Wakeup Flag for LLWU_P4"]
pub type Wuf4R = crate::BitReader<Wuf4>;
impl Wuf4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf4 {
        match self.bits {
            false => Wuf4::_0,
            true => Wuf4::_1,
        }
    }
    #[doc = "LLWU_P4 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf4::_0
    }
    #[doc = "LLWU_P4 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf4::_1
    }
}
#[doc = "Field `WUF4` writer - Wakeup Flag for LLWU_P4"]
pub type Wuf4W<'a, REG> = crate::BitWriter<'a, REG, Wuf4>;
impl<'a, REG> Wuf4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P4 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf4::_0)
    }
    #[doc = "LLWU_P4 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf4::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf5 {
    #[doc = "0: LLWU_P5 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P5 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf5> for bool {
    #[inline(always)]
    fn from(variant: Wuf5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF5` reader - Wakeup Flag for LLWU_P5"]
pub type Wuf5R = crate::BitReader<Wuf5>;
impl Wuf5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf5 {
        match self.bits {
            false => Wuf5::_0,
            true => Wuf5::_1,
        }
    }
    #[doc = "LLWU_P5 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf5::_0
    }
    #[doc = "LLWU_P5 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf5::_1
    }
}
#[doc = "Field `WUF5` writer - Wakeup Flag for LLWU_P5"]
pub type Wuf5W<'a, REG> = crate::BitWriter<'a, REG, Wuf5>;
impl<'a, REG> Wuf5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P5 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf5::_0)
    }
    #[doc = "LLWU_P5 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf5::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf6 {
    #[doc = "0: LLWU_P6 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P6 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf6> for bool {
    #[inline(always)]
    fn from(variant: Wuf6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF6` reader - Wakeup Flag for LLWU_P6"]
pub type Wuf6R = crate::BitReader<Wuf6>;
impl Wuf6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf6 {
        match self.bits {
            false => Wuf6::_0,
            true => Wuf6::_1,
        }
    }
    #[doc = "LLWU_P6 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf6::_0
    }
    #[doc = "LLWU_P6 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf6::_1
    }
}
#[doc = "Field `WUF6` writer - Wakeup Flag for LLWU_P6"]
pub type Wuf6W<'a, REG> = crate::BitWriter<'a, REG, Wuf6>;
impl<'a, REG> Wuf6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P6 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf6::_0)
    }
    #[doc = "LLWU_P6 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf6::_1)
    }
}
#[doc = "Wakeup Flag for LLWU_P7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuf7 {
    #[doc = "0: LLWU_P7 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P7 input was a wakeup source"]
    _1 = 1,
}
impl From<Wuf7> for bool {
    #[inline(always)]
    fn from(variant: Wuf7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF7` reader - Wakeup Flag for LLWU_P7"]
pub type Wuf7R = crate::BitReader<Wuf7>;
impl Wuf7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuf7 {
        match self.bits {
            false => Wuf7::_0,
            true => Wuf7::_1,
        }
    }
    #[doc = "LLWU_P7 input was not a wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wuf7::_0
    }
    #[doc = "LLWU_P7 input was a wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wuf7::_1
    }
}
#[doc = "Field `WUF7` writer - Wakeup Flag for LLWU_P7"]
pub type Wuf7W<'a, REG> = crate::BitWriter<'a, REG, Wuf7>;
impl<'a, REG> Wuf7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLWU_P7 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf7::_0)
    }
    #[doc = "LLWU_P7 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuf7::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Flag for LLWU_P0"]
    #[inline(always)]
    pub fn wuf0(&self) -> Wuf0R {
        Wuf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Flag for LLWU_P1"]
    #[inline(always)]
    pub fn wuf1(&self) -> Wuf1R {
        Wuf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag for LLWU_P2"]
    #[inline(always)]
    pub fn wuf2(&self) -> Wuf2R {
        Wuf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup Flag for LLWU_P3"]
    #[inline(always)]
    pub fn wuf3(&self) -> Wuf3R {
        Wuf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup Flag for LLWU_P4"]
    #[inline(always)]
    pub fn wuf4(&self) -> Wuf4R {
        Wuf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup Flag for LLWU_P5"]
    #[inline(always)]
    pub fn wuf5(&self) -> Wuf5R {
        Wuf5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup Flag for LLWU_P6"]
    #[inline(always)]
    pub fn wuf6(&self) -> Wuf6R {
        Wuf6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Flag for LLWU_P7"]
    #[inline(always)]
    pub fn wuf7(&self) -> Wuf7R {
        Wuf7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Flag for LLWU_P0"]
    #[inline(always)]
    #[must_use]
    pub fn wuf0(&mut self) -> Wuf0W<F1Spec> {
        Wuf0W::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup Flag for LLWU_P1"]
    #[inline(always)]
    #[must_use]
    pub fn wuf1(&mut self) -> Wuf1W<F1Spec> {
        Wuf1W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup Flag for LLWU_P2"]
    #[inline(always)]
    #[must_use]
    pub fn wuf2(&mut self) -> Wuf2W<F1Spec> {
        Wuf2W::new(self, 2)
    }
    #[doc = "Bit 3 - Wakeup Flag for LLWU_P3"]
    #[inline(always)]
    #[must_use]
    pub fn wuf3(&mut self) -> Wuf3W<F1Spec> {
        Wuf3W::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup Flag for LLWU_P4"]
    #[inline(always)]
    #[must_use]
    pub fn wuf4(&mut self) -> Wuf4W<F1Spec> {
        Wuf4W::new(self, 4)
    }
    #[doc = "Bit 5 - Wakeup Flag for LLWU_P5"]
    #[inline(always)]
    #[must_use]
    pub fn wuf5(&mut self) -> Wuf5W<F1Spec> {
        Wuf5W::new(self, 5)
    }
    #[doc = "Bit 6 - Wakeup Flag for LLWU_P6"]
    #[inline(always)]
    #[must_use]
    pub fn wuf6(&mut self) -> Wuf6W<F1Spec> {
        Wuf6W::new(self, 6)
    }
    #[doc = "Bit 7 - Wakeup Flag for LLWU_P7"]
    #[inline(always)]
    #[must_use]
    pub fn wuf7(&mut self) -> Wuf7W<F1Spec> {
        Wuf7W::new(self, 7)
    }
}
#[doc = "LLWU Flag 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`f1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F1Spec;
impl crate::RegisterSpec for F1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`f1::R`](R) reader structure"]
impl crate::Readable for F1Spec {}
#[doc = "`write(|w| ..)` method takes [`f1::W`](W) writer structure"]
impl crate::Writable for F1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets F1 to value 0"]
impl crate::Resettable for F1Spec {
    const RESET_VALUE: u8 = 0;
}
