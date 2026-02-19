#[doc = "Register `C1` reader"]
pub type R = crate::R<C1Spec>;
#[doc = "Register `C1` writer"]
pub type W = crate::W<C1Spec>;
#[doc = "Internal Reference Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irefsten {
    #[doc = "0: Internal reference clock is disabled in Stop mode."]
    _0 = 0,
    #[doc = "1: Internal reference clock is enabled in Stop mode if IRCLKEN is set or if MCG is in FEI, FBI, or BLPI modes before entering Stop mode."]
    _1 = 1,
}
impl From<Irefsten> for bool {
    #[inline(always)]
    fn from(variant: Irefsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREFSTEN` reader - Internal Reference Stop Enable"]
pub type IrefstenR = crate::BitReader<Irefsten>;
impl IrefstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irefsten {
        match self.bits {
            false => Irefsten::_0,
            true => Irefsten::_1,
        }
    }
    #[doc = "Internal reference clock is disabled in Stop mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irefsten::_0
    }
    #[doc = "Internal reference clock is enabled in Stop mode if IRCLKEN is set or if MCG is in FEI, FBI, or BLPI modes before entering Stop mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irefsten::_1
    }
}
#[doc = "Field `IREFSTEN` writer - Internal Reference Stop Enable"]
pub type IrefstenW<'a, REG> = crate::BitWriter<'a, REG, Irefsten>;
impl<'a, REG> IrefstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal reference clock is disabled in Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irefsten::_0)
    }
    #[doc = "Internal reference clock is enabled in Stop mode if IRCLKEN is set or if MCG is in FEI, FBI, or BLPI modes before entering Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irefsten::_1)
    }
}
#[doc = "Internal Reference Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irclken {
    #[doc = "0: MCGIRCLK inactive."]
    _0 = 0,
    #[doc = "1: MCGIRCLK active."]
    _1 = 1,
}
impl From<Irclken> for bool {
    #[inline(always)]
    fn from(variant: Irclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCLKEN` reader - Internal Reference Clock Enable"]
pub type IrclkenR = crate::BitReader<Irclken>;
impl IrclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irclken {
        match self.bits {
            false => Irclken::_0,
            true => Irclken::_1,
        }
    }
    #[doc = "MCGIRCLK inactive."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irclken::_0
    }
    #[doc = "MCGIRCLK active."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irclken::_1
    }
}
#[doc = "Field `IRCLKEN` writer - Internal Reference Clock Enable"]
pub type IrclkenW<'a, REG> = crate::BitWriter<'a, REG, Irclken>;
impl<'a, REG> IrclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCGIRCLK inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irclken::_0)
    }
    #[doc = "MCGIRCLK active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irclken::_1)
    }
}
#[doc = "Internal Reference Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irefs {
    #[doc = "0: External reference clock is selected."]
    _0 = 0,
    #[doc = "1: The slow internal reference clock is selected."]
    _1 = 1,
}
impl From<Irefs> for bool {
    #[inline(always)]
    fn from(variant: Irefs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREFS` reader - Internal Reference Select"]
pub type IrefsR = crate::BitReader<Irefs>;
impl IrefsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irefs {
        match self.bits {
            false => Irefs::_0,
            true => Irefs::_1,
        }
    }
    #[doc = "External reference clock is selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irefs::_0
    }
    #[doc = "The slow internal reference clock is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irefs::_1
    }
}
#[doc = "Field `IREFS` writer - Internal Reference Select"]
pub type IrefsW<'a, REG> = crate::BitWriter<'a, REG, Irefs>;
impl<'a, REG> IrefsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External reference clock is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irefs::_0)
    }
    #[doc = "The slow internal reference clock is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irefs::_1)
    }
}
#[doc = "FLL External Reference Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frdiv {
    #[doc = "0: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE 0 values, Divide Factor is 32."]
    _000 = 0,
    #[doc = "1: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE 0 values, Divide Factor is 64."]
    _001 = 1,
    #[doc = "2: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE 0 values, Divide Factor is 128."]
    _010 = 2,
    #[doc = "3: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE 0 values, Divide Factor is 256."]
    _011 = 3,
    #[doc = "4: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE 0 values, Divide Factor is 512."]
    _100 = 4,
    #[doc = "5: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE 0 values, Divide Factor is 1024."]
    _101 = 5,
    #[doc = "6: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE 0 values, Divide Factor is 1280 ."]
    _110 = 6,
    #[doc = "7: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE 0 values, Divide Factor is 1536 ."]
    _111 = 7,
}
impl From<Frdiv> for u8 {
    #[inline(always)]
    fn from(variant: Frdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frdiv {
    type Ux = u8;
}
impl crate::IsEnum for Frdiv {}
#[doc = "Field `FRDIV` reader - FLL External Reference Divider"]
pub type FrdivR = crate::FieldReader<Frdiv>;
impl FrdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frdiv {
        match self.bits {
            0 => Frdiv::_000,
            1 => Frdiv::_001,
            2 => Frdiv::_010,
            3 => Frdiv::_011,
            4 => Frdiv::_100,
            5 => Frdiv::_101,
            6 => Frdiv::_110,
            7 => Frdiv::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE 0 values, Divide Factor is 32."]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Frdiv::_000
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE 0 values, Divide Factor is 64."]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Frdiv::_001
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE 0 values, Divide Factor is 128."]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Frdiv::_010
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE 0 values, Divide Factor is 256."]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Frdiv::_011
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE 0 values, Divide Factor is 512."]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Frdiv::_100
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE 0 values, Divide Factor is 1024."]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Frdiv::_101
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE 0 values, Divide Factor is 1280 ."]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Frdiv::_110
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE 0 values, Divide Factor is 1536 ."]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Frdiv::_111
    }
}
#[doc = "Field `FRDIV` writer - FLL External Reference Divider"]
pub type FrdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Frdiv, crate::Safe>;
impl<'a, REG> FrdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE 0 values, Divide Factor is 32."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_000)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE 0 values, Divide Factor is 64."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_001)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE 0 values, Divide Factor is 128."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_010)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE 0 values, Divide Factor is 256."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_011)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE 0 values, Divide Factor is 512."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_100)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE 0 values, Divide Factor is 1024."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_101)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE 0 values, Divide Factor is 1280 ."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_110)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE 0 values, Divide Factor is 1536 ."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_111)
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clks {
    #[doc = "0: Encoding 0 - Output of FLL or PLL is selected (depends on PLLS control bit)."]
    _00 = 0,
    #[doc = "1: Encoding 1 - Internal reference clock is selected."]
    _01 = 1,
    #[doc = "2: Encoding 2 - External reference clock is selected."]
    _10 = 2,
    #[doc = "3: Encoding 3 - Reserved, defaults to 00."]
    _11 = 3,
}
impl From<Clks> for u8 {
    #[inline(always)]
    fn from(variant: Clks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clks {
    type Ux = u8;
}
impl crate::IsEnum for Clks {}
#[doc = "Field `CLKS` reader - Clock Source Select"]
pub type ClksR = crate::FieldReader<Clks>;
impl ClksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clks {
        match self.bits {
            0 => Clks::_00,
            1 => Clks::_01,
            2 => Clks::_10,
            3 => Clks::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Encoding 0 - Output of FLL or PLL is selected (depends on PLLS control bit)."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Clks::_00
    }
    #[doc = "Encoding 1 - Internal reference clock is selected."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Clks::_01
    }
    #[doc = "Encoding 2 - External reference clock is selected."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Clks::_10
    }
    #[doc = "Encoding 3 - Reserved, defaults to 00."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Clks::_11
    }
}
#[doc = "Field `CLKS` writer - Clock Source Select"]
pub type ClksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clks, crate::Safe>;
impl<'a, REG> ClksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Encoding 0 - Output of FLL or PLL is selected (depends on PLLS control bit)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_00)
    }
    #[doc = "Encoding 1 - Internal reference clock is selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_01)
    }
    #[doc = "Encoding 2 - External reference clock is selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_10)
    }
    #[doc = "Encoding 3 - Reserved, defaults to 00."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline(always)]
    pub fn irefsten(&self) -> IrefstenR {
        IrefstenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline(always)]
    pub fn irclken(&self) -> IrclkenR {
        IrclkenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal Reference Select"]
    #[inline(always)]
    pub fn irefs(&self) -> IrefsR {
        IrefsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FLL External Reference Divider"]
    #[inline(always)]
    pub fn frdiv(&self) -> FrdivR {
        FrdivR::new((self.bits >> 3) & 7)
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline(always)]
    pub fn clks(&self) -> ClksR {
        ClksR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irefsten(&mut self) -> IrefstenW<C1Spec> {
        IrefstenW::new(self, 0)
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irclken(&mut self) -> IrclkenW<C1Spec> {
        IrclkenW::new(self, 1)
    }
    #[doc = "Bit 2 - Internal Reference Select"]
    #[inline(always)]
    #[must_use]
    pub fn irefs(&mut self) -> IrefsW<C1Spec> {
        IrefsW::new(self, 2)
    }
    #[doc = "Bits 3:5 - FLL External Reference Divider"]
    #[inline(always)]
    #[must_use]
    pub fn frdiv(&mut self) -> FrdivW<C1Spec> {
        FrdivW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clks(&mut self) -> ClksW<C1Spec> {
        ClksW::new(self, 6)
    }
}
#[doc = "MCG Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Spec;
impl crate::RegisterSpec for C1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c1::R`](R) reader structure"]
impl crate::Readable for C1Spec {}
#[doc = "`write(|w| ..)` method takes [`c1::W`](W) writer structure"]
impl crate::Writable for C1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C1 to value 0x04"]
impl crate::Resettable for C1Spec {
    const RESET_VALUE: u8 = 0x04;
}
