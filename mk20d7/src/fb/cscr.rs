#[doc = "Register `CSCR%s` reader"]
pub type R = crate::R<CscrSpec>;
#[doc = "Register `CSCR%s` writer"]
pub type W = crate::W<CscrSpec>;
#[doc = "Burst-write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bstw {
    #[doc = "0: Break data larger than the specified port size into individual, port-sized, non-burst writes. For example, a longword write to an 8-bit port takes four byte writes."]
    _0 = 0,
    #[doc = "1: Enables burst write of data larger than the specified port size, including longword writes to 8 and 16-bit ports, word writes to 8-bit ports, and line writes to 8-, 16-, and 32-bit ports."]
    _1 = 1,
}
impl From<Bstw> for bool {
    #[inline(always)]
    fn from(variant: Bstw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSTW` reader - Burst-write enable"]
pub type BstwR = crate::BitReader<Bstw>;
impl BstwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bstw {
        match self.bits {
            false => Bstw::_0,
            true => Bstw::_1,
        }
    }
    #[doc = "Break data larger than the specified port size into individual, port-sized, non-burst writes. For example, a longword write to an 8-bit port takes four byte writes."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bstw::_0
    }
    #[doc = "Enables burst write of data larger than the specified port size, including longword writes to 8 and 16-bit ports, word writes to 8-bit ports, and line writes to 8-, 16-, and 32-bit ports."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bstw::_1
    }
}
#[doc = "Field `BSTW` writer - Burst-write enable"]
pub type BstwW<'a, REG> = crate::BitWriter<'a, REG, Bstw>;
impl<'a, REG> BstwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break data larger than the specified port size into individual, port-sized, non-burst writes. For example, a longword write to an 8-bit port takes four byte writes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bstw::_0)
    }
    #[doc = "Enables burst write of data larger than the specified port size, including longword writes to 8 and 16-bit ports, word writes to 8-bit ports, and line writes to 8-, 16-, and 32-bit ports."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bstw::_1)
    }
}
#[doc = "Burst-read enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bstr {
    #[doc = "0: Data exceeding the specified port size is broken into individual, port-sized, non-burst reads. For example, a longword read from an 8-bit port is broken into four 8-bit reads."]
    _0 = 0,
    #[doc = "1: Enables data burst reads larger than the specified port size, including longword reads from 8- and 16-bit ports, word reads from 8-bit ports, and line reads from 8, 16-, and 32-bit ports."]
    _1 = 1,
}
impl From<Bstr> for bool {
    #[inline(always)]
    fn from(variant: Bstr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSTR` reader - Burst-read enable"]
pub type BstrR = crate::BitReader<Bstr>;
impl BstrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bstr {
        match self.bits {
            false => Bstr::_0,
            true => Bstr::_1,
        }
    }
    #[doc = "Data exceeding the specified port size is broken into individual, port-sized, non-burst reads. For example, a longword read from an 8-bit port is broken into four 8-bit reads."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bstr::_0
    }
    #[doc = "Enables data burst reads larger than the specified port size, including longword reads from 8- and 16-bit ports, word reads from 8-bit ports, and line reads from 8, 16-, and 32-bit ports."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bstr::_1
    }
}
#[doc = "Field `BSTR` writer - Burst-read enable"]
pub type BstrW<'a, REG> = crate::BitWriter<'a, REG, Bstr>;
impl<'a, REG> BstrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data exceeding the specified port size is broken into individual, port-sized, non-burst reads. For example, a longword read from an 8-bit port is broken into four 8-bit reads."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bstr::_0)
    }
    #[doc = "Enables data burst reads larger than the specified port size, including longword reads from 8- and 16-bit ports, word reads from 8-bit ports, and line reads from 8, 16-, and 32-bit ports."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bstr::_1)
    }
}
#[doc = "Byte-enable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bem {
    #[doc = "0: The FB_BE n signals are not asserted for reads. The FB_BE n signals are asserted for data write only."]
    _0 = 0,
    #[doc = "1: The FB_BE n signals are asserted for read and write accesses"]
    _1 = 1,
}
impl From<Bem> for bool {
    #[inline(always)]
    fn from(variant: Bem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEM` reader - Byte-enable mode"]
pub type BemR = crate::BitReader<Bem>;
impl BemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bem {
        match self.bits {
            false => Bem::_0,
            true => Bem::_1,
        }
    }
    #[doc = "The FB_BE n signals are not asserted for reads. The FB_BE n signals are asserted for data write only."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bem::_0
    }
    #[doc = "The FB_BE n signals are asserted for read and write accesses"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bem::_1
    }
}
#[doc = "Field `BEM` writer - Byte-enable mode"]
pub type BemW<'a, REG> = crate::BitWriter<'a, REG, Bem>;
impl<'a, REG> BemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The FB_BE n signals are not asserted for reads. The FB_BE n signals are asserted for data write only."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bem::_0)
    }
    #[doc = "The FB_BE n signals are asserted for read and write accesses"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bem::_1)
    }
}
#[doc = "Port size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ps {
    #[doc = "0: 32-bit port size. Valid data sampled and driven on FB_D\\[31:0\\]"]
    _00 = 0,
    #[doc = "1: 8-bit port size. Valid data sampled and driven on FB_D\\[31:24\\]
if BLS = 0 or FB_D\\[7:0\\]
if BLS = 1"]
    _01 = 1,
    #[doc = "2: 16-bit port size. Valid data sampled and driven on FB_D\\[31:16\\]
if BLS = 0 or FB_D\\[15:0\\]
if BLS = 1"]
    _10 = 2,
    #[doc = "3: 16-bit port size. Valid data sampled and driven on FB_D\\[31:16\\]
if BLS = 0 or FB_D\\[15:0\\]
if BLS = 1"]
    _11 = 3,
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
#[doc = "Field `PS` reader - Port size"]
pub type PsR = crate::FieldReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            0 => Ps::_00,
            1 => Ps::_01,
            2 => Ps::_10,
            3 => Ps::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "32-bit port size. Valid data sampled and driven on FB_D\\[31:0\\]"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ps::_00
    }
    #[doc = "8-bit port size. Valid data sampled and driven on FB_D\\[31:24\\]
if BLS = 0 or FB_D\\[7:0\\]
if BLS = 1"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ps::_01
    }
    #[doc = "16-bit port size. Valid data sampled and driven on FB_D\\[31:16\\]
if BLS = 0 or FB_D\\[15:0\\]
if BLS = 1"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ps::_10
    }
    #[doc = "16-bit port size. Valid data sampled and driven on FB_D\\[31:16\\]
if BLS = 0 or FB_D\\[15:0\\]
if BLS = 1"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Ps::_11
    }
}
#[doc = "Field `PS` writer - Port size"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ps, crate::Safe>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-bit port size. Valid data sampled and driven on FB_D\\[31:0\\]"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_00)
    }
    #[doc = "8-bit port size. Valid data sampled and driven on FB_D\\[31:24\\]
if BLS = 0 or FB_D\\[7:0\\]
if BLS = 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_01)
    }
    #[doc = "16-bit port size. Valid data sampled and driven on FB_D\\[31:16\\]
if BLS = 0 or FB_D\\[15:0\\]
if BLS = 1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_10)
    }
    #[doc = "16-bit port size. Valid data sampled and driven on FB_D\\[31:16\\]
if BLS = 0 or FB_D\\[15:0\\]
if BLS = 1"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::_11)
    }
}
#[doc = "Auto-acknowledge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aa {
    #[doc = "0: No internal FB_TA is asserted. Cycle is terminated externally"]
    _0 = 0,
    #[doc = "1: Internal transfer acknowledge is asserted as specified by WS"]
    _1 = 1,
}
impl From<Aa> for bool {
    #[inline(always)]
    fn from(variant: Aa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AA` reader - Auto-acknowledge enable"]
pub type AaR = crate::BitReader<Aa>;
impl AaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aa {
        match self.bits {
            false => Aa::_0,
            true => Aa::_1,
        }
    }
    #[doc = "No internal FB_TA is asserted. Cycle is terminated externally"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aa::_0
    }
    #[doc = "Internal transfer acknowledge is asserted as specified by WS"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aa::_1
    }
}
#[doc = "Field `AA` writer - Auto-acknowledge enable"]
pub type AaW<'a, REG> = crate::BitWriter<'a, REG, Aa>;
impl<'a, REG> AaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No internal FB_TA is asserted. Cycle is terminated externally"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aa::_0)
    }
    #[doc = "Internal transfer acknowledge is asserted as specified by WS"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aa::_1)
    }
}
#[doc = "Byte-lane shift\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bls {
    #[doc = "0: Not shifted. Data is left-justfied on FB_AD."]
    _0 = 0,
    #[doc = "1: Shifted. Data is right justified on FB_AD."]
    _1 = 1,
}
impl From<Bls> for bool {
    #[inline(always)]
    fn from(variant: Bls) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLS` reader - Byte-lane shift"]
pub type BlsR = crate::BitReader<Bls>;
impl BlsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bls {
        match self.bits {
            false => Bls::_0,
            true => Bls::_1,
        }
    }
    #[doc = "Not shifted. Data is left-justfied on FB_AD."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bls::_0
    }
    #[doc = "Shifted. Data is right justified on FB_AD."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bls::_1
    }
}
#[doc = "Field `BLS` writer - Byte-lane shift"]
pub type BlsW<'a, REG> = crate::BitWriter<'a, REG, Bls>;
impl<'a, REG> BlsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not shifted. Data is left-justfied on FB_AD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bls::_0)
    }
    #[doc = "Shifted. Data is right justified on FB_AD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bls::_1)
    }
}
#[doc = "Field `WS` reader - Wait states"]
pub type WsR = crate::FieldReader;
#[doc = "Field `WS` writer - Wait states"]
pub type WsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Write address hold or deselect\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wrah {
    #[doc = "0: Hold address and attributes one cycle after FB_CSn negates on writes. (Default FB_CSn)"]
    _00 = 0,
    #[doc = "1: Hold address and attributes two cycles after FB_CSn negates on writes."]
    _01 = 1,
    #[doc = "2: Hold address and attributes three cycles after FB_CSn negates on writes."]
    _10 = 2,
    #[doc = "3: Hold address and attributes four cycles after FB_CSn negates on writes. (Default FB_CS0)"]
    _11 = 3,
}
impl From<Wrah> for u8 {
    #[inline(always)]
    fn from(variant: Wrah) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wrah {
    type Ux = u8;
}
impl crate::IsEnum for Wrah {}
#[doc = "Field `WRAH` reader - Write address hold or deselect"]
pub type WrahR = crate::FieldReader<Wrah>;
impl WrahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrah {
        match self.bits {
            0 => Wrah::_00,
            1 => Wrah::_01,
            2 => Wrah::_10,
            3 => Wrah::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Hold address and attributes one cycle after FB_CSn negates on writes. (Default FB_CSn)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Wrah::_00
    }
    #[doc = "Hold address and attributes two cycles after FB_CSn negates on writes."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Wrah::_01
    }
    #[doc = "Hold address and attributes three cycles after FB_CSn negates on writes."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Wrah::_10
    }
    #[doc = "Hold address and attributes four cycles after FB_CSn negates on writes. (Default FB_CS0)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Wrah::_11
    }
}
#[doc = "Field `WRAH` writer - Write address hold or deselect"]
pub type WrahW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wrah, crate::Safe>;
impl<'a, REG> WrahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Hold address and attributes one cycle after FB_CSn negates on writes. (Default FB_CSn)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Wrah::_00)
    }
    #[doc = "Hold address and attributes two cycles after FB_CSn negates on writes."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Wrah::_01)
    }
    #[doc = "Hold address and attributes three cycles after FB_CSn negates on writes."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Wrah::_10)
    }
    #[doc = "Hold address and attributes four cycles after FB_CSn negates on writes. (Default FB_CS0)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Wrah::_11)
    }
}
#[doc = "Read address hold or deselect\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rdah {
    #[doc = "0: If AA is cleared, 1 cycle. If AA is set, 0 cycles."]
    _00 = 0,
    #[doc = "1: If AA is cleared, 2 cycles. If AA is set, 1 cycle."]
    _01 = 1,
    #[doc = "2: If AA is cleared, 3 cycles. If AA is set, 2 cycles."]
    _10 = 2,
    #[doc = "3: If AA is cleared, 4 cycles. If AA is set, 3 cycles."]
    _11 = 3,
}
impl From<Rdah> for u8 {
    #[inline(always)]
    fn from(variant: Rdah) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rdah {
    type Ux = u8;
}
impl crate::IsEnum for Rdah {}
#[doc = "Field `RDAH` reader - Read address hold or deselect"]
pub type RdahR = crate::FieldReader<Rdah>;
impl RdahR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdah {
        match self.bits {
            0 => Rdah::_00,
            1 => Rdah::_01,
            2 => Rdah::_10,
            3 => Rdah::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "If AA is cleared, 1 cycle. If AA is set, 0 cycles."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Rdah::_00
    }
    #[doc = "If AA is cleared, 2 cycles. If AA is set, 1 cycle."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Rdah::_01
    }
    #[doc = "If AA is cleared, 3 cycles. If AA is set, 2 cycles."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rdah::_10
    }
    #[doc = "If AA is cleared, 4 cycles. If AA is set, 3 cycles."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Rdah::_11
    }
}
#[doc = "Field `RDAH` writer - Read address hold or deselect"]
pub type RdahW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rdah, crate::Safe>;
impl<'a, REG> RdahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "If AA is cleared, 1 cycle. If AA is set, 0 cycles."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Rdah::_00)
    }
    #[doc = "If AA is cleared, 2 cycles. If AA is set, 1 cycle."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Rdah::_01)
    }
    #[doc = "If AA is cleared, 3 cycles. If AA is set, 2 cycles."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rdah::_10)
    }
    #[doc = "If AA is cleared, 4 cycles. If AA is set, 3 cycles."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Rdah::_11)
    }
}
#[doc = "Address setup\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aset {
    #[doc = "0: Assert FB_CSn on first rising clock edge after address is asserted. (Default FB_CSn)"]
    _00 = 0,
    #[doc = "1: Assert FB_CSn on second rising clock edge after address is asserted."]
    _01 = 1,
    #[doc = "2: Assert FB_CSn on third rising clock edge after address is asserted."]
    _10 = 2,
    #[doc = "3: Assert FB_CSn on fourth rising clock edge after address is asserted. (Default FB_CS0)"]
    _11 = 3,
}
impl From<Aset> for u8 {
    #[inline(always)]
    fn from(variant: Aset) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aset {
    type Ux = u8;
}
impl crate::IsEnum for Aset {}
#[doc = "Field `ASET` reader - Address setup"]
pub type AsetR = crate::FieldReader<Aset>;
impl AsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aset {
        match self.bits {
            0 => Aset::_00,
            1 => Aset::_01,
            2 => Aset::_10,
            3 => Aset::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Assert FB_CSn on first rising clock edge after address is asserted. (Default FB_CSn)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Aset::_00
    }
    #[doc = "Assert FB_CSn on second rising clock edge after address is asserted."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Aset::_01
    }
    #[doc = "Assert FB_CSn on third rising clock edge after address is asserted."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Aset::_10
    }
    #[doc = "Assert FB_CSn on fourth rising clock edge after address is asserted. (Default FB_CS0)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Aset::_11
    }
}
#[doc = "Field `ASET` writer - Address setup"]
pub type AsetW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aset, crate::Safe>;
impl<'a, REG> AsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Assert FB_CSn on first rising clock edge after address is asserted. (Default FB_CSn)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Aset::_00)
    }
    #[doc = "Assert FB_CSn on second rising clock edge after address is asserted."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Aset::_01)
    }
    #[doc = "Assert FB_CSn on third rising clock edge after address is asserted."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Aset::_10)
    }
    #[doc = "Assert FB_CSn on fourth rising clock edge after address is asserted. (Default FB_CS0)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Aset::_11)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exts {
    #[doc = "0: FB_TS /FB_ALE asserts for one bus clock cycle"]
    _0 = 0,
    #[doc = "1: FB_TS /FB_ALE remains asserted until the first positive clock edge after FB_CSn asserts"]
    _1 = 1,
}
impl From<Exts> for bool {
    #[inline(always)]
    fn from(variant: Exts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTS` reader - no description available"]
pub type ExtsR = crate::BitReader<Exts>;
impl ExtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exts {
        match self.bits {
            false => Exts::_0,
            true => Exts::_1,
        }
    }
    #[doc = "FB_TS /FB_ALE asserts for one bus clock cycle"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Exts::_0
    }
    #[doc = "FB_TS /FB_ALE remains asserted until the first positive clock edge after FB_CSn asserts"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Exts::_1
    }
}
#[doc = "Field `EXTS` writer - no description available"]
pub type ExtsW<'a, REG> = crate::BitWriter<'a, REG, Exts>;
impl<'a, REG> ExtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FB_TS /FB_ALE asserts for one bus clock cycle"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Exts::_0)
    }
    #[doc = "FB_TS /FB_ALE remains asserted until the first positive clock edge after FB_CSn asserts"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Exts::_1)
    }
}
#[doc = "Secondary wait state enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swsen {
    #[doc = "0: The WS value inserts wait states before an internal transfer acknowledge is generated for all transfers"]
    _0 = 0,
    #[doc = "1: The SWS value inserts wait states before an internal transfer acknowledge is generated for burst transfer secondary terminations"]
    _1 = 1,
}
impl From<Swsen> for bool {
    #[inline(always)]
    fn from(variant: Swsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWSEN` reader - Secondary wait state enable"]
pub type SwsenR = crate::BitReader<Swsen>;
impl SwsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swsen {
        match self.bits {
            false => Swsen::_0,
            true => Swsen::_1,
        }
    }
    #[doc = "The WS value inserts wait states before an internal transfer acknowledge is generated for all transfers"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Swsen::_0
    }
    #[doc = "The SWS value inserts wait states before an internal transfer acknowledge is generated for burst transfer secondary terminations"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Swsen::_1
    }
}
#[doc = "Field `SWSEN` writer - Secondary wait state enable"]
pub type SwsenW<'a, REG> = crate::BitWriter<'a, REG, Swsen>;
impl<'a, REG> SwsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The WS value inserts wait states before an internal transfer acknowledge is generated for all transfers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Swsen::_0)
    }
    #[doc = "The SWS value inserts wait states before an internal transfer acknowledge is generated for burst transfer secondary terminations"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Swsen::_1)
    }
}
#[doc = "Field `SWS` reader - Secondary wait states"]
pub type SwsR = crate::FieldReader;
#[doc = "Field `SWS` writer - Secondary wait states"]
pub type SwsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 3 - Burst-write enable"]
    #[inline(always)]
    pub fn bstw(&self) -> BstwR {
        BstwR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Burst-read enable"]
    #[inline(always)]
    pub fn bstr(&self) -> BstrR {
        BstrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Byte-enable mode"]
    #[inline(always)]
    pub fn bem(&self) -> BemR {
        BemR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Port size"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Auto-acknowledge enable"]
    #[inline(always)]
    pub fn aa(&self) -> AaR {
        AaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Byte-lane shift"]
    #[inline(always)]
    pub fn bls(&self) -> BlsR {
        BlsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - Wait states"]
    #[inline(always)]
    pub fn ws(&self) -> WsR {
        WsR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - Write address hold or deselect"]
    #[inline(always)]
    pub fn wrah(&self) -> WrahR {
        WrahR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Read address hold or deselect"]
    #[inline(always)]
    pub fn rdah(&self) -> RdahR {
        RdahR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Address setup"]
    #[inline(always)]
    pub fn aset(&self) -> AsetR {
        AsetR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn exts(&self) -> ExtsR {
        ExtsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Secondary wait state enable"]
    #[inline(always)]
    pub fn swsen(&self) -> SwsenR {
        SwsenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:31 - Secondary wait states"]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Burst-write enable"]
    #[inline(always)]
    #[must_use]
    pub fn bstw(&mut self) -> BstwW<CscrSpec> {
        BstwW::new(self, 3)
    }
    #[doc = "Bit 4 - Burst-read enable"]
    #[inline(always)]
    #[must_use]
    pub fn bstr(&mut self) -> BstrW<CscrSpec> {
        BstrW::new(self, 4)
    }
    #[doc = "Bit 5 - Byte-enable mode"]
    #[inline(always)]
    #[must_use]
    pub fn bem(&mut self) -> BemW<CscrSpec> {
        BemW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Port size"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<CscrSpec> {
        PsW::new(self, 6)
    }
    #[doc = "Bit 8 - Auto-acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn aa(&mut self) -> AaW<CscrSpec> {
        AaW::new(self, 8)
    }
    #[doc = "Bit 9 - Byte-lane shift"]
    #[inline(always)]
    #[must_use]
    pub fn bls(&mut self) -> BlsW<CscrSpec> {
        BlsW::new(self, 9)
    }
    #[doc = "Bits 10:15 - Wait states"]
    #[inline(always)]
    #[must_use]
    pub fn ws(&mut self) -> WsW<CscrSpec> {
        WsW::new(self, 10)
    }
    #[doc = "Bits 16:17 - Write address hold or deselect"]
    #[inline(always)]
    #[must_use]
    pub fn wrah(&mut self) -> WrahW<CscrSpec> {
        WrahW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Read address hold or deselect"]
    #[inline(always)]
    #[must_use]
    pub fn rdah(&mut self) -> RdahW<CscrSpec> {
        RdahW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Address setup"]
    #[inline(always)]
    #[must_use]
    pub fn aset(&mut self) -> AsetW<CscrSpec> {
        AsetW::new(self, 20)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn exts(&mut self) -> ExtsW<CscrSpec> {
        ExtsW::new(self, 22)
    }
    #[doc = "Bit 23 - Secondary wait state enable"]
    #[inline(always)]
    #[must_use]
    pub fn swsen(&mut self) -> SwsenW<CscrSpec> {
        SwsenW::new(self, 23)
    }
    #[doc = "Bits 26:31 - Secondary wait states"]
    #[inline(always)]
    #[must_use]
    pub fn sws(&mut self) -> SwsW<CscrSpec> {
        SwsW::new(self, 26)
    }
}
#[doc = "Chip select control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscrSpec;
impl crate::RegisterSpec for CscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cscr::R`](R) reader structure"]
impl crate::Readable for CscrSpec {}
#[doc = "`write(|w| ..)` method takes [`cscr::W`](W) writer structure"]
impl crate::Writable for CscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSCR%s to value 0x003f_fc00"]
impl crate::Resettable for CscrSpec {
    const RESET_VALUE: u32 = 0x003f_fc00;
}
