#[doc = "Register `CV%s` reader"]
pub type R = crate::R<CvSpec>;
#[doc = "Register `CV%s` writer"]
pub type W = crate::W<CvSpec>;
#[doc = "Field `CV` reader - Compare value"]
pub type CvR = crate::FieldReader<u16>;
#[doc = "Field `CV` writer - Compare value"]
pub type CvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare value"]
    #[inline(always)]
    pub fn cv(&self) -> CvR {
        CvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn cv(&mut self) -> CvW<CvSpec> {
        CvW::new(self, 0)
    }
}
#[doc = "Compare value registers\n\nYou can [`read`](crate::Reg::read) this register and get [`cv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvSpec;
impl crate::RegisterSpec for CvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cv::R`](R) reader structure"]
impl crate::Readable for CvSpec {}
#[doc = "`write(|w| ..)` method takes [`cv::W`](W) writer structure"]
impl crate::Writable for CvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CV%s to value 0"]
impl crate::Resettable for CvSpec {
    const RESET_VALUE: u32 = 0;
}
