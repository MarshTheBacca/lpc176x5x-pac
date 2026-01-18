#[doc = "Register `PINMODE9` reader"]
pub type R = crate::R<Pinmode9Spec>;
#[doc = "Register `PINMODE9` writer"]
pub type W = crate::W<Pinmode9Spec>;
#[doc = "Port 4 pin 28 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode9P4_28modeEnum {
    #[doc = "0: Pull-up. P4.28 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P4.28 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P4.28 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P4.28 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode9P4_28modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode9P4_28modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode9P4_28modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode9P4_28modeEnum {}
#[doc = "Field `P4_28MODE` reader - Port 4 pin 28 control."]
pub type P4_28modeR = crate::FieldReader<PinconnectPinmode9P4_28modeEnum>;
impl P4_28modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode9P4_28modeEnum {
        match self.bits {
            0 => PinconnectPinmode9P4_28modeEnum::PullUp,
            1 => PinconnectPinmode9P4_28modeEnum::Repeater,
            2 => PinconnectPinmode9P4_28modeEnum::Disabled,
            3 => PinconnectPinmode9P4_28modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P4.28 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode9P4_28modeEnum::PullUp
    }
    #[doc = "Repeater. P4.28 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode9P4_28modeEnum::Repeater
    }
    #[doc = "Disabled. P4.28 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode9P4_28modeEnum::Disabled
    }
    #[doc = "Pull-down. P4.28 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode9P4_28modeEnum::PullDown
    }
}
#[doc = "Field `P4_28MODE` writer - Port 4 pin 28 control."]
pub type P4_28modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode9P4_28modeEnum, crate::Safe>;
impl<'a, REG> P4_28modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P4.28 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode9P4_28modeEnum::PullUp)
    }
    #[doc = "Repeater. P4.28 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode9P4_28modeEnum::Repeater)
    }
    #[doc = "Disabled. P4.28 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode9P4_28modeEnum::Disabled)
    }
    #[doc = "Pull-down. P4.28 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode9P4_28modeEnum::PullDown)
    }
}
#[doc = "Port 4 pin 29 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode9P4_29modeEnum {
    #[doc = "0: Pull-up. P4.29 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P4.29 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P4.29 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P4.29 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode9P4_29modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode9P4_29modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode9P4_29modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode9P4_29modeEnum {}
#[doc = "Field `P4_29MODE` reader - Port 4 pin 29 control."]
pub type P4_29modeR = crate::FieldReader<PinconnectPinmode9P4_29modeEnum>;
impl P4_29modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode9P4_29modeEnum {
        match self.bits {
            0 => PinconnectPinmode9P4_29modeEnum::PullUp,
            1 => PinconnectPinmode9P4_29modeEnum::Repeater,
            2 => PinconnectPinmode9P4_29modeEnum::Disabled,
            3 => PinconnectPinmode9P4_29modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P4.29 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode9P4_29modeEnum::PullUp
    }
    #[doc = "Repeater. P4.29 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode9P4_29modeEnum::Repeater
    }
    #[doc = "Disabled. P4.29 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode9P4_29modeEnum::Disabled
    }
    #[doc = "Pull-down. P4.29 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode9P4_29modeEnum::PullDown
    }
}
#[doc = "Field `P4_29MODE` writer - Port 4 pin 29 control."]
pub type P4_29modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode9P4_29modeEnum, crate::Safe>;
impl<'a, REG> P4_29modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P4.29 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode9P4_29modeEnum::PullUp)
    }
    #[doc = "Repeater. P4.29 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode9P4_29modeEnum::Repeater)
    }
    #[doc = "Disabled. P4.29 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode9P4_29modeEnum::Disabled)
    }
    #[doc = "Pull-down. P4.29 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode9P4_29modeEnum::PullDown)
    }
}
impl R {
    #[doc = "Bits 24:25 - Port 4 pin 28 control."]
    #[inline(always)]
    pub fn p4_28mode(&self) -> P4_28modeR {
        P4_28modeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port 4 pin 29 control."]
    #[inline(always)]
    pub fn p4_29mode(&self) -> P4_29modeR {
        P4_29modeR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25 - Port 4 pin 28 control."]
    #[inline(always)]
    pub fn p4_28mode(&mut self) -> P4_28modeW<'_, Pinmode9Spec> {
        P4_28modeW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port 4 pin 29 control."]
    #[inline(always)]
    pub fn p4_29mode(&mut self) -> P4_29modeW<'_, Pinmode9Spec> {
        P4_29modeW::new(self, 26)
    }
}
#[doc = "Pin mode select register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinmode9Spec;
impl crate::RegisterSpec for Pinmode9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode9::R`](R) reader structure"]
impl crate::Readable for Pinmode9Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode9::W`](W) writer structure"]
impl crate::Writable for Pinmode9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINMODE9 to value 0"]
impl crate::Resettable for Pinmode9Spec {}
