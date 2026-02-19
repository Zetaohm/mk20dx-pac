#[doc = "Register `CHCFG%s` reader"]
pub type R = crate::R<ChcfgSpec>;
#[doc = "Register `CHCFG%s` writer"]
pub type W = crate::W<ChcfgSpec>;
#[doc = "Field `SOURCE` reader - DMA Channel Source (slot)"]
pub type SourceR = crate::FieldReader;
#[doc = "Field `SOURCE` writer - DMA Channel Source (slot)"]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "DMA Channel Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig {
    #[doc = "0: Triggering is disabled. If triggering is disabled, and the ENBL bit is set, the DMA Channel will simply route the specified source to the DMA channel. (normal mode)"]
    _0 = 0,
    #[doc = "1: Triggering is enabled. If triggering is enabled, and the ENBL bit is set, the DMAMUX is in periodic trigger mode."]
    _1 = 1,
}
impl From<Trig> for bool {
    #[inline(always)]
    fn from(variant: Trig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG` reader - DMA Channel Trigger Enable"]
pub type TrigR = crate::BitReader<Trig>;
impl TrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trig {
        match self.bits {
            false => Trig::_0,
            true => Trig::_1,
        }
    }
    #[doc = "Triggering is disabled. If triggering is disabled, and the ENBL bit is set, the DMA Channel will simply route the specified source to the DMA channel. (normal mode)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trig::_0
    }
    #[doc = "Triggering is enabled. If triggering is enabled, and the ENBL bit is set, the DMAMUX is in periodic trigger mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trig::_1
    }
}
#[doc = "Field `TRIG` writer - DMA Channel Trigger Enable"]
pub type TrigW<'a, REG> = crate::BitWriter<'a, REG, Trig>;
impl<'a, REG> TrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Triggering is disabled. If triggering is disabled, and the ENBL bit is set, the DMA Channel will simply route the specified source to the DMA channel. (normal mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::_0)
    }
    #[doc = "Triggering is enabled. If triggering is enabled, and the ENBL bit is set, the DMAMUX is in periodic trigger mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::_1)
    }
}
#[doc = "DMA Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enbl {
    #[doc = "0: DMA channel is disabled. This mode is primarily used during configuration of the DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or re-configure a DMA channel."]
    _0 = 0,
    #[doc = "1: DMA channel is enabled"]
    _1 = 1,
}
impl From<Enbl> for bool {
    #[inline(always)]
    fn from(variant: Enbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENBL` reader - DMA Channel Enable"]
pub type EnblR = crate::BitReader<Enbl>;
impl EnblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enbl {
        match self.bits {
            false => Enbl::_0,
            true => Enbl::_1,
        }
    }
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or re-configure a DMA channel."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enbl::_0
    }
    #[doc = "DMA channel is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enbl::_1
    }
}
#[doc = "Field `ENBL` writer - DMA Channel Enable"]
pub type EnblW<'a, REG> = crate::BitWriter<'a, REG, Enbl>;
impl<'a, REG> EnblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or re-configure a DMA channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enbl::_0)
    }
    #[doc = "DMA channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enbl::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - DMA Channel Source (slot)"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline(always)]
    pub fn trig(&self) -> TrigR {
        TrigR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline(always)]
    pub fn enbl(&self) -> EnblR {
        EnblR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DMA Channel Source (slot)"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SourceW<ChcfgSpec> {
        SourceW::new(self, 0)
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TrigW<ChcfgSpec> {
        TrigW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbl(&mut self) -> EnblW<ChcfgSpec> {
        EnblW::new(self, 7)
    }
}
#[doc = "Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChcfgSpec;
impl crate::RegisterSpec for ChcfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chcfg::R`](R) reader structure"]
impl crate::Readable for ChcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`chcfg::W`](W) writer structure"]
impl crate::Writable for ChcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CHCFG%s to value 0"]
impl crate::Resettable for ChcfgSpec {
    const RESET_VALUE: u8 = 0;
}
