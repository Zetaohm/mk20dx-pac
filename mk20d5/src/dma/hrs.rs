#[doc = "Register `HRS` reader"]
pub type R = crate::R<HrsSpec>;
#[doc = "Register `HRS` writer"]
pub type W = crate::W<HrsSpec>;
#[doc = "Hardware Request Status Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs0 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs0> for bool {
    #[inline(always)]
    fn from(variant: Hrs0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS0` reader - Hardware Request Status Channel 0"]
pub type Hrs0R = crate::BitReader<Hrs0>;
impl Hrs0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs0 {
        match self.bits {
            false => Hrs0::_0,
            true => Hrs0::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs0::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs0::_1
    }
}
#[doc = "Field `HRS0` writer - Hardware Request Status Channel 0"]
pub type Hrs0W<'a, REG> = crate::BitWriter<'a, REG, Hrs0>;
impl<'a, REG> Hrs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs0::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs0::_1)
    }
}
#[doc = "Hardware Request Status Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs1 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs1> for bool {
    #[inline(always)]
    fn from(variant: Hrs1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS1` reader - Hardware Request Status Channel 1"]
pub type Hrs1R = crate::BitReader<Hrs1>;
impl Hrs1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs1 {
        match self.bits {
            false => Hrs1::_0,
            true => Hrs1::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs1::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs1::_1
    }
}
#[doc = "Field `HRS1` writer - Hardware Request Status Channel 1"]
pub type Hrs1W<'a, REG> = crate::BitWriter<'a, REG, Hrs1>;
impl<'a, REG> Hrs1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs1::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs1::_1)
    }
}
#[doc = "Hardware Request Status Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs2 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs2> for bool {
    #[inline(always)]
    fn from(variant: Hrs2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS2` reader - Hardware Request Status Channel 2"]
pub type Hrs2R = crate::BitReader<Hrs2>;
impl Hrs2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs2 {
        match self.bits {
            false => Hrs2::_0,
            true => Hrs2::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs2::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs2::_1
    }
}
#[doc = "Field `HRS2` writer - Hardware Request Status Channel 2"]
pub type Hrs2W<'a, REG> = crate::BitWriter<'a, REG, Hrs2>;
impl<'a, REG> Hrs2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs2::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs2::_1)
    }
}
#[doc = "Hardware Request Status Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrs3 {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<Hrs3> for bool {
    #[inline(always)]
    fn from(variant: Hrs3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRS3` reader - Hardware Request Status Channel 3"]
pub type Hrs3R = crate::BitReader<Hrs3>;
impl Hrs3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrs3 {
        match self.bits {
            false => Hrs3::_0,
            true => Hrs3::_1,
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hrs3::_0
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hrs3::_1
    }
}
#[doc = "Field `HRS3` writer - Hardware Request Status Channel 3"]
pub type Hrs3W<'a, REG> = crate::BitWriter<'a, REG, Hrs3>;
impl<'a, REG> Hrs3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs3::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrs3::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline(always)]
    pub fn hrs0(&self) -> Hrs0R {
        Hrs0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline(always)]
    pub fn hrs1(&self) -> Hrs1R {
        Hrs1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline(always)]
    pub fn hrs2(&self) -> Hrs2R {
        Hrs2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline(always)]
    pub fn hrs3(&self) -> Hrs3R {
        Hrs3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn hrs0(&mut self) -> Hrs0W<HrsSpec> {
        Hrs0W::new(self, 0)
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn hrs1(&mut self) -> Hrs1W<HrsSpec> {
        Hrs1W::new(self, 1)
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn hrs2(&mut self) -> Hrs2W<HrsSpec> {
        Hrs2W::new(self, 2)
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn hrs3(&mut self) -> Hrs3W<HrsSpec> {
        Hrs3W::new(self, 3)
    }
}
#[doc = "Hardware Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrsSpec;
impl crate::RegisterSpec for HrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrs::R`](R) reader structure"]
impl crate::Readable for HrsSpec {}
#[doc = "`write(|w| ..)` method takes [`hrs::W`](W) writer structure"]
impl crate::Writable for HrsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HRS to value 0"]
impl crate::Resettable for HrsSpec {
    const RESET_VALUE: u32 = 0;
}
