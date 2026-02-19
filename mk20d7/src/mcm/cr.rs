#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "SRAM_U arbitration priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sramuap {
    #[doc = "0: Round robin"]
    _00 = 0,
    #[doc = "1: Special round robin (favors SRAM backoor accesses over the processor)"]
    _01 = 1,
    #[doc = "2: Fixed priority. Processor has highest, backdoor has lowest"]
    _10 = 2,
    #[doc = "3: Fixed priority. Backdoor has highest, processor has lowest"]
    _11 = 3,
}
impl From<Sramuap> for u8 {
    #[inline(always)]
    fn from(variant: Sramuap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sramuap {
    type Ux = u8;
}
impl crate::IsEnum for Sramuap {}
#[doc = "Field `SRAMUAP` reader - SRAM_U arbitration priority"]
pub type SramuapR = crate::FieldReader<Sramuap>;
impl SramuapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramuap {
        match self.bits {
            0 => Sramuap::_00,
            1 => Sramuap::_01,
            2 => Sramuap::_10,
            3 => Sramuap::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Round robin"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sramuap::_00
    }
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sramuap::_01
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sramuap::_10
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sramuap::_11
    }
}
#[doc = "Field `SRAMUAP` writer - SRAM_U arbitration priority"]
pub type SramuapW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sramuap, crate::Safe>;
impl<'a, REG> SramuapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Round robin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sramuap::_00)
    }
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sramuap::_01)
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sramuap::_10)
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sramuap::_11)
    }
}
#[doc = "Field `SRAMUWP` reader - SRAM_U write protect"]
pub type SramuwpR = crate::BitReader;
#[doc = "Field `SRAMUWP` writer - SRAM_U write protect"]
pub type SramuwpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SRAM_L arbitration priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sramlap {
    #[doc = "0: Round robin"]
    _00 = 0,
    #[doc = "1: Special round robin (favors SRAM backoor accesses over the processor)"]
    _01 = 1,
    #[doc = "2: Fixed priority. Processor has highest, backdoor has lowest"]
    _10 = 2,
    #[doc = "3: Fixed priority. Backdoor has highest, processor has lowest"]
    _11 = 3,
}
impl From<Sramlap> for u8 {
    #[inline(always)]
    fn from(variant: Sramlap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sramlap {
    type Ux = u8;
}
impl crate::IsEnum for Sramlap {}
#[doc = "Field `SRAMLAP` reader - SRAM_L arbitration priority"]
pub type SramlapR = crate::FieldReader<Sramlap>;
impl SramlapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramlap {
        match self.bits {
            0 => Sramlap::_00,
            1 => Sramlap::_01,
            2 => Sramlap::_10,
            3 => Sramlap::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Round robin"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sramlap::_00
    }
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sramlap::_01
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sramlap::_10
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sramlap::_11
    }
}
#[doc = "Field `SRAMLAP` writer - SRAM_L arbitration priority"]
pub type SramlapW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sramlap, crate::Safe>;
impl<'a, REG> SramlapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Round robin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sramlap::_00)
    }
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sramlap::_01)
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sramlap::_10)
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sramlap::_11)
    }
}
#[doc = "Field `SRAMLWP` reader - SRAM_L write protect"]
pub type SramlwpR = crate::BitReader;
#[doc = "Field `SRAMLWP` writer - SRAM_L write protect"]
pub type SramlwpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 24:25 - SRAM_U arbitration priority"]
    #[inline(always)]
    pub fn sramuap(&self) -> SramuapR {
        SramuapR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - SRAM_U write protect"]
    #[inline(always)]
    pub fn sramuwp(&self) -> SramuwpR {
        SramuwpR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:29 - SRAM_L arbitration priority"]
    #[inline(always)]
    pub fn sramlap(&self) -> SramlapR {
        SramlapR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - SRAM_L write protect"]
    #[inline(always)]
    pub fn sramlwp(&self) -> SramlwpR {
        SramlwpR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - SRAM_U arbitration priority"]
    #[inline(always)]
    #[must_use]
    pub fn sramuap(&mut self) -> SramuapW<CrSpec> {
        SramuapW::new(self, 24)
    }
    #[doc = "Bit 26 - SRAM_U write protect"]
    #[inline(always)]
    #[must_use]
    pub fn sramuwp(&mut self) -> SramuwpW<CrSpec> {
        SramuwpW::new(self, 26)
    }
    #[doc = "Bits 28:29 - SRAM_L arbitration priority"]
    #[inline(always)]
    #[must_use]
    pub fn sramlap(&mut self) -> SramlapW<CrSpec> {
        SramlapW::new(self, 28)
    }
    #[doc = "Bit 30 - SRAM_L write protect"]
    #[inline(always)]
    #[must_use]
    pub fn sramlwp(&mut self) -> SramlwpW<CrSpec> {
        SramlwpW::new(self, 30)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
