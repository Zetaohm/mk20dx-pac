#[doc = "Register `DFER` reader"]
pub type R = crate::R<DferSpec>;
#[doc = "Register `DFER` writer"]
pub type W = crate::W<DferSpec>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Dfe {
    #[doc = "0: Digital Filter is disabled on the corresponding pin and output of the digital filter is reset to zero.Each bit in the field enables the digital filter of the same number as the bit."]
    _0 = 0,
    #[doc = "1: Digital Filter is enabled on the corresponding pin, provided pin is configured as a digital input."]
    _1 = 1,
}
impl From<Dfe> for u32 {
    #[inline(always)]
    fn from(variant: Dfe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dfe {
    type Ux = u32;
}
impl crate::IsEnum for Dfe {}
#[doc = "Field `DFE` reader - Digital Filter Enable"]
pub type DfeR = crate::FieldReader<Dfe>;
impl DfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dfe> {
        match self.bits {
            0 => Some(Dfe::_0),
            1 => Some(Dfe::_1),
            _ => None,
        }
    }
    #[doc = "Digital Filter is disabled on the corresponding pin and output of the digital filter is reset to zero.Each bit in the field enables the digital filter of the same number as the bit."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dfe::_0
    }
    #[doc = "Digital Filter is enabled on the corresponding pin, provided pin is configured as a digital input."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dfe::_1
    }
}
#[doc = "Field `DFE` writer - Digital Filter Enable"]
pub type DfeW<'a, REG> = crate::FieldWriter<'a, REG, 32, Dfe>;
impl<'a, REG> DfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Digital Filter is disabled on the corresponding pin and output of the digital filter is reset to zero.Each bit in the field enables the digital filter of the same number as the bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfe::_0)
    }
    #[doc = "Digital Filter is enabled on the corresponding pin, provided pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfe::_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe(&self) -> DfeR {
        DfeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe(&mut self) -> DfeW<DferSpec> {
        DfeW::new(self, 0)
    }
}
#[doc = "Digital Filter Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DferSpec;
impl crate::RegisterSpec for DferSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfer::R`](R) reader structure"]
impl crate::Readable for DferSpec {}
#[doc = "`write(|w| ..)` method takes [`dfer::W`](W) writer structure"]
impl crate::Writable for DferSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFER to value 0"]
impl crate::Resettable for DferSpec {
    const RESET_VALUE: u32 = 0;
}
