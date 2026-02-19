#[doc = "Register `SCGC1` reader"]
pub type R = crate::R<Scgc1Spec>;
#[doc = "Register `SCGC1` writer"]
pub type W = crate::W<Scgc1Spec>;
#[doc = "UART4 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart4 {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<Uart4> for bool {
    #[inline(always)]
    fn from(variant: Uart4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART4` reader - UART4 Clock Gate Control"]
pub type Uart4R = crate::BitReader<Uart4>;
impl Uart4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart4 {
        match self.bits {
            false => Uart4::_0,
            true => Uart4::_1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uart4::_0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uart4::_1
    }
}
#[doc = "Field `UART4` writer - UART4 Clock Gate Control"]
pub type Uart4W<'a, REG> = crate::BitWriter<'a, REG, Uart4>;
impl<'a, REG> Uart4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4::_1)
    }
}
impl R {
    #[doc = "Bit 10 - UART4 Clock Gate Control"]
    #[inline(always)]
    pub fn uart4(&self) -> Uart4R {
        Uart4R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - UART4 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn uart4(&mut self) -> Uart4W<Scgc1Spec> {
        Uart4W::new(self, 10)
    }
}
#[doc = "System Clock Gating Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgc1Spec;
impl crate::RegisterSpec for Scgc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc1::R`](R) reader structure"]
impl crate::Readable for Scgc1Spec {}
#[doc = "`write(|w| ..)` method takes [`scgc1::W`](W) writer structure"]
impl crate::Writable for Scgc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCGC1 to value 0"]
impl crate::Resettable for Scgc1Spec {
    const RESET_VALUE: u32 = 0;
}
