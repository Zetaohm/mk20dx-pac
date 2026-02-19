#[doc = "Register `TAGVDW2S%s` reader"]
pub type R = crate::R<Tagvdw2sSpec>;
#[doc = "Register `TAGVDW2S%s` writer"]
pub type W = crate::W<Tagvdw2sSpec>;
#[doc = "Field `valid` reader - 1-bit valid for cache entry"]
pub type ValidR = crate::BitReader;
#[doc = "Field `valid` writer - 1-bit valid for cache entry"]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tag` reader - 13-bit tag for cache entry"]
pub type TagR = crate::FieldReader<u16>;
#[doc = "Field `tag` writer - 13-bit tag for cache entry"]
pub type TagW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - 1-bit valid for cache entry"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 6:18 - 13-bit tag for cache entry"]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(((self.bits >> 6) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 1-bit valid for cache entry"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<Tagvdw2sSpec> {
        ValidW::new(self, 0)
    }
    #[doc = "Bits 6:18 - 13-bit tag for cache entry"]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TagW<Tagvdw2sSpec> {
        TagW::new(self, 6)
    }
}
#[doc = "Cache Tag Storage\n\nYou can [`read`](crate::Reg::read) this register and get [`tagvdw2s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tagvdw2s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tagvdw2sSpec;
impl crate::RegisterSpec for Tagvdw2sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tagvdw2s::R`](R) reader structure"]
impl crate::Readable for Tagvdw2sSpec {}
#[doc = "`write(|w| ..)` method takes [`tagvdw2s::W`](W) writer structure"]
impl crate::Writable for Tagvdw2sSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAGVDW2S%s to value 0"]
impl crate::Resettable for Tagvdw2sSpec {
    const RESET_VALUE: u32 = 0;
}
