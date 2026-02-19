#[doc = "Register `MDR` reader"]
pub type R = crate::R<MdrSpec>;
#[doc = "Register `MDR` writer"]
pub type W = crate::W<MdrSpec>;
#[doc = "Field `DIVIDE` reader - MCLK Divide"]
pub type DivideR = crate::FieldReader<u16>;
#[doc = "Field `DIVIDE` writer - MCLK Divide"]
pub type DivideW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FRACT` reader - MCLK Fraction"]
pub type FractR = crate::FieldReader;
#[doc = "Field `FRACT` writer - MCLK Fraction"]
pub type FractW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11 - MCLK Divide"]
    #[inline(always)]
    pub fn divide(&self) -> DivideR {
        DivideR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - MCLK Fraction"]
    #[inline(always)]
    pub fn fract(&self) -> FractR {
        FractR::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - MCLK Divide"]
    #[inline(always)]
    #[must_use]
    pub fn divide(&mut self) -> DivideW<MdrSpec> {
        DivideW::new(self, 0)
    }
    #[doc = "Bits 12:19 - MCLK Fraction"]
    #[inline(always)]
    #[must_use]
    pub fn fract(&mut self) -> FractW<MdrSpec> {
        FractW::new(self, 12)
    }
}
#[doc = "MCLK Divide Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdrSpec;
impl crate::RegisterSpec for MdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdr::R`](R) reader structure"]
impl crate::Readable for MdrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdr::W`](W) writer structure"]
impl crate::Writable for MdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MdrSpec {
    const RESET_VALUE: u32 = 0;
}
