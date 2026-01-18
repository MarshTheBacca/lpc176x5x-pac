#[doc = "Register `I2CPADCFG` reader"]
pub type R = crate::R<I2cpadcfgSpec>;
#[doc = "Register `I2CPADCFG` writer"]
pub type W = crate::W<I2cpadcfgSpec>;
#[doc = "Drive mode control for the SDA0 pin, P0.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectI2cpadcfgSdadrv0Enum {
    #[doc = "0: Standard. The SDA0 pin is in the standard drive mode."]
    Standard = 0,
    #[doc = "1: Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    FastModePlus = 1,
}
impl From<PinconnectI2cpadcfgSdadrv0Enum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectI2cpadcfgSdadrv0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDADRV0` reader - Drive mode control for the SDA0 pin, P0.27."]
pub type Sdadrv0R = crate::BitReader<PinconnectI2cpadcfgSdadrv0Enum>;
impl Sdadrv0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectI2cpadcfgSdadrv0Enum {
        match self.bits {
            false => PinconnectI2cpadcfgSdadrv0Enum::Standard,
            true => PinconnectI2cpadcfgSdadrv0Enum::FastModePlus,
        }
    }
    #[doc = "Standard. The SDA0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == PinconnectI2cpadcfgSdadrv0Enum::Standard
    }
    #[doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == PinconnectI2cpadcfgSdadrv0Enum::FastModePlus
    }
}
#[doc = "Field `SDADRV0` writer - Drive mode control for the SDA0 pin, P0.27."]
pub type Sdadrv0W<'a, REG> = crate::BitWriter<'a, REG, PinconnectI2cpadcfgSdadrv0Enum>;
impl<'a, REG> Sdadrv0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard. The SDA0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectI2cpadcfgSdadrv0Enum::Standard)
    }
    #[doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectI2cpadcfgSdadrv0Enum::FastModePlus)
    }
}
#[doc = "I 2C filter mode control for the SDA0 pin, P0.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectI2cpadcfgSdai2c0Enum {
    #[doc = "0: Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    Enabled = 0,
    #[doc = "1: Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    Disabled = 1,
}
impl From<PinconnectI2cpadcfgSdai2c0Enum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectI2cpadcfgSdai2c0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDAI2C0` reader - I 2C filter mode control for the SDA0 pin, P0.27."]
pub type Sdai2c0R = crate::BitReader<PinconnectI2cpadcfgSdai2c0Enum>;
impl Sdai2c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectI2cpadcfgSdai2c0Enum {
        match self.bits {
            false => PinconnectI2cpadcfgSdai2c0Enum::Enabled,
            true => PinconnectI2cpadcfgSdai2c0Enum::Disabled,
        }
    }
    #[doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PinconnectI2cpadcfgSdai2c0Enum::Enabled
    }
    #[doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectI2cpadcfgSdai2c0Enum::Disabled
    }
}
#[doc = "Field `SDAI2C0` writer - I 2C filter mode control for the SDA0 pin, P0.27."]
pub type Sdai2c0W<'a, REG> = crate::BitWriter<'a, REG, PinconnectI2cpadcfgSdai2c0Enum>;
impl<'a, REG> Sdai2c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectI2cpadcfgSdai2c0Enum::Enabled)
    }
    #[doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectI2cpadcfgSdai2c0Enum::Disabled)
    }
}
#[doc = "Drive mode control for the SCL0 pin, P0.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectI2cpadcfgScldrv0Enum {
    #[doc = "0: Standard. The SCL0 pin is in the standard drive mode."]
    Standard = 0,
    #[doc = "1: Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    FastModePlus = 1,
}
impl From<PinconnectI2cpadcfgScldrv0Enum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectI2cpadcfgScldrv0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLDRV0` reader - Drive mode control for the SCL0 pin, P0.28."]
pub type Scldrv0R = crate::BitReader<PinconnectI2cpadcfgScldrv0Enum>;
impl Scldrv0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectI2cpadcfgScldrv0Enum {
        match self.bits {
            false => PinconnectI2cpadcfgScldrv0Enum::Standard,
            true => PinconnectI2cpadcfgScldrv0Enum::FastModePlus,
        }
    }
    #[doc = "Standard. The SCL0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == PinconnectI2cpadcfgScldrv0Enum::Standard
    }
    #[doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == PinconnectI2cpadcfgScldrv0Enum::FastModePlus
    }
}
#[doc = "Field `SCLDRV0` writer - Drive mode control for the SCL0 pin, P0.28."]
pub type Scldrv0W<'a, REG> = crate::BitWriter<'a, REG, PinconnectI2cpadcfgScldrv0Enum>;
impl<'a, REG> Scldrv0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard. The SCL0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectI2cpadcfgScldrv0Enum::Standard)
    }
    #[doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectI2cpadcfgScldrv0Enum::FastModePlus)
    }
}
#[doc = "I 2C filter mode control for the SCL0 pin, P0.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectI2cpadcfgScli2c0Enum {
    #[doc = "0: Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    Enabled = 0,
    #[doc = "1: Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    Disabled = 1,
}
impl From<PinconnectI2cpadcfgScli2c0Enum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectI2cpadcfgScli2c0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLI2C0` reader - I 2C filter mode control for the SCL0 pin, P0.28."]
pub type Scli2c0R = crate::BitReader<PinconnectI2cpadcfgScli2c0Enum>;
impl Scli2c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectI2cpadcfgScli2c0Enum {
        match self.bits {
            false => PinconnectI2cpadcfgScli2c0Enum::Enabled,
            true => PinconnectI2cpadcfgScli2c0Enum::Disabled,
        }
    }
    #[doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PinconnectI2cpadcfgScli2c0Enum::Enabled
    }
    #[doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectI2cpadcfgScli2c0Enum::Disabled
    }
}
#[doc = "Field `SCLI2C0` writer - I 2C filter mode control for the SCL0 pin, P0.28."]
pub type Scli2c0W<'a, REG> = crate::BitWriter<'a, REG, PinconnectI2cpadcfgScli2c0Enum>;
impl<'a, REG> Scli2c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectI2cpadcfgScli2c0Enum::Enabled)
    }
    #[doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectI2cpadcfgScli2c0Enum::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - Drive mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    pub fn sdadrv0(&self) -> Sdadrv0R {
        Sdadrv0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I 2C filter mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    pub fn sdai2c0(&self) -> Sdai2c0R {
        Sdai2c0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drive mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    pub fn scldrv0(&self) -> Scldrv0R {
        Scldrv0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I 2C filter mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    pub fn scli2c0(&self) -> Scli2c0R {
        Scli2c0R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    pub fn sdadrv0(&mut self) -> Sdadrv0W<'_, I2cpadcfgSpec> {
        Sdadrv0W::new(self, 0)
    }
    #[doc = "Bit 1 - I 2C filter mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    pub fn sdai2c0(&mut self) -> Sdai2c0W<'_, I2cpadcfgSpec> {
        Sdai2c0W::new(self, 1)
    }
    #[doc = "Bit 2 - Drive mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    pub fn scldrv0(&mut self) -> Scldrv0W<'_, I2cpadcfgSpec> {
        Scldrv0W::new(self, 2)
    }
    #[doc = "Bit 3 - I 2C filter mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    pub fn scli2c0(&mut self) -> Scli2c0W<'_, I2cpadcfgSpec> {
        Scli2c0W::new(self, 3)
    }
}
#[doc = "I2C Pin Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cpadcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cpadcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cpadcfgSpec;
impl crate::RegisterSpec for I2cpadcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cpadcfg::R`](R) reader structure"]
impl crate::Readable for I2cpadcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cpadcfg::W`](W) writer structure"]
impl crate::Writable for I2cpadcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CPADCFG to value 0"]
impl crate::Resettable for I2cpadcfgSpec {}
