#[doc = "Register `SC` reader"]
pub type R = crate::R<ScSpec>;
#[doc = "Register `SC` writer"]
pub type W = crate::W<ScSpec>;
#[doc = "Field `LDOK` reader - Load OK"]
pub type LdokR = crate::BitReader;
#[doc = "Field `LDOK` writer - Load OK"]
pub type LdokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Continuous Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cont {
    #[doc = "0: PDB operation in One-Shot mode"]
    _0 = 0,
    #[doc = "1: PDB operation in Continuous mode"]
    _1 = 1,
}
impl From<Cont> for bool {
    #[inline(always)]
    fn from(variant: Cont) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - Continuous Mode Enable"]
pub type ContR = crate::BitReader<Cont>;
impl ContR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cont {
        match self.bits {
            false => Cont::_0,
            true => Cont::_1,
        }
    }
    #[doc = "PDB operation in One-Shot mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cont::_0
    }
    #[doc = "PDB operation in Continuous mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cont::_1
    }
}
#[doc = "Field `CONT` writer - Continuous Mode Enable"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG, Cont>;
impl<'a, REG> ContW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PDB operation in One-Shot mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cont::_0)
    }
    #[doc = "PDB operation in Continuous mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cont::_1)
    }
}
#[doc = "Multiplication Factor Select for Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mult {
    #[doc = "0: Multiplication factor is 1"]
    _00 = 0,
    #[doc = "1: Multiplication factor is 10"]
    _01 = 1,
    #[doc = "2: Multiplication factor is 20"]
    _10 = 2,
    #[doc = "3: Multiplication factor is 40"]
    _11 = 3,
}
impl From<Mult> for u8 {
    #[inline(always)]
    fn from(variant: Mult) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mult {
    type Ux = u8;
}
impl crate::IsEnum for Mult {}
#[doc = "Field `MULT` reader - Multiplication Factor Select for Prescaler"]
pub type MultR = crate::FieldReader<Mult>;
impl MultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mult {
        match self.bits {
            0 => Mult::_00,
            1 => Mult::_01,
            2 => Mult::_10,
            3 => Mult::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Multiplication factor is 1"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Mult::_00
    }
    #[doc = "Multiplication factor is 10"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Mult::_01
    }
    #[doc = "Multiplication factor is 20"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Mult::_10
    }
    #[doc = "Multiplication factor is 40"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Mult::_11
    }
}
#[doc = "Field `MULT` writer - Multiplication Factor Select for Prescaler"]
pub type MultW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mult, crate::Safe>;
impl<'a, REG> MultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Multiplication factor is 1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::_00)
    }
    #[doc = "Multiplication factor is 10"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::_01)
    }
    #[doc = "Multiplication factor is 20"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::_10)
    }
    #[doc = "Multiplication factor is 40"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::_11)
    }
}
#[doc = "PDB Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdbie {
    #[doc = "0: PDB interrupt disabled"]
    _0 = 0,
    #[doc = "1: PDB interrupt enabled"]
    _1 = 1,
}
impl From<Pdbie> for bool {
    #[inline(always)]
    fn from(variant: Pdbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDBIE` reader - PDB Interrupt Enable."]
pub type PdbieR = crate::BitReader<Pdbie>;
impl PdbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdbie {
        match self.bits {
            false => Pdbie::_0,
            true => Pdbie::_1,
        }
    }
    #[doc = "PDB interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdbie::_0
    }
    #[doc = "PDB interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdbie::_1
    }
}
#[doc = "Field `PDBIE` writer - PDB Interrupt Enable."]
pub type PdbieW<'a, REG> = crate::BitWriter<'a, REG, Pdbie>;
impl<'a, REG> PdbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PDB interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdbie::_0)
    }
    #[doc = "PDB interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdbie::_1)
    }
}
#[doc = "Field `PDBIF` reader - PDB Interrupt Flag"]
pub type PdbifR = crate::BitReader;
#[doc = "Field `PDBIF` writer - PDB Interrupt Flag"]
pub type PdbifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PDB Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdben {
    #[doc = "0: PDB disabled. Counter is off."]
    _0 = 0,
    #[doc = "1: PDB enabled"]
    _1 = 1,
}
impl From<Pdben> for bool {
    #[inline(always)]
    fn from(variant: Pdben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDBEN` reader - PDB Enable"]
pub type PdbenR = crate::BitReader<Pdben>;
impl PdbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdben {
        match self.bits {
            false => Pdben::_0,
            true => Pdben::_1,
        }
    }
    #[doc = "PDB disabled. Counter is off."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdben::_0
    }
    #[doc = "PDB enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdben::_1
    }
}
#[doc = "Field `PDBEN` writer - PDB Enable"]
pub type PdbenW<'a, REG> = crate::BitWriter<'a, REG, Pdben>;
impl<'a, REG> PdbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PDB disabled. Counter is off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdben::_0)
    }
    #[doc = "PDB enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdben::_1)
    }
}
#[doc = "Trigger Input Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsel {
    #[doc = "0: Trigger-In 0 is selected"]
    _0000 = 0,
    #[doc = "1: Trigger-In 1 is selected"]
    _0001 = 1,
    #[doc = "2: Trigger-In 2 is selected"]
    _0010 = 2,
    #[doc = "3: Trigger-In 3 is selected"]
    _0011 = 3,
    #[doc = "4: Trigger-In 4 is selected"]
    _0100 = 4,
    #[doc = "5: Trigger-In 5 is selected"]
    _0101 = 5,
    #[doc = "6: Trigger-In 6 is selected"]
    _0110 = 6,
    #[doc = "7: Trigger-In 7 is selected"]
    _0111 = 7,
    #[doc = "8: Trigger-In 8 is selected"]
    _1000 = 8,
    #[doc = "9: Trigger-In 9 is selected"]
    _1001 = 9,
    #[doc = "10: Trigger-In 10 is selected"]
    _1010 = 10,
    #[doc = "11: Trigger-In 11 is selected"]
    _1011 = 11,
    #[doc = "12: Trigger-In 12 is selected"]
    _1100 = 12,
    #[doc = "13: Trigger-In 13 is selected"]
    _1101 = 13,
    #[doc = "14: Trigger-In 14 is selected"]
    _1110 = 14,
    #[doc = "15: Software trigger is selected"]
    _1111 = 15,
}
impl From<Trgsel> for u8 {
    #[inline(always)]
    fn from(variant: Trgsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsel {
    type Ux = u8;
}
impl crate::IsEnum for Trgsel {}
#[doc = "Field `TRGSEL` reader - Trigger Input Source Select"]
pub type TrgselR = crate::FieldReader<Trgsel>;
impl TrgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsel {
        match self.bits {
            0 => Trgsel::_0000,
            1 => Trgsel::_0001,
            2 => Trgsel::_0010,
            3 => Trgsel::_0011,
            4 => Trgsel::_0100,
            5 => Trgsel::_0101,
            6 => Trgsel::_0110,
            7 => Trgsel::_0111,
            8 => Trgsel::_1000,
            9 => Trgsel::_1001,
            10 => Trgsel::_1010,
            11 => Trgsel::_1011,
            12 => Trgsel::_1100,
            13 => Trgsel::_1101,
            14 => Trgsel::_1110,
            15 => Trgsel::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Trigger-In 0 is selected"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Trgsel::_0000
    }
    #[doc = "Trigger-In 1 is selected"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Trgsel::_0001
    }
    #[doc = "Trigger-In 2 is selected"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Trgsel::_0010
    }
    #[doc = "Trigger-In 3 is selected"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Trgsel::_0011
    }
    #[doc = "Trigger-In 4 is selected"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Trgsel::_0100
    }
    #[doc = "Trigger-In 5 is selected"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Trgsel::_0101
    }
    #[doc = "Trigger-In 6 is selected"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Trgsel::_0110
    }
    #[doc = "Trigger-In 7 is selected"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == Trgsel::_0111
    }
    #[doc = "Trigger-In 8 is selected"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Trgsel::_1000
    }
    #[doc = "Trigger-In 9 is selected"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Trgsel::_1001
    }
    #[doc = "Trigger-In 10 is selected"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == Trgsel::_1010
    }
    #[doc = "Trigger-In 11 is selected"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == Trgsel::_1011
    }
    #[doc = "Trigger-In 12 is selected"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == Trgsel::_1100
    }
    #[doc = "Trigger-In 13 is selected"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == Trgsel::_1101
    }
    #[doc = "Trigger-In 14 is selected"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == Trgsel::_1110
    }
    #[doc = "Software trigger is selected"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == Trgsel::_1111
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Input Source Select"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsel, crate::Safe>;
impl<'a, REG> TrgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger-In 0 is selected"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_0000)
    }
    #[doc = "Trigger-In 1 is selected"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_0001)
    }
    #[doc = "Trigger-In 2 is selected"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_0010)
    }
    #[doc = "Trigger-In 3 is selected"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_0011)
    }
    #[doc = "Trigger-In 4 is selected"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_0100)
    }
    #[doc = "Trigger-In 5 is selected"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_0101)
    }
    #[doc = "Trigger-In 6 is selected"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_0110)
    }
    #[doc = "Trigger-In 7 is selected"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_0111)
    }
    #[doc = "Trigger-In 8 is selected"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_1000)
    }
    #[doc = "Trigger-In 9 is selected"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_1001)
    }
    #[doc = "Trigger-In 10 is selected"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_1010)
    }
    #[doc = "Trigger-In 11 is selected"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_1011)
    }
    #[doc = "Trigger-In 12 is selected"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_1100)
    }
    #[doc = "Trigger-In 13 is selected"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_1101)
    }
    #[doc = "Trigger-In 14 is selected"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_1110)
    }
    #[doc = "Software trigger is selected"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::_1111)
    }
}
#[doc = "Prescaler Divider Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescaler {
    #[doc = "0: Counting uses the peripheral clock divided by multiplication factor selected by MULT."]
    _000 = 0,
    #[doc = "1: Counting uses the peripheral clock divided by twice of the multiplication factor selected by MULT."]
    _001 = 1,
    #[doc = "2: Counting uses the peripheral clock divided by four times of the multiplication factor selected by MULT."]
    _010 = 2,
    #[doc = "3: Counting uses the peripheral clock divided by eight times of the multiplication factor selected by MULT."]
    _011 = 3,
    #[doc = "4: Counting uses the peripheral clock divided by 16 times of the multiplication factor selected by MULT."]
    _100 = 4,
    #[doc = "5: Counting uses the peripheral clock divided by 32 times of the multiplication factor selected by MULT."]
    _101 = 5,
    #[doc = "6: Counting uses the peripheral clock divided by 64 times of the multiplication factor selected by MULT."]
    _110 = 6,
    #[doc = "7: Counting uses the peripheral clock divided by 128 times of the multiplication factor selected by MULT."]
    _111 = 7,
}
impl From<Prescaler> for u8 {
    #[inline(always)]
    fn from(variant: Prescaler) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescaler {
    type Ux = u8;
}
impl crate::IsEnum for Prescaler {}
#[doc = "Field `PRESCALER` reader - Prescaler Divider Select"]
pub type PrescalerR = crate::FieldReader<Prescaler>;
impl PrescalerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescaler {
        match self.bits {
            0 => Prescaler::_000,
            1 => Prescaler::_001,
            2 => Prescaler::_010,
            3 => Prescaler::_011,
            4 => Prescaler::_100,
            5 => Prescaler::_101,
            6 => Prescaler::_110,
            7 => Prescaler::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Counting uses the peripheral clock divided by multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Prescaler::_000
    }
    #[doc = "Counting uses the peripheral clock divided by twice of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Prescaler::_001
    }
    #[doc = "Counting uses the peripheral clock divided by four times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Prescaler::_010
    }
    #[doc = "Counting uses the peripheral clock divided by eight times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Prescaler::_011
    }
    #[doc = "Counting uses the peripheral clock divided by 16 times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Prescaler::_100
    }
    #[doc = "Counting uses the peripheral clock divided by 32 times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Prescaler::_101
    }
    #[doc = "Counting uses the peripheral clock divided by 64 times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Prescaler::_110
    }
    #[doc = "Counting uses the peripheral clock divided by 128 times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Prescaler::_111
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler Divider Select"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prescaler, crate::Safe>;
impl<'a, REG> PrescalerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counting uses the peripheral clock divided by multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::_000)
    }
    #[doc = "Counting uses the peripheral clock divided by twice of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::_001)
    }
    #[doc = "Counting uses the peripheral clock divided by four times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::_010)
    }
    #[doc = "Counting uses the peripheral clock divided by eight times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::_011)
    }
    #[doc = "Counting uses the peripheral clock divided by 16 times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::_100)
    }
    #[doc = "Counting uses the peripheral clock divided by 32 times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::_101)
    }
    #[doc = "Counting uses the peripheral clock divided by 64 times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::_110)
    }
    #[doc = "Counting uses the peripheral clock divided by 128 times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::_111)
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: DMA disabled"]
    _0 = 0,
    #[doc = "1: DMA enabled"]
    _1 = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
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
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dmaen::_0
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dmaen::_1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::_0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::_1)
    }
}
#[doc = "Field `SWTRIG` writer - Software Trigger"]
pub type SwtrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PDB Sequence Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdbeie {
    #[doc = "0: PDB sequence error interrupt disabled."]
    _0 = 0,
    #[doc = "1: PDB sequence error interrupt enabled."]
    _1 = 1,
}
impl From<Pdbeie> for bool {
    #[inline(always)]
    fn from(variant: Pdbeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDBEIE` reader - PDB Sequence Error Interrupt Enable"]
pub type PdbeieR = crate::BitReader<Pdbeie>;
impl PdbeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdbeie {
        match self.bits {
            false => Pdbeie::_0,
            true => Pdbeie::_1,
        }
    }
    #[doc = "PDB sequence error interrupt disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdbeie::_0
    }
    #[doc = "PDB sequence error interrupt enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdbeie::_1
    }
}
#[doc = "Field `PDBEIE` writer - PDB Sequence Error Interrupt Enable"]
pub type PdbeieW<'a, REG> = crate::BitWriter<'a, REG, Pdbeie>;
impl<'a, REG> PdbeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PDB sequence error interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdbeie::_0)
    }
    #[doc = "PDB sequence error interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdbeie::_1)
    }
}
#[doc = "Load Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldmod {
    #[doc = "0: The internal registers are loaded with the values from their buffers immediately after 1 is written to LDOK."]
    _00 = 0,
    #[doc = "1: The internal registers are loaded with the values from their buffers when the PDB counter reaches the MOD register value after 1 is written to LDOK."]
    _01 = 1,
    #[doc = "2: The internal registers are loaded with the values from their buffers when a trigger input event is detected after 1 is written to LDOK."]
    _10 = 2,
    #[doc = "3: The internal registers are loaded with the values from their buffers when either the PDB counter reaches the MOD register value or a trigger input event is detected, after 1 is written to LDOK."]
    _11 = 3,
}
impl From<Ldmod> for u8 {
    #[inline(always)]
    fn from(variant: Ldmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ldmod {
    type Ux = u8;
}
impl crate::IsEnum for Ldmod {}
#[doc = "Field `LDMOD` reader - Load Mode Select"]
pub type LdmodR = crate::FieldReader<Ldmod>;
impl LdmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldmod {
        match self.bits {
            0 => Ldmod::_00,
            1 => Ldmod::_01,
            2 => Ldmod::_10,
            3 => Ldmod::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "The internal registers are loaded with the values from their buffers immediately after 1 is written to LDOK."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ldmod::_00
    }
    #[doc = "The internal registers are loaded with the values from their buffers when the PDB counter reaches the MOD register value after 1 is written to LDOK."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ldmod::_01
    }
    #[doc = "The internal registers are loaded with the values from their buffers when a trigger input event is detected after 1 is written to LDOK."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ldmod::_10
    }
    #[doc = "The internal registers are loaded with the values from their buffers when either the PDB counter reaches the MOD register value or a trigger input event is detected, after 1 is written to LDOK."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Ldmod::_11
    }
}
#[doc = "Field `LDMOD` writer - Load Mode Select"]
pub type LdmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ldmod, crate::Safe>;
impl<'a, REG> LdmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The internal registers are loaded with the values from their buffers immediately after 1 is written to LDOK."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ldmod::_00)
    }
    #[doc = "The internal registers are loaded with the values from their buffers when the PDB counter reaches the MOD register value after 1 is written to LDOK."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ldmod::_01)
    }
    #[doc = "The internal registers are loaded with the values from their buffers when a trigger input event is detected after 1 is written to LDOK."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ldmod::_10)
    }
    #[doc = "The internal registers are loaded with the values from their buffers when either the PDB counter reaches the MOD register value or a trigger input event is detected, after 1 is written to LDOK."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Ldmod::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Load OK"]
    #[inline(always)]
    pub fn ldok(&self) -> LdokR {
        LdokR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous Mode Enable"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Multiplication Factor Select for Prescaler"]
    #[inline(always)]
    pub fn mult(&self) -> MultR {
        MultR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - PDB Interrupt Enable."]
    #[inline(always)]
    pub fn pdbie(&self) -> PdbieR {
        PdbieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDB Interrupt Flag"]
    #[inline(always)]
    pub fn pdbif(&self) -> PdbifR {
        PdbifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDB Enable"]
    #[inline(always)]
    pub fn pdben(&self) -> PdbenR {
        PdbenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Trigger Input Source Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Prescaler Divider Select"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - PDB Sequence Error Interrupt Enable"]
    #[inline(always)]
    pub fn pdbeie(&self) -> PdbeieR {
        PdbeieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Load Mode Select"]
    #[inline(always)]
    pub fn ldmod(&self) -> LdmodR {
        LdmodR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Load OK"]
    #[inline(always)]
    #[must_use]
    pub fn ldok(&mut self) -> LdokW<ScSpec> {
        LdokW::new(self, 0)
    }
    #[doc = "Bit 1 - Continuous Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> ContW<ScSpec> {
        ContW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Multiplication Factor Select for Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn mult(&mut self) -> MultW<ScSpec> {
        MultW::new(self, 2)
    }
    #[doc = "Bit 5 - PDB Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn pdbie(&mut self) -> PdbieW<ScSpec> {
        PdbieW::new(self, 5)
    }
    #[doc = "Bit 6 - PDB Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pdbif(&mut self) -> PdbifW<ScSpec> {
        PdbifW::new(self, 6)
    }
    #[doc = "Bit 7 - PDB Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdben(&mut self) -> PdbenW<ScSpec> {
        PdbenW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Trigger Input Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TrgselW<ScSpec> {
        TrgselW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Prescaler Divider Select"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<ScSpec> {
        PrescalerW::new(self, 12)
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<ScSpec> {
        DmaenW::new(self, 15)
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig(&mut self) -> SwtrigW<ScSpec> {
        SwtrigW::new(self, 16)
    }
    #[doc = "Bit 17 - PDB Sequence Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdbeie(&mut self) -> PdbeieW<ScSpec> {
        PdbeieW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Load Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn ldmod(&mut self) -> LdmodW<ScSpec> {
        LdmodW::new(self, 18)
    }
}
#[doc = "Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScSpec;
impl crate::RegisterSpec for ScSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sc::R`](R) reader structure"]
impl crate::Readable for ScSpec {}
#[doc = "`write(|w| ..)` method takes [`sc::W`](W) writer structure"]
impl crate::Writable for ScSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for ScSpec {
    const RESET_VALUE: u32 = 0;
}
