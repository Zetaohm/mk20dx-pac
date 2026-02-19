#[doc = "Register `IR` reader"]
pub type R = crate::R<IrSpec>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IrSpec>;
#[doc = "Transmitter narrow pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tnp {
    #[doc = "0: 3/16."]
    _00 = 0,
    #[doc = "1: 1/16."]
    _01 = 1,
    #[doc = "2: 1/32."]
    _10 = 2,
    #[doc = "3: 1/4."]
    _11 = 3,
}
impl From<Tnp> for u8 {
    #[inline(always)]
    fn from(variant: Tnp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tnp {
    type Ux = u8;
}
impl crate::IsEnum for Tnp {}
#[doc = "Field `TNP` reader - Transmitter narrow pulse"]
pub type TnpR = crate::FieldReader<Tnp>;
impl TnpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnp {
        match self.bits {
            0 => Tnp::_00,
            1 => Tnp::_01,
            2 => Tnp::_10,
            3 => Tnp::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "3/16."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tnp::_00
    }
    #[doc = "1/16."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tnp::_01
    }
    #[doc = "1/32."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tnp::_10
    }
    #[doc = "1/4."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tnp::_11
    }
}
#[doc = "Field `TNP` writer - Transmitter narrow pulse"]
pub type TnpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tnp, crate::Safe>;
impl<'a, REG> TnpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3/16."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tnp::_00)
    }
    #[doc = "1/16."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tnp::_01)
    }
    #[doc = "1/32."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tnp::_10)
    }
    #[doc = "1/4."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tnp::_11)
    }
}
#[doc = "Infrared enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iren {
    #[doc = "0: IR disabled."]
    _0 = 0,
    #[doc = "1: IR enabled."]
    _1 = 1,
}
impl From<Iren> for bool {
    #[inline(always)]
    fn from(variant: Iren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - Infrared enable"]
pub type IrenR = crate::BitReader<Iren>;
impl IrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iren {
        match self.bits {
            false => Iren::_0,
            true => Iren::_1,
        }
    }
    #[doc = "IR disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iren::_0
    }
    #[doc = "IR enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iren::_1
    }
}
#[doc = "Field `IREN` writer - Infrared enable"]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG, Iren>;
impl<'a, REG> IrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IR disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iren::_0)
    }
    #[doc = "IR enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iren::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Transmitter narrow pulse"]
    #[inline(always)]
    pub fn tnp(&self) -> TnpR {
        TnpR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmitter narrow pulse"]
    #[inline(always)]
    #[must_use]
    pub fn tnp(&mut self) -> TnpW<IrSpec> {
        TnpW::new(self, 0)
    }
    #[doc = "Bit 2 - Infrared enable"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IrenW<IrSpec> {
        IrenW::new(self, 2)
    }
}
#[doc = "UART Infrared Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrSpec;
impl crate::RegisterSpec for IrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IrSpec {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IrSpec {
    const RESET_VALUE: u8 = 0;
}
