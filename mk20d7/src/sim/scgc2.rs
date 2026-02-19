#[doc = "Register `SCGC2` reader"]
pub type R = crate::R<Scgc2Spec>;
#[doc = "Register `SCGC2` writer"]
pub type W = crate::W<Scgc2Spec>;
#[doc = "DAC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac0 {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<Dac0> for bool {
    #[inline(always)]
    fn from(variant: Dac0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC0` reader - DAC0 Clock Gate Control"]
pub type Dac0R = crate::BitReader<Dac0>;
impl Dac0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac0 {
        match self.bits {
            false => Dac0::_0,
            true => Dac0::_1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dac0::_0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dac0::_1
    }
}
#[doc = "Field `DAC0` writer - DAC0 Clock Gate Control"]
pub type Dac0W<'a, REG> = crate::BitWriter<'a, REG, Dac0>;
impl<'a, REG> Dac0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac0::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac0::_1)
    }
}
impl R {
    #[doc = "Bit 12 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&self) -> Dac0R {
        Dac0R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - DAC0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn dac0(&mut self) -> Dac0W<Scgc2Spec> {
        Dac0W::new(self, 12)
    }
}
#[doc = "System Clock Gating Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgc2Spec;
impl crate::RegisterSpec for Scgc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc2::R`](R) reader structure"]
impl crate::Readable for Scgc2Spec {}
#[doc = "`write(|w| ..)` method takes [`scgc2::W`](W) writer structure"]
impl crate::Writable for Scgc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCGC2 to value 0"]
impl crate::Resettable for Scgc2Spec {
    const RESET_VALUE: u32 = 0;
}
