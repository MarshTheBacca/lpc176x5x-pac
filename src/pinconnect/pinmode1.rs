#[doc = "Register `PINMODE1` reader"]
pub type R = crate::R<Pinmode1Spec>;
#[doc = "Register `PINMODE1` writer"]
pub type W = crate::W<Pinmode1Spec>;
#[doc = "Port 1 pin 16 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode1P0_16modeEnum {
    #[doc = "0: Pull-up. P0.16 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.16 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.16 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.16 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode1P0_16modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode1P0_16modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode1P0_16modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode1P0_16modeEnum {}
#[doc = "Field `P0_16MODE` reader - Port 1 pin 16 control."]
pub type P0_16modeR = crate::FieldReader<PinconnectPinmode1P0_16modeEnum>;
impl P0_16modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode1P0_16modeEnum {
        match self.bits {
            0 => PinconnectPinmode1P0_16modeEnum::PullUp,
            1 => PinconnectPinmode1P0_16modeEnum::Repeater,
            2 => PinconnectPinmode1P0_16modeEnum::Disabled,
            3 => PinconnectPinmode1P0_16modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.16 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode1P0_16modeEnum::PullUp
    }
    #[doc = "Repeater. P0.16 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode1P0_16modeEnum::Repeater
    }
    #[doc = "Disabled. P0.16 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode1P0_16modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.16 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode1P0_16modeEnum::PullDown
    }
}
#[doc = "Field `P0_16MODE` writer - Port 1 pin 16 control."]
pub type P0_16modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode1P0_16modeEnum, crate::Safe>;
impl<'a, REG> P0_16modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.16 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_16modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.16 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_16modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.16 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_16modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.16 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_16modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 17 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode1P0_17modeEnum {
    #[doc = "0: Pull-up. P0.17 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.17 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.17 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.17 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode1P0_17modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode1P0_17modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode1P0_17modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode1P0_17modeEnum {}
#[doc = "Field `P0_17MODE` reader - Port 1 pin 17 control."]
pub type P0_17modeR = crate::FieldReader<PinconnectPinmode1P0_17modeEnum>;
impl P0_17modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode1P0_17modeEnum {
        match self.bits {
            0 => PinconnectPinmode1P0_17modeEnum::PullUp,
            1 => PinconnectPinmode1P0_17modeEnum::Repeater,
            2 => PinconnectPinmode1P0_17modeEnum::Disabled,
            3 => PinconnectPinmode1P0_17modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.17 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode1P0_17modeEnum::PullUp
    }
    #[doc = "Repeater. P0.17 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode1P0_17modeEnum::Repeater
    }
    #[doc = "Disabled. P0.17 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode1P0_17modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.17 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode1P0_17modeEnum::PullDown
    }
}
#[doc = "Field `P0_17MODE` writer - Port 1 pin 17 control."]
pub type P0_17modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode1P0_17modeEnum, crate::Safe>;
impl<'a, REG> P0_17modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.17 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_17modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.17 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_17modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.17 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_17modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.17 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_17modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 18 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode1P0_18modeEnum {
    #[doc = "0: Pull-up. P0.18 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.18 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.18 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.18 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode1P0_18modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode1P0_18modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode1P0_18modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode1P0_18modeEnum {}
#[doc = "Field `P0_18MODE` reader - Port 1 pin 18 control."]
pub type P0_18modeR = crate::FieldReader<PinconnectPinmode1P0_18modeEnum>;
impl P0_18modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode1P0_18modeEnum {
        match self.bits {
            0 => PinconnectPinmode1P0_18modeEnum::PullUp,
            1 => PinconnectPinmode1P0_18modeEnum::Repeater,
            2 => PinconnectPinmode1P0_18modeEnum::Disabled,
            3 => PinconnectPinmode1P0_18modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.18 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode1P0_18modeEnum::PullUp
    }
    #[doc = "Repeater. P0.18 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode1P0_18modeEnum::Repeater
    }
    #[doc = "Disabled. P0.18 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode1P0_18modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.18 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode1P0_18modeEnum::PullDown
    }
}
#[doc = "Field `P0_18MODE` writer - Port 1 pin 18 control."]
pub type P0_18modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode1P0_18modeEnum, crate::Safe>;
impl<'a, REG> P0_18modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.18 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_18modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.18 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_18modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.18 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_18modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.18 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_18modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 19 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode1P0_19modeEnum {
    #[doc = "0: Pull-up. P0.19 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.19 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.19 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.19 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode1P0_19modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode1P0_19modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode1P0_19modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode1P0_19modeEnum {}
