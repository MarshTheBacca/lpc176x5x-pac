#[doc = "Register `PINMODE_OD4` reader"]
pub type R = crate::R<PinmodeOd4Spec>;
#[doc = "Register `PINMODE_OD4` writer"]
pub type W = crate::W<PinmodeOd4Spec>;
#[doc = "Port 4 pin 28 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd4P4_28odEnum {
    #[doc = "0: Normal. P4.28 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P4.28 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd4P4_28odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd4P4_28odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P4_28OD` reader - Port 4 pin 28 open drain mode control."]
pub type P4_28odR = crate::BitReader<PinconnectPinmodeOd4P4_28odEnum>;
impl P4_28odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd4P4_28odEnum {
        match self.bits {
            false => PinconnectPinmodeOd4P4_28odEnum::Normal,
            true => PinconnectPinmodeOd4P4_28odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P4.28 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd4P4_28odEnum::Normal
    }
    #[doc = "Open-drain. P4.28 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd4P4_28odEnum::OpenDrain
    }
}
#[doc = "Field `P4_28OD` writer - Port 4 pin 28 open drain mode control."]
pub type P4_28odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd4P4_28odEnum>;
impl<'a, REG> P4_28odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P4.28 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd4P4_28odEnum::Normal)
    }
    #[doc = "Open-drain. P4.28 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd4P4_28odEnum::OpenDrain)
    }
}
#[doc = "Field `P4_29OD` reader - Port 4 pin 29 open drain mode control, see P4.28OD"]
pub type P4_29odR = crate::BitReader;
#[doc = "Field `P4_29OD` writer - Port 4 pin 29 open drain mode control, see P4.28OD"]
pub type P4_29odW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - Port 4 pin 28 open drain mode control."]
    #[inline(always)]
    pub fn p4_28od(&self) -> P4_28odR {
        P4_28odR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port 4 pin 29 open drain mode control, see P4.28OD"]
    #[inline(always)]
    pub fn p4_29od(&self) -> P4_29odR {
        P4_29odR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Port 4 pin 28 open drain mode control."]
    #[inline(always)]
    pub fn p4_28od(&mut self) -> P4_28odW<'_, PinmodeOd4Spec> {
        P4_28odW::new(self, 28)
    }
    #[doc = "Bit 29 - Port 4 pin 29 open drain mode control, see P4.28OD"]
    #[inline(always)]
    pub fn p4_29od(&mut self) -> P4_29odW<'_, PinmodeOd4Spec> {
        P4_29odW::new(self, 29)
    }
}
#[doc = "Open drain mode control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinmodeOd4Spec;
impl crate::RegisterSpec for PinmodeOd4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode_od4::R`](R) reader structure"]
impl crate::Readable for PinmodeOd4Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode_od4::W`](W) writer structure"]
impl crate::Writable for PinmodeOd4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINMODE_OD4 to value 0"]
impl crate::Resettable for PinmodeOd4Spec {}
