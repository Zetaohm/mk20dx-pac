#[doc = "Register `SCGC7` reader"]
pub type R = crate::R<Scgc7Spec>;
#[doc = "Register `SCGC7` writer"]
pub type W = crate::W<Scgc7Spec>;
#[doc = "DMA Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA Clock Gate Control"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            false => Dma::_0,
            true => Dma::_1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dma::_0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dma::_1
    }
}
#[doc = "Field `DMA` writer - DMA Clock Gate Control"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, Dma>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::_1)
    }
}
impl R {
    #[doc = "Bit 1 - DMA Clock Gate Control"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DMA Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<Scgc7Spec> {
        DmaW::new(self, 1)
    }
}
#[doc = "System Clock Gating Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgc7Spec;
impl crate::RegisterSpec for Scgc7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc7::R`](R) reader structure"]
impl crate::Readable for Scgc7Spec {}
#[doc = "`write(|w| ..)` method takes [`scgc7::W`](W) writer structure"]
impl crate::Writable for Scgc7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCGC7 to value 0x02"]
impl crate::Resettable for Scgc7Spec {
    const RESET_VALUE: u32 = 0x02;
}
