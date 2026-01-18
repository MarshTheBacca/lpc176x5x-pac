#[doc = "Register `PINSEL9` reader"]
pub type R = crate::R<Pinsel9Spec>;
#[doc = "Register `PINSEL9` writer"]
pub type W = crate::W<Pinsel9Spec>;
#[doc = "Pin function select P4.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel9P4_28Enum {
    #[doc = "0: GPIO P4.28"]
    GpioP4 = 0,
    #[doc = "1: RX_MCLK"]
    RxMclk = 1,
    #[doc = "2: MAT2.0"]
    Mat2 = 2,
    #[doc = "3: TXD3"]
    Txd3 = 3,
}
impl From<PinconnectPinsel9P4_28Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel9P4_28Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel9P4_28Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel9P4_28Enum {}
#[doc = "Field `P4_28` reader - Pin function select P4.28."]
pub type P4_28R = crate::FieldReader<PinconnectPinsel9P4_28Enum>;
impl P4_28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel9P4_28Enum {
        match self.bits {
            0 => PinconnectPinsel9P4_28Enum::GpioP4,
            1 => PinconnectPinsel9P4_28Enum::RxMclk,
            2 => PinconnectPinsel9P4_28Enum::Mat2,
            3 => PinconnectPinsel9P4_28Enum::Txd3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P4.28"]
    #[inline(always)]
    pub fn is_gpio_p4(&self) -> bool {
        *self == PinconnectPinsel9P4_28Enum::GpioP4
    }
    #[doc = "RX_MCLK"]
    #[inline(always)]
    pub fn is_rx_mclk(&self) -> bool {
        *self == PinconnectPinsel9P4_28Enum::RxMclk
    }
    #[doc = "MAT2.0"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == PinconnectPinsel9P4_28Enum::Mat2
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn is_txd3(&self) -> bool {
        *self == PinconnectPinsel9P4_28Enum::Txd3
    }
}
#[doc = "Field `P4_28` writer - Pin function select P4.28."]
pub type P4_28W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel9P4_28Enum, crate::Safe>;
impl<'a, REG> P4_28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P4.28"]
    #[inline(always)]
    pub fn gpio_p4(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel9P4_28Enum::GpioP4)
    }
    #[doc = "RX_MCLK"]
    #[inline(always)]
    pub fn rx_mclk(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel9P4_28Enum::RxMclk)
    }
    #[doc = "MAT2.0"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel9P4_28Enum::Mat2)
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn txd3(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel9P4_28Enum::Txd3)
    }
}
#[doc = "Pin function select P4.29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel9P4_29Enum {
    #[doc = "0: GPIO P4.29"]
    GpioP4 = 0,
    #[doc = "1: TX_MCLK"]
    TxMclk = 1,
    #[doc = "2: MAT2.1"]
    Mat2 = 2,
    #[doc = "3: RXD3"]
    Rxd3 = 3,
}
impl From<PinconnectPinsel9P4_29Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel9P4_29Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel9P4_29Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel9P4_29Enum {}
#[doc = "Field `P4_29` reader - Pin function select P4.29."]
pub type P4_29R = crate::FieldReader<PinconnectPinsel9P4_29Enum>;
impl P4_29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel9P4_29Enum {
        match self.bits {
            0 => PinconnectPinsel9P4_29Enum::GpioP4,
            1 => PinconnectPinsel9P4_29Enum::TxMclk,
            2 => PinconnectPinsel9P4_29Enum::Mat2,
            3 => PinconnectPinsel9P4_29Enum::Rxd3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P4.29"]
    #[inline(always)]
    pub fn is_gpio_p4(&self) -> bool {
        *self == PinconnectPinsel9P4_29Enum::GpioP4
    }
    #[doc = "TX_MCLK"]
    #[inline(always)]
    pub fn is_tx_mclk(&self) -> bool {
        *self == PinconnectPinsel9P4_29Enum::TxMclk
    }
    #[doc = "MAT2.1"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == PinconnectPinsel9P4_29Enum::Mat2
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn is_rxd3(&self) -> bool {
        *self == PinconnectPinsel9P4_29Enum::Rxd3
    }
}
#[doc = "Field `P4_29` writer - Pin function select P4.29."]
pub type P4_29W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel9P4_29Enum, crate::Safe>;
impl<'a, REG> P4_29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P4.29"]
    #[inline(always)]
    pub fn gpio_p4(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel9P4_29Enum::GpioP4)
    }
    #[doc = "TX_MCLK"]
    #[inline(always)]
    pub fn tx_mclk(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel9P4_29Enum::TxMclk)
    }
    #[doc = "MAT2.1"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel9P4_29Enum::Mat2)
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn rxd3(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel9P4_29Enum::Rxd3)
    }
}
impl R {
    #[doc = "Bits 24:25 - Pin function select P4.28."]
    #[inline(always)]
    pub fn p4_28(&self) -> P4_28R {
        P4_28R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P4.29."]
    #[inline(always)]
    pub fn p4_29(&self) -> P4_29R {
        P4_29R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25 - Pin function select P4.28."]
    #[inline(always)]
    pub fn p4_28(&mut self) -> P4_28W<'_, Pinsel9Spec> {
        P4_28W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Pin function select P4.29."]
    #[inline(always)]
    pub fn p4_29(&mut self) -> P4_29W<'_, Pinsel9Spec> {
        P4_29W::new(self, 26)
    }
}
#[doc = "Pin function select register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinsel9Spec;
impl crate::RegisterSpec for Pinsel9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel9::R`](R) reader structure"]
impl crate::Readable for Pinsel9Spec {}
#[doc = "`write(|w| ..)` method takes [`pinsel9::W`](W) writer structure"]
impl crate::Writable for Pinsel9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINSEL9 to value 0"]
impl crate::Resettable for Pinsel9Spec {}
