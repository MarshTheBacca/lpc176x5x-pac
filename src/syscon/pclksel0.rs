#[doc = "Register `PCLKSEL0` reader"]
pub type R = crate::R<Pclksel0Spec>;
#[doc = "Register `PCLKSEL0` writer"]
pub type W = crate::W<Pclksel0Spec>;
#[doc = "Peripheral clock selection for WDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkWdtEnum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel0PclkWdtEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkWdtEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkWdtEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkWdtEnum {}
#[doc = "Field `PCLK_WDT` reader - Peripheral clock selection for WDT."]
pub type PclkWdtR = crate::FieldReader<SysconPclksel0PclkWdtEnum>;
impl PclkWdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkWdtEnum {
        match self.bits {
            0 => SysconPclksel0PclkWdtEnum::CclkDiv4,
            1 => SysconPclksel0PclkWdtEnum::Cclk,
            2 => SysconPclksel0PclkWdtEnum::CclkDiv2,
            3 => SysconPclksel0PclkWdtEnum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkWdtEnum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkWdtEnum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkWdtEnum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel0PclkWdtEnum::CclkDiv8
    }
}
#[doc = "Field `PCLK_WDT` writer - Peripheral clock selection for WDT."]
pub type PclkWdtW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkWdtEnum, crate::Safe>;
impl<'a, REG> PclkWdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkWdtEnum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkWdtEnum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkWdtEnum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkWdtEnum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for TIMER0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkTimer0Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel0PclkTimer0Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkTimer0Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkTimer0Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkTimer0Enum {}
#[doc = "Field `PCLK_TIMER0` reader - Peripheral clock selection for TIMER0."]
pub type PclkTimer0R = crate::FieldReader<SysconPclksel0PclkTimer0Enum>;
impl PclkTimer0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkTimer0Enum {
        match self.bits {
            0 => SysconPclksel0PclkTimer0Enum::CclkDiv4,
            1 => SysconPclksel0PclkTimer0Enum::Cclk,
            2 => SysconPclksel0PclkTimer0Enum::CclkDiv2,
            3 => SysconPclksel0PclkTimer0Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkTimer0Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkTimer0Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkTimer0Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel0PclkTimer0Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_TIMER0` writer - Peripheral clock selection for TIMER0."]
pub type PclkTimer0W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkTimer0Enum, crate::Safe>;
impl<'a, REG> PclkTimer0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkTimer0Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkTimer0Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkTimer0Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkTimer0Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for TIMER1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkTimer1Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel0PclkTimer1Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkTimer1Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkTimer1Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkTimer1Enum {}
#[doc = "Field `PCLK_TIMER1` reader - Peripheral clock selection for TIMER1."]
pub type PclkTimer1R = crate::FieldReader<SysconPclksel0PclkTimer1Enum>;
impl PclkTimer1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkTimer1Enum {
        match self.bits {
            0 => SysconPclksel0PclkTimer1Enum::CclkDiv4,
            1 => SysconPclksel0PclkTimer1Enum::Cclk,
            2 => SysconPclksel0PclkTimer1Enum::CclkDiv2,
            3 => SysconPclksel0PclkTimer1Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkTimer1Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkTimer1Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkTimer1Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel0PclkTimer1Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_TIMER1` writer - Peripheral clock selection for TIMER1."]
pub type PclkTimer1W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkTimer1Enum, crate::Safe>;
impl<'a, REG> PclkTimer1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkTimer1Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkTimer1Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkTimer1Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkTimer1Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for UART0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkUart0Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel0PclkUart0Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkUart0Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkUart0Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkUart0Enum {}
#[doc = "Field `PCLK_UART0` reader - Peripheral clock selection for UART0."]
pub type PclkUart0R = crate::FieldReader<SysconPclksel0PclkUart0Enum>;
impl PclkUart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkUart0Enum {
        match self.bits {
            0 => SysconPclksel0PclkUart0Enum::CclkDiv4,
            1 => SysconPclksel0PclkUart0Enum::Cclk,
            2 => SysconPclksel0PclkUart0Enum::CclkDiv2,
            3 => SysconPclksel0PclkUart0Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkUart0Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkUart0Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkUart0Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel0PclkUart0Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_UART0` writer - Peripheral clock selection for UART0."]
pub type PclkUart0W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkUart0Enum, crate::Safe>;
impl<'a, REG> PclkUart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkUart0Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkUart0Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkUart0Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkUart0Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for UART1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkUart1Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel0PclkUart1Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkUart1Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkUart1Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkUart1Enum {}
#[doc = "Field `PCLK_UART1` reader - Peripheral clock selection for UART1."]
pub type PclkUart1R = crate::FieldReader<SysconPclksel0PclkUart1Enum>;
impl PclkUart1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkUart1Enum {
        match self.bits {
            0 => SysconPclksel0PclkUart1Enum::CclkDiv4,
            1 => SysconPclksel0PclkUart1Enum::Cclk,
            2 => SysconPclksel0PclkUart1Enum::CclkDiv2,
            3 => SysconPclksel0PclkUart1Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkUart1Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkUart1Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkUart1Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel0PclkUart1Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_UART1` writer - Peripheral clock selection for UART1."]
pub type PclkUart1W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkUart1Enum, crate::Safe>;
impl<'a, REG> PclkUart1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkUart1Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkUart1Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkUart1Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkUart1Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for PWM1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkPwm1Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel0PclkPwm1Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkPwm1Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkPwm1Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkPwm1Enum {}
#[doc = "Field `PCLK_PWM1` reader - Peripheral clock selection for PWM1."]
pub type PclkPwm1R = crate::FieldReader<SysconPclksel0PclkPwm1Enum>;
impl PclkPwm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkPwm1Enum {
        match self.bits {
            0 => SysconPclksel0PclkPwm1Enum::CclkDiv4,
            1 => SysconPclksel0PclkPwm1Enum::Cclk,
            2 => SysconPclksel0PclkPwm1Enum::CclkDiv2,
            3 => SysconPclksel0PclkPwm1Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkPwm1Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkPwm1Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkPwm1Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel0PclkPwm1Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_PWM1` writer - Peripheral clock selection for PWM1."]
pub type PclkPwm1W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkPwm1Enum, crate::Safe>;
impl<'a, REG> PclkPwm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkPwm1Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkPwm1Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkPwm1Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkPwm1Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for I2C0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkI2c0Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel0PclkI2c0Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkI2c0Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkI2c0Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkI2c0Enum {}
#[doc = "Field `PCLK_I2C0` reader - Peripheral clock selection for I2C0."]
pub type PclkI2c0R = crate::FieldReader<SysconPclksel0PclkI2c0Enum>;
impl PclkI2c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkI2c0Enum {
        match self.bits {
            0 => SysconPclksel0PclkI2c0Enum::CclkDiv4,
            1 => SysconPclksel0PclkI2c0Enum::Cclk,
            2 => SysconPclksel0PclkI2c0Enum::CclkDiv2,
            3 => SysconPclksel0PclkI2c0Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkI2c0Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkI2c0Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkI2c0Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel0PclkI2c0Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_I2C0` writer - Peripheral clock selection for I2C0."]
pub type PclkI2c0W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkI2c0Enum, crate::Safe>;
impl<'a, REG> PclkI2c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkI2c0Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkI2c0Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkI2c0Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkI2c0Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for SPI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkSpiEnum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel0PclkSpiEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkSpiEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkSpiEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkSpiEnum {}
#[doc = "Field `PCLK_SPI` reader - Peripheral clock selection for SPI."]
pub type PclkSpiR = crate::FieldReader<SysconPclksel0PclkSpiEnum>;
impl PclkSpiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkSpiEnum {
        match self.bits {
            0 => SysconPclksel0PclkSpiEnum::CclkDiv4,
            1 => SysconPclksel0PclkSpiEnum::Cclk,
            2 => SysconPclksel0PclkSpiEnum::CclkDiv2,
            3 => SysconPclksel0PclkSpiEnum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkSpiEnum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkSpiEnum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkSpiEnum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel0PclkSpiEnum::CclkDiv8
    }
}
#[doc = "Field `PCLK_SPI` writer - Peripheral clock selection for SPI."]
pub type PclkSpiW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkSpiEnum, crate::Safe>;
impl<'a, REG> PclkSpiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkSpiEnum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkSpiEnum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkSpiEnum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkSpiEnum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for SSP1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkSsp1Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel0PclkSsp1Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkSsp1Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkSsp1Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkSsp1Enum {}
#[doc = "Field `PCLK_SSP1` reader - Peripheral clock selection for SSP1."]
pub type PclkSsp1R = crate::FieldReader<SysconPclksel0PclkSsp1Enum>;
impl PclkSsp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkSsp1Enum {
        match self.bits {
            0 => SysconPclksel0PclkSsp1Enum::CclkDiv4,
            1 => SysconPclksel0PclkSsp1Enum::Cclk,
            2 => SysconPclksel0PclkSsp1Enum::CclkDiv2,
            3 => SysconPclksel0PclkSsp1Enum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkSsp1Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkSsp1Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkSsp1Enum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel0PclkSsp1Enum::CclkDiv8
    }
}
#[doc = "Field `PCLK_SSP1` writer - Peripheral clock selection for SSP1."]
pub type PclkSsp1W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkSsp1Enum, crate::Safe>;
impl<'a, REG> PclkSsp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkSsp1Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkSsp1Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkSsp1Enum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkSsp1Enum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for DAC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkDacEnum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel0PclkDacEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkDacEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkDacEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkDacEnum {}
#[doc = "Field `PCLK_DAC` reader - Peripheral clock selection for DAC."]
pub type PclkDacR = crate::FieldReader<SysconPclksel0PclkDacEnum>;
impl PclkDacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkDacEnum {
        match self.bits {
            0 => SysconPclksel0PclkDacEnum::CclkDiv4,
            1 => SysconPclksel0PclkDacEnum::Cclk,
            2 => SysconPclksel0PclkDacEnum::CclkDiv2,
            3 => SysconPclksel0PclkDacEnum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkDacEnum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkDacEnum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkDacEnum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel0PclkDacEnum::CclkDiv8
    }
}
#[doc = "Field `PCLK_DAC` writer - Peripheral clock selection for DAC."]
pub type PclkDacW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkDacEnum, crate::Safe>;
impl<'a, REG> PclkDacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkDacEnum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkDacEnum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkDacEnum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkDacEnum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkAdcEnum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CclkDiv8 = 3,
}
impl From<SysconPclksel0PclkAdcEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkAdcEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkAdcEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkAdcEnum {}
#[doc = "Field `PCLK_ADC` reader - Peripheral clock selection for ADC."]
pub type PclkAdcR = crate::FieldReader<SysconPclksel0PclkAdcEnum>;
impl PclkAdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkAdcEnum {
        match self.bits {
            0 => SysconPclksel0PclkAdcEnum::CclkDiv4,
            1 => SysconPclksel0PclkAdcEnum::Cclk,
            2 => SysconPclksel0PclkAdcEnum::CclkDiv2,
            3 => SysconPclksel0PclkAdcEnum::CclkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkAdcEnum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkAdcEnum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkAdcEnum::CclkDiv2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == SysconPclksel0PclkAdcEnum::CclkDiv8
    }
}
#[doc = "Field `PCLK_ADC` writer - Peripheral clock selection for ADC."]
pub type PclkAdcW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkAdcEnum, crate::Safe>;
impl<'a, REG> PclkAdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkAdcEnum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkAdcEnum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkAdcEnum::CclkDiv2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkAdcEnum::CclkDiv8)
    }
}
#[doc = "Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkCan1Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 6. PCLK_peripheral = CCLK/6."]
    CclkDiv6 = 3,
}
impl From<SysconPclksel0PclkCan1Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkCan1Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkCan1Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkCan1Enum {}
#[doc = "Field `PCLK_CAN1` reader - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PclkCan1R = crate::FieldReader<SysconPclksel0PclkCan1Enum>;
impl PclkCan1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkCan1Enum {
        match self.bits {
            0 => SysconPclksel0PclkCan1Enum::CclkDiv4,
            1 => SysconPclksel0PclkCan1Enum::Cclk,
            2 => SysconPclksel0PclkCan1Enum::CclkDiv2,
            3 => SysconPclksel0PclkCan1Enum::CclkDiv6,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkCan1Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkCan1Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkCan1Enum::CclkDiv2
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == SysconPclksel0PclkCan1Enum::CclkDiv6
    }
}
#[doc = "Field `PCLK_CAN1` writer - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PclkCan1W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkCan1Enum, crate::Safe>;
impl<'a, REG> PclkCan1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkCan1Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkCan1Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkCan1Enum::CclkDiv2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkCan1Enum::CclkDiv6)
    }
}
#[doc = "Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkCan2Enum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 6. PCLK_peripheral = CCLK/6,"]
    CclkDiv6 = 3,
}
impl From<SysconPclksel0PclkCan2Enum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkCan2Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkCan2Enum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkCan2Enum {}
#[doc = "Field `PCLK_CAN2` reader - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PclkCan2R = crate::FieldReader<SysconPclksel0PclkCan2Enum>;
impl PclkCan2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkCan2Enum {
        match self.bits {
            0 => SysconPclksel0PclkCan2Enum::CclkDiv4,
            1 => SysconPclksel0PclkCan2Enum::Cclk,
            2 => SysconPclksel0PclkCan2Enum::CclkDiv2,
            3 => SysconPclksel0PclkCan2Enum::CclkDiv6,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkCan2Enum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkCan2Enum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkCan2Enum::CclkDiv2
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == SysconPclksel0PclkCan2Enum::CclkDiv6
    }
}
#[doc = "Field `PCLK_CAN2` writer - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PclkCan2W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkCan2Enum, crate::Safe>;
impl<'a, REG> PclkCan2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkCan2Enum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkCan2Enum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkCan2Enum::CclkDiv2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkCan2Enum::CclkDiv6)
    }
}
#[doc = "Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysconPclksel0PclkAcfEnum {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CclkDiv4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    Cclk = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CclkDiv2 = 2,
    #[doc = "3: CCLK div 6. PCLK_peripheral = CCLK/6"]
    CclkDiv6 = 3,
}
impl From<SysconPclksel0PclkAcfEnum> for u8 {
    #[inline(always)]
    fn from(variant: SysconPclksel0PclkAcfEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysconPclksel0PclkAcfEnum {
    type Ux = u8;
}
impl crate::IsEnum for SysconPclksel0PclkAcfEnum {}
#[doc = "Field `PCLK_ACF` reader - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PclkAcfR = crate::FieldReader<SysconPclksel0PclkAcfEnum>;
impl PclkAcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysconPclksel0PclkAcfEnum {
        match self.bits {
            0 => SysconPclksel0PclkAcfEnum::CclkDiv4,
            1 => SysconPclksel0PclkAcfEnum::Cclk,
            2 => SysconPclksel0PclkAcfEnum::CclkDiv2,
            3 => SysconPclksel0PclkAcfEnum::CclkDiv6,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == SysconPclksel0PclkAcfEnum::CclkDiv4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == SysconPclksel0PclkAcfEnum::Cclk
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == SysconPclksel0PclkAcfEnum::CclkDiv2
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == SysconPclksel0PclkAcfEnum::CclkDiv6
    }
}
#[doc = "Field `PCLK_ACF` writer - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PclkAcfW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysconPclksel0PclkAcfEnum, crate::Safe>;
impl<'a, REG> PclkAcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkAcfEnum::CclkDiv4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkAcfEnum::Cclk)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkAcfEnum::CclkDiv2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut crate::W<REG> {
        self.variant(SysconPclksel0PclkAcfEnum::CclkDiv6)
    }
}
impl R {
    #[doc = "Bits 0:1 - Peripheral clock selection for WDT."]
    #[inline(always)]
    pub fn pclk_wdt(&self) -> PclkWdtR {
        PclkWdtR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for TIMER0."]
    #[inline(always)]
    pub fn pclk_timer0(&self) -> PclkTimer0R {
        PclkTimer0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for TIMER1."]
    #[inline(always)]
    pub fn pclk_timer1(&self) -> PclkTimer1R {
        PclkTimer1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for UART0."]
    #[inline(always)]
    pub fn pclk_uart0(&self) -> PclkUart0R {
        PclkUart0R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Peripheral clock selection for UART1."]
    #[inline(always)]
    pub fn pclk_uart1(&self) -> PclkUart1R {
        PclkUart1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for PWM1."]
    #[inline(always)]
    pub fn pclk_pwm1(&self) -> PclkPwm1R {
        PclkPwm1R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for I2C0."]
    #[inline(always)]
    pub fn pclk_i2c0(&self) -> PclkI2c0R {
        PclkI2c0R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for SPI."]
    #[inline(always)]
    pub fn pclk_spi(&self) -> PclkSpiR {
        PclkSpiR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for SSP1."]
    #[inline(always)]
    pub fn pclk_ssp1(&self) -> PclkSsp1R {
        PclkSsp1R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for DAC."]
    #[inline(always)]
    pub fn pclk_dac(&self) -> PclkDacR {
        PclkDacR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Peripheral clock selection for ADC."]
    #[inline(always)]
    pub fn pclk_adc(&self) -> PclkAdcR {
        PclkAdcR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can1(&self) -> PclkCan1R {
        PclkCan1R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can2(&self) -> PclkCan2R {
        PclkCan2R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_acf(&self) -> PclkAcfR {
        PclkAcfR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Peripheral clock selection for WDT."]
    #[inline(always)]
    pub fn pclk_wdt(&mut self) -> PclkWdtW<'_, Pclksel0Spec> {
        PclkWdtW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for TIMER0."]
    #[inline(always)]
    pub fn pclk_timer0(&mut self) -> PclkTimer0W<'_, Pclksel0Spec> {
        PclkTimer0W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for TIMER1."]
    #[inline(always)]
    pub fn pclk_timer1(&mut self) -> PclkTimer1W<'_, Pclksel0Spec> {
        PclkTimer1W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for UART0."]
    #[inline(always)]
    pub fn pclk_uart0(&mut self) -> PclkUart0W<'_, Pclksel0Spec> {
        PclkUart0W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Peripheral clock selection for UART1."]
    #[inline(always)]
    pub fn pclk_uart1(&mut self) -> PclkUart1W<'_, Pclksel0Spec> {
        PclkUart1W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for PWM1."]
    #[inline(always)]
    pub fn pclk_pwm1(&mut self) -> PclkPwm1W<'_, Pclksel0Spec> {
        PclkPwm1W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for I2C0."]
    #[inline(always)]
    pub fn pclk_i2c0(&mut self) -> PclkI2c0W<'_, Pclksel0Spec> {
        PclkI2c0W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for SPI."]
    #[inline(always)]
    pub fn pclk_spi(&mut self) -> PclkSpiW<'_, Pclksel0Spec> {
        PclkSpiW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for SSP1."]
    #[inline(always)]
    pub fn pclk_ssp1(&mut self) -> PclkSsp1W<'_, Pclksel0Spec> {
        PclkSsp1W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for DAC."]
    #[inline(always)]
    pub fn pclk_dac(&mut self) -> PclkDacW<'_, Pclksel0Spec> {
        PclkDacW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Peripheral clock selection for ADC."]
    #[inline(always)]
    pub fn pclk_adc(&mut self) -> PclkAdcW<'_, Pclksel0Spec> {
        PclkAdcW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can1(&mut self) -> PclkCan1W<'_, Pclksel0Spec> {
        PclkCan1W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can2(&mut self) -> PclkCan2W<'_, Pclksel0Spec> {
        PclkCan2W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_acf(&mut self) -> PclkAcfW<'_, Pclksel0Spec> {
        PclkAcfW::new(self, 30)
    }
}
#[doc = "Peripheral Clock Selection register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclksel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclksel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pclksel0Spec;
impl crate::RegisterSpec for Pclksel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclksel0::R`](R) reader structure"]
impl crate::Readable for Pclksel0Spec {}
#[doc = "`write(|w| ..)` method takes [`pclksel0::W`](W) writer structure"]
impl crate::Writable for Pclksel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCLKSEL0 to value 0"]
impl crate::Resettable for Pclksel0Spec {}
