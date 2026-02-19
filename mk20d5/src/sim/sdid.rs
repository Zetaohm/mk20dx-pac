#[doc = "Register `SDID` reader"]
pub type R = crate::R<SdidSpec>;
#[doc = "Pincount identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pinid {
    #[doc = "2: 32-pin"]
    _0010 = 2,
    #[doc = "4: 48-pin"]
    _0100 = 4,
    #[doc = "5: 64-pin"]
    _0101 = 5,
}
impl From<Pinid> for u8 {
    #[inline(always)]
    fn from(variant: Pinid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pinid {
    type Ux = u8;
}
impl crate::IsEnum for Pinid {}
#[doc = "Field `PINID` reader - Pincount identification"]
pub type PinidR = crate::FieldReader<Pinid>;
impl PinidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pinid> {
        match self.bits {
            2 => Some(Pinid::_0010),
            4 => Some(Pinid::_0100),
            5 => Some(Pinid::_0101),
            _ => None,
        }
    }
    #[doc = "32-pin"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Pinid::_0010
    }
    #[doc = "48-pin"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Pinid::_0100
    }
    #[doc = "64-pin"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Pinid::_0101
    }
}
#[doc = "Kinetis family identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Famid {
    #[doc = "0: K10"]
    _000 = 0,
    #[doc = "1: K20"]
    _001 = 1,
}
impl From<Famid> for u8 {
    #[inline(always)]
    fn from(variant: Famid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Famid {
    type Ux = u8;
}
impl crate::IsEnum for Famid {}
#[doc = "Field `FAMID` reader - Kinetis family identification"]
pub type FamidR = crate::FieldReader<Famid>;
impl FamidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Famid> {
        match self.bits {
            0 => Some(Famid::_000),
            1 => Some(Famid::_001),
            _ => None,
        }
    }
    #[doc = "K10"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Famid::_000
    }
    #[doc = "K20"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Famid::_001
    }
}
#[doc = "Field `REVID` reader - Device revision number"]
pub type RevidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline(always)]
    pub fn pinid(&self) -> PinidR {
        PinidR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Kinetis family identification"]
    #[inline(always)]
    pub fn famid(&self) -> FamidR {
        FamidR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:15 - Device revision number"]
    #[inline(always)]
    pub fn revid(&self) -> RevidR {
        RevidR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "System Device Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdidSpec;
impl crate::RegisterSpec for SdidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdid::R`](R) reader structure"]
impl crate::Readable for SdidSpec {}
#[doc = "`reset()` method sets SDID to value 0"]
impl crate::Resettable for SdidSpec {
    const RESET_VALUE: u32 = 0;
}
