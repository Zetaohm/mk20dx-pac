#[doc = "Register `ME` reader"]
pub type R = crate::R<MeSpec>;
#[doc = "Register `ME` writer"]
pub type W = crate::W<MeSpec>;
#[doc = "Wakeup Module Enable for Module 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wume0 {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<Wume0> for bool {
    #[inline(always)]
    fn from(variant: Wume0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME0` reader - Wakeup Module Enable for Module 0"]
pub type Wume0R = crate::BitReader<Wume0>;
impl Wume0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wume0 {
        match self.bits {
            false => Wume0::_0,
            true => Wume0::_1,
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wume0::_0
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wume0::_1
    }
}
#[doc = "Field `WUME0` writer - Wakeup Module Enable for Module 0"]
pub type Wume0W<'a, REG> = crate::BitWriter<'a, REG, Wume0>;
impl<'a, REG> Wume0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wume0::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wume0::_1)
    }
}
#[doc = "Wakeup Module Enable for Module 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wume1 {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<Wume1> for bool {
    #[inline(always)]
    fn from(variant: Wume1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME1` reader - Wakeup Module Enable for Module 1"]
pub type Wume1R = crate::BitReader<Wume1>;
impl Wume1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wume1 {
        match self.bits {
            false => Wume1::_0,
            true => Wume1::_1,
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wume1::_0
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wume1::_1
    }
}
#[doc = "Field `WUME1` writer - Wakeup Module Enable for Module 1"]
pub type Wume1W<'a, REG> = crate::BitWriter<'a, REG, Wume1>;
impl<'a, REG> Wume1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wume1::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wume1::_1)
    }
}
#[doc = "Wakeup Module Enable for Module 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wume2 {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<Wume2> for bool {
    #[inline(always)]
    fn from(variant: Wume2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME2` reader - Wakeup Module Enable for Module 2"]
pub type Wume2R = crate::BitReader<Wume2>;
impl Wume2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wume2 {
        match self.bits {
            false => Wume2::_0,
            true => Wume2::_1,
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wume2::_0
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wume2::_1
    }
}
#[doc = "Field `WUME2` writer - Wakeup Module Enable for Module 2"]
pub type Wume2W<'a, REG> = crate::BitWriter<'a, REG, Wume2>;
impl<'a, REG> Wume2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wume2::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wume2::_1)
    }
}
#[doc = "Wakeup Module Enable for Module 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wume3 {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<Wume3> for bool {
    #[inline(always)]
    fn from(variant: Wume3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME3` reader - Wakeup Module Enable for Module 3"]
pub type Wume3R = crate::BitReader<Wume3>;
impl Wume3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wume3 {
        match self.bits {
            false => Wume3::_0,
            true => Wume3::_1,
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wume3::_0
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wume3::_1
    }
}
#[doc = "Field `WUME3` writer - Wakeup Module Enable for Module 3"]
pub type Wume3W<'a, REG> = crate::BitWriter<'a, REG, Wume3>;
impl<'a, REG> Wume3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wume3::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wume3::_1)
    }
}
#[doc = "Wakeup Module Enable for Module 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wume4 {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<Wume4> for bool {
    #[inline(always)]
    fn from(variant: Wume4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME4` reader - Wakeup Module Enable for Module 4"]
pub type Wume4R = crate::BitReader<Wume4>;
impl Wume4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wume4 {
        match self.bits {
            false => Wume4::_0,
            true => Wume4::_1,
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wume4::_0
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wume4::_1
    }
}
#[doc = "Field `WUME4` writer - Wakeup Module Enable for Module 4"]
pub type Wume4W<'a, REG> = crate::BitWriter<'a, REG, Wume4>;
impl<'a, REG> Wume4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wume4::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wume4::_1)
    }
}
#[doc = "Wakeup Module Enable for Module 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wume5 {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<Wume5> for bool {
    #[inline(always)]
    fn from(variant: Wume5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME5` reader - Wakeup Module Enable for Module 5"]
pub type Wume5R = crate::BitReader<Wume5>;
impl Wume5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wume5 {
        match self.bits {
            false => Wume5::_0,
            true => Wume5::_1,
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wume5::_0
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wume5::_1
    }
}
#[doc = "Field `WUME5` writer - Wakeup Module Enable for Module 5"]
pub type Wume5W<'a, REG> = crate::BitWriter<'a, REG, Wume5>;
impl<'a, REG> Wume5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wume5::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wume5::_1)
    }
}
#[doc = "Wakeup Module Enable for Module 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wume6 {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<Wume6> for bool {
    #[inline(always)]
    fn from(variant: Wume6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME6` reader - Wakeup Module Enable for Module 6"]
pub type Wume6R = crate::BitReader<Wume6>;
impl Wume6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wume6 {
        match self.bits {
            false => Wume6::_0,
            true => Wume6::_1,
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wume6::_0
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wume6::_1
    }
}
#[doc = "Field `WUME6` writer - Wakeup Module Enable for Module 6"]
pub type Wume6W<'a, REG> = crate::BitWriter<'a, REG, Wume6>;
impl<'a, REG> Wume6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wume6::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wume6::_1)
    }
}
#[doc = "Wakeup Module Enable for Module 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wume7 {
    #[doc = "0: Internal module flag not used as wakeup source"]
    _0 = 0,
    #[doc = "1: Internal module flag used as wakeup source"]
    _1 = 1,
}
impl From<Wume7> for bool {
    #[inline(always)]
    fn from(variant: Wume7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUME7` reader - Wakeup Module Enable for Module 7"]
pub type Wume7R = crate::BitReader<Wume7>;
impl Wume7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wume7 {
        match self.bits {
            false => Wume7::_0,
            true => Wume7::_1,
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wume7::_0
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wume7::_1
    }
}
#[doc = "Field `WUME7` writer - Wakeup Module Enable for Module 7"]
pub type Wume7W<'a, REG> = crate::BitWriter<'a, REG, Wume7>;
impl<'a, REG> Wume7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wume7::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wume7::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Module Enable for Module 0"]
    #[inline(always)]
    pub fn wume0(&self) -> Wume0R {
        Wume0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Module Enable for Module 1"]
    #[inline(always)]
    pub fn wume1(&self) -> Wume1R {
        Wume1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Module Enable for Module 2"]
    #[inline(always)]
    pub fn wume2(&self) -> Wume2R {
        Wume2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup Module Enable for Module 3"]
    #[inline(always)]
    pub fn wume3(&self) -> Wume3R {
        Wume3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup Module Enable for Module 4"]
    #[inline(always)]
    pub fn wume4(&self) -> Wume4R {
        Wume4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup Module Enable for Module 5"]
    #[inline(always)]
    pub fn wume5(&self) -> Wume5R {
        Wume5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup Module Enable for Module 6"]
    #[inline(always)]
    pub fn wume6(&self) -> Wume6R {
        Wume6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Module Enable for Module 7"]
    #[inline(always)]
    pub fn wume7(&self) -> Wume7R {
        Wume7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Module Enable for Module 0"]
    #[inline(always)]
    #[must_use]
    pub fn wume0(&mut self) -> Wume0W<MeSpec> {
        Wume0W::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup Module Enable for Module 1"]
    #[inline(always)]
    #[must_use]
    pub fn wume1(&mut self) -> Wume1W<MeSpec> {
        Wume1W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup Module Enable for Module 2"]
    #[inline(always)]
    #[must_use]
    pub fn wume2(&mut self) -> Wume2W<MeSpec> {
        Wume2W::new(self, 2)
    }
    #[doc = "Bit 3 - Wakeup Module Enable for Module 3"]
    #[inline(always)]
    #[must_use]
    pub fn wume3(&mut self) -> Wume3W<MeSpec> {
        Wume3W::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup Module Enable for Module 4"]
    #[inline(always)]
    #[must_use]
    pub fn wume4(&mut self) -> Wume4W<MeSpec> {
        Wume4W::new(self, 4)
    }
    #[doc = "Bit 5 - Wakeup Module Enable for Module 5"]
    #[inline(always)]
    #[must_use]
    pub fn wume5(&mut self) -> Wume5W<MeSpec> {
        Wume5W::new(self, 5)
    }
    #[doc = "Bit 6 - Wakeup Module Enable for Module 6"]
    #[inline(always)]
    #[must_use]
    pub fn wume6(&mut self) -> Wume6W<MeSpec> {
        Wume6W::new(self, 6)
    }
    #[doc = "Bit 7 - Wakeup Module Enable for Module 7"]
    #[inline(always)]
    #[must_use]
    pub fn wume7(&mut self) -> Wume7W<MeSpec> {
        Wume7W::new(self, 7)
    }
}
#[doc = "LLWU Module Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`me::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`me::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MeSpec;
impl crate::RegisterSpec for MeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`me::R`](R) reader structure"]
impl crate::Readable for MeSpec {}
#[doc = "`write(|w| ..)` method takes [`me::W`](W) writer structure"]
impl crate::Writable for MeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ME to value 0"]
impl crate::Resettable for MeSpec {
    const RESET_VALUE: u8 = 0;
}
