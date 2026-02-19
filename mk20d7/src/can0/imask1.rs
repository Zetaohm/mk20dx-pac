#[doc = "Register `IMASK1` reader"]
pub type R = crate::R<Imask1Spec>;
#[doc = "Register `IMASK1` writer"]
pub type W = crate::W<Imask1Spec>;
#[doc = "Buffer MBi Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Buflm {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<Buflm> for u32 {
    #[inline(always)]
    fn from(variant: Buflm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Buflm {
    type Ux = u32;
}
impl crate::IsEnum for Buflm {}
#[doc = "Field `BUFLM` reader - Buffer MBi Mask"]
pub type BuflmR = crate::FieldReader<Buflm>;
impl BuflmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Buflm> {
        match self.bits {
            0 => Some(Buflm::_0),
            1 => Some(Buflm::_1),
            _ => None,
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Buflm::_0
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Buflm::_1
    }
}
#[doc = "Field `BUFLM` writer - Buffer MBi Mask"]
pub type BuflmW<'a, REG> = crate::FieldWriter<'a, REG, 32, Buflm>;
impl<'a, REG> BuflmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Buflm::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Buflm::_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer MBi Mask"]
    #[inline(always)]
    pub fn buflm(&self) -> BuflmR {
        BuflmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MBi Mask"]
    #[inline(always)]
    #[must_use]
    pub fn buflm(&mut self) -> BuflmW<Imask1Spec> {
        BuflmW::new(self, 0)
    }
}
#[doc = "Interrupt Masks 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imask1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imask1Spec;
impl crate::RegisterSpec for Imask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask1::R`](R) reader structure"]
impl crate::Readable for Imask1Spec {}
#[doc = "`write(|w| ..)` method takes [`imask1::W`](W) writer structure"]
impl crate::Writable for Imask1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK1 to value 0"]
impl crate::Resettable for Imask1Spec {
    const RESET_VALUE: u32 = 0;
}
