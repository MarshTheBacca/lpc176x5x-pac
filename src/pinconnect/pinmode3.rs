#[doc = "Register `PINMODE3` reader"]
pub type R = crate::R<Pinmode3Spec>;
#[doc = "Register `PINMODE3` writer"]
pub type W = crate::W<Pinmode3Spec>;
#[doc = "Port 1 pin 16 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_16modeEnum {
    #[doc = "0: Pull-up. P1.16 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.16 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.16 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.16 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_16modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_16modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_16modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_16modeEnum {}
#[doc = "Field `P1_16MODE` reader - Port 1 pin 16 control."]
pub type P1_16modeR = crate::FieldReader<PinconnectPinmode3P1_16modeEnum>;
impl P1_16modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_16modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_16modeEnum::PullUp,
            1 => PinconnectPinmode3P1_16modeEnum::Repeater,
            2 => PinconnectPinmode3P1_16modeEnum::Disabled,
            3 => PinconnectPinmode3P1_16modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.16 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_16modeEnum::PullUp
    }
    #[doc = "Repeater. P1.16 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_16modeEnum::Repeater
    }
    #[doc = "Disabled. P1.16 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_16modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.16 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_16modeEnum::PullDown
    }
}
#[doc = "Field `P1_16MODE` writer - Port 1 pin 16 control."]
pub type P1_16modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_16modeEnum, crate::Safe>;
impl<'a, REG> P1_16modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.16 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_16modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.16 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_16modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.16 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_16modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.16 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_16modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 17 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_17modeEnum {
    #[doc = "0: Pull-up. P1.17 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.17 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.17 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.17 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_17modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_17modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_17modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_17modeEnum {}
#[doc = "Field `P1_17MODE` reader - Port 1 pin 17 control."]
pub type P1_17modeR = crate::FieldReader<PinconnectPinmode3P1_17modeEnum>;
impl P1_17modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_17modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_17modeEnum::PullUp,
            1 => PinconnectPinmode3P1_17modeEnum::Repeater,
            2 => PinconnectPinmode3P1_17modeEnum::Disabled,
            3 => PinconnectPinmode3P1_17modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.17 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_17modeEnum::PullUp
    }
    #[doc = "Repeater. P1.17 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_17modeEnum::Repeater
    }
    #[doc = "Disabled. P1.17 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_17modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.17 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_17modeEnum::PullDown
    }
}
#[doc = "Field `P1_17MODE` writer - Port 1 pin 17 control."]
pub type P1_17modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_17modeEnum, crate::Safe>;
impl<'a, REG> P1_17modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.17 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_17modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.17 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_17modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.17 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_17modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.17 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_17modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 18 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_18modeEnum {
    #[doc = "0: Pull-up. P1.18 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.18 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.18 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.18 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_18modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_18modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_18modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_18modeEnum {}
#[doc = "Field `P1_18MODE` reader - Port 1 pin 18 control."]
pub type P1_18modeR = crate::FieldReader<PinconnectPinmode3P1_18modeEnum>;
impl P1_18modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_18modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_18modeEnum::PullUp,
            1 => PinconnectPinmode3P1_18modeEnum::Repeater,
            2 => PinconnectPinmode3P1_18modeEnum::Disabled,
            3 => PinconnectPinmode3P1_18modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.18 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_18modeEnum::PullUp
    }
    #[doc = "Repeater. P1.18 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_18modeEnum::Repeater
    }
    #[doc = "Disabled. P1.18 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_18modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.18 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_18modeEnum::PullDown
    }
}
#[doc = "Field `P1_18MODE` writer - Port 1 pin 18 control."]
pub type P1_18modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_18modeEnum, crate::Safe>;
impl<'a, REG> P1_18modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.18 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_18modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.18 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_18modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.18 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_18modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.18 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_18modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 19 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_19modeEnum {
    #[doc = "0: Pull-up. P1.19 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.19 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.19 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.19 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_19modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_19modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_19modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_19modeEnum {}
#[doc = "Field `P1_19MODE` reader - Port 1 pin 19 control."]
pub type P1_19modeR = crate::FieldReader<PinconnectPinmode3P1_19modeEnum>;
impl P1_19modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_19modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_19modeEnum::PullUp,
            1 => PinconnectPinmode3P1_19modeEnum::Repeater,
            2 => PinconnectPinmode3P1_19modeEnum::Disabled,
            3 => PinconnectPinmode3P1_19modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.19 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_19modeEnum::PullUp
    }
    #[doc = "Repeater. P1.19 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_19modeEnum::Repeater
    }
    #[doc = "Disabled. P1.19 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_19modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.19 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_19modeEnum::PullDown
    }
}
#[doc = "Field `P1_19MODE` writer - Port 1 pin 19 control."]
pub type P1_19modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_19modeEnum, crate::Safe>;
impl<'a, REG> P1_19modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.19 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_19modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.19 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_19modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.19 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_19modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.19 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_19modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 20 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_20modeEnum {
    #[doc = "0: Pull-up. P1.20 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.20 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.20 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.20 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_20modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_20modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_20modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_20modeEnum {}