#[doc = "Field `P0_19MODE` reader - Port 1 pin 19 control."]
pub type P0_19modeR = crate::FieldReader<PinconnectPinmode1P0_19modeEnum>;
impl P0_19modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode1P0_19modeEnum {
        match self.bits {
            0 => PinconnectPinmode1P0_19modeEnum::PullUp,
            1 => PinconnectPinmode1P0_19modeEnum::Repeater,
            2 => PinconnectPinmode1P0_19modeEnum::Disabled,
            3 => PinconnectPinmode1P0_19modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.19 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode1P0_19modeEnum::PullUp
    }
    #[doc = "Repeater. P0.19 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode1P0_19modeEnum::Repeater
    }
    #[doc = "Disabled. P0.19 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode1P0_19modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.19 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode1P0_19modeEnum::PullDown
    }
}
#[doc = "Field `P0_19MODE` writer - Port 1 pin 19 control."]
pub type P0_19modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode1P0_19modeEnum, crate::Safe>;
impl<'a, REG> P0_19modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.19 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_19modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.19 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_19modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.19 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_19modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.19 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_19modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 20 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode1P0_20modeEnum {
    #[doc = "0: Pull-up. P0.20 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.20 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.20 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.20 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode1P0_20modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode1P0_20modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode1P0_20modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode1P0_20modeEnum {}
#[doc = "Field `P0_20MODE` reader - Port 1 pin 20 control."]
pub type P0_20modeR = crate::FieldReader<PinconnectPinmode1P0_20modeEnum>;
impl P0_20modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode1P0_20modeEnum {
        match self.bits {
            0 => PinconnectPinmode1P0_20modeEnum::PullUp,
            1 => PinconnectPinmode1P0_20modeEnum::Repeater,
            2 => PinconnectPinmode1P0_20modeEnum::Disabled,
            3 => PinconnectPinmode1P0_20modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.20 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode1P0_20modeEnum::PullUp
    }
    #[doc = "Repeater. P0.20 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode1P0_20modeEnum::Repeater
    }
    #[doc = "Disabled. P0.20 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode1P0_20modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.20 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode1P0_20modeEnum::PullDown
    }
}
#[doc = "Field `P0_20MODE` writer - Port 1 pin 20 control."]
pub type P0_20modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode1P0_20modeEnum, crate::Safe>;
impl<'a, REG> P0_20modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.20 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_20modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.20 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_20modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.20 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_20modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.20 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_20modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 21 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode1P0_21modeEnum {
    #[doc = "0: Pull-up. P0.21 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.21 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.21 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.21 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode1P0_21modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode1P0_21modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode1P0_21modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode1P0_21modeEnum {}
#[doc = "Field `P0_21MODE` reader - Port 1 pin 21 control."]
pub type P0_21modeR = crate::FieldReader<PinconnectPinmode1P0_21modeEnum>;
impl P0_21modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode1P0_21modeEnum {
        match self.bits {
            0 => PinconnectPinmode1P0_21modeEnum::PullUp,
            1 => PinconnectPinmode1P0_21modeEnum::Repeater,
            2 => PinconnectPinmode1P0_21modeEnum::Disabled,
            3 => PinconnectPinmode1P0_21modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.21 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode1P0_21modeEnum::PullUp
    }
    #[doc = "Repeater. P0.21 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode1P0_21modeEnum::Repeater
    }
    #[doc = "Disabled. P0.21 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode1P0_21modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.21 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode1P0_21modeEnum::PullDown
    }
}
#[doc = "Field `P0_21MODE` writer - Port 1 pin 21 control."]
pub type P0_21modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode1P0_21modeEnum, crate::Safe>;
impl<'a, REG> P0_21modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.21 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_21modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.21 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_21modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.21 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_21modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.21 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_21modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 22 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode1P0_22modeEnum {
    #[doc = "0: Pull-up. P0.22 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.22 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.22 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.22 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode1P0_22modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode1P0_22modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode1P0_22modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode1P0_22modeEnum {}
