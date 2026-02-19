#[doc = "Register `FSEC` reader"]
pub type R = crate::R<FsecSpec>;
#[doc = "Flash Security\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sec {
    #[doc = "2: MCU security status is unsecure"]
    _10 = 2,
    #[doc = "3: MCU security status is secure"]
    _11 = 3,
}
impl From<Sec> for u8 {
    #[inline(always)]
    fn from(variant: Sec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sec {
    type Ux = u8;
}
impl crate::IsEnum for Sec {}
#[doc = "Field `SEC` reader - Flash Security"]
pub type SecR = crate::FieldReader<Sec>;
impl SecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sec> {
        match self.bits {
            2 => Some(Sec::_10),
            3 => Some(Sec::_11),
            _ => None,
        }
    }
    #[doc = "MCU security status is unsecure"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sec::_10
    }
    #[doc = "MCU security status is secure"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sec::_11
    }
}
#[doc = "Freescale Failure Analysis Access Code\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fslacc {
    #[doc = "2: Freescale factory access denied"]
    _10 = 2,
    #[doc = "3: Freescale factory access granted"]
    _11 = 3,
}
impl From<Fslacc> for u8 {
    #[inline(always)]
    fn from(variant: Fslacc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fslacc {
    type Ux = u8;
}
impl crate::IsEnum for Fslacc {}
#[doc = "Field `FSLACC` reader - Freescale Failure Analysis Access Code"]
pub type FslaccR = crate::FieldReader<Fslacc>;
impl FslaccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fslacc> {
        match self.bits {
            2 => Some(Fslacc::_10),
            3 => Some(Fslacc::_11),
            _ => None,
        }
    }
    #[doc = "Freescale factory access denied"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Fslacc::_10
    }
    #[doc = "Freescale factory access granted"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Fslacc::_11
    }
}
#[doc = "no description available\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Meen {
    #[doc = "2: Mass erase is disabled"]
    _10 = 2,
    #[doc = "3: Mass erase is enabled"]
    _11 = 3,
}
impl From<Meen> for u8 {
    #[inline(always)]
    fn from(variant: Meen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Meen {
    type Ux = u8;
}
impl crate::IsEnum for Meen {}
#[doc = "Field `MEEN` reader - no description available"]
pub type MeenR = crate::FieldReader<Meen>;
impl MeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Meen> {
        match self.bits {
            2 => Some(Meen::_10),
            3 => Some(Meen::_11),
            _ => None,
        }
    }
    #[doc = "Mass erase is disabled"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Meen::_10
    }
    #[doc = "Mass erase is enabled"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Meen::_11
    }
}
#[doc = "Backdoor Key Security Enable\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyen {
    #[doc = "2: Backdoor key access enabled"]
    _10 = 2,
    #[doc = "3: Backdoor key access disabled"]
    _11 = 3,
}
impl From<Keyen> for u8 {
    #[inline(always)]
    fn from(variant: Keyen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyen {
    type Ux = u8;
}
impl crate::IsEnum for Keyen {}
#[doc = "Field `KEYEN` reader - Backdoor Key Security Enable"]
pub type KeyenR = crate::FieldReader<Keyen>;
impl KeyenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keyen> {
        match self.bits {
            2 => Some(Keyen::_10),
            3 => Some(Keyen::_11),
            _ => None,
        }
    }
    #[doc = "Backdoor key access enabled"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Keyen::_10
    }
    #[doc = "Backdoor key access disabled"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Keyen::_11
    }
}
impl R {
    #[doc = "Bits 0:1 - Flash Security"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Freescale Failure Analysis Access Code"]
    #[inline(always)]
    pub fn fslacc(&self) -> FslaccR {
        FslaccR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - no description available"]
    #[inline(always)]
    pub fn meen(&self) -> MeenR {
        MeenR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Backdoor Key Security Enable"]
    #[inline(always)]
    pub fn keyen(&self) -> KeyenR {
        KeyenR::new((self.bits >> 6) & 3)
    }
}
#[doc = "Non-volatile Flash Security Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsecSpec;
impl crate::RegisterSpec for FsecSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fsec::R`](R) reader structure"]
impl crate::Readable for FsecSpec {}
#[doc = "`reset()` method sets FSEC to value 0xff"]
impl crate::Resettable for FsecSpec {
    const RESET_VALUE: u8 = 0xff;
}