#[doc = "Field `P1_20MODE` reader - Port 1 pin 20 control."]
pub type P1_20modeR = crate::FieldReader<PinconnectPinmode3P1_20modeEnum>;
impl P1_20modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_20modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_20modeEnum::PullUp,
            1 => PinconnectPinmode3P1_20modeEnum::Repeater,
            2 => PinconnectPinmode3P1_20modeEnum::Disabled,
            3 => PinconnectPinmode3P1_20modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.20 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_20modeEnum::PullUp
    }
    #[doc = "Repeater. P1.20 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_20modeEnum::Repeater
    }
    #[doc = "Disabled. P1.20 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_20modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.20 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_20modeEnum::PullDown
    }
}
#[doc = "Field `P1_20MODE` writer - Port 1 pin 20 control."]
pub type P1_20modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_20modeEnum, crate::Safe>;
impl<'a, REG> P1_20modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.20 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_20modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.20 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_20modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.20 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_20modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.20 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_20modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 21 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_21modeEnum {
    #[doc = "0: Pull-up. P1.21 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.21 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.21 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.21 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_21modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_21modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_21modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_21modeEnum {}
#[doc = "Field `P1_21MODE` reader - Port 1 pin 21 control."]
pub type P1_21modeR = crate::FieldReader<PinconnectPinmode3P1_21modeEnum>;
impl P1_21modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_21modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_21modeEnum::PullUp,
            1 => PinconnectPinmode3P1_21modeEnum::Repeater,
            2 => PinconnectPinmode3P1_21modeEnum::Disabled,
            3 => PinconnectPinmode3P1_21modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.21 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_21modeEnum::PullUp
    }
    #[doc = "Repeater. P1.21 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_21modeEnum::Repeater
    }
    #[doc = "Disabled. P1.21 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_21modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.21 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_21modeEnum::PullDown
    }
}
#[doc = "Field `P1_21MODE` writer - Port 1 pin 21 control."]
pub type P1_21modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_21modeEnum, crate::Safe>;
impl<'a, REG> P1_21modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.21 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_21modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.21 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_21modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.21 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_21modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.21 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_21modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 22 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_22modeEnum {
    #[doc = "0: Pull-up. P1.22 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.22 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.22 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.22 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_22modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_22modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_22modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_22modeEnum {}
#[doc = "Field `P1_22MODE` reader - Port 1 pin 22 control."]
pub type P1_22modeR = crate::FieldReader<PinconnectPinmode3P1_22modeEnum>;
impl P1_22modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_22modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_22modeEnum::PullUp,
            1 => PinconnectPinmode3P1_22modeEnum::Repeater,
            2 => PinconnectPinmode3P1_22modeEnum::Disabled,
            3 => PinconnectPinmode3P1_22modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.22 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_22modeEnum::PullUp
    }
    #[doc = "Repeater. P1.22 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_22modeEnum::Repeater
    }
    #[doc = "Disabled. P1.22 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_22modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.22 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_22modeEnum::PullDown
    }
}
#[doc = "Field `P1_22MODE` writer - Port 1 pin 22 control."]
pub type P1_22modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_22modeEnum, crate::Safe>;
impl<'a, REG> P1_22modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.22 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_22modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.22 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_22modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.22 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_22modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.22 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_22modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 23 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_23modeEnum {
    #[doc = "0: Pull-up. P1.23 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.23 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.23 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.23 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_23modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_23modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_23modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_23modeEnum {}
