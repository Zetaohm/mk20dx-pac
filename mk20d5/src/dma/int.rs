#[doc = "Register `INT` reader"]
pub type R = crate::R<IntSpec>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<IntSpec>;
#[doc = "Interrupt Request 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int0 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int0> for bool {
    #[inline(always)]
    fn from(variant: Int0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT0` reader - Interrupt Request 0"]
pub type Int0R = crate::BitReader<Int0>;
impl Int0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int0 {
        match self.bits {
            false => Int0::_0,
            true => Int0::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int0::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int0::_1
    }
}
#[doc = "Field `INT0` writer - Interrupt Request 0"]
pub type Int0W<'a, REG> = crate::BitWriter<'a, REG, Int0>;
impl<'a, REG> Int0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int0::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int0::_1)
    }
}
#[doc = "Interrupt Request 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int1 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int1> for bool {
    #[inline(always)]
    fn from(variant: Int1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT1` reader - Interrupt Request 1"]
pub type Int1R = crate::BitReader<Int1>;
impl Int1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int1 {
        match self.bits {
            false => Int1::_0,
            true => Int1::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int1::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int1::_1
    }
}
#[doc = "Field `INT1` writer - Interrupt Request 1"]
pub type Int1W<'a, REG> = crate::BitWriter<'a, REG, Int1>;
impl<'a, REG> Int1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int1::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int1::_1)
    }
}
#[doc = "Interrupt Request 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int2 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int2> for bool {
    #[inline(always)]
    fn from(variant: Int2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT2` reader - Interrupt Request 2"]
pub type Int2R = crate::BitReader<Int2>;
impl Int2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int2 {
        match self.bits {
            false => Int2::_0,
            true => Int2::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int2::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int2::_1
    }
}
#[doc = "Field `INT2` writer - Interrupt Request 2"]
pub type Int2W<'a, REG> = crate::BitWriter<'a, REG, Int2>;
impl<'a, REG> Int2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int2::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int2::_1)
    }
}
#[doc = "Interrupt Request 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int3 {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    _0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    _1 = 1,
}
impl From<Int3> for bool {
    #[inline(always)]
    fn from(variant: Int3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT3` reader - Interrupt Request 3"]
pub type Int3R = crate::BitReader<Int3>;
impl Int3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int3 {
        match self.bits {
            false => Int3::_0,
            true => Int3::_1,
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Int3::_0
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Int3::_1
    }
}
#[doc = "Field `INT3` writer - Interrupt Request 3"]
pub type Int3W<'a, REG> = crate::BitWriter<'a, REG, Int3>;
impl<'a, REG> Int3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Int3::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Int3::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline(always)]
    pub fn int2(&self) -> Int2R {
        Int2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline(always)]
    pub fn int3(&self) -> Int3R {
        Int3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> Int0W<IntSpec> {
        Int0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> Int1W<IntSpec> {
        Int1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> Int2W<IntSpec> {
        Int2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> Int3W<IntSpec> {
        Int3W::new(self, 3)
    }
}
#[doc = "Interrupt Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSpec;
impl crate::RegisterSpec for IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for IntSpec {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for IntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for IntSpec {
    const RESET_VALUE: u32 = 0;
}
