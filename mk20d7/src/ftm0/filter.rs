#[doc = "Register `FILTER` reader"]
pub type R = crate::R<FilterSpec>;
#[doc = "Register `FILTER` writer"]
pub type W = crate::W<FilterSpec>;
#[doc = "Field `CH0FVAL` reader - Channel 0 Input Filter"]
pub type Ch0fvalR = crate::FieldReader;
#[doc = "Field `CH0FVAL` writer - Channel 0 Input Filter"]
pub type Ch0fvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH1FVAL` reader - Channel 1 Input Filter"]
pub type Ch1fvalR = crate::FieldReader;
#[doc = "Field `CH1FVAL` writer - Channel 1 Input Filter"]
pub type Ch1fvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH2FVAL` reader - Channel 2 Input Filter"]
pub type Ch2fvalR = crate::FieldReader;
#[doc = "Field `CH2FVAL` writer - Channel 2 Input Filter"]
pub type Ch2fvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH3FVAL` reader - Channel 3 Input Filter"]
pub type Ch3fvalR = crate::FieldReader;
#[doc = "Field `CH3FVAL` writer - Channel 3 Input Filter"]
pub type Ch3fvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Channel 0 Input Filter"]
    #[inline(always)]
    pub fn ch0fval(&self) -> Ch0fvalR {
        Ch0fvalR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 Input Filter"]
    #[inline(always)]
    pub fn ch1fval(&self) -> Ch1fvalR {
        Ch1fvalR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 2 Input Filter"]
    #[inline(always)]
    pub fn ch2fval(&self) -> Ch2fvalR {
        Ch2fvalR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 3 Input Filter"]
    #[inline(always)]
    pub fn ch3fval(&self) -> Ch3fvalR {
        Ch3fvalR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel 0 Input Filter"]
    #[inline(always)]
    #[must_use]
    pub fn ch0fval(&mut self) -> Ch0fvalW<FilterSpec> {
        Ch0fvalW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Channel 1 Input Filter"]
    #[inline(always)]
    #[must_use]
    pub fn ch1fval(&mut self) -> Ch1fvalW<FilterSpec> {
        Ch1fvalW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Channel 2 Input Filter"]
    #[inline(always)]
    #[must_use]
    pub fn ch2fval(&mut self) -> Ch2fvalW<FilterSpec> {
        Ch2fvalW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Channel 3 Input Filter"]
    #[inline(always)]
    #[must_use]
    pub fn ch3fval(&mut self) -> Ch3fvalW<FilterSpec> {
        Ch3fvalW::new(self, 12)
    }
}
#[doc = "Input Capture Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`filter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FilterSpec;
impl crate::RegisterSpec for FilterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter::R`](R) reader structure"]
impl crate::Readable for FilterSpec {}
#[doc = "`write(|w| ..)` method takes [`filter::W`](W) writer structure"]
impl crate::Writable for FilterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTER to value 0"]
impl crate::Resettable for FilterSpec {
    const RESET_VALUE: u32 = 0;
}