#[doc = "Field `P1_23MODE` reader - Port 1 pin 23 control."]
pub type P1_23modeR = crate::FieldReader<PinconnectPinmode3P1_23modeEnum>;
impl P1_23modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_23modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_23modeEnum::PullUp,
            1 => PinconnectPinmode3P1_23modeEnum::Repeater,
            2 => PinconnectPinmode3P1_23modeEnum::Disabled,
            3 => PinconnectPinmode3P1_23modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.23 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_23modeEnum::PullUp
    }
    #[doc = "Repeater. P1.23 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_23modeEnum::Repeater
    }
    #[doc = "Disabled. P1.23 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_23modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.23 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_23modeEnum::PullDown
    }
}
#[doc = "Field `P1_23MODE` writer - Port 1 pin 23 control."]
pub type P1_23modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_23modeEnum, crate::Safe>;
impl<'a, REG> P1_23modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.23 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_23modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.23 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_23modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.23 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_23modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.23 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_23modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 24 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_24modeEnum {
    #[doc = "0: Pull-up. P1.24 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.24 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.24 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.24 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_24modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_24modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_24modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_24modeEnum {}
#[doc = "Field `P1_24MODE` reader - Port 1 pin 24 control."]
pub type P1_24modeR = crate::FieldReader<PinconnectPinmode3P1_24modeEnum>;
impl P1_24modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_24modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_24modeEnum::PullUp,
            1 => PinconnectPinmode3P1_24modeEnum::Repeater,
            2 => PinconnectPinmode3P1_24modeEnum::Disabled,
            3 => PinconnectPinmode3P1_24modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.24 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_24modeEnum::PullUp
    }
    #[doc = "Repeater. P1.24 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_24modeEnum::Repeater
    }
    #[doc = "Disabled. P1.24 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_24modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.24 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_24modeEnum::PullDown
    }
}
#[doc = "Field `P1_24MODE` writer - Port 1 pin 24 control."]
pub type P1_24modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_24modeEnum, crate::Safe>;
impl<'a, REG> P1_24modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.24 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_24modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.24 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_24modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.24 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_24modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.24 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_24modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 25 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_25modeEnum {
    #[doc = "0: Pull-up. P1.25 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.25 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.25 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.25 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_25modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_25modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_25modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_25modeEnum {}
#[doc = "Field `P1_25MODE` reader - Port 1 pin 25 control."]
pub type P1_25modeR = crate::FieldReader<PinconnectPinmode3P1_25modeEnum>;
impl P1_25modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_25modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_25modeEnum::PullUp,
            1 => PinconnectPinmode3P1_25modeEnum::Repeater,
            2 => PinconnectPinmode3P1_25modeEnum::Disabled,
            3 => PinconnectPinmode3P1_25modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_25modeEnum::PullUp
    }
    #[doc = "Repeater. P1.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_25modeEnum::Repeater
    }
    #[doc = "Disabled. P1.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_25modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_25modeEnum::PullDown
    }
}
#[doc = "Field `P1_25MODE` writer - Port 1 pin 25 control."]
pub type P1_25modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_25modeEnum, crate::Safe>;
impl<'a, REG> P1_25modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_25modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_25modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_25modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_25modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 26 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_26modeEnum {
    #[doc = "0: Pull-up. P1.26 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.26 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.26 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.26 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_26modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_26modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_26modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_26modeEnum {}
#[doc = "Field `P1_26MODE` reader - Port 1 pin 26 control."]
pub type P1_26modeR = crate::FieldReader<PinconnectPinmode3P1_26modeEnum>;
impl P1_26modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_26modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_26modeEnum::PullUp,
            1 => PinconnectPinmode3P1_26modeEnum::Repeater,
            2 => PinconnectPinmode3P1_26modeEnum::Disabled,
            3 => PinconnectPinmode3P1_26modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_26modeEnum::PullUp
    }
    #[doc = "Repeater. P1.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_26modeEnum::Repeater
    }
    #[doc = "Disabled. P1.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_26modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_26modeEnum::PullDown
    }
}
#[doc = "Field `P1_26MODE` writer - Port 1 pin 26 control."]
pub type P1_26modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_26modeEnum, crate::Safe>;
impl<'a, REG> P1_26modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_26modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_26modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_26modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_26modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 27 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_27modeEnum {
    #[doc = "0: Pull-up. P1.27 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.27 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.27 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.27 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_27modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_27modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_27modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_27modeEnum {}
#[doc = "Field `P1_27MODE` reader - Port 1 pin 27 control."]
pub type P1_27modeR = crate::FieldReader<PinconnectPinmode3P1_27modeEnum>;
impl P1_27modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_27modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_27modeEnum::PullUp,
            1 => PinconnectPinmode3P1_27modeEnum::Repeater,
            2 => PinconnectPinmode3P1_27modeEnum::Disabled,
            3 => PinconnectPinmode3P1_27modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.27 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_27modeEnum::PullUp
    }
    #[doc = "Repeater. P1.27 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_27modeEnum::Repeater
    }
    #[doc = "Disabled. P1.27 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_27modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.27 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_27modeEnum::PullDown
    }
}
#[doc = "Field `P1_27MODE` writer - Port 1 pin 27 control."]
pub type P1_27modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_27modeEnum, crate::Safe>;
impl<'a, REG> P1_27modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.27 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_27modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.27 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_27modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.27 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_27modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.27 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_27modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 28 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_28modeEnum {
    #[doc = "0: Pull-up. P1.28 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.28 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.28 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.28 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_28modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_28modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_28modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_28modeEnum {}