#[doc = "Field `P0_22MODE` reader - Port 1 pin 22 control."]
pub type P0_22modeR = crate::FieldReader<PinconnectPinmode1P0_22modeEnum>;
impl P0_22modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode1P0_22modeEnum {
        match self.bits {
            0 => PinconnectPinmode1P0_22modeEnum::PullUp,
            1 => PinconnectPinmode1P0_22modeEnum::Repeater,
            2 => PinconnectPinmode1P0_22modeEnum::Disabled,
            3 => PinconnectPinmode1P0_22modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.22 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode1P0_22modeEnum::PullUp
    }
    #[doc = "Repeater. P0.22 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode1P0_22modeEnum::Repeater
    }
    #[doc = "Disabled. P0.22 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode1P0_22modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.22 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode1P0_22modeEnum::PullDown
    }
}
#[doc = "Field `P0_22MODE` writer - Port 1 pin 22 control."]
pub type P0_22modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode1P0_22modeEnum, crate::Safe>;
impl<'a, REG> P0_22modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.22 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_22modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.22 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_22modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.22 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_22modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.22 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_22modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 23 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode1P0_23modeEnum {
    #[doc = "0: Pull-up. P0.23 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.23 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.23 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.23 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode1P0_23modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode1P0_23modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode1P0_23modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode1P0_23modeEnum {}
#[doc = "Field `P0_23MODE` reader - Port 1 pin 23 control."]
pub type P0_23modeR = crate::FieldReader<PinconnectPinmode1P0_23modeEnum>;
impl P0_23modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode1P0_23modeEnum {
        match self.bits {
            0 => PinconnectPinmode1P0_23modeEnum::PullUp,
            1 => PinconnectPinmode1P0_23modeEnum::Repeater,
            2 => PinconnectPinmode1P0_23modeEnum::Disabled,
            3 => PinconnectPinmode1P0_23modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.23 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode1P0_23modeEnum::PullUp
    }
    #[doc = "Repeater. P0.23 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode1P0_23modeEnum::Repeater
    }
    #[doc = "Disabled. P0.23 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode1P0_23modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.23 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode1P0_23modeEnum::PullDown
    }
}
#[doc = "Field `P0_23MODE` writer - Port 1 pin 23 control."]
pub type P0_23modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode1P0_23modeEnum, crate::Safe>;
impl<'a, REG> P0_23modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.23 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_23modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.23 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_23modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.23 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_23modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.23 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_23modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 24 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode1P0_24modeEnum {
    #[doc = "0: Pull-up. P0.24 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.24 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.24 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.24 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode1P0_24modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode1P0_24modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode1P0_24modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode1P0_24modeEnum {}
#[doc = "Field `P0_24MODE` reader - Port 1 pin 24 control."]
pub type P0_24modeR = crate::FieldReader<PinconnectPinmode1P0_24modeEnum>;
impl P0_24modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode1P0_24modeEnum {
        match self.bits {
            0 => PinconnectPinmode1P0_24modeEnum::PullUp,
            1 => PinconnectPinmode1P0_24modeEnum::Repeater,
            2 => PinconnectPinmode1P0_24modeEnum::Disabled,
            3 => PinconnectPinmode1P0_24modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.24 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode1P0_24modeEnum::PullUp
    }
    #[doc = "Repeater. P0.24 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode1P0_24modeEnum::Repeater
    }
    #[doc = "Disabled. P0.24 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode1P0_24modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.24 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode1P0_24modeEnum::PullDown
    }
}
#[doc = "Field `P0_24MODE` writer - Port 1 pin 24 control."]
pub type P0_24modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode1P0_24modeEnum, crate::Safe>;
impl<'a, REG> P0_24modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.24 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_24modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.24 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_24modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.24 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_24modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.24 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_24modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 25 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode1P0_25modeEnum {
    #[doc = "0: Pull-up. P0.25 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.25 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.25 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.25 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode1P0_25modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode1P0_25modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode1P0_25modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode1P0_25modeEnum {}
