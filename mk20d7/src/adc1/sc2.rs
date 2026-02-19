#[doc = "Register `SC2` reader"]
pub type R = crate::R<Sc2Spec>;
#[doc = "Register `SC2` writer"]
pub type W = crate::W<Sc2Spec>;
#[doc = "Voltage reference selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refsel {
    #[doc = "0: Default voltage reference pin pair (external pins VREFH and VREFL)"]
    _00 = 0,
    #[doc = "1: Alternate reference pair (VALTH and VALTL). This pair may be additional external pins or internal sources depending on MCU configuration. Consult the Chip Configuration information for details specific to this MCU."]
    _01 = 1,
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refsel {
    type Ux = u8;
}
impl crate::IsEnum for Refsel {}
#[doc = "Field `REFSEL` reader - Voltage reference selection"]
pub type RefselR = crate::FieldReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refsel> {
        match self.bits {
            0 => Some(Refsel::_00),
            1 => Some(Refsel::_01),
            _ => None,
        }
    }
    #[doc = "Default voltage reference pin pair (external pins VREFH and VREFL)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Refsel::_00
    }
    #[doc = "Alternate reference pair (VALTH and VALTL). This pair may be additional external pins or internal sources depending on MCU configuration. Consult the Chip Configuration information for details specific to this MCU."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Refsel::_01
    }
}
#[doc = "Field `REFSEL` writer - Voltage reference selection"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refsel>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default voltage reference pin pair (external pins VREFH and VREFL)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::_00)
    }
    #[doc = "Alternate reference pair (VALTH and VALTL). This pair may be additional external pins or internal sources depending on MCU configuration. Consult the Chip Configuration information for details specific to this MCU."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::_01)
    }
}
#[doc = "DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: DMA is disabled."]
    _0 = 0,
    #[doc = "1: DMA is enabled and will assert the ADC DMA request during a ADC conversion complete event noted by the assertion of any of the ADC COCO flags."]
    _1 = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::_0,
            true => Dmaen::_1,
        }
    }
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dmaen::_0
    }
    #[doc = "DMA is enabled and will assert the ADC DMA request during a ADC conversion complete event noted by the assertion of any of the ADC COCO flags."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dmaen::_1
    }
}
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::_0)
    }
    #[doc = "DMA is enabled and will assert the ADC DMA request during a ADC conversion complete event noted by the assertion of any of the ADC COCO flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::_1)
    }
}
#[doc = "Compare function range enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acren {
    #[doc = "0: Range function disabled. Only the compare value 1 register (CV1) is compared."]
    _0 = 0,
    #[doc = "1: Range function enabled. Both compare value registers (CV1 and CV2) are compared."]
    _1 = 1,
}
impl From<Acren> for bool {
    #[inline(always)]
    fn from(variant: Acren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACREN` reader - Compare function range enable"]
pub type AcrenR = crate::BitReader<Acren>;
impl AcrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acren {
        match self.bits {
            false => Acren::_0,
            true => Acren::_1,
        }
    }
    #[doc = "Range function disabled. Only the compare value 1 register (CV1) is compared."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Acren::_0
    }
    #[doc = "Range function enabled. Both compare value registers (CV1 and CV2) are compared."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Acren::_1
    }
}
#[doc = "Field `ACREN` writer - Compare function range enable"]
pub type AcrenW<'a, REG> = crate::BitWriter<'a, REG, Acren>;
impl<'a, REG> AcrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Range function disabled. Only the compare value 1 register (CV1) is compared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Acren::_0)
    }
    #[doc = "Range function enabled. Both compare value registers (CV1 and CV2) are compared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Acren::_1)
    }
}
#[doc = "Compare function greater than enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acfgt {
    #[doc = "0: Configures less than threshold, outside range not inclusive and inside range not inclusive functionality based on the values placed in the CV1 and CV2 registers."]
    _0 = 0,
    #[doc = "1: Configures greater than or equal to threshold, outside range inclusive and inside range inclusive functionality based on the values placed in the CV1 and CV2 registers."]
    _1 = 1,
}
impl From<Acfgt> for bool {
    #[inline(always)]
    fn from(variant: Acfgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACFGT` reader - Compare function greater than enable"]
pub type AcfgtR = crate::BitReader<Acfgt>;
impl AcfgtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acfgt {
        match self.bits {
            false => Acfgt::_0,
            true => Acfgt::_1,
        }
    }
    #[doc = "Configures less than threshold, outside range not inclusive and inside range not inclusive functionality based on the values placed in the CV1 and CV2 registers."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Acfgt::_0
    }
    #[doc = "Configures greater than or equal to threshold, outside range inclusive and inside range inclusive functionality based on the values placed in the CV1 and CV2 registers."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Acfgt::_1
    }
}
#[doc = "Field `ACFGT` writer - Compare function greater than enable"]
pub type AcfgtW<'a, REG> = crate::BitWriter<'a, REG, Acfgt>;
impl<'a, REG> AcfgtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configures less than threshold, outside range not inclusive and inside range not inclusive functionality based on the values placed in the CV1 and CV2 registers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Acfgt::_0)
    }
    #[doc = "Configures greater than or equal to threshold, outside range inclusive and inside range inclusive functionality based on the values placed in the CV1 and CV2 registers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Acfgt::_1)
    }
}
#[doc = "Compare function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acfe {
    #[doc = "0: Compare function disabled."]
    _0 = 0,
    #[doc = "1: Compare function enabled."]
    _1 = 1,
}
impl From<Acfe> for bool {
    #[inline(always)]
    fn from(variant: Acfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACFE` reader - Compare function enable"]
pub type AcfeR = crate::BitReader<Acfe>;
impl AcfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acfe {
        match self.bits {
            false => Acfe::_0,
            true => Acfe::_1,
        }
    }
    #[doc = "Compare function disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Acfe::_0
    }
    #[doc = "Compare function enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Acfe::_1
    }
}
#[doc = "Field `ACFE` writer - Compare function enable"]
pub type AcfeW<'a, REG> = crate::BitWriter<'a, REG, Acfe>;
impl<'a, REG> AcfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare function disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Acfe::_0)
    }
    #[doc = "Compare function enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Acfe::_1)
    }
}
#[doc = "Conversion trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adtrg {
    #[doc = "0: Software trigger selected."]
    _0 = 0,
    #[doc = "1: Hardware trigger selected."]
    _1 = 1,
}
impl From<Adtrg> for bool {
    #[inline(always)]
    fn from(variant: Adtrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADTRG` reader - Conversion trigger select"]
pub type AdtrgR = crate::BitReader<Adtrg>;
impl AdtrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adtrg {
        match self.bits {
            false => Adtrg::_0,
            true => Adtrg::_1,
        }
    }
    #[doc = "Software trigger selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adtrg::_0
    }
    #[doc = "Hardware trigger selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adtrg::_1
    }
}
#[doc = "Field `ADTRG` writer - Conversion trigger select"]
pub type AdtrgW<'a, REG> = crate::BitWriter<'a, REG, Adtrg>;
impl<'a, REG> AdtrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adtrg::_0)
    }
    #[doc = "Hardware trigger selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adtrg::_1)
    }
}
#[doc = "Conversion active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adact {
    #[doc = "0: Conversion not in progress."]
    _0 = 0,
    #[doc = "1: Conversion in progress."]
    _1 = 1,
}
impl From<Adact> for bool {
    #[inline(always)]
    fn from(variant: Adact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADACT` reader - Conversion active"]
pub type AdactR = crate::BitReader<Adact>;
impl AdactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adact {
        match self.bits {
            false => Adact::_0,
            true => Adact::_1,
        }
    }
    #[doc = "Conversion not in progress."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adact::_0
    }
    #[doc = "Conversion in progress."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adact::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Voltage reference selection"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare function range enable"]
    #[inline(always)]
    pub fn acren(&self) -> AcrenR {
        AcrenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare function greater than enable"]
    #[inline(always)]
    pub fn acfgt(&self) -> AcfgtR {
        AcfgtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare function enable"]
    #[inline(always)]
    pub fn acfe(&self) -> AcfeR {
        AcfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Conversion trigger select"]
    #[inline(always)]
    pub fn adtrg(&self) -> AdtrgR {
        AdtrgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Conversion active"]
    #[inline(always)]
    pub fn adact(&self) -> AdactR {
        AdactR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Voltage reference selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> RefselW<Sc2Spec> {
        RefselW::new(self, 0)
    }
    #[doc = "Bit 2 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<Sc2Spec> {
        DmaenW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare function range enable"]
    #[inline(always)]
    #[must_use]
    pub fn acren(&mut self) -> AcrenW<Sc2Spec> {
        AcrenW::new(self, 3)
    }
    #[doc = "Bit 4 - Compare function greater than enable"]
    #[inline(always)]
    #[must_use]
    pub fn acfgt(&mut self) -> AcfgtW<Sc2Spec> {
        AcfgtW::new(self, 4)
    }
    #[doc = "Bit 5 - Compare function enable"]
    #[inline(always)]
    #[must_use]
    pub fn acfe(&mut self) -> AcfeW<Sc2Spec> {
        AcfeW::new(self, 5)
    }
    #[doc = "Bit 6 - Conversion trigger select"]
    #[inline(always)]
    #[must_use]
    pub fn adtrg(&mut self) -> AdtrgW<Sc2Spec> {
        AdtrgW::new(self, 6)
    }
}
#[doc = "Status and control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sc2Spec;
impl crate::RegisterSpec for Sc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sc2::R`](R) reader structure"]
impl crate::Readable for Sc2Spec {}
#[doc = "`write(|w| ..)` method takes [`sc2::W`](W) writer structure"]
impl crate::Writable for Sc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SC2 to value 0"]
impl crate::Resettable for Sc2Spec {
    const RESET_VALUE: u32 = 0;
}
