#[doc = "Register `C4` reader"]
pub type R = crate::R<C4Spec>;
#[doc = "Register `C4` writer"]
pub type W = crate::W<C4Spec>;
#[doc = "Field `SCFTRIM` reader - Slow Internal Reference Clock Fine Trim"]
pub type ScftrimR = crate::BitReader;
#[doc = "Field `SCFTRIM` writer - Slow Internal Reference Clock Fine Trim"]
pub type ScftrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCTRIM` reader - Fast Internal Reference Clock Trim Setting"]
pub type FctrimR = crate::FieldReader;
#[doc = "Field `FCTRIM` writer - Fast Internal Reference Clock Trim Setting"]
pub type FctrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "DCO Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DrstDrs {
    #[doc = "0: Encoding 0 - Low range (reset default)."]
    _00 = 0,
    #[doc = "1: Encoding 1 - Mid range."]
    _01 = 1,
    #[doc = "2: Encoding 2 - Mid-high range."]
    _10 = 2,
    #[doc = "3: Encoding 3 - High range."]
    _11 = 3,
}
impl From<DrstDrs> for u8 {
    #[inline(always)]
    fn from(variant: DrstDrs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DrstDrs {
    type Ux = u8;
}
impl crate::IsEnum for DrstDrs {}
#[doc = "Field `DRST_DRS` reader - DCO Range Select"]
pub type DrstDrsR = crate::FieldReader<DrstDrs>;
impl DrstDrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DrstDrs {
        match self.bits {
            0 => DrstDrs::_00,
            1 => DrstDrs::_01,
            2 => DrstDrs::_10,
            3 => DrstDrs::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Encoding 0 - Low range (reset default)."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DrstDrs::_00
    }
    #[doc = "Encoding 1 - Mid range."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DrstDrs::_01
    }
    #[doc = "Encoding 2 - Mid-high range."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DrstDrs::_10
    }
    #[doc = "Encoding 3 - High range."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DrstDrs::_11
    }
}
#[doc = "Field `DRST_DRS` writer - DCO Range Select"]
pub type DrstDrsW<'a, REG> = crate::FieldWriter<'a, REG, 2, DrstDrs, crate::Safe>;
impl<'a, REG> DrstDrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Encoding 0 - Low range (reset default)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DrstDrs::_00)
    }
    #[doc = "Encoding 1 - Mid range."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DrstDrs::_01)
    }
    #[doc = "Encoding 2 - Mid-high range."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(DrstDrs::_10)
    }
    #[doc = "Encoding 3 - High range."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(DrstDrs::_11)
    }
}
#[doc = "DCO Maximum Frequency with 32.768 kHz Reference\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmx32 {
    #[doc = "0: DCO has a default range of 25%."]
    _0 = 0,
    #[doc = "1: DCO is fine-tuned for maximum frequency with 32.768 kHz reference."]
    _1 = 1,
}
impl From<Dmx32> for bool {
    #[inline(always)]
    fn from(variant: Dmx32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMX32` reader - DCO Maximum Frequency with 32.768 kHz Reference"]
pub type Dmx32R = crate::BitReader<Dmx32>;
impl Dmx32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmx32 {
        match self.bits {
            false => Dmx32::_0,
            true => Dmx32::_1,
        }
    }
    #[doc = "DCO has a default range of 25%."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dmx32::_0
    }
    #[doc = "DCO is fine-tuned for maximum frequency with 32.768 kHz reference."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dmx32::_1
    }
}
#[doc = "Field `DMX32` writer - DCO Maximum Frequency with 32.768 kHz Reference"]
pub type Dmx32W<'a, REG> = crate::BitWriter<'a, REG, Dmx32>;
impl<'a, REG> Dmx32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DCO has a default range of 25%."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmx32::_0)
    }
    #[doc = "DCO is fine-tuned for maximum frequency with 32.768 kHz reference."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmx32::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Slow Internal Reference Clock Fine Trim"]
    #[inline(always)]
    pub fn scftrim(&self) -> ScftrimR {
        ScftrimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Fast Internal Reference Clock Trim Setting"]
    #[inline(always)]
    pub fn fctrim(&self) -> FctrimR {
        FctrimR::new((self.bits >> 1) & 0x0f)
    }
    #[doc = "Bits 5:6 - DCO Range Select"]
    #[inline(always)]
    pub fn drst_drs(&self) -> DrstDrsR {
        DrstDrsR::new((self.bits >> 5) & 3)
    }
    #[doc = "Bit 7 - DCO Maximum Frequency with 32.768 kHz Reference"]
    #[inline(always)]
    pub fn dmx32(&self) -> Dmx32R {
        Dmx32R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slow Internal Reference Clock Fine Trim"]
    #[inline(always)]
    #[must_use]
    pub fn scftrim(&mut self) -> ScftrimW<C4Spec> {
        ScftrimW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Fast Internal Reference Clock Trim Setting"]
    #[inline(always)]
    #[must_use]
    pub fn fctrim(&mut self) -> FctrimW<C4Spec> {
        FctrimW::new(self, 1)
    }
    #[doc = "Bits 5:6 - DCO Range Select"]
    #[inline(always)]
    #[must_use]
    pub fn drst_drs(&mut self) -> DrstDrsW<C4Spec> {
        DrstDrsW::new(self, 5)
    }
    #[doc = "Bit 7 - DCO Maximum Frequency with 32.768 kHz Reference"]
    #[inline(always)]
    #[must_use]
    pub fn dmx32(&mut self) -> Dmx32W<C4Spec> {
        Dmx32W::new(self, 7)
    }
}
#[doc = "MCG Control 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C4Spec;
impl crate::RegisterSpec for C4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c4::R`](R) reader structure"]
impl crate::Readable for C4Spec {}
#[doc = "`write(|w| ..)` method takes [`c4::W`](W) writer structure"]
impl crate::Writable for C4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C4 to value 0"]
impl crate::Resettable for C4Spec {
    const RESET_VALUE: u8 = 0;
}
