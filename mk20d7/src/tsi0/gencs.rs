#[doc = "Register `GENCS` reader"]
pub type R = crate::R<GencsSpec>;
#[doc = "Register `GENCS` writer"]
pub type W = crate::W<GencsSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stpe {
    #[doc = "0: Disable TSI when MCU goes into low power modes."]
    _0 = 0,
    #[doc = "1: Allows TSI to continue running in all low power modes."]
    _1 = 1,
}
impl From<Stpe> for bool {
    #[inline(always)]
    fn from(variant: Stpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPE` reader - no description available"]
pub type StpeR = crate::BitReader<Stpe>;
impl StpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stpe {
        match self.bits {
            false => Stpe::_0,
            true => Stpe::_1,
        }
    }
    #[doc = "Disable TSI when MCU goes into low power modes."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stpe::_0
    }
    #[doc = "Allows TSI to continue running in all low power modes."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stpe::_1
    }
}
#[doc = "Field `STPE` writer - no description available"]
pub type StpeW<'a, REG> = crate::BitWriter<'a, REG, Stpe>;
impl<'a, REG> StpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable TSI when MCU goes into low power modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stpe::_0)
    }
    #[doc = "Allows TSI to continue running in all low power modes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Stpe::_1)
    }
}
#[doc = "Scan Trigger Mode. This bit-field can only be changed if the TSI module is disabled (TSIEN bit = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stm {
    #[doc = "0: Software trigger scan."]
    _0 = 0,
    #[doc = "1: Periodical Scan."]
    _1 = 1,
}
impl From<Stm> for bool {
    #[inline(always)]
    fn from(variant: Stm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STM` reader - Scan Trigger Mode. This bit-field can only be changed if the TSI module is disabled (TSIEN bit = 0)."]
pub type StmR = crate::BitReader<Stm>;
impl StmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stm {
        match self.bits {
            false => Stm::_0,
            true => Stm::_1,
        }
    }
    #[doc = "Software trigger scan."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stm::_0
    }
    #[doc = "Periodical Scan."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stm::_1
    }
}
#[doc = "Field `STM` writer - Scan Trigger Mode. This bit-field can only be changed if the TSI module is disabled (TSIEN bit = 0)."]
pub type StmW<'a, REG> = crate::BitWriter<'a, REG, Stm>;
impl<'a, REG> StmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger scan."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stm::_0)
    }
    #[doc = "Periodical Scan."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Stm::_1)
    }
}
#[doc = "End-of-Scan or Out-of-Range Interrupt select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esor {
    #[doc = "0: Out-of-Range interrupt is allowed."]
    _0 = 0,
    #[doc = "1: End-of-Scan interrupt is allowed."]
    _1 = 1,
}
impl From<Esor> for bool {
    #[inline(always)]
    fn from(variant: Esor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESOR` reader - End-of-Scan or Out-of-Range Interrupt select"]
pub type EsorR = crate::BitReader<Esor>;
impl EsorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Esor {
        match self.bits {
            false => Esor::_0,
            true => Esor::_1,
        }
    }
    #[doc = "Out-of-Range interrupt is allowed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Esor::_0
    }
    #[doc = "End-of-Scan interrupt is allowed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Esor::_1
    }
}
#[doc = "Field `ESOR` writer - End-of-Scan or Out-of-Range Interrupt select"]
pub type EsorW<'a, REG> = crate::BitWriter<'a, REG, Esor>;
impl<'a, REG> EsorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Out-of-Range interrupt is allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Esor::_0)
    }
    #[doc = "End-of-Scan interrupt is allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Esor::_1)
    }
}
#[doc = "Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erie {
    #[doc = "0: Interrupt disabled for error."]
    _0 = 0,
    #[doc = "1: Interrupt enabled for error."]
    _1 = 1,
}
impl From<Erie> for bool {
    #[inline(always)]
    fn from(variant: Erie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERIE` reader - Error Interrupt Enable"]
pub type ErieR = crate::BitReader<Erie>;
impl ErieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erie {
        match self.bits {
            false => Erie::_0,
            true => Erie::_1,
        }
    }
    #[doc = "Interrupt disabled for error."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Erie::_0
    }
    #[doc = "Interrupt enabled for error."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Erie::_1
    }
}
#[doc = "Field `ERIE` writer - Error Interrupt Enable"]
pub type ErieW<'a, REG> = crate::BitWriter<'a, REG, Erie>;
impl<'a, REG> ErieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled for error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Erie::_0)
    }
    #[doc = "Interrupt enabled for error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Erie::_1)
    }
}
#[doc = "Touch Sensing Input Interrupt Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsiie {
    #[doc = "0: Interrupt from TSI is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt from TSI is enabled"]
    _1 = 1,
}
impl From<Tsiie> for bool {
    #[inline(always)]
    fn from(variant: Tsiie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIIE` reader - Touch Sensing Input Interrupt Module Enable"]
pub type TsiieR = crate::BitReader<Tsiie>;
impl TsiieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsiie {
        match self.bits {
            false => Tsiie::_0,
            true => Tsiie::_1,
        }
    }
    #[doc = "Interrupt from TSI is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsiie::_0
    }
    #[doc = "Interrupt from TSI is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsiie::_1
    }
}
#[doc = "Field `TSIIE` writer - Touch Sensing Input Interrupt Module Enable"]
pub type TsiieW<'a, REG> = crate::BitWriter<'a, REG, Tsiie>;
impl<'a, REG> TsiieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt from TSI is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsiie::_0)
    }
    #[doc = "Interrupt from TSI is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsiie::_1)
    }
}
#[doc = "Touch Sensing Input Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsien {
    #[doc = "0: TSI module is disabled"]
    _0 = 0,
    #[doc = "1: TSI module is enabled"]
    _1 = 1,
}
impl From<Tsien> for bool {
    #[inline(always)]
    fn from(variant: Tsien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIEN` reader - Touch Sensing Input Module Enable"]
pub type TsienR = crate::BitReader<Tsien>;
impl TsienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsien {
        match self.bits {
            false => Tsien::_0,
            true => Tsien::_1,
        }
    }
    #[doc = "TSI module is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsien::_0
    }
    #[doc = "TSI module is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsien::_1
    }
}
#[doc = "Field `TSIEN` writer - Touch Sensing Input Module Enable"]
pub type TsienW<'a, REG> = crate::BitWriter<'a, REG, Tsien>;
impl<'a, REG> TsienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TSI module is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsien::_0)
    }
    #[doc = "TSI module is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsien::_1)
    }
}
#[doc = "Field `SWTS` writer - Software Trigger Start"]
pub type SwtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCNIP` reader - Scan In Progress status"]
pub type ScnipR = crate::BitReader;
#[doc = "Overrun error Flag. This flag is set when a scan trigger occurs while a scan is still in progress. Write \"1\", when this flag is set, to clear it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrf {
    #[doc = "0: No over run."]
    _0 = 0,
    #[doc = "1: Over Run occurred."]
    _1 = 1,
}
impl From<Ovrf> for bool {
    #[inline(always)]
    fn from(variant: Ovrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRF` reader - Overrun error Flag. This flag is set when a scan trigger occurs while a scan is still in progress. Write \"1\", when this flag is set, to clear it."]
