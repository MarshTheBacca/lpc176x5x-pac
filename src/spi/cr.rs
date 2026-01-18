#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "The SPI controller sends and receives 8 bits of data per transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiCrBitenableEnum {
    #[doc = "1: The SPI controller sends and receives the number of bits selected by bits 11:8."]
    TheSpiControllerS = 1,
}
impl From<SpiCrBitenableEnum> for bool {
    #[inline(always)]
    fn from(variant: SpiCrBitenableEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BITENABLE` reader - The SPI controller sends and receives 8 bits of data per transfer."]
pub type BitenableR = crate::BitReader<SpiCrBitenableEnum>;
impl BitenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SpiCrBitenableEnum> {
        match self.bits {
            true => Some(SpiCrBitenableEnum::TheSpiControllerS),
            _ => None,
        }
    }
    #[doc = "The SPI controller sends and receives the number of bits selected by bits 11:8."]
    #[inline(always)]
    pub fn is_the_spi_controller_s(&self) -> bool {
        *self == SpiCrBitenableEnum::TheSpiControllerS
    }
}
#[doc = "Field `BITENABLE` writer - The SPI controller sends and receives 8 bits of data per transfer."]
pub type BitenableW<'a, REG> = crate::BitWriter<'a, REG, SpiCrBitenableEnum>;
impl<'a, REG> BitenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SPI controller sends and receives the number of bits selected by bits 11:8."]
    #[inline(always)]
    pub fn the_spi_controller_s(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrBitenableEnum::TheSpiControllerS)
    }
}
#[doc = "Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiCrCphaEnum {
    #[doc = "0: Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    FirstEdge = 0,
    #[doc = "1: Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    SecondEdge = 1,
}
impl From<SpiCrCphaEnum> for bool {
    #[inline(always)]
    fn from(variant: SpiCrCphaEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
pub type CphaR = crate::BitReader<SpiCrCphaEnum>;
impl CphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpiCrCphaEnum {
        match self.bits {
            false => SpiCrCphaEnum::FirstEdge,
            true => SpiCrCphaEnum::SecondEdge,
        }
    }
    #[doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == SpiCrCphaEnum::FirstEdge
    }
    #[doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == SpiCrCphaEnum::SecondEdge
    }
}
#[doc = "Field `CPHA` writer - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, SpiCrCphaEnum>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrCphaEnum::FirstEdge)
    }
    #[doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrCphaEnum::SecondEdge)
    }
}
#[doc = "Clock polarity control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiCrCpolEnum {
    #[doc = "0: SCK is active high."]
    SckIsActiveHigh_ = 0,
    #[doc = "1: SCK is active low."]
    SckIsActiveLow_ = 1,
}
impl From<SpiCrCpolEnum> for bool {
    #[inline(always)]
    fn from(variant: SpiCrCpolEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock polarity control."]
pub type CpolR = crate::BitReader<SpiCrCpolEnum>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpiCrCpolEnum {
        match self.bits {
            false => SpiCrCpolEnum::SckIsActiveHigh_,
            true => SpiCrCpolEnum::SckIsActiveLow_,
        }
    }
    #[doc = "SCK is active high."]
    #[inline(always)]
    pub fn is_sck_is_active_high_(&self) -> bool {
        *self == SpiCrCpolEnum::SckIsActiveHigh_
    }
    #[doc = "SCK is active low."]
    #[inline(always)]
    pub fn is_sck_is_active_low_(&self) -> bool {
        *self == SpiCrCpolEnum::SckIsActiveLow_
    }
}
#[doc = "Field `CPOL` writer - Clock polarity control."]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, SpiCrCpolEnum>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCK is active high."]
    #[inline(always)]
    pub fn sck_is_active_high_(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrCpolEnum::SckIsActiveHigh_)
    }
    #[doc = "SCK is active low."]
    #[inline(always)]
    pub fn sck_is_active_low_(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrCpolEnum::SckIsActiveLow_)
    }
}
#[doc = "Master mode select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiCrMstrEnum {
    #[doc = "0: The SPI operates in Slave mode."]
    Slave = 0,
    #[doc = "1: The SPI operates in Master mode."]
    Master = 1,
}
impl From<SpiCrMstrEnum> for bool {
    #[inline(always)]
    fn from(variant: SpiCrMstrEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTR` reader - Master mode select."]
pub type MstrR = crate::BitReader<SpiCrMstrEnum>;
impl MstrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpiCrMstrEnum {
        match self.bits {
            false => SpiCrMstrEnum::Slave,
            true => SpiCrMstrEnum::Master,
        }
    }
    #[doc = "The SPI operates in Slave mode."]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == SpiCrMstrEnum::Slave
    }
    #[doc = "The SPI operates in Master mode."]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == SpiCrMstrEnum::Master
    }
}
#[doc = "Field `MSTR` writer - Master mode select."]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG, SpiCrMstrEnum>;
impl<'a, REG> MstrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SPI operates in Slave mode."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrMstrEnum::Slave)
    }
    #[doc = "The SPI operates in Master mode."]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrMstrEnum::Master)
    }
}
#[doc = "LSB First controls which direction each byte is shifted when transferred.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiCrLsbfEnum {
    #[doc = "0: SPI data is transferred MSB (bit 7) first."]
    Msb = 0,
    #[doc = "1: SPI data is transferred LSB (bit 0) first."]
    Lsb = 1,
}
impl From<SpiCrLsbfEnum> for bool {
    #[inline(always)]
    fn from(variant: SpiCrLsbfEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBF` reader - LSB First controls which direction each byte is shifted when transferred."]
pub type LsbfR = crate::BitReader<SpiCrLsbfEnum>;
impl LsbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpiCrLsbfEnum {
        match self.bits {
            false => SpiCrLsbfEnum::Msb,
            true => SpiCrLsbfEnum::Lsb,
        }
    }
    #[doc = "SPI data is transferred MSB (bit 7) first."]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == SpiCrLsbfEnum::Msb
    }
    #[doc = "SPI data is transferred LSB (bit 0) first."]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == SpiCrLsbfEnum::Lsb
    }
}
#[doc = "Field `LSBF` writer - LSB First controls which direction each byte is shifted when transferred."]
pub type LsbfW<'a, REG> = crate::BitWriter<'a, REG, SpiCrLsbfEnum>;
impl<'a, REG> LsbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI data is transferred MSB (bit 7) first."]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrLsbfEnum::Msb)
    }
    #[doc = "SPI data is transferred LSB (bit 0) first."]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrLsbfEnum::Lsb)
    }
}
#[doc = "Serial peripheral interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiCrSpieEnum {
    #[doc = "0: SPI interrupts are inhibited."]
    Intblock = 0,
    #[doc = "1: A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    Hwint = 1,
}
impl From<SpiCrSpieEnum> for bool {
    #[inline(always)]
    fn from(variant: SpiCrSpieEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIE` reader - Serial peripheral interrupt enable."]
pub type SpieR = crate::BitReader<SpiCrSpieEnum>;
impl SpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpiCrSpieEnum {
        match self.bits {
            false => SpiCrSpieEnum::Intblock,
            true => SpiCrSpieEnum::Hwint,
        }
    }
    #[doc = "SPI interrupts are inhibited."]
    #[inline(always)]
    pub fn is_intblock(&self) -> bool {
        *self == SpiCrSpieEnum::Intblock
    }
    #[doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    #[inline(always)]
    pub fn is_hwint(&self) -> bool {
        *self == SpiCrSpieEnum::Hwint
    }
}
#[doc = "Field `SPIE` writer - Serial peripheral interrupt enable."]
pub type SpieW<'a, REG> = crate::BitWriter<'a, REG, SpiCrSpieEnum>;
impl<'a, REG> SpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI interrupts are inhibited."]
    #[inline(always)]
    pub fn intblock(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrSpieEnum::Intblock)
    }
    #[doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    #[inline(always)]
    pub fn hwint(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrSpieEnum::Hwint)
    }
}
#[doc = "When bit 2 of this register is 1, this field controls the number of bits per transfer:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpiCrBitsEnum {
    #[doc = "8: 8 bits per transfer"]
    _8BitsPerTransfer = 8,
    #[doc = "9: 9 bits per transfer"]
    _9BitsPerTransfer = 9,
    #[doc = "10: 10 bits per transfer"]
    _10BitsPerTransfer = 10,
    #[doc = "11: 11 bits per transfer"]
    _11BitsPerTransfer = 11,
    #[doc = "12: 12 bits per transfer"]
    _12BitsPerTransfer = 12,
    #[doc = "13: 13 bits per transfer"]
    _13BitsPerTransfer = 13,
    #[doc = "14: 14 bits per transfer"]
    _14BitsPerTransfer = 14,
    #[doc = "15: 15 bits per transfer"]
    _15BitsPerTransfer = 15,
    #[doc = "0: 16 bits per transfer"]
    _16BitsPerTransfer = 0,
}
impl From<SpiCrBitsEnum> for u8 {
    #[inline(always)]
    fn from(variant: SpiCrBitsEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpiCrBitsEnum {
    type Ux = u8;
}
impl crate::IsEnum for SpiCrBitsEnum {}
#[doc = "Field `BITS` reader - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
pub type BitsR = crate::FieldReader<SpiCrBitsEnum>;
impl BitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SpiCrBitsEnum> {
        match self.bits {
            8 => Some(SpiCrBitsEnum::_8BitsPerTransfer),
            9 => Some(SpiCrBitsEnum::_9BitsPerTransfer),
            10 => Some(SpiCrBitsEnum::_10BitsPerTransfer),
            11 => Some(SpiCrBitsEnum::_11BitsPerTransfer),
            12 => Some(SpiCrBitsEnum::_12BitsPerTransfer),
            13 => Some(SpiCrBitsEnum::_13BitsPerTransfer),
            14 => Some(SpiCrBitsEnum::_14BitsPerTransfer),
            15 => Some(SpiCrBitsEnum::_15BitsPerTransfer),
            0 => Some(SpiCrBitsEnum::_16BitsPerTransfer),
            _ => None,
        }
    }
    #[doc = "8 bits per transfer"]
    #[inline(always)]
    pub fn is_8_bits_per_transfer(&self) -> bool {
        *self == SpiCrBitsEnum::_8BitsPerTransfer
    }
    #[doc = "9 bits per transfer"]
    #[inline(always)]
    pub fn is_9_bits_per_transfer(&self) -> bool {
        *self == SpiCrBitsEnum::_9BitsPerTransfer
    }
    #[doc = "10 bits per transfer"]
    #[inline(always)]
    pub fn is_10_bits_per_transfer(&self) -> bool {
        *self == SpiCrBitsEnum::_10BitsPerTransfer
    }
    #[doc = "11 bits per transfer"]
    #[inline(always)]
    pub fn is_11_bits_per_transfer(&self) -> bool {
        *self == SpiCrBitsEnum::_11BitsPerTransfer
    }
    #[doc = "12 bits per transfer"]
    #[inline(always)]
    pub fn is_12_bits_per_transfer(&self) -> bool {
        *self == SpiCrBitsEnum::_12BitsPerTransfer
    }
    #[doc = "13 bits per transfer"]
    #[inline(always)]
    pub fn is_13_bits_per_transfer(&self) -> bool {
        *self == SpiCrBitsEnum::_13BitsPerTransfer
    }
    #[doc = "14 bits per transfer"]
    #[inline(always)]
    pub fn is_14_bits_per_transfer(&self) -> bool {
        *self == SpiCrBitsEnum::_14BitsPerTransfer
    }
    #[doc = "15 bits per transfer"]
    #[inline(always)]
    pub fn is_15_bits_per_transfer(&self) -> bool {
        *self == SpiCrBitsEnum::_15BitsPerTransfer
    }
    #[doc = "16 bits per transfer"]
    #[inline(always)]
    pub fn is_16_bits_per_transfer(&self) -> bool {
        *self == SpiCrBitsEnum::_16BitsPerTransfer
    }
}
#[doc = "Field `BITS` writer - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
pub type BitsW<'a, REG> = crate::FieldWriter<'a, REG, 4, SpiCrBitsEnum>;
impl<'a, REG> BitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits per transfer"]
    #[inline(always)]
    pub fn _8_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrBitsEnum::_8BitsPerTransfer)
    }
    #[doc = "9 bits per transfer"]
    #[inline(always)]
    pub fn _9_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrBitsEnum::_9BitsPerTransfer)
    }
    #[doc = "10 bits per transfer"]
    #[inline(always)]
    pub fn _10_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrBitsEnum::_10BitsPerTransfer)
    }
    #[doc = "11 bits per transfer"]
    #[inline(always)]
    pub fn _11_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrBitsEnum::_11BitsPerTransfer)
    }
    #[doc = "12 bits per transfer"]
    #[inline(always)]
    pub fn _12_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrBitsEnum::_12BitsPerTransfer)
    }
    #[doc = "13 bits per transfer"]
    #[inline(always)]
    pub fn _13_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrBitsEnum::_13BitsPerTransfer)
    }
    #[doc = "14 bits per transfer"]
    #[inline(always)]
    pub fn _14_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrBitsEnum::_14BitsPerTransfer)
    }
    #[doc = "15 bits per transfer"]
    #[inline(always)]
    pub fn _15_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrBitsEnum::_15BitsPerTransfer)
    }
    #[doc = "16 bits per transfer"]
    #[inline(always)]
    pub fn _16_bits_per_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(SpiCrBitsEnum::_16BitsPerTransfer)
    }
}
impl R {
    #[doc = "Bit 2 - The SPI controller sends and receives 8 bits of data per transfer."]
    #[inline(always)]
    pub fn bitenable(&self) -> BitenableR {
        BitenableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock polarity control."]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master mode select."]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSB First controls which direction each byte is shifted when transferred."]
    #[inline(always)]
    pub fn lsbf(&self) -> LsbfR {
        LsbfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Serial peripheral interrupt enable."]
    #[inline(always)]
    pub fn spie(&self) -> SpieR {
        SpieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
    #[inline(always)]
    pub fn bits_(&self) -> BitsR {
        BitsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - The SPI controller sends and receives 8 bits of data per transfer."]
    #[inline(always)]
    pub fn bitenable(&mut self) -> BitenableW<'_, CrSpec> {
        BitenableW::new(self, 2)
    }
    #[doc = "Bit 3 - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, CrSpec> {
        CphaW::new(self, 3)
    }
    #[doc = "Bit 4 - Clock polarity control."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, CrSpec> {
        CpolW::new(self, 4)
    }
    #[doc = "Bit 5 - Master mode select."]
    #[inline(always)]
    pub fn mstr(&mut self) -> MstrW<'_, CrSpec> {
        MstrW::new(self, 5)
    }
    #[doc = "Bit 6 - LSB First controls which direction each byte is shifted when transferred."]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LsbfW<'_, CrSpec> {
        LsbfW::new(self, 6)
    }
    #[doc = "Bit 7 - Serial peripheral interrupt enable."]
    #[inline(always)]
    pub fn spie(&mut self) -> SpieW<'_, CrSpec> {
        SpieW::new(self, 7)
    }
    #[doc = "Bits 8:11 - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
    #[inline(always)]
    pub fn bits_(&mut self) -> BitsW<'_, CrSpec> {
        BitsW::new(self, 8)
    }
}
#[doc = "SPI Control Register. This register controls the operation of the SPI.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
