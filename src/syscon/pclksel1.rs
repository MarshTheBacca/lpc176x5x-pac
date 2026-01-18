#[doc = "Register `PCLKSEL1` reader"]
pub type R = crate::R<Pclksel1Spec>;
#[doc = "Register `PCLKSEL1` writer"]
pub type W = crate::W<Pclksel1Spec>;
#[doc = "Peripheral clock selection for the Quadrature Encoder Interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkQeiEnum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkQeiEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkQeiEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkQeiEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkQeiEnum {}
#[doc = "Field `PCLK_QEI` reader - Peripheral clock selection for the Quadrature Encoder Interface."]
pub type PclkQeiR = crate::FieldReader<SysconPclksel1PclkQeiEnum>;
impl PclkQeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkQeiEnum {
        match self.bits {
            0 => SysconPclksel1PclkQeiEnum::CclkDiv4,
            1 => SysconPclksel1PclkQeiEnum::Cclk,
            2 => SysconPclksel1PclkQeiEnum::CclkDiv2,
            3 => SysconPclksel1PclkQeiEnum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkQeiEnum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkQeiEnum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkQeiEnum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkQeiEnum::CclkDiv8
    }
}
#[doc = "Field `PCLK_QEI` writer - Peripheral clock selection for the Quadrature Encoder Interface."]
pub type PclkQeiW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkQeiEnum, crate::Safe>;
impl<'a, REG> PclkQeiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkQeiEnum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkQeiEnum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkQeiEnum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkQeiEnum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for GPIO interrupts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkGpiointEnum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkGpiointEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkGpiointEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkGpiointEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkGpiointEnum {}
#[doc = "Field `PCLK_GPIOINT` reader - Peripheral clock selection for GPIO interrupts."]
pub type PclkGpiointR = crate::FieldReader<SysconPclksel1PclkGpiointEnum>;
impl PclkGpiointR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkGpiointEnum {
        match self.bits {
            0 => SysconPclksel1PclkGpiointEnum::CclkDiv4,
            1 => SysconPclksel1PclkGpiointEnum::Cclk,
            2 => SysconPclksel1PclkGpiointEnum::CclkDiv2,
            3 => SysconPclksel1PclkGpiointEnum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkGpiointEnum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkGpiointEnum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkGpiointEnum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkGpiointEnum::CclkDiv8
    }
}
#[doc = "Field `PCLK_GPIOINT` writer - Peripheral clock selection for GPIO interrupts."]
pub type PclkGpiointW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkGpiointEnum, crate::Safe>;
impl<'a, REG> PclkGpiointW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkGpiointEnum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkGpiointEnum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkGpiointEnum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkGpiointEnum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for the Pin Connect block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkPcbEnum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkPcbEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkPcbEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkPcbEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkPcbEnum {}
#[doc = "Field `PCLK_PCB` reader - Peripheral clock selection for the Pin Connect block."]
pub type PclkPcbR = crate::FieldReader<SysconPclksel1PclkPcbEnum>;
impl PclkPcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkPcbEnum {
        match self.bits {
            0 => SysconPclksel1PclkPcbEnum::CclkDiv4,
            1 => SysconPclksel1PclkPcbEnum::Cclk,
            2 => SysconPclksel1PclkPcbEnum::CclkDiv2,
            3 => SysconPclksel1PclkPcbEnum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkPcbEnum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkPcbEnum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkPcbEnum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkPcbEnum::CclkDiv8
    }
}
#[doc = "Field `PCLK_PCB` writer - Peripheral clock selection for the Pin Connect block."]
pub type PclkPcbW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkPcbEnum, crate::Safe>;
impl<'a, REG> PclkPcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkPcbEnum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkPcbEnum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkPcbEnum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkPcbEnum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for I2C1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkI2c1Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkI2c1Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkI2c1Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkI2c1Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkI2c1Enum {}
#[doc = "Field `PCLK_I2C1` reader - Peripheral clock selection for I2C1."]
pub type PclkI2c1R = crate::FieldReader<SysconPclksel1PclkI2c1Enum>;
impl PclkI2c1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkI2c1Enum {
        match self.bits {
            0 => SysconPclksel1PclkI2c1Enum::CclkDiv4,
            1 => SysconPclksel1PclkI2c1Enum::Cclk,
            2 => SysconPclksel1PclkI2c1Enum::CclkDiv2,
            3 => SysconPclksel1PclkI2c1Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkI2c1Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkI2c1Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkI2c1Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkI2c1Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_I2C1` writer - Peripheral clock selection for I2C1."]
pub type PclkI2c1W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkI2c1Enum, crate::Safe>;
impl<'a, REG> PclkI2c1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkI2c1Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkI2c1Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkI2c1Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkI2c1Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for SSP0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkSsp0Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkSsp0Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkSsp0Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkSsp0Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkSsp0Enum {}
#[doc = "Field `PCLK_SSP0` reader - Peripheral clock selection for SSP0."]
pub type PclkSsp0R = crate::FieldReader<SysconPclksel1PclkSsp0Enum>;
impl PclkSsp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkSsp0Enum {
        match self.bits {
            0 => SysconPclksel1PclkSsp0Enum::CclkDiv4,
            1 => SysconPclksel1PclkSsp0Enum::Cclk,
            2 => SysconPclksel1PclkSsp0Enum::CclkDiv2,
            3 => SysconPclksel1PclkSsp0Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkSsp0Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkSsp0Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkSsp0Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkSsp0Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_SSP0` writer - Peripheral clock selection for SSP0."]
pub type PclkSsp0W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkSsp0Enum, crate::Safe>;
impl<'a, REG> PclkSsp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkSsp0Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkSsp0Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkSsp0Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkSsp0Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for TIMER2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkTimer2Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkTimer2Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkTimer2Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkTimer2Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkTimer2Enum {}
#[doc = "Field `PCLK_TIMER2` reader - Peripheral clock selection for TIMER2."]
pub type PclkTimer2R = crate::FieldReader<SysconPclksel1PclkTimer2Enum>;
impl PclkTimer2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkTimer2Enum {
        match self.bits {
            0 => SysconPclksel1PclkTimer2Enum::CclkDiv4,
            1 => SysconPclksel1PclkTimer2Enum::Cclk,
            2 => SysconPclksel1PclkTimer2Enum::CclkDiv2,
            3 => SysconPclksel1PclkTimer2Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkTimer2Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkTimer2Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkTimer2Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkTimer2Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_TIMER2` writer - Peripheral clock selection for TIMER2."]
pub type PclkTimer2W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkTimer2Enum, crate::Safe>;
impl<'a, REG> PclkTimer2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkTimer2Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkTimer2Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkTimer2Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkTimer2Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for TIMER3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkTimer3Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkTimer3Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkTimer3Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkTimer3Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkTimer3Enum {}
#[doc = "Field `PCLK_TIMER3` reader - Peripheral clock selection for TIMER3."]
pub type PclkTimer3R = crate::FieldReader<SysconPclksel1PclkTimer3Enum>;
impl PclkTimer3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkTimer3Enum {
        match self.bits {
            0 => SysconPclksel1PclkTimer3Enum::CclkDiv4,
            1 => SysconPclksel1PclkTimer3Enum::Cclk,
            2 => SysconPclksel1PclkTimer3Enum::CclkDiv2,
            3 => SysconPclksel1PclkTimer3Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkTimer3Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkTimer3Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkTimer3Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkTimer3Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_TIMER3` writer - Peripheral clock selection for TIMER3."]
pub type PclkTimer3W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkTimer3Enum, crate::Safe>;
impl<'a, REG> PclkTimer3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkTimer3Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkTimer3Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkTimer3Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkTimer3Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for UART2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkUart2Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkUart2Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkUart2Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkUart2Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkUart2Enum {}
#[doc = "Field `PCLK_UART2` reader - Peripheral clock selection for UART2."]
pub type PclkUart2R = crate::FieldReader<SysconPclksel1PclkUart2Enum>;
impl PclkUart2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkUart2Enum {
        match self.bits {
            0 => SysconPclksel1PclkUart2Enum::CclkDiv4,
            1 => SysconPclksel1PclkUart2Enum::Cclk,
            2 => SysconPclksel1PclkUart2Enum::CclkDiv2,
            3 => SysconPclksel1PclkUart2Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkUart2Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkUart2Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkUart2Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkUart2Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_UART2` writer - Peripheral clock selection for UART2."]
pub type PclkUart2W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkUart2Enum, crate::Safe>;
impl<'a, REG> PclkUart2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkUart2Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkUart2Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkUart2Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkUart2Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for UART3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkUart3Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkUart3Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkUart3Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkUart3Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkUart3Enum {}
#[doc = "Field `PCLK_UART3` reader - Peripheral clock selection for UART3."]
pub type PclkUart3R = crate::FieldReader<SysconPclksel1PclkUart3Enum>;
impl PclkUart3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkUart3Enum {
        match self.bits {
            0 => SysconPclksel1PclkUart3Enum::CclkDiv4,
            1 => SysconPclksel1PclkUart3Enum::Cclk,
            2 => SysconPclksel1PclkUart3Enum::CclkDiv2,
            3 => SysconPclksel1PclkUart3Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkUart3Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkUart3Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkUart3Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkUart3Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_UART3` writer - Peripheral clock selection for UART3."]
pub type PclkUart3W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkUart3Enum, crate::Safe>;
impl<'a, REG> PclkUart3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkUart3Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkUart3Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkUart3Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkUart3Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for I2C2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkI2c2Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkI2c2Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkI2c2Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkI2c2Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkI2c2Enum {}
#[doc = "Field `PCLK_I2C2` reader - Peripheral clock selection for I2C2."]
pub type PclkI2c2R = crate::FieldReader<SysconPclksel1PclkI2c2Enum>;
impl PclkI2c2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkI2c2Enum {
        match self.bits {
            0 => SysconPclksel1PclkI2c2Enum::CclkDiv4,
            1 => SysconPclksel1PclkI2c2Enum::Cclk,
            2 => SysconPclksel1PclkI2c2Enum::CclkDiv2,
            3 => SysconPclksel1PclkI2c2Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkI2c2Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkI2c2Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkI2c2Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkI2c2Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_I2C2` writer - Peripheral clock selection for I2C2."]
pub type PclkI2c2W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkI2c2Enum, crate::Safe>;
impl<'a, REG> PclkI2c2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkI2c2Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkI2c2Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkI2c2Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkI2c2Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for I2S.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkI2sEnum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkI2sEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkI2sEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkI2sEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkI2sEnum {}
#[doc = "Field `PCLK_I2S` reader - Peripheral clock selection for I2S."]
pub type PclkI2sR = crate::FieldReader<SysconPclksel1PclkI2sEnum>;
impl PclkI2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkI2sEnum {
        match self.bits {
            0 => SysconPclksel1PclkI2sEnum::CclkDiv4,
            1 => SysconPclksel1PclkI2sEnum::Cclk,
            2 => SysconPclksel1PclkI2sEnum::CclkDiv2,
            3 => SysconPclksel1PclkI2sEnum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkI2sEnum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkI2sEnum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkI2sEnum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkI2sEnum::CclkDiv8
    }
}
#[doc = "Field `PCLK_I2S` writer - Peripheral clock selection for I2S."]
pub type PclkI2sW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkI2sEnum, crate::Safe>;
impl<'a, REG> PclkI2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkI2sEnum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkI2sEnum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkI2sEnum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkI2sEnum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for Repetitive Interrupt Timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkRitEnum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkRitEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkRitEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkRitEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkRitEnum {}
#[doc = "Field `PCLK_RIT` reader - Peripheral clock selection for Repetitive Interrupt Timer."]
pub type PclkRitR = crate::FieldReader<SysconPclksel1PclkRitEnum>;
impl PclkRitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkRitEnum {
        match self.bits {
            0 => SysconPclksel1PclkRitEnum::CclkDiv4,
            1 => SysconPclksel1PclkRitEnum::Cclk,
            2 => SysconPclksel1PclkRitEnum::CclkDiv2,
            3 => SysconPclksel1PclkRitEnum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkRitEnum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkRitEnum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkRitEnum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkRitEnum::CclkDiv8
    }
}
#[doc = "Field `PCLK_RIT` writer - Peripheral clock selection for Repetitive Interrupt Timer."]
pub type PclkRitW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkRitEnum, crate::Safe>;
impl<'a, REG> PclkRitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkRitEnum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkRitEnum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkRitEnum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkRitEnum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for the System Control block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkSysconEnum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkSysconEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkSysconEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkSysconEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkSysconEnum {}
#[doc = "Field `PCLK_SYSCON` reader - Peripheral clock selection for the System Control block."]
pub type PclkSysconR = crate::FieldReader<SysconPclksel1PclkSysconEnum>;
impl PclkSysconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkSysconEnum {
        match self.bits {
            0 => SysconPclksel1PclkSysconEnum::CclkDiv4,
            1 => SysconPclksel1PclkSysconEnum::Cclk,
            2 => SysconPclksel1PclkSysconEnum::CclkDiv2,
            3 => SysconPclksel1PclkSysconEnum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkSysconEnum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkSysconEnum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkSysconEnum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkSysconEnum::CclkDiv8
    }
}
#[doc = "Field `PCLK_SYSCON` writer - Peripheral clock selection for the System Control block."]
pub type PclkSysconW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkSysconEnum, crate::Safe>;
impl<'a, REG> PclkSysconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkSysconEnum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkSysconEnum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkSysconEnum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkSysconEnum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for the Motor Control PWM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel1PclkMcEnum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel1PclkMcEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel1PclkMcEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel1PclkMcEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel1PclkMcEnum {}
#[doc = "Field `PCLK_MC` reader - Peripheral clock selection for the Motor Control PWM."]
pub type PclkMcR = crate::FieldReader<SysconPclksel1PclkMcEnum>;
impl PclkMcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel1PclkMcEnum {
        match self.bits {
            0 => SysconPclksel1PclkMcEnum::CclkDiv4,
            1 => SysconPclksel1PclkMcEnum::Cclk,
            2 => SysconPclksel1PclkMcEnum::CclkDiv2,
            3 => SysconPclksel1PclkMcEnum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel1PclkMcEnum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel1PclkMcEnum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel1PclkMcEnum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel1PclkMcEnum::CclkDiv8
    }
}
#[doc = "Field `PCLK_MC` writer - Peripheral clock selection for the Motor Control PWM."]
pub type PclkMcW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysconPclksel1PclkMcEnum, crate::Safe>;
impl<'a, REG> PclkMcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkMcEnum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkMcEnum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkMcEnum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel1PclkMcEnum::CclkDiv8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Peripheral clock selection for the Quadrature Encoder Interface."]
    #[inline(always)]
    pub fn pclk_qei(&self) -> PclkQeiR {
        PclkQeiR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for GPIO interrupts."]
    #[inline(always)]
    pub fn pclk_gpioint(&self) -> PclkGpiointR {
        PclkGpiointR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for the Pin Connect block."]
    #[inline(always)]
    pub fn pclk_pcb(&self) -> PclkPcbR {
        PclkPcbR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for I2C1."]
    #[inline(always)]
    pub fn pclk_i2c1(&self) -> PclkI2c1R {
        PclkI2c1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Peripheral clock selection for SSP0."]
    #[inline(always)]
    pub fn pclk_ssp0(&self) -> PclkSsp0R {
        PclkSsp0R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for TIMER2."]
    #[inline(always)]
    pub fn pclk_timer2(&self) -> PclkTimer2R {
        PclkTimer2R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for TIMER3."]
    #[inline(always)]
    pub fn pclk_timer3(&self) -> PclkTimer3R {
        PclkTimer3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for UART2."]
    #[inline(always)]
    pub fn pclk_uart2(&self) -> PclkUart2R {
        PclkUart2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Peripheral clock selection for UART3."]
    #[inline(always)]
    pub fn pclk_uart3(&self) -> PclkUart3R {
        PclkUart3R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for I2C2."]
    #[inline(always)]
    pub fn pclk_i2c2(&self) -> PclkI2c2R {
        PclkI2c2R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for I2S."]
    #[inline(always)]
    pub fn pclk_i2s(&self) -> PclkI2sR {
        PclkI2sR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn pclk_rit(&self) -> PclkRitR {
        PclkRitR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for the System Control block."]
    #[inline(always)]
    pub fn pclk_syscon(&self) -> PclkSysconR {
        PclkSysconR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for the Motor Control PWM."]
    #[inline(always)]
    pub fn pclk_mc(&self) -> PclkMcR {
        PclkMcR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Peripheral clock selection for the Quadrature Encoder Interface."]
    #[inline(always)]
    pub fn pclk_qei(&mut self) -> PclkQeiW<'_, Pclksel1Spec> {
        PclkQeiW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for GPIO interrupts."]
    #[inline(always)]
    pub fn pclk_gpioint(&mut self) -> PclkGpiointW<'_, Pclksel1Spec> {
        PclkGpiointW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for the Pin Connect block."]
    #[inline(always)]
    pub fn pclk_pcb(&mut self) -> PclkPcbW<'_, Pclksel1Spec> {
        PclkPcbW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for I2C1."]
    #[inline(always)]
    pub fn pclk_i2c1(&mut self) -> PclkI2c1W<'_, Pclksel1Spec> {
        PclkI2c1W::new(self, 6)
    }
    #[doc = "Bits 10:11 - Peripheral clock selection for SSP0."]
    #[inline(always)]
    pub fn pclk_ssp0(&mut self) -> PclkSsp0W<'_, Pclksel1Spec> {
        PclkSsp0W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for TIMER2."]
    #[inline(always)]
    pub fn pclk_timer2(&mut self) -> PclkTimer2W<'_, Pclksel1Spec> {
        PclkTimer2W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for TIMER3."]
    #[inline(always)]
    pub fn pclk_timer3(&mut self) -> PclkTimer3W<'_, Pclksel1Spec> {
        PclkTimer3W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for UART2."]
    #[inline(always)]
    pub fn pclk_uart2(&mut self) -> PclkUart2W<'_, Pclksel1Spec> {
        PclkUart2W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Peripheral clock selection for UART3."]
    #[inline(always)]
    pub fn pclk_uart3(&mut self) -> PclkUart3W<'_, Pclksel1Spec> {
        PclkUart3W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for I2C2."]
    #[inline(always)]
    pub fn pclk_i2c2(&mut self) -> PclkI2c2W<'_, Pclksel1Spec> {
        PclkI2c2W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for I2S."]
    #[inline(always)]
    pub fn pclk_i2s(&mut self) -> PclkI2sW<'_, Pclksel1Spec> {
        PclkI2sW::new(self, 22)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn pclk_rit(&mut self) -> PclkRitW<'_, Pclksel1Spec> {
        PclkRitW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for the System Control block."]
    #[inline(always)]
    pub fn pclk_syscon(&mut self) -> PclkSysconW<'_, Pclksel1Spec> {
        PclkSysconW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for the Motor Control PWM."]
    #[inline(always)]
    pub fn pclk_mc(&mut self) -> PclkMcW<'_, Pclksel1Spec> {
        PclkMcW::new(self, 30)
    }
}
#[doc = "Peripheral Clock Selection register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclksel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclksel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pclksel1Spec;
impl crate::RegisterSpec for Pclksel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclksel1::R`](R) reader structure"]
impl crate::Readable for Pclksel1Spec {}
#[doc = "`write(|w| ..)` method takes [`pclksel1::W`](W) writer structure"]
impl crate::Writable for Pclksel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCLKSEL1 to value 0"]
impl crate::Resettable for Pclksel1Spec {}