pub type OvrfR = crate::BitReader<Ovrf>;
impl OvrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrf {
        match self.bits {
            false => Ovrf::_0,
            true => Ovrf::_1,
        }
    }
    #[doc = "No over run."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovrf::_0
    }
    #[doc = "Over Run occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovrf::_1
    }
}
#[doc = "Field `OVRF` writer - Overrun error Flag. This flag is set when a scan trigger occurs while a scan is still in progress. Write \"1\", when this flag is set, to clear it."]
pub type OvrfW<'a, REG> = crate::BitWriter<'a, REG, Ovrf>;
impl<'a, REG> OvrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No over run."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrf::_0)
    }
    #[doc = "Over Run occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrf::_1)
    }
}
#[doc = "External Electrode error occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exterf {
    #[doc = "0: No fault happend on TSI electrodes"]
    _0 = 0,
    #[doc = "1: Short to VDD or VSS was detected on one or more electrodes."]
    _1 = 1,
}
impl From<Exterf> for bool {
    #[inline(always)]
    fn from(variant: Exterf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTERF` reader - External Electrode error occurred"]
pub type ExterfR = crate::BitReader<Exterf>;
impl ExterfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exterf {
        match self.bits {
            false => Exterf::_0,
            true => Exterf::_1,
        }
    }
    #[doc = "No fault happend on TSI electrodes"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Exterf::_0
    }
    #[doc = "Short to VDD or VSS was detected on one or more electrodes."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Exterf::_1
    }
}
#[doc = "Field `EXTERF` writer - External Electrode error occurred"]
pub type ExterfW<'a, REG> = crate::BitWriter<'a, REG, Exterf>;
impl<'a, REG> ExterfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No fault happend on TSI electrodes"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Exterf::_0)
    }
    #[doc = "Short to VDD or VSS was detected on one or more electrodes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Exterf::_1)
    }
}
#[doc = "Field `OUTRGF` reader - Out of Range Flag."]
pub type OutrgfR = crate::BitReader;
#[doc = "Field `OUTRGF` writer - Out of Range Flag."]
pub type OutrgfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSF` reader - End of Scan Flag."]
pub type EosfR = crate::BitReader;
#[doc = "Field `EOSF` writer - End of Scan Flag."]
pub type EosfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Electrode Oscillator prescaler. .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ps {
    #[doc = "0: Electrode Oscillator Frequency divided by 1"]
    _000 = 0,
    #[doc = "1: Electrode Oscillator Frequency divided by 2"]
    _001 = 1,
    #[doc = "2: Electrode Oscillator Frequency divided by 4"]
    _010 = 2,
    #[doc = "3: Electrode Oscillator Frequency divided by 8"]
    _011 = 3,
    #[doc = "4: Electrode Oscillator Frequency divided by 16"]
    _100 = 4,
    #[doc = "5: Electrode Oscillator Frequency divided by 32"]
    _101 = 5,
    #[doc = "6: Electrode Oscillator Frequency divided by 64"]
    _110 = 6,
    #[doc = "7: Electrode Oscillator Frequency divided by 128"]
    _111 = 7,
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ps {
    type Ux = u8;
}
impl crate::IsEnum for Ps {}
#[doc = "Field `PS` reader - Electrode Oscillator prescaler. ."]
pub type PsR = crate::FieldReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            0 => Ps::_000,
            1 => Ps::_001,
            2 => Ps::_010,
            3 => Ps::_011,
            4 => Ps::_100,
            5 => Ps::_101,
            6 => Ps::_110,
            7 => Ps::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Electrode Oscillator Frequency divided by 1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ps::_000
    }
    #[doc = "Electrode Oscillator Frequency divided by 2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ps::_001
    }
    #[doc = "Electrode Oscillator Frequency divided by 4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ps::_010
    }
    #[doc = "Electrode Oscillator Frequency divided by 8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ps::_011
    }
    #[doc = "Electrode Oscillator Frequency divided by 16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ps::_100
    }
    #[doc = "Electrode Oscillator Frequency divided by 32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ps::_101
    }
    #[doc = "Electrode Oscillator Frequency divided by 64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Ps::_110
    }
    #[doc = "Electrode Oscillator Frequency divided by 128"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Ps::_111
    }
}
#[doc = "Field `PS` writer - Electrode Oscillator prescaler. ."]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ps, crate::Safe>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Electrode Oscillator Frequency divided by 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_000)
    }
    #[doc = "Electrode Oscillator Frequency divided by 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_001)
    }
    #[doc = "Electrode Oscillator Frequency divided by 4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_010)
    }
    #[doc = "Electrode Oscillator Frequency divided by 8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_011)
    }
    #[doc = "Electrode Oscillator Frequency divided by 16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_100)
    }
    #[doc = "Electrode Oscillator Frequency divided by 32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_101)
    }
    #[doc = "Electrode Oscillator Frequency divided by 64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_110)
    }
    #[doc = "Electrode Oscillator Frequency divided by 128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_111)
    }
}
#[doc = "Number of Consecutive Scans per Electrode electrode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nscn {
    #[doc = "0: Once per electrode"]
    _00000 = 0,
    #[doc = "1: Twice per electrode"]
    _00001 = 1,
    #[doc = "2: 3 times per electrode"]
    _00010 = 2,
    #[doc = "3: 4 times per electrode"]
    _00011 = 3,
    #[doc = "4: 5 times per electrode"]
    _00100 = 4,
    #[doc = "5: 6 times per electrode"]
    _00101 = 5,
    #[doc = "6: 7 times per electrode"]
    _00110 = 6,
    #[doc = "7: 8 times per electrode"]
    _00111 = 7,
    #[doc = "8: 9 times per electrode"]
    _01000 = 8,
    #[doc = "9: 10 times per electrode"]
    _01001 = 9,
    #[doc = "10: 11 times per electrode"]
    _01010 = 10,
    #[doc = "11: 12 times per electrode"]
    _01011 = 11,
    #[doc = "12: 13 times per electrode"]
    _01100 = 12,
    #[doc = "13: 14 times per electrode"]
    _01101 = 13,
    #[doc = "14: 15 times per electrode"]
    _01110 = 14,
    #[doc = "15: 16 times per electrode"]
    _01111 = 15,
    #[doc = "16: 17 times per electrode"]
    _10000 = 16,
    #[doc = "17: 18 times per electrode"]
    _10001 = 17,
    #[doc = "18: 19 times per electrode"]
    _10010 = 18,
    #[doc = "19: 20 times per electrode"]
    _10011 = 19,
    #[doc = "20: 21 times per electrode"]
    _10100 = 20,
    #[doc = "21: 22 times per electrode"]
    _10101 = 21,
    #[doc = "22: 23 times per electrode"]
    _10110 = 22,
    #[doc = "23: 24 times per electrode"]
    _10111 = 23,
    #[doc = "24: 25 times per electrode"]
    _11000 = 24,
    #[doc = "25: 26 times per electrode"]
    _11001 = 25,
    #[doc = "26: 27 times per electrode"]
    _11010 = 26,
    #[doc = "27: 28 times per electrode"]
    _11011 = 27,
    #[doc = "28: 29 times per electrode"]
    _11100 = 28,
    #[doc = "29: 30 times per electrode"]
    _11101 = 29,
    #[doc = "30: 31 times per electrode"]
    _11110 = 30,
    #[doc = "31: 32 times per electrode"]
    _11111 = 31,
}
impl From<Nscn> for u8 {
    #[inline(always)]
    fn from(variant: Nscn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nscn {
    type Ux = u8;
}
impl crate::IsEnum for Nscn {}
#[doc = "Field `NSCN` reader - Number of Consecutive Scans per Electrode electrode."]
pub type NscnR = crate::FieldReader<Nscn>;
impl NscnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nscn {
        match self.bits {
            0 => Nscn::_00000,
            1 => Nscn::_00001,
            2 => Nscn::_00010,
            3 => Nscn::_00011,
            4 => Nscn::_00100,
            5 => Nscn::_00101,
            6 => Nscn::_00110,
            7 => Nscn::_00111,
            8 => Nscn::_01000,
            9 => Nscn::_01001,
            10 => Nscn::_01010,
            11 => Nscn::_01011,
            12 => Nscn::_01100,
            13 => Nscn::_01101,
            14 => Nscn::_01110,
            15 => Nscn::_01111,
            16 => Nscn::_10000,
            17 => Nscn::_10001,
            18 => Nscn::_10010,
            19 => Nscn::_10011,
            20 => Nscn::_10100,
            21 => Nscn::_10101,
            22 => Nscn::_10110,
            23 => Nscn::_10111,
            24 => Nscn::_11000,
            25 => Nscn::_11001,
            26 => Nscn::_11010,
            27 => Nscn::_11011,
            28 => Nscn::_11100,
            29 => Nscn::_11101,
            30 => Nscn::_11110,
            31 => Nscn::_11111,
            _ => unreachable!(),
        }
    }
    #[doc = "Once per electrode"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == Nscn::_00000
    }
    #[doc = "Twice per electrode"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == Nscn::_00001
    }
    #[doc = "3 times per electrode"]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == Nscn::_00010
    }
    #[doc = "4 times per electrode"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == Nscn::_00011
    }
    #[doc = "5 times per electrode"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == Nscn::_00100
    }
    #[doc = "6 times per electrode"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == Nscn::_00101
    }
    #[doc = "7 times per electrode"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == Nscn::_00110
    }
    #[doc = "8 times per electrode"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == Nscn::_00111
    }
    #[doc = "9 times per electrode"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == Nscn::_01000
    }
    #[doc = "10 times per electrode"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == Nscn::_01001
    }
    #[doc = "11 times per electrode"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == Nscn::_01010
    }
    #[doc = "12 times per electrode"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == Nscn::_01011
    }
    #[doc = "13 times per electrode"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == Nscn::_01100
    }
    #[doc = "14 times per electrode"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == Nscn::_01101
    }
    #[doc = "15 times per electrode"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == Nscn::_01110
    }
    #[doc = "16 times per electrode"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == Nscn::_01111
    }
    #[doc = "17 times per electrode"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == Nscn::_10000
    }
    #[doc = "18 times per electrode"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == Nscn::_10001
    }
    #[doc = "19 times per electrode"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == Nscn::_10010
    }
    #[doc = "20 times per electrode"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == Nscn::_10011
    }
    #[doc = "21 times per electrode"]
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == Nscn::_10100
    }
    #[doc = "22 times per electrode"]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == Nscn::_10101
    }
    #[doc = "23 times per electrode"]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == Nscn::_10110
    }
    #[doc = "24 times per electrode"]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == Nscn::_10111
    }
    #[doc = "25 times per electrode"]
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == Nscn::_11000
    }
    #[doc = "26 times per electrode"]
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == Nscn::_11001
    }
    #[doc = "27 times per electrode"]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == Nscn::_11010
    }
    #[doc = "28 times per electrode"]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == Nscn::_11011
    }
    #[doc = "29 times per electrode"]
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == Nscn::_11100
    }
    #[doc = "30 times per electrode"]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == Nscn::_11101
    }
    #[doc = "31 times per electrode"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == Nscn::_11110
    }
    #[doc = "32 times per electrode"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == Nscn::_11111
    }
}
#[doc = "Field `NSCN` writer - Number of Consecutive Scans per Electrode electrode."]
pub type NscnW<'a, REG> = crate::FieldWriter<'a, REG, 5, Nscn, crate::Safe>;
impl<'a, REG> NscnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Once per electrode"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_00000)
    }
    #[doc = "Twice per electrode"]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_00001)
    }
    #[doc = "3 times per electrode"]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_00010)
    }
    #[doc = "4 times per electrode"]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_00011)
    }
    #[doc = "5 times per electrode"]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_00100)
    }
    #[doc = "6 times per electrode"]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_00101)
    }
    #[doc = "7 times per electrode"]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_00110)
    }
    #[doc = "8 times per electrode"]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_00111)
    }
    #[doc = "9 times per electrode"]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_01000)
    }
    #[doc = "10 times per electrode"]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_01001)
    }
    #[doc = "11 times per electrode"]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_01010)
    }
    #[doc = "12 times per electrode"]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_01011)
    }
    #[doc = "13 times per electrode"]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_01100)
    }
    #[doc = "14 times per electrode"]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_01101)
    }
    #[doc = "15 times per electrode"]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_01110)
    }
    #[doc = "16 times per electrode"]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_01111)
    }
    #[doc = "17 times per electrode"]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_10000)
    }
    #[doc = "18 times per electrode"]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_10001)
    }
    #[doc = "19 times per electrode"]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_10010)
    }
    #[doc = "20 times per electrode"]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_10011)
    }
    #[doc = "21 times per electrode"]
    #[inline(always)]
    pub fn _10100(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_10100)
    }
    #[doc = "22 times per electrode"]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_10101)
    }
    #[doc = "23 times per electrode"]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_10110)
    }
    #[doc = "24 times per electrode"]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_10111)
    }
    #[doc = "25 times per electrode"]
    #[inline(always)]
    pub fn _11000(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_11000)
    }
    #[doc = "26 times per electrode"]
    #[inline(always)]
    pub fn _11001(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_11001)
    }
    #[doc = "27 times per electrode"]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_11010)
    }
    #[doc = "28 times per electrode"]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_11011)
    }
    #[doc = "29 times per electrode"]
    #[inline(always)]
    pub fn _11100(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_11100)
    }
    #[doc = "30 times per electrode"]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_11101)
    }
    #[doc = "31 times per electrode"]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_11110)
    }
    #[doc = "32 times per electrode"]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut crate::W<REG> {
        self.variant(Nscn::_11111)
    }
}
#[doc = "TSI Low Power Mode Scan Interval.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpscnitv {
    #[doc = "0: 1 ms scan interval"]
    _0000 = 0,
    #[doc = "1: 5 ms scan interval"]
    _0001 = 1,
    #[doc = "2: 10 ms scan interval"]
    _0010 = 2,
    #[doc = "3: 15 ms scan interval"]
    _0011 = 3,
    #[doc = "4: 20 ms scan interval"]
    _0100 = 4,
    #[doc = "5: 30 ms scan interval"]
    _0101 = 5,
    #[doc = "6: 40 ms scan interval"]
    _0110 = 6,
    #[doc = "7: 50 ms scan interval"]
    _0111 = 7,
    #[doc = "8: 75 ms scan interval"]
    _1000 = 8,
    #[doc = "9: 100 ms scan interval"]
    _1001 = 9,
    #[doc = "10: 125 ms scan interval"]
    _1010 = 10,
    #[doc = "11: 150 ms scan interval"]
    _1011 = 11,
    #[doc = "12: 200 ms scan interval"]
    _1100 = 12,
    #[doc = "13: 300 ms scan interval"]
    _1101 = 13,
    #[doc = "14: 400 ms scan interval"]
    _1110 = 14,
    #[doc = "15: 500 ms scan interval"]
    _1111 = 15,
}
impl From<Lpscnitv> for u8 {
    #[inline(always)]
    fn from(variant: Lpscnitv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpscnitv {
    type Ux = u8;
}
impl crate::IsEnum for Lpscnitv {}
#[doc = "Field `LPSCNITV` reader - TSI Low Power Mode Scan Interval."]
pub type LpscnitvR = crate::FieldReader<Lpscnitv>;
impl LpscnitvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpscnitv {
        match self.bits {
            0 => Lpscnitv::_0000,
            1 => Lpscnitv::_0001,
            2 => Lpscnitv::_0010,
            3 => Lpscnitv::_0011,
            4 => Lpscnitv::_0100,
            5 => Lpscnitv::_0101,
            6 => Lpscnitv::_0110,
            7 => Lpscnitv::_0111,
            8 => Lpscnitv::_1000,
            9 => Lpscnitv::_1001,
            10 => Lpscnitv::_1010,
            11 => Lpscnitv::_1011,
            12 => Lpscnitv::_1100,
            13 => Lpscnitv::_1101,
            14 => Lpscnitv::_1110,
            15 => Lpscnitv::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "1 ms scan interval"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Lpscnitv::_0000
    }
    #[doc = "5 ms scan interval"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Lpscnitv::_0001
    }
    #[doc = "10 ms scan interval"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Lpscnitv::_0010
    }
    #[doc = "15 ms scan interval"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Lpscnitv::_0011
    }
    #[doc = "20 ms scan interval"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Lpscnitv::_0100
    }
    #[doc = "30 ms scan interval"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Lpscnitv::_0101
    }
    #[doc = "40 ms scan interval"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Lpscnitv::_0110
    }
    #[doc = "50 ms scan interval"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == Lpscnitv::_0111
    }
    #[doc = "75 ms scan interval"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Lpscnitv::_1000
    }
    #[doc = "100 ms scan interval"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Lpscnitv::_1001
    }
    #[doc = "125 ms scan interval"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == Lpscnitv::_1010
    }
    #[doc = "150 ms scan interval"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == Lpscnitv::_1011
    }
    #[doc = "200 ms scan interval"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == Lpscnitv::_1100
    }
    #[doc = "300 ms scan interval"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == Lpscnitv::_1101
    }
    #[doc = "400 ms scan interval"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == Lpscnitv::_1110
    }
    #[doc = "500 ms scan interval"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == Lpscnitv::_1111
    }
}
#[doc = "Field `LPSCNITV` writer - TSI Low Power Mode Scan Interval."]
pub type LpscnitvW<'a, REG> = crate::FieldWriter<'a, REG, 4, Lpscnitv, crate::Safe>;
impl<'a, REG> LpscnitvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 ms scan interval"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_0000)
    }
    #[doc = "5 ms scan interval"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_0001)
    }
    #[doc = "10 ms scan interval"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_0010)
    }
    #[doc = "15 ms scan interval"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_0011)
    }
    #[doc = "20 ms scan interval"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_0100)
    }
    #[doc = "30 ms scan interval"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_0101)
    }
    #[doc = "40 ms scan interval"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_0110)
    }
    #[doc = "50 ms scan interval"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_0111)
    }
    #[doc = "75 ms scan interval"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_1000)
    }
    #[doc = "100 ms scan interval"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_1001)
    }
    #[doc = "125 ms scan interval"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_1010)
    }
    #[doc = "150 ms scan interval"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_1011)
    }
    #[doc = "200 ms scan interval"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_1100)
    }
    #[doc = "300 ms scan interval"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_1101)
    }
    #[doc = "400 ms scan interval"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_1110)
    }
    #[doc = "500 ms scan interval"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(Lpscnitv::_1111)
    }
}
#[doc = "Low Power Mode Clock Source Selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpclks {
    #[doc = "0: LPOCLK is selected to determine the scan period in low power mode"]
    _0 = 0,
    #[doc = "1: VLPOSCCLK is selected to determine the scan period in low power mode"]
    _1 = 1,
}
impl From<Lpclks> for bool {
    #[inline(always)]
    fn from(variant: Lpclks) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPCLKS` reader - Low Power Mode Clock Source Selection."]
pub type LpclksR = crate::BitReader<Lpclks>;
impl LpclksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpclks {
        match self.bits {
            false => Lpclks::_0,
            true => Lpclks::_1,
        }
    }
    #[doc = "LPOCLK is selected to determine the scan period in low power mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lpclks::_0
    }
    #[doc = "VLPOSCCLK is selected to determine the scan period in low power mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lpclks::_1
    }
}
#[doc = "Field `LPCLKS` writer - Low Power Mode Clock Source Selection."]
pub type LpclksW<'a, REG> = crate::BitWriter<'a, REG, Lpclks>;
impl<'a, REG> LpclksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPOCLK is selected to determine the scan period in low power mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpclks::_0)
    }
    #[doc = "VLPOSCCLK is selected to determine the scan period in low power mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpclks::_1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn stpe(&self) -> StpeR {
        StpeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Trigger Mode. This bit-field can only be changed if the TSI module is disabled (TSIEN bit = 0)."]
    #[inline(always)]
    pub fn stm(&self) -> StmR {
        StmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - End-of-Scan or Out-of-Range Interrupt select"]
    #[inline(always)]
    pub fn esor(&self) -> EsorR {
        EsorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&self) -> ErieR {
        ErieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Touch Sensing Input Interrupt Module Enable"]
    #[inline(always)]
    pub fn tsiie(&self) -> TsiieR {
        TsiieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Touch Sensing Input Module Enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TsienR {
        TsienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan In Progress status"]
    #[inline(always)]
    pub fn scnip(&self) -> ScnipR {
        ScnipR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun error Flag. This flag is set when a scan trigger occurs while a scan is still in progress. Write \"1\", when this flag is set, to clear it."]
    #[inline(always)]
    pub fn ovrf(&self) -> OvrfR {
        OvrfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Electrode error occurred"]
    #[inline(always)]
    pub fn exterf(&self) -> ExterfR {
        ExterfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Out of Range Flag."]
    #[inline(always)]
    pub fn outrgf(&self) -> OutrgfR {
        OutrgfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Scan Flag."]
    #[inline(always)]
    pub fn eosf(&self) -> EosfR {
        EosfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Electrode Oscillator prescaler. ."]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:23 - Number of Consecutive Scans per Electrode electrode."]
    #[inline(always)]
    pub fn nscn(&self) -> NscnR {
        NscnR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - TSI Low Power Mode Scan Interval."]
    #[inline(always)]
    pub fn lpscnitv(&self) -> LpscnitvR {
        LpscnitvR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Low Power Mode Clock Source Selection."]
    #[inline(always)]
    pub fn lpclks(&self) -> LpclksR {
        LpclksR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn stpe(&mut self) -> StpeW<GencsSpec> {
        StpeW::new(self, 0)
    }
    #[doc = "Bit 1 - Scan Trigger Mode. This bit-field can only be changed if the TSI module is disabled (TSIEN bit = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn stm(&mut self) -> StmW<GencsSpec> {
        StmW::new(self, 1)
    }
    #[doc = "Bit 4 - End-of-Scan or Out-of-Range Interrupt select"]
    #[inline(always)]
    #[must_use]
    pub fn esor(&mut self) -> EsorW<GencsSpec> {
        EsorW::new(self, 4)
    }
    #[doc = "Bit 5 - Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ErieW<GencsSpec> {
        ErieW::new(self, 5)
    }
    #[doc = "Bit 6 - Touch Sensing Input Interrupt Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsiie(&mut self) -> TsiieW<GencsSpec> {
        TsiieW::new(self, 6)
    }
    #[doc = "Bit 7 - Touch Sensing Input Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsien(&mut self) -> TsienW<GencsSpec> {
        TsienW::new(self, 7)
    }
    #[doc = "Bit 8 - Software Trigger Start"]
    #[inline(always)]
    #[must_use]
    pub fn swts(&mut self) -> SwtsW<GencsSpec> {
        SwtsW::new(self, 8)
    }
    #[doc = "Bit 12 - Overrun error Flag. This flag is set when a scan trigger occurs while a scan is still in progress. Write \"1\", when this flag is set, to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn ovrf(&mut self) -> OvrfW<GencsSpec> {
        OvrfW::new(self, 12)
    }
    #[doc = "Bit 13 - External Electrode error occurred"]
    #[inline(always)]
    #[must_use]
    pub fn exterf(&mut self) -> ExterfW<GencsSpec> {
        ExterfW::new(self, 13)
    }
    #[doc = "Bit 14 - Out of Range Flag."]
    #[inline(always)]
    #[must_use]
    pub fn outrgf(&mut self) -> OutrgfW<GencsSpec> {
        OutrgfW::new(self, 14)
    }
    #[doc = "Bit 15 - End of Scan Flag."]
    #[inline(always)]
    #[must_use]
    pub fn eosf(&mut self) -> EosfW<GencsSpec> {
        EosfW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Electrode Oscillator prescaler. ."]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<GencsSpec> {
        PsW::new(self, 16)
    }
    #[doc = "Bits 19:23 - Number of Consecutive Scans per Electrode electrode."]
    #[inline(always)]
    #[must_use]
    pub fn nscn(&mut self) -> NscnW<GencsSpec> {
        NscnW::new(self, 19)
    }
    #[doc = "Bits 24:27 - TSI Low Power Mode Scan Interval."]
    #[inline(always)]
    #[must_use]
    pub fn lpscnitv(&mut self) -> LpscnitvW<GencsSpec> {
        LpscnitvW::new(self, 24)
    }
    #[doc = "Bit 28 - Low Power Mode Clock Source Selection."]
    #[inline(always)]
    #[must_use]
    pub fn lpclks(&mut self) -> LpclksW<GencsSpec> {
        LpclksW::new(self, 28)
    }
}
#[doc = "General Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gencs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gencs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GencsSpec;
impl crate::RegisterSpec for GencsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gencs::R`](R) reader structure"]
impl crate::Readable for GencsSpec {}
#[doc = "`write(|w| ..)` method takes [`gencs::W`](W) writer structure"]
impl crate::Writable for GencsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GENCS to value 0"]
impl crate::Resettable for GencsSpec {
    const RESET_VALUE: u32 = 0;
}