#[doc = "Field `P1_28MODE` reader - Port 1 pin 28 control."]
pub type P1_28modeR = crate::FieldReader<PinconnectPinmode3P1_28modeEnum>;
impl P1_28modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_28modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_28modeEnum::PullUp,
            1 => PinconnectPinmode3P1_28modeEnum::Repeater,
            2 => PinconnectPinmode3P1_28modeEnum::Disabled,
            3 => PinconnectPinmode3P1_28modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.28 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_28modeEnum::PullUp
    }
    #[doc = "Repeater. P1.28 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_28modeEnum::Repeater
    }
    #[doc = "Disabled. P1.28 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_28modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.28 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_28modeEnum::PullDown
    }
}
#[doc = "Field `P1_28MODE` writer - Port 1 pin 28 control."]
pub type P1_28modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_28modeEnum, crate::Safe>;
impl<'a, REG> P1_28modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.28 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_28modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.28 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_28modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.28 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_28modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.28 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_28modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 29 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_29modeEnum {
    #[doc = "0: Pull-up. P1.29 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.29 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.29 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.29 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_29modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_29modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_29modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_29modeEnum {}
#[doc = "Field `P1_29MODE` reader - Port 1 pin 29 control."]
pub type P1_29modeR = crate::FieldReader<PinconnectPinmode3P1_29modeEnum>;
impl P1_29modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_29modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_29modeEnum::PullUp,
            1 => PinconnectPinmode3P1_29modeEnum::Repeater,
            2 => PinconnectPinmode3P1_29modeEnum::Disabled,
            3 => PinconnectPinmode3P1_29modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.29 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_29modeEnum::PullUp
    }
    #[doc = "Repeater. P1.29 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_29modeEnum::Repeater
    }
    #[doc = "Disabled. P1.29 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_29modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.29 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_29modeEnum::PullDown
    }
}
#[doc = "Field `P1_29MODE` writer - Port 1 pin 29 control."]
pub type P1_29modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_29modeEnum, crate::Safe>;
impl<'a, REG> P1_29modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.29 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_29modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.29 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_29modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.29 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_29modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.29 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_29modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 30 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_30modeEnum {
    #[doc = "0: Pull-up. P1.30 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.30 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.30 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.30 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_30modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_30modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_30modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_30modeEnum {}
#[doc = "Field `P1_30MODE` reader - Port 1 pin 30 control."]
pub type P1_30modeR = crate::FieldReader<PinconnectPinmode3P1_30modeEnum>;
impl P1_30modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_30modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_30modeEnum::PullUp,
            1 => PinconnectPinmode3P1_30modeEnum::Repeater,
            2 => PinconnectPinmode3P1_30modeEnum::Disabled,
            3 => PinconnectPinmode3P1_30modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.30 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_30modeEnum::PullUp
    }
    #[doc = "Repeater. P1.30 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_30modeEnum::Repeater
    }
    #[doc = "Disabled. P1.30 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_30modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.30 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_30modeEnum::PullDown
    }
}
#[doc = "Field `P1_30MODE` writer - Port 1 pin 30 control."]
pub type P1_30modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_30modeEnum, crate::Safe>;
impl<'a, REG> P1_30modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.30 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_30modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.30 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_30modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.30 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_30modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.30 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_30modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 31 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode3P1_31modeEnum {
    #[doc = "0: Pull-up. P1.31 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.31 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.31 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.31 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode3P1_31modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode3P1_31modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode3P1_31modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode3P1_31modeEnum {}
