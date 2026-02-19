#[doc = "Register `EEI` reader"]
pub type R = crate::R<EeiSpec>;
#[doc = "Register `EEI` writer"]
pub type W = crate::W<EeiSpec>;
#[doc = "Enable Error Interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eei0 {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<Eei0> for bool {
    #[inline(always)]
    fn from(variant: Eei0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI0` reader - Enable Error Interrupt 0"]
pub type Eei0R = crate::BitReader<Eei0>;
impl Eei0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eei0 {
        match self.bits {
            false => Eei0::_0,
            true => Eei0::_1,
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eei0::_0
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eei0::_1
    }
}
#[doc = "Field `EEI0` writer - Enable Error Interrupt 0"]
pub type Eei0W<'a, REG> = crate::BitWriter<'a, REG, Eei0>;
impl<'a, REG> Eei0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eei0::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eei0::_1)
    }
}
#[doc = "Enable Error Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eei1 {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<Eei1> for bool {
    #[inline(always)]
    fn from(variant: Eei1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI1` reader - Enable Error Interrupt 1"]
pub type Eei1R = crate::BitReader<Eei1>;
impl Eei1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eei1 {
        match self.bits {
            false => Eei1::_0,
            true => Eei1::_1,
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eei1::_0
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eei1::_1
    }
}
#[doc = "Field `EEI1` writer - Enable Error Interrupt 1"]
pub type Eei1W<'a, REG> = crate::BitWriter<'a, REG, Eei1>;
impl<'a, REG> Eei1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eei1::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eei1::_1)
    }
}
#[doc = "Enable Error Interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eei2 {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<Eei2> for bool {
    #[inline(always)]
    fn from(variant: Eei2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI2` reader - Enable Error Interrupt 2"]
pub type Eei2R = crate::BitReader<Eei2>;
impl Eei2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eei2 {
        match self.bits {
            false => Eei2::_0,
            true => Eei2::_1,
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eei2::_0
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eei2::_1
    }
}
#[doc = "Field `EEI2` writer - Enable Error Interrupt 2"]
pub type Eei2W<'a, REG> = crate::BitWriter<'a, REG, Eei2>;
impl<'a, REG> Eei2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eei2::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eei2::_1)
    }
}
#[doc = "Enable Error Interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eei3 {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<Eei3> for bool {
    #[inline(always)]
    fn from(variant: Eei3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI3` reader - Enable Error Interrupt 3"]
pub type Eei3R = crate::BitReader<Eei3>;
impl Eei3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eei3 {
        match self.bits {
            false => Eei3::_0,
            true => Eei3::_1,
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eei3::_0
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eei3::_1
    }
}
#[doc = "Field `EEI3` writer - Enable Error Interrupt 3"]
pub type Eei3W<'a, REG> = crate::BitWriter<'a, REG, Eei3>;
impl<'a, REG> Eei3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eei3::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eei3::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    pub fn eei0(&self) -> Eei0R {
        Eei0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    pub fn eei1(&self) -> Eei1R {
        Eei1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    pub fn eei2(&self) -> Eei2R {
        Eei2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    pub fn eei3(&self) -> Eei3R {
        Eei3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn eei0(&mut self) -> Eei0W<EeiSpec> {
        Eei0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn eei1(&mut self) -> Eei1W<EeiSpec> {
        Eei1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn eei2(&mut self) -> Eei2W<EeiSpec> {
        Eei2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn eei3(&mut self) -> Eei3W<EeiSpec> {
        Eei3W::new(self, 3)
    }
}
#[doc = "Enable Error Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eei::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eei::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeiSpec;
impl crate::RegisterSpec for EeiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eei::R`](R) reader structure"]
impl crate::Readable for EeiSpec {}
#[doc = "`write(|w| ..)` method takes [`eei::W`](W) writer structure"]
impl crate::Writable for EeiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EEI to value 0"]
impl crate::Resettable for EeiSpec {
    const RESET_VALUE: u32 = 0;
}
