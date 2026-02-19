#[doc = "Register `FRMNUML` reader"]
pub type R = crate::R<FrmnumlSpec>;
#[doc = "Register `FRMNUML` writer"]
pub type W = crate::W<FrmnumlSpec>;
#[doc = "Field `FRM` reader - no description available"]
pub type FrmR = crate::FieldReader;
#[doc = "Field `FRM` writer - no description available"]
pub type FrmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn frm(&self) -> FrmR {
        FrmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn frm(&mut self) -> FrmW<FrmnumlSpec> {
        FrmW::new(self, 0)
    }
}
#[doc = "Frame Number Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`frmnuml::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmnuml::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrmnumlSpec;
impl crate::RegisterSpec for FrmnumlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`frmnuml::R`](R) reader structure"]
impl crate::Readable for FrmnumlSpec {}
#[doc = "`write(|w| ..)` method takes [`frmnuml::W`](W) writer structure"]
impl crate::Writable for FrmnumlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FRMNUML to value 0"]
impl crate::Resettable for FrmnumlSpec {
    const RESET_VALUE: u8 = 0;
}
