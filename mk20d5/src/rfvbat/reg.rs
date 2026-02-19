#[doc = "Register `REG%s` reader"]
pub type R = crate::R<RegSpec>;
#[doc = "Register `REG%s` writer"]
pub type W = crate::W<RegSpec>;
#[doc = "Field `LL` reader - no description available"]
pub type LlR = crate::FieldReader;
#[doc = "Field `LL` writer - no description available"]
pub type LlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LH` reader - no description available"]
pub type LhR = crate::FieldReader;
#[doc = "Field `LH` writer - no description available"]
pub type LhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HL` reader - no description available"]
pub type HlR = crate::FieldReader;
#[doc = "Field `HL` writer - no description available"]
pub type HlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HH` reader - no description available"]
pub type HhR = crate::FieldReader;
#[doc = "Field `HH` writer - no description available"]
pub type HhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn ll(&self) -> LlR {
        LlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - no description available"]
    #[inline(always)]
    pub fn lh(&self) -> LhR {
        LhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - no description available"]
    #[inline(always)]
    pub fn hl(&self) -> HlR {
        HlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - no description available"]
    #[inline(always)]
    pub fn hh(&self) -> HhR {
        HhR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn ll(&mut self) -> LlW<RegSpec> {
        LlW::new(self, 0)
    }
    #[doc = "Bits 8:15 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn lh(&mut self) -> LhW<RegSpec> {
        LhW::new(self, 8)
    }
    #[doc = "Bits 16:23 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn hl(&mut self) -> HlW<RegSpec> {
        HlW::new(self, 16)
    }
    #[doc = "Bits 24:31 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn hh(&mut self) -> HhW<RegSpec> {
        HhW::new(self, 24)
    }
}
#[doc = "VBAT register file register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSpec;
impl crate::RegisterSpec for RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg::R`](R) reader structure"]
impl crate::Readable for RegSpec {}
#[doc = "`write(|w| ..)` method takes [`reg::W`](W) writer structure"]
impl crate::Writable for RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG%s to value 0"]
impl crate::Resettable for RegSpec {
    const RESET_VALUE: u32 = 0;
}