#[doc = "Field `P0_25MODE` reader - Port 1 pin 25 control."]
pub type P0_25modeR = crate::FieldReader<PinconnectPinmode1P0_25modeEnum>;
impl P0_25modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode1P0_25modeEnum {
        match self.bits {
            0 => PinconnectPinmode1P0_25modeEnum::PullUp,
            1 => PinconnectPinmode1P0_25modeEnum::Repeater,
            2 => PinconnectPinmode1P0_25modeEnum::Disabled,
            3 => PinconnectPinmode1P0_25modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode1P0_25modeEnum::PullUp
    }
    #[doc = "Repeater. P0.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode1P0_25modeEnum::Repeater
    }
    #[doc = "Disabled. P0.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode1P0_25modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode1P0_25modeEnum::PullDown
    }
}
#[doc = "Field `P0_25MODE` writer - Port 1 pin 25 control."]
pub type P0_25modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode1P0_25modeEnum, crate::Safe>;
impl<'a, REG> P0_25modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_25modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_25modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_25modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_25modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 26 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode1P0_26modeEnum {
    #[doc = "0: Pull-up. P0.26 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.26 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.26 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.26 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode1P0_26modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode1P0_26modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode1P0_26modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode1P0_26modeEnum {}
#[doc = "Field `P0_26MODE` reader - Port 1 pin 26 control."]
pub type P0_26modeR = crate::FieldReader<PinconnectPinmode1P0_26modeEnum>;
impl P0_26modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode1P0_26modeEnum {
        match self.bits {
            0 => PinconnectPinmode1P0_26modeEnum::PullUp,
            1 => PinconnectPinmode1P0_26modeEnum::Repeater,
            2 => PinconnectPinmode1P0_26modeEnum::Disabled,
            3 => PinconnectPinmode1P0_26modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode1P0_26modeEnum::PullUp
    }
    #[doc = "Repeater. P0.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode1P0_26modeEnum::Repeater
    }
    #[doc = "Disabled. P0.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode1P0_26modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode1P0_26modeEnum::PullDown
    }
}
#[doc = "Field `P0_26MODE` writer - Port 1 pin 26 control."]
pub type P0_26modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode1P0_26modeEnum, crate::Safe>;
impl<'a, REG> P0_26modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_26modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_26modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_26modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode1P0_26modeEnum::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p0_16mode(&self) -> P0_16modeR {
        P0_16modeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p0_17mode(&self) -> P0_17modeR {
        P0_17modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p0_18mode(&self) -> P0_18modeR {
        P0_18modeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p0_19mode(&self) -> P0_19modeR {
        P0_19modeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p0_20mode(&self) -> P0_20modeR {
        P0_20modeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p0_21mode(&self) -> P0_21modeR {
        P0_21modeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p0_22mode(&self) -> P0_22modeR {
        P0_22modeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p0_23mode(&self) -> P0_23modeR {
        P0_23modeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p0_24mode(&self) -> P0_24modeR {
        P0_24modeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p0_25mode(&self) -> P0_25modeR {
        P0_25modeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p0_26mode(&self) -> P0_26modeR {
        P0_26modeR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p0_16mode(&mut self) -> P0_16modeW<'_, Pinmode1Spec> {
        P0_16modeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p0_17mode(&mut self) -> P0_17modeW<'_, Pinmode1Spec> {
        P0_17modeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p0_18mode(&mut self) -> P0_18modeW<'_, Pinmode1Spec> {
        P0_18modeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p0_19mode(&mut self) -> P0_19modeW<'_, Pinmode1Spec> {
        P0_19modeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p0_20mode(&mut self) -> P0_20modeW<'_, Pinmode1Spec> {
        P0_20modeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p0_21mode(&mut self) -> P0_21modeW<'_, Pinmode1Spec> {
        P0_21modeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p0_22mode(&mut self) -> P0_22modeW<'_, Pinmode1Spec> {
        P0_22modeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p0_23mode(&mut self) -> P0_23modeW<'_, Pinmode1Spec> {
        P0_23modeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p0_24mode(&mut self) -> P0_24modeW<'_, Pinmode1Spec> {
        P0_24modeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p0_25mode(&mut self) -> P0_25modeW<'_, Pinmode1Spec> {
        P0_25modeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p0_26mode(&mut self) -> P0_26modeW<'_, Pinmode1Spec> {
        P0_26modeW::new(self, 20)
    }
}
#[doc = "Pin mode select register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinmode1Spec;
impl crate::RegisterSpec for Pinmode1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode1::R`](R) reader structure"]
impl crate::Readable for Pinmode1Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode1::W`](W) writer structure"]
impl crate::Writable for Pinmode1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINMODE1 to value 0"]
impl crate::Resettable for Pinmode1Spec {}
