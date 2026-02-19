#[doc = "Register `FEPROT` reader"]
pub type R = crate::R<FeprotSpec>;
#[doc = "Register `FEPROT` writer"]
pub type W = crate::W<FeprotSpec>;
#[doc = "EEPROM Region Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eprot {
    #[doc = "0: EEPROM region is protected"]
    _0 = 0,
    #[doc = "1: EEPROM region is not protected"]
    _1 = 1,
}
impl From<Eprot> for u8 {
    #[inline(always)]
    fn from(variant: Eprot) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eprot {
    type Ux = u8;
}
impl crate::IsEnum for Eprot {}
#[doc = "Field `EPROT` reader - EEPROM Region Protect"]
pub type EprotR = crate::FieldReader<Eprot>;
impl EprotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Eprot> {
        match self.bits {
            0 => Some(Eprot::_0),
            1 => Some(Eprot::_1),
            _ => None,
        }
    }
    #[doc = "EEPROM region is protected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eprot::_0
    }
    #[doc = "EEPROM region is not protected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eprot::_1
    }
}
#[doc = "Field `EPROT` writer - EEPROM Region Protect"]
pub type EprotW<'a, REG> = crate::FieldWriter<'a, REG, 8, Eprot>;
impl<'a, REG> EprotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EEPROM region is protected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eprot::_0)
    }
    #[doc = "EEPROM region is not protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eprot::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - EEPROM Region Protect"]
    #[inline(always)]
    pub fn eprot(&self) -> EprotR {
        EprotR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - EEPROM Region Protect"]
    #[inline(always)]
    #[must_use]
    pub fn eprot(&mut self) -> EprotW<FeprotSpec> {
        EprotW::new(self, 0)
    }
}
#[doc = "EEPROM Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`feprot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feprot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeprotSpec;
impl crate::RegisterSpec for FeprotSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`feprot::R`](R) reader structure"]
impl crate::Readable for FeprotSpec {}
#[doc = "`write(|w| ..)` method takes [`feprot::W`](W) writer structure"]
impl crate::Writable for FeprotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FEPROT to value 0"]
impl crate::Resettable for FeprotSpec {
    const RESET_VALUE: u8 = 0;
}
