#[doc = "Register `LCR` reader"]
pub type R = crate::R<LcrSpec>;
#[doc = "Register `LCR` writer"]
pub type W = crate::W<LcrSpec>;
#[doc = "Word Length Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart1LcrWlsEnum {
    #[doc = "0: 5-bit character length."]
    _5BitCharacterLeng = 0,
    #[doc = "1: 6-bit character length."]
    _6BitCharacterLeng = 1,
    #[doc = "2: 7-bit character length."]
    _7BitCharacterLeng = 2,
    #[doc = "3: 8-bit character length."]
    _8BitCharacterLeng = 3,
}
impl From<Uart1LcrWlsEnum> for u8 {
    #[inline(always)]
    fn from(variant: Uart1LcrWlsEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart1LcrWlsEnum {
    type Ux = u8;
}
impl crate::IsEnum for Uart1LcrWlsEnum {}
#[doc = "Field `WLS` reader - Word Length Select."]
pub type WlsR = crate::FieldReader<Uart1LcrWlsEnum>;
impl WlsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LcrWlsEnum {
        match self.bits {
            0 => Uart1LcrWlsEnum::_5BitCharacterLeng,
            1 => Uart1LcrWlsEnum::_6BitCharacterLeng,
            2 => Uart1LcrWlsEnum::_7BitCharacterLeng,
            3 => Uart1LcrWlsEnum::_8BitCharacterLeng,
            _ => unreachable!(),
        }
    }
    #[doc = "5-bit character length."]
    #[inline(always)]
    pub fn is_5_bit_character_leng(&self) -> bool {
        *self == Uart1LcrWlsEnum::_5BitCharacterLeng
    }
    #[doc = "6-bit character length."]
    #[inline(always)]
    pub fn is_6_bit_character_leng(&self) -> bool {
        *self == Uart1LcrWlsEnum::_6BitCharacterLeng
    }
    #[doc = "7-bit character length."]
    #[inline(always)]
    pub fn is_7_bit_character_leng(&self) -> bool {
        *self == Uart1LcrWlsEnum::_7BitCharacterLeng
    }
    #[doc = "8-bit character length."]
    #[inline(always)]
    pub fn is_8_bit_character_leng(&self) -> bool {
        *self == Uart1LcrWlsEnum::_8BitCharacterLeng
    }
}
#[doc = "Field `WLS` writer - Word Length Select."]
pub type WlsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart1LcrWlsEnum, crate::Safe>;
impl<'a, REG> WlsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5-bit character length."]
    #[inline(always)]
    pub fn _5_bit_character_leng(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrWlsEnum::_5BitCharacterLeng)
    }
    #[doc = "6-bit character length."]
    #[inline(always)]
    pub fn _6_bit_character_leng(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrWlsEnum::_6BitCharacterLeng)
    }
    #[doc = "7-bit character length."]
    #[inline(always)]
    pub fn _7_bit_character_leng(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrWlsEnum::_7BitCharacterLeng)
    }
    #[doc = "8-bit character length."]
    #[inline(always)]
    pub fn _8_bit_character_leng(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrWlsEnum::_8BitCharacterLeng)
    }
}
#[doc = "Stop Bit Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1LcrSbsEnum {
    #[doc = "0: 1 stop bit."]
    _1StopBit_ = 0,
    #[doc = "1: 2 stop bits (1.5 if LCR\\[1:0\\]=00)."]
    _2StopBits1_5If_ = 1,
}
impl From<Uart1LcrSbsEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1LcrSbsEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBS` reader - Stop Bit Select."]
pub type SbsR = crate::BitReader<Uart1LcrSbsEnum>;
impl SbsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LcrSbsEnum {
        match self.bits {
            false => Uart1LcrSbsEnum::_1StopBit_,
            true => Uart1LcrSbsEnum::_2StopBits1_5If_,
        }
    }
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn is_1_stop_bit_(&self) -> bool {
        *self == Uart1LcrSbsEnum::_1StopBit_
    }
    #[doc = "2 stop bits (1.5 if LCR\\[1:0\\]=00)."]
    #[inline(always)]
    pub fn is_2_stop_bits_1_5_if_(&self) -> bool {
        *self == Uart1LcrSbsEnum::_2StopBits1_5If_
    }
}
#[doc = "Field `SBS` writer - Stop Bit Select."]
pub type SbsW<'a, REG> = crate::BitWriter<'a, REG, Uart1LcrSbsEnum>;
impl<'a, REG> SbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn _1_stop_bit_(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrSbsEnum::_1StopBit_)
    }
    #[doc = "2 stop bits (1.5 if LCR\\[1:0\\]=00)."]
    #[inline(always)]
    pub fn _2_stop_bits_1_5_if_(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrSbsEnum::_2StopBits1_5If_)
    }
}
#[doc = "Parity Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1LcrPeEnum {
    #[doc = "0: Disable parity generation and checking."]
    DisableParityGener = 0,
    #[doc = "1: Enable parity generation and checking."]
    EnableParityGenera = 1,
}
impl From<Uart1LcrPeEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1LcrPeEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity Enable."]
pub type PeR = crate::BitReader<Uart1LcrPeEnum>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LcrPeEnum {
        match self.bits {
            false => Uart1LcrPeEnum::DisableParityGener,
            true => Uart1LcrPeEnum::EnableParityGenera,
        }
    }
    #[doc = "Disable parity generation and checking."]
    #[inline(always)]
    pub fn is_disable_parity_gener(&self) -> bool {
        *self == Uart1LcrPeEnum::DisableParityGener
    }
    #[doc = "Enable parity generation and checking."]
    #[inline(always)]
    pub fn is_enable_parity_genera(&self) -> bool {
        *self == Uart1LcrPeEnum::EnableParityGenera
    }
}
#[doc = "Field `PE` writer - Parity Enable."]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Uart1LcrPeEnum>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable parity generation and checking."]
    #[inline(always)]
    pub fn disable_parity_gener(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrPeEnum::DisableParityGener)
    }
    #[doc = "Enable parity generation and checking."]
    #[inline(always)]
    pub fn enable_parity_genera(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrPeEnum::EnableParityGenera)
    }
}
#[doc = "Parity Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart1LcrPsEnum {
    #[doc = "0: Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    OddParityNumberO = 0,
    #[doc = "1: Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    EvenParityNumber_ = 1,
    #[doc = "2: Forced 1 stick parity."]
    Forced1stickPar = 2,
    #[doc = "3: Forced 0 stick parity."]
    Forced0stickPar = 3,
}
impl From<Uart1LcrPsEnum> for u8 {
    #[inline(always)]
    fn from(variant: Uart1LcrPsEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart1LcrPsEnum {
    type Ux = u8;
}
impl crate::IsEnum for Uart1LcrPsEnum {}
#[doc = "Field `PS` reader - Parity Select."]
pub type PsR = crate::FieldReader<Uart1LcrPsEnum>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LcrPsEnum {
        match self.bits {
            0 => Uart1LcrPsEnum::OddParityNumberO,
            1 => Uart1LcrPsEnum::EvenParityNumber_,
            2 => Uart1LcrPsEnum::Forced1stickPar,
            3 => Uart1LcrPsEnum::Forced0stickPar,
            _ => unreachable!(),
        }
    }
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    #[inline(always)]
    pub fn is_odd_parity_number_o(&self) -> bool {
        *self == Uart1LcrPsEnum::OddParityNumberO
    }
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    #[inline(always)]
    pub fn is_even_parity_number_(&self) -> bool {
        *self == Uart1LcrPsEnum::EvenParityNumber_
    }
    #[doc = "Forced 1 stick parity."]
    #[inline(always)]
    pub fn is_forced1stick_par(&self) -> bool {
        *self == Uart1LcrPsEnum::Forced1stickPar
    }
    #[doc = "Forced 0 stick parity."]
    #[inline(always)]
    pub fn is_forced0stick_par(&self) -> bool {
        *self == Uart1LcrPsEnum::Forced0stickPar
    }
}
#[doc = "Field `PS` writer - Parity Select."]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart1LcrPsEnum, crate::Safe>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    #[inline(always)]
    pub fn odd_parity_number_o(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrPsEnum::OddParityNumberO)
    }
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    #[inline(always)]
    pub fn even_parity_number_(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrPsEnum::EvenParityNumber_)
    }
    #[doc = "Forced 1 stick parity."]
    #[inline(always)]
    pub fn forced1stick_par(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrPsEnum::Forced1stickPar)
    }
    #[doc = "Forced 0 stick parity."]
    #[inline(always)]
    pub fn forced0stick_par(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrPsEnum::Forced0stickPar)
    }
}
#[doc = "Break Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1LcrBcEnum {
    #[doc = "0: Disable break transmission."]
    DisableBreakTransm = 0,
    #[doc = "1: Enable break transmission. Output pin UART1 TXD is forced to logic 0 when LCR\\[6\\] is active high."]
    EnableBreakTransmi = 1,
}
impl From<Uart1LcrBcEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1LcrBcEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC` reader - Break Control."]
pub type BcR = crate::BitReader<Uart1LcrBcEnum>;
impl BcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LcrBcEnum {
        match self.bits {
            false => Uart1LcrBcEnum::DisableBreakTransm,
            true => Uart1LcrBcEnum::EnableBreakTransmi,
        }
    }
    #[doc = "Disable break transmission."]
    #[inline(always)]
    pub fn is_disable_break_transm(&self) -> bool {
        *self == Uart1LcrBcEnum::DisableBreakTransm
    }
    #[doc = "Enable break transmission. Output pin UART1 TXD is forced to logic 0 when LCR\\[6\\] is active high."]
    #[inline(always)]
    pub fn is_enable_break_transmi(&self) -> bool {
        *self == Uart1LcrBcEnum::EnableBreakTransmi
    }
}
#[doc = "Field `BC` writer - Break Control."]
pub type BcW<'a, REG> = crate::BitWriter<'a, REG, Uart1LcrBcEnum>;
impl<'a, REG> BcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable break transmission."]
    #[inline(always)]
    pub fn disable_break_transm(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrBcEnum::DisableBreakTransm)
    }
    #[doc = "Enable break transmission. Output pin UART1 TXD is forced to logic 0 when LCR\\[6\\] is active high."]
    #[inline(always)]
    pub fn enable_break_transmi(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrBcEnum::EnableBreakTransmi)
    }
}
#[doc = "Divisor Latch Access Bit (DLAB)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1LcrDlabEnum {
    #[doc = "0: Disable access to Divisor Latches."]
    DisableAccessToDi = 0,
    #[doc = "1: Enable access to Divisor Latches."]
    EnableAccessToDiv = 1,
}
impl From<Uart1LcrDlabEnum> for bool {
    #[inline(always)]
    fn from(variant: Uart1LcrDlabEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLAB` reader - Divisor Latch Access Bit (DLAB)"]
pub type DlabR = crate::BitReader<Uart1LcrDlabEnum>;
impl DlabR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1LcrDlabEnum {
        match self.bits {
            false => Uart1LcrDlabEnum::DisableAccessToDi,
            true => Uart1LcrDlabEnum::EnableAccessToDiv,
        }
    }
    #[doc = "Disable access to Divisor Latches."]
    #[inline(always)]
    pub fn is_disable_access_to_di(&self) -> bool {
        *self == Uart1LcrDlabEnum::DisableAccessToDi
    }
    #[doc = "Enable access to Divisor Latches."]
    #[inline(always)]
    pub fn is_enable_access_to_div(&self) -> bool {
        *self == Uart1LcrDlabEnum::EnableAccessToDiv
    }
}
#[doc = "Field `DLAB` writer - Divisor Latch Access Bit (DLAB)"]
pub type DlabW<'a, REG> = crate::BitWriter<'a, REG, Uart1LcrDlabEnum>;
impl<'a, REG> DlabW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable access to Divisor Latches."]
    #[inline(always)]
    pub fn disable_access_to_di(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrDlabEnum::DisableAccessToDi)
    }
    #[doc = "Enable access to Divisor Latches."]
    #[inline(always)]
    pub fn enable_access_to_div(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1LcrDlabEnum::EnableAccessToDiv)
    }
}
impl R {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&self) -> WlsR {
        WlsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Stop Bit Select."]
    #[inline(always)]
    pub fn sbs(&self) -> SbsR {
        SbsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Select."]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Break Control."]
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit (DLAB)"]
    #[inline(always)]
    pub fn dlab(&self) -> DlabR {
        DlabR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&mut self) -> WlsW<'_, LcrSpec> {
        WlsW::new(self, 0)
    }
    #[doc = "Bit 2 - Stop Bit Select."]
    #[inline(always)]
    pub fn sbs(&mut self) -> SbsW<'_, LcrSpec> {
        SbsW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, LcrSpec> {
        PeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Parity Select."]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, LcrSpec> {
        PsW::new(self, 4)
    }
    #[doc = "Bit 6 - Break Control."]
    #[inline(always)]
    pub fn bc(&mut self) -> BcW<'_, LcrSpec> {
        BcW::new(self, 6)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit (DLAB)"]
    #[inline(always)]
    pub fn dlab(&mut self) -> DlabW<'_, LcrSpec> {
        DlabW::new(self, 7)
    }
}
#[doc = "Line Control Register. Contains controls for frame formatting and break generation.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrSpec;
impl crate::RegisterSpec for LcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LcrSpec {}