#[doc = "Field `P1_31MODE` reader - Port 1 pin 31 control."]
pub type P1_31modeR = crate::FieldReader<PinconnectPinmode3P1_31modeEnum>;
impl P1_31modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode3P1_31modeEnum {
        match self.bits {
            0 => PinconnectPinmode3P1_31modeEnum::PullUp,
            1 => PinconnectPinmode3P1_31modeEnum::Repeater,
            2 => PinconnectPinmode3P1_31modeEnum::Disabled,
            3 => PinconnectPinmode3P1_31modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.31 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode3P1_31modeEnum::PullUp
    }
    #[doc = "Repeater. P1.31 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode3P1_31modeEnum::Repeater
    }
    #[doc = "Disabled. P1.31 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode3P1_31modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.31 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode3P1_31modeEnum::PullDown
    }
}
#[doc = "Field `P1_31MODE` writer - Port 1 pin 31 control."]
pub type P1_31modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode3P1_31modeEnum, crate::Safe>;
impl<'a, REG> P1_31modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.31 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_31modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.31 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_31modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.31 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_31modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.31 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode3P1_31modeEnum::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p1_16mode(&self) -> P1_16modeR {
        P1_16modeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p1_17mode(&self) -> P1_17modeR {
        P1_17modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p1_18mode(&self) -> P1_18modeR {
        P1_18modeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p1_19mode(&self) -> P1_19modeR {
        P1_19modeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p1_20mode(&self) -> P1_20modeR {
        P1_20modeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p1_21mode(&self) -> P1_21modeR {
        P1_21modeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p1_22mode(&self) -> P1_22modeR {
        P1_22modeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p1_23mode(&self) -> P1_23modeR {
        P1_23modeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p1_24mode(&self) -> P1_24modeR {
        P1_24modeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p1_25mode(&self) -> P1_25modeR {
        P1_25modeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p1_26mode(&self) -> P1_26modeR {
        P1_26modeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 1 pin 27 control."]
    #[inline(always)]
    pub fn p1_27mode(&self) -> P1_27modeR {
        P1_27modeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port 1 pin 28 control."]
    #[inline(always)]
    pub fn p1_28mode(&self) -> P1_28modeR {
        P1_28modeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port 1 pin 29 control."]
    #[inline(always)]
    pub fn p1_29mode(&self) -> P1_29modeR {
        P1_29modeR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port 1 pin 30 control."]
    #[inline(always)]
    pub fn p1_30mode(&self) -> P1_30modeR {
        P1_30modeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 1 pin 31 control."]
    #[inline(always)]
    pub fn p1_31mode(&self) -> P1_31modeR {
        P1_31modeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p1_16mode(&mut self) -> P1_16modeW<'_, Pinmode3Spec> {
        P1_16modeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p1_17mode(&mut self) -> P1_17modeW<'_, Pinmode3Spec> {
        P1_17modeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p1_18mode(&mut self) -> P1_18modeW<'_, Pinmode3Spec> {
        P1_18modeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p1_19mode(&mut self) -> P1_19modeW<'_, Pinmode3Spec> {
        P1_19modeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p1_20mode(&mut self) -> P1_20modeW<'_, Pinmode3Spec> {
        P1_20modeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p1_21mode(&mut self) -> P1_21modeW<'_, Pinmode3Spec> {
        P1_21modeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p1_22mode(&mut self) -> P1_22modeW<'_, Pinmode3Spec> {
        P1_22modeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p1_23mode(&mut self) -> P1_23modeW<'_, Pinmode3Spec> {
        P1_23modeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p1_24mode(&mut self) -> P1_24modeW<'_, Pinmode3Spec> {
        P1_24modeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p1_25mode(&mut self) -> P1_25modeW<'_, Pinmode3Spec> {
        P1_25modeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p1_26mode(&mut self) -> P1_26modeW<'_, Pinmode3Spec> {
        P1_26modeW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port 1 pin 27 control."]
    #[inline(always)]
    pub fn p1_27mode(&mut self) -> P1_27modeW<'_, Pinmode3Spec> {
        P1_27modeW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port 1 pin 28 control."]
    #[inline(always)]
    pub fn p1_28mode(&mut self) -> P1_28modeW<'_, Pinmode3Spec> {
        P1_28modeW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port 1 pin 29 control."]
    #[inline(always)]
    pub fn p1_29mode(&mut self) -> P1_29modeW<'_, Pinmode3Spec> {
        P1_29modeW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port 1 pin 30 control."]
    #[inline(always)]
    pub fn p1_30mode(&mut self) -> P1_30modeW<'_, Pinmode3Spec> {
        P1_30modeW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port 1 pin 31 control."]
    #[inline(always)]
    pub fn p1_31mode(&mut self) -> P1_31modeW<'_, Pinmode3Spec> {
        P1_31modeW::new(self, 30)
    }
}
#[doc = "Pin mode select register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinmode3Spec;
impl crate::RegisterSpec for Pinmode3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode3::R`](R) reader structure"]
impl crate::Readable for Pinmode3Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode3::W`](W) writer structure"]
impl crate::Writable for Pinmode3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINMODE3 to value 0"]
impl crate::Resettable for Pinmode3Spec {}
