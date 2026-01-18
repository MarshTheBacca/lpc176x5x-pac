#[doc = "Register `MOD` reader"]
pub type R = crate::R<ModSpec>;
#[doc = "Register `MOD` writer"]
pub type W = crate::W<ModSpec>;
#[doc = "Watchdog enable bit. This bit is Set Only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtModWdenEnum {
    #[doc = "0: The watchdog timer is stopped."]
    Stop = 0,
    #[doc = "1: The watchdog timer is running."]
    Run = 1,
}
impl From<WdtModWdenEnum> for bool {
    #[inline(always)]
    fn from(variant: WdtModWdenEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDEN` reader - Watchdog enable bit. This bit is Set Only."]
pub type WdenR = crate::BitReader<WdtModWdenEnum>;
impl WdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtModWdenEnum {
        match self.bits {
            false => WdtModWdenEnum::Stop,
            true => WdtModWdenEnum::Run,
        }
    }
    #[doc = "The watchdog timer is stopped."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == WdtModWdenEnum::Stop
    }
    #[doc = "The watchdog timer is running."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == WdtModWdenEnum::Run
    }
}
#[doc = "Field `WDEN` writer - Watchdog enable bit. This bit is Set Only."]
pub type WdenW<'a, REG> = crate::BitWriter<'a, REG, WdtModWdenEnum>;
impl<'a, REG> WdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The watchdog timer is stopped."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(WdtModWdenEnum::Stop)
    }
    #[doc = "The watchdog timer is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(WdtModWdenEnum::Run)
    }
}
#[doc = "Watchdog reset enable bit. This bit is Set Only. See Table 652.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtModWdresetEnum {
    #[doc = "0: A watchdog timeout will not cause a chip reset."]
    Noreset = 0,
    #[doc = "1: A watchdog timeout will cause a chip reset."]
    Reset = 1,
}
impl From<WdtModWdresetEnum> for bool {
    #[inline(always)]
    fn from(variant: WdtModWdresetEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDRESET` reader - Watchdog reset enable bit. This bit is Set Only. See Table 652."]
pub type WdresetR = crate::BitReader<WdtModWdresetEnum>;
impl WdresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtModWdresetEnum {
        match self.bits {
            false => WdtModWdresetEnum::Noreset,
            true => WdtModWdresetEnum::Reset,
        }
    }
    #[doc = "A watchdog timeout will not cause a chip reset."]
    #[inline(always)]
    pub fn is_noreset(&self) -> bool {
        *self == WdtModWdresetEnum::Noreset
    }
    #[doc = "A watchdog timeout will cause a chip reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WdtModWdresetEnum::Reset
    }
}
#[doc = "Field `WDRESET` writer - Watchdog reset enable bit. This bit is Set Only. See Table 652."]
pub type WdresetW<'a, REG> = crate::BitWriter<'a, REG, WdtModWdresetEnum>;
impl<'a, REG> WdresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A watchdog timeout will not cause a chip reset."]
    #[inline(always)]
    pub fn noreset(self) -> &'a mut crate::W<REG> {
        self.variant(WdtModWdresetEnum::Noreset)
    }
    #[doc = "A watchdog timeout will cause a chip reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(WdtModWdresetEnum::Reset)
    }
}
#[doc = "Field `WDTOF` reader - Watchdog time-out flag. Set when the watchdog timer times out, cleared by software."]
pub type WdtofR = crate::BitReader;
#[doc = "Field `WDTOF` writer - Watchdog time-out flag. Set when the watchdog timer times out, cleared by software."]
pub type WdtofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDINT` reader - Watchdog interrupt flag. Cleared by software."]
pub type WdintR = crate::BitReader;
#[doc = "Field `WDINT` writer - Watchdog interrupt flag. Cleared by software."]
pub type WdintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog enable bit. This bit is Set Only."]
    #[inline(always)]
    pub fn wden(&self) -> WdenR {
        WdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only. See Table 652."]
    #[inline(always)]
    pub fn wdreset(&self) -> WdresetR {
        WdresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, cleared by software."]
    #[inline(always)]
    pub fn wdtof(&self) -> WdtofR {
        WdtofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog interrupt flag. Cleared by software."]
    #[inline(always)]
    pub fn wdint(&self) -> WdintR {
        WdintR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog enable bit. This bit is Set Only."]
    #[inline(always)]
    pub fn wden(&mut self) -> WdenW<'_, ModSpec> {
        WdenW::new(self, 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only. See Table 652."]
    #[inline(always)]
    pub fn wdreset(&mut self) -> WdresetW<'_, ModSpec> {
        WdresetW::new(self, 1)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, cleared by software."]
    #[inline(always)]
    pub fn wdtof(&mut self) -> WdtofW<'_, ModSpec> {
        WdtofW::new(self, 2)
    }
    #[doc = "Bit 3 - Watchdog interrupt flag. Cleared by software."]
    #[inline(always)]
    pub fn wdint(&mut self) -> WdintW<'_, ModSpec> {
        WdintW::new(self, 3)
    }
}
#[doc = "Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`mod_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModSpec;
impl crate::RegisterSpec for ModSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mod_::R`](R) reader structure"]
impl crate::Readable for ModSpec {}
#[doc = "`write(|w| ..)` method takes [`mod_::W`](W) writer structure"]
impl crate::Writable for ModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOD to value 0"]
impl crate::Resettable for ModSpec {}
