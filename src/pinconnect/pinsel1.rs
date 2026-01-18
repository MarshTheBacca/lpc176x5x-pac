#[doc = "Register `PINSEL1` reader"]
pub type R = crate::R<Pinsel1Spec>;
#[doc = "Register `PINSEL1` writer"]
pub type W = crate::W<Pinsel1Spec>;
#[doc = "Pin function select P0.16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_16Enum {
    #[doc = "0: GPIO P0.16"]
    GpioP0 = 0,
    #[doc = "1: RXD1"]
    Rxd1 = 1,
    #[doc = "2: SSEL0"]
    Ssel0 = 2,
    #[doc = "3: SSEL"]
    Ssel = 3,
}
impl From<PinconnectPinsel1P0_16Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_16Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_16Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_16Enum {}
#[doc = "Field `P0_16` reader - Pin function select P0.16."]
pub type P0_16R = crate::FieldReader<PinconnectPinsel1P0_16Enum>;
impl P0_16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_16Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_16Enum::GpioP0,
            1 => PinconnectPinsel1P0_16Enum::Rxd1,
            2 => PinconnectPinsel1P0_16Enum::Ssel0,
            3 => PinconnectPinsel1P0_16Enum::Ssel,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.16"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == PinconnectPinsel1P0_16Enum::GpioP0
    }
    #[doc = "RXD1"]
    #[inline(always)]
    pub fn is_rxd1(&self) -> bool {
        *self == PinconnectPinsel1P0_16Enum::Rxd1
    }
    #[doc = "SSEL0"]
    #[inline(always)]
    pub fn is_ssel0(&self) -> bool {
        *self == PinconnectPinsel1P0_16Enum::Ssel0
    }
    #[doc = "SSEL"]
    #[inline(always)]
    pub fn is_ssel(&self) -> bool {
        *self == PinconnectPinsel1P0_16Enum::Ssel
    }
}
#[doc = "Field `P0_16` writer - Pin function select P0.16."]
pub type P0_16W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_16Enum, crate::Safe>;
impl<'a, REG> P0_16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.16"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_16Enum::GpioP0)
    }
    #[doc = "RXD1"]
    #[inline(always)]
    pub fn rxd1(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_16Enum::Rxd1)
    }
    #[doc = "SSEL0"]
    #[inline(always)]
    pub fn ssel0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_16Enum::Ssel0)
    }
    #[doc = "SSEL"]
    #[inline(always)]
    pub fn ssel(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_16Enum::Ssel)
    }
}
#[doc = "Pin function select P0.17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_17Enum {
    #[doc = "0: GPIO P0.17"]
    GpioP0 = 0,
    #[doc = "1: CTS1"]
    Cts1 = 1,
    #[doc = "2: MISO0"]
    Miso0 = 2,
    #[doc = "3: MISO"]
    Miso = 3,
}
impl From<PinconnectPinsel1P0_17Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_17Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_17Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_17Enum {}
#[doc = "Field `P0_17` reader - Pin function select P0.17."]
pub type P0_17R = crate::FieldReader<PinconnectPinsel1P0_17Enum>;
impl P0_17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_17Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_17Enum::GpioP0,
            1 => PinconnectPinsel1P0_17Enum::Cts1,
            2 => PinconnectPinsel1P0_17Enum::Miso0,
            3 => PinconnectPinsel1P0_17Enum::Miso,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.17"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == PinconnectPinsel1P0_17Enum::GpioP0
    }
    #[doc = "CTS1"]
    #[inline(always)]
    pub fn is_cts1(&self) -> bool {
        *self == PinconnectPinsel1P0_17Enum::Cts1
    }
    #[doc = "MISO0"]
    #[inline(always)]
    pub fn is_miso0(&self) -> bool {
        *self == PinconnectPinsel1P0_17Enum::Miso0
    }
    #[doc = "MISO"]
    #[inline(always)]
    pub fn is_miso(&self) -> bool {
        *self == PinconnectPinsel1P0_17Enum::Miso
    }
}
#[doc = "Field `P0_17` writer - Pin function select P0.17."]
pub type P0_17W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_17Enum, crate::Safe>;
impl<'a, REG> P0_17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.17"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_17Enum::GpioP0)
    }
    #[doc = "CTS1"]
    #[inline(always)]
    pub fn cts1(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_17Enum::Cts1)
    }
    #[doc = "MISO0"]
    #[inline(always)]
    pub fn miso0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_17Enum::Miso0)
    }
    #[doc = "MISO"]
    #[inline(always)]
    pub fn miso(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_17Enum::Miso)
    }
}
#[doc = "Pin function select P0.18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_18Enum {
    #[doc = "0: GPIO P0.18"]
    GpioP0 = 0,
    #[doc = "1: DCD1"]
    Dcd1 = 1,
    #[doc = "2: MOSI0"]
    Mosi0 = 2,
    #[doc = "3: MOSI"]
    Mosi = 3,
}
impl From<PinconnectPinsel1P0_18Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_18Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_18Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_18Enum {}
#[doc = "Field `P0_18` reader - Pin function select P0.18."]
pub type P0_18R = crate::FieldReader<PinconnectPinsel1P0_18Enum>;
impl P0_18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_18Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_18Enum::GpioP0,
            1 => PinconnectPinsel1P0_18Enum::Dcd1,
            2 => PinconnectPinsel1P0_18Enum::Mosi0,
            3 => PinconnectPinsel1P0_18Enum::Mosi,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.18"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == PinconnectPinsel1P0_18Enum::GpioP0
    }
    #[doc = "DCD1"]
    #[inline(always)]
    pub fn is_dcd1(&self) -> bool {
        *self == PinconnectPinsel1P0_18Enum::Dcd1
    }
    #[doc = "MOSI0"]
    #[inline(always)]
    pub fn is_mosi0(&self) -> bool {
        *self == PinconnectPinsel1P0_18Enum::Mosi0
    }
    #[doc = "MOSI"]
    #[inline(always)]
    pub fn is_mosi(&self) -> bool {
        *self == PinconnectPinsel1P0_18Enum::Mosi
    }
}
#[doc = "Field `P0_18` writer - Pin function select P0.18."]
pub type P0_18W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_18Enum, crate::Safe>;
impl<'a, REG> P0_18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.18"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_18Enum::GpioP0)
    }
    #[doc = "DCD1"]
    #[inline(always)]
    pub fn dcd1(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_18Enum::Dcd1)
    }
    #[doc = "MOSI0"]
    #[inline(always)]
    pub fn mosi0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_18Enum::Mosi0)
    }
    #[doc = "MOSI"]
    #[inline(always)]
    pub fn mosi(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_18Enum::Mosi)
    }
}
#[doc = "Pin function select P019.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_19Enum {
    #[doc = "0: GPIO P0.19."]
    GpioP0 = 0,
    #[doc = "1: DSR1"]
    Dsr1 = 1,
    #[doc = "3: SDA1"]
    Sda1 = 3,
}
impl From<PinconnectPinsel1P0_19Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_19Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_19Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_19Enum {}
#[doc = "Field `P0_19` reader - Pin function select P019."]
pub type P0_19R = crate::FieldReader<PinconnectPinsel1P0_19Enum>;
impl P0_19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_19Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_19Enum::GpioP0,
            1 => PinconnectPinsel1P0_19Enum::Dsr1,
            3 => PinconnectPinsel1P0_19Enum::Sda1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.19."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == PinconnectPinsel1P0_19Enum::GpioP0
    }
    #[doc = "DSR1"]
    #[inline(always)]
    pub fn is_dsr1(&self) -> bool {
        *self == PinconnectPinsel1P0_19Enum::Dsr1
    }
    #[doc = "SDA1"]
    #[inline(always)]
    pub fn is_sda1(&self) -> bool {
        *self == PinconnectPinsel1P0_19Enum::Sda1
    }
}
#[doc = "Field `P0_19` writer - Pin function select P019."]
pub type P0_19W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_19Enum>;
impl<'a, REG> P0_19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.19."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_19Enum::GpioP0)
    }
    #[doc = "DSR1"]
    #[inline(always)]
    pub fn dsr1(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_19Enum::Dsr1)
    }
    #[doc = "SDA1"]
    #[inline(always)]
    pub fn sda1(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_19Enum::Sda1)
    }
}
#[doc = "Pin function select P0.20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_20Enum {
    #[doc = "0: GPIO P0.20."]
    GpioP0 = 0,
    #[doc = "1: DTR1"]
    Dtr1 = 1,
    #[doc = "3: SCL1"]
    Scl1 = 3,
}
impl From<PinconnectPinsel1P0_20Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_20Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_20Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_20Enum {}
#[doc = "Field `P0_20` reader - Pin function select P0.20."]
pub type P0_20R = crate::FieldReader<PinconnectPinsel1P0_20Enum>;
impl P0_20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_20Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_20Enum::GpioP0,
            1 => PinconnectPinsel1P0_20Enum::Dtr1,
            3 => PinconnectPinsel1P0_20Enum::Scl1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.20."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == PinconnectPinsel1P0_20Enum::GpioP0
    }
    #[doc = "DTR1"]
    #[inline(always)]
    pub fn is_dtr1(&self) -> bool {
        *self == PinconnectPinsel1P0_20Enum::Dtr1
    }
    #[doc = "SCL1"]
    #[inline(always)]
    pub fn is_scl1(&self) -> bool {
        *self == PinconnectPinsel1P0_20Enum::Scl1
    }
}
#[doc = "Field `P0_20` writer - Pin function select P0.20."]
pub type P0_20W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_20Enum>;
impl<'a, REG> P0_20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.20."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_20Enum::GpioP0)
    }
    #[doc = "DTR1"]
    #[inline(always)]
    pub fn dtr1(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_20Enum::Dtr1)
    }
    #[doc = "SCL1"]
    #[inline(always)]
    pub fn scl1(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_20Enum::Scl1)
    }
}
#[doc = "Pin function select P0.21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_21Enum {
    #[doc = "0: GPIO Port 0.21."]
    GpioPort0 = 0,
    #[doc = "1: RI1"]
    Ri1 = 1,
    #[doc = "3: RD1"]
    Rd1 = 3,
}
impl From<PinconnectPinsel1P0_21Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_21Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_21Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_21Enum {}
#[doc = "Field `P0_21` reader - Pin function select P0.21."]
pub type P0_21R = crate::FieldReader<PinconnectPinsel1P0_21Enum>;
impl P0_21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_21Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_21Enum::GpioPort0,
            1 => PinconnectPinsel1P0_21Enum::Ri1,
            3 => PinconnectPinsel1P0_21Enum::Rd1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO Port 0.21."]
    #[inline(always)]
    pub fn is_gpio_port_0(&self) -> bool {
        *self == PinconnectPinsel1P0_21Enum::GpioPort0
    }
    #[doc = "RI1"]
    #[inline(always)]
    pub fn is_ri1(&self) -> bool {
        *self == PinconnectPinsel1P0_21Enum::Ri1
    }
    #[doc = "RD1"]
    #[inline(always)]
    pub fn is_rd1(&self) -> bool {
        *self == PinconnectPinsel1P0_21Enum::Rd1
    }
}
#[doc = "Field `P0_21` writer - Pin function select P0.21."]
pub type P0_21W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_21Enum>;
impl<'a, REG> P0_21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO Port 0.21."]
    #[inline(always)]
    pub fn gpio_port_0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_21Enum::GpioPort0)
    }
    #[doc = "RI1"]
    #[inline(always)]
    pub fn ri1(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_21Enum::Ri1)
    }
    #[doc = "RD1"]
    #[inline(always)]
    pub fn rd1(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_21Enum::Rd1)
    }
}
#[doc = "Pin function select P022\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_22Enum {
    #[doc = "0: GPIO P0.22."]
    GpioP0 = 0,
    #[doc = "1: RTS1"]
    Rts1 = 1,
    #[doc = "3: TD1"]
    Td1 = 3,
}
impl From<PinconnectPinsel1P0_22Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_22Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_22Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_22Enum {}
#[doc = "Field `P0_22` reader - Pin function select P022"]
pub type P0_22R = crate::FieldReader<PinconnectPinsel1P0_22Enum>;
impl P0_22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_22Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_22Enum::GpioP0,
            1 => PinconnectPinsel1P0_22Enum::Rts1,
            3 => PinconnectPinsel1P0_22Enum::Td1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.22."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == PinconnectPinsel1P0_22Enum::GpioP0
    }
    #[doc = "RTS1"]
    #[inline(always)]
    pub fn is_rts1(&self) -> bool {
        *self == PinconnectPinsel1P0_22Enum::Rts1
    }
    #[doc = "TD1"]
    #[inline(always)]
    pub fn is_td1(&self) -> bool {
        *self == PinconnectPinsel1P0_22Enum::Td1
    }
}
#[doc = "Field `P0_22` writer - Pin function select P022"]
pub type P0_22W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_22Enum>;
impl<'a, REG> P0_22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.22."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_22Enum::GpioP0)
    }
    #[doc = "RTS1"]
    #[inline(always)]
    pub fn rts1(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_22Enum::Rts1)
    }
    #[doc = "TD1"]
    #[inline(always)]
    pub fn td1(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_22Enum::Td1)
    }
}
#[doc = "Pin function select P023.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_23Enum {
    #[doc = "0: GPIO P0.23."]
    GpioP0 = 0,
    #[doc = "1: AD0.0"]
    Ad0 = 1,
    #[doc = "2: I2SRX_CLK"]
    I2srxClk = 2,
    #[doc = "3: CAP3.0"]
    Cap3 = 3,
}
impl From<PinconnectPinsel1P0_23Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_23Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_23Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_23Enum {}
#[doc = "Field `P0_23` reader - Pin function select P023."]
pub type P0_23R = crate::FieldReader<PinconnectPinsel1P0_23Enum>;
impl P0_23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_23Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_23Enum::GpioP0,
            1 => PinconnectPinsel1P0_23Enum::Ad0,
            2 => PinconnectPinsel1P0_23Enum::I2srxClk,
            3 => PinconnectPinsel1P0_23Enum::Cap3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.23."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == PinconnectPinsel1P0_23Enum::GpioP0
    }
    #[doc = "AD0.0"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == PinconnectPinsel1P0_23Enum::Ad0
    }
    #[doc = "I2SRX_CLK"]
    #[inline(always)]
    pub fn is_i2srx_clk(&self) -> bool {
        *self == PinconnectPinsel1P0_23Enum::I2srxClk
    }
    #[doc = "CAP3.0"]
    #[inline(always)]
    pub fn is_cap3(&self) -> bool {
        *self == PinconnectPinsel1P0_23Enum::Cap3
    }
}
#[doc = "Field `P0_23` writer - Pin function select P023."]
pub type P0_23W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_23Enum, crate::Safe>;
impl<'a, REG> P0_23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.23."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_23Enum::GpioP0)
    }
    #[doc = "AD0.0"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_23Enum::Ad0)
    }
    #[doc = "I2SRX_CLK"]
    #[inline(always)]
    pub fn i2srx_clk(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_23Enum::I2srxClk)
    }
    #[doc = "CAP3.0"]
    #[inline(always)]
    pub fn cap3(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_23Enum::Cap3)
    }
}
#[doc = "Pin function select P0.24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_24Enum {
    #[doc = "0: GPIO P0.24."]
    GpioP0 = 0,
    #[doc = "1: AD0.1"]
    Ad0 = 1,
    #[doc = "2: I2SRX_WS"]
    I2srxWs = 2,
    #[doc = "3: CAP3.1"]
    Cap3 = 3,
}
impl From<PinconnectPinsel1P0_24Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_24Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_24Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_24Enum {}
#[doc = "Field `P0_24` reader - Pin function select P0.24."]
pub type P0_24R = crate::FieldReader<PinconnectPinsel1P0_24Enum>;
impl P0_24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_24Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_24Enum::GpioP0,
            1 => PinconnectPinsel1P0_24Enum::Ad0,
            2 => PinconnectPinsel1P0_24Enum::I2srxWs,
            3 => PinconnectPinsel1P0_24Enum::Cap3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.24."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == PinconnectPinsel1P0_24Enum::GpioP0
    }
    #[doc = "AD0.1"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == PinconnectPinsel1P0_24Enum::Ad0
    }
    #[doc = "I2SRX_WS"]
    #[inline(always)]
    pub fn is_i2srx_ws(&self) -> bool {
        *self == PinconnectPinsel1P0_24Enum::I2srxWs
    }
    #[doc = "CAP3.1"]
    #[inline(always)]
    pub fn is_cap3(&self) -> bool {
        *self == PinconnectPinsel1P0_24Enum::Cap3
    }
}
#[doc = "Field `P0_24` writer - Pin function select P0.24."]
pub type P0_24W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_24Enum, crate::Safe>;
impl<'a, REG> P0_24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.24."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_24Enum::GpioP0)
    }
    #[doc = "AD0.1"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_24Enum::Ad0)
    }
    #[doc = "I2SRX_WS"]
    #[inline(always)]
    pub fn i2srx_ws(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_24Enum::I2srxWs)
    }
    #[doc = "CAP3.1"]
    #[inline(always)]
    pub fn cap3(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_24Enum::Cap3)
    }
}
#[doc = "Pin function select P0.25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_25Enum {
    #[doc = "0: GPIO P0.25"]
    GpioP0 = 0,
    #[doc = "1: AD0.2"]
    Ad0 = 1,
    #[doc = "2: I2SRX_SDA"]
    I2srxSda = 2,
    #[doc = "3: TXD3"]
    Txd3 = 3,
}
impl From<PinconnectPinsel1P0_25Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_25Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_25Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_25Enum {}
#[doc = "Field `P0_25` reader - Pin function select P0.25."]
pub type P0_25R = crate::FieldReader<PinconnectPinsel1P0_25Enum>;
impl P0_25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_25Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_25Enum::GpioP0,
            1 => PinconnectPinsel1P0_25Enum::Ad0,
            2 => PinconnectPinsel1P0_25Enum::I2srxSda,
            3 => PinconnectPinsel1P0_25Enum::Txd3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.25"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == PinconnectPinsel1P0_25Enum::GpioP0
    }
    #[doc = "AD0.2"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == PinconnectPinsel1P0_25Enum::Ad0
    }
    #[doc = "I2SRX_SDA"]
    #[inline(always)]
    pub fn is_i2srx_sda(&self) -> bool {
        *self == PinconnectPinsel1P0_25Enum::I2srxSda
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn is_txd3(&self) -> bool {
        *self == PinconnectPinsel1P0_25Enum::Txd3
    }
}
#[doc = "Field `P0_25` writer - Pin function select P0.25."]
pub type P0_25W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_25Enum, crate::Safe>;
impl<'a, REG> P0_25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.25"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_25Enum::GpioP0)
    }
    #[doc = "AD0.2"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_25Enum::Ad0)
    }
    #[doc = "I2SRX_SDA"]
    #[inline(always)]
    pub fn i2srx_sda(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_25Enum::I2srxSda)
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn txd3(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_25Enum::Txd3)
    }
}
#[doc = "Pin function select P0.26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_26Enum {
    #[doc = "0: GPIO P0.26"]
    GpioP0 = 0,
    #[doc = "1: AD0.3"]
    Ad0 = 1,
    #[doc = "2: AOUT"]
    Aout = 2,
    #[doc = "3: RXD3"]
    Rxd3 = 3,
}
impl From<PinconnectPinsel1P0_26Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_26Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_26Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_26Enum {}
#[doc = "Field `P0_26` reader - Pin function select P0.26."]
pub type P0_26R = crate::FieldReader<PinconnectPinsel1P0_26Enum>;
impl P0_26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_26Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_26Enum::GpioP0,
            1 => PinconnectPinsel1P0_26Enum::Ad0,
            2 => PinconnectPinsel1P0_26Enum::Aout,
            3 => PinconnectPinsel1P0_26Enum::Rxd3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.26"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == PinconnectPinsel1P0_26Enum::GpioP0
    }
    #[doc = "AD0.3"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == PinconnectPinsel1P0_26Enum::Ad0
    }
    #[doc = "AOUT"]
    #[inline(always)]
    pub fn is_aout(&self) -> bool {
        *self == PinconnectPinsel1P0_26Enum::Aout
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn is_rxd3(&self) -> bool {
        *self == PinconnectPinsel1P0_26Enum::Rxd3
    }
}
#[doc = "Field `P0_26` writer - Pin function select P0.26."]
pub type P0_26W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_26Enum, crate::Safe>;
impl<'a, REG> P0_26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.26"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_26Enum::GpioP0)
    }
    #[doc = "AD0.3"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_26Enum::Ad0)
    }
    #[doc = "AOUT"]
    #[inline(always)]
    pub fn aout(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_26Enum::Aout)
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn rxd3(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_26Enum::Rxd3)
    }
}
#[doc = "Pin function select P0.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_27Enum {
    #[doc = "0: GPIO P0.27"]
    GpioP0 = 0,
    #[doc = "1: SDA0"]
    Sda0 = 1,
    #[doc = "2: USB_SDA"]
    UsbSda = 2,
}
impl From<PinconnectPinsel1P0_27Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_27Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_27Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_27Enum {}
#[doc = "Field `P0_27` reader - Pin function select P0.27."]
pub type P0_27R = crate::FieldReader<PinconnectPinsel1P0_27Enum>;
impl P0_27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_27Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_27Enum::GpioP0,
            1 => PinconnectPinsel1P0_27Enum::Sda0,
            2 => PinconnectPinsel1P0_27Enum::UsbSda,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.27"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == PinconnectPinsel1P0_27Enum::GpioP0
    }
    #[doc = "SDA0"]
    #[inline(always)]
    pub fn is_sda0(&self) -> bool {
        *self == PinconnectPinsel1P0_27Enum::Sda0
    }
    #[doc = "USB_SDA"]
    #[inline(always)]
    pub fn is_usb_sda(&self) -> bool {
        *self == PinconnectPinsel1P0_27Enum::UsbSda
    }
}
#[doc = "Field `P0_27` writer - Pin function select P0.27."]
pub type P0_27W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_27Enum>;
impl<'a, REG> P0_27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.27"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_27Enum::GpioP0)
    }
    #[doc = "SDA0"]
    #[inline(always)]
    pub fn sda0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_27Enum::Sda0)
    }
    #[doc = "USB_SDA"]
    #[inline(always)]
    pub fn usb_sda(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_27Enum::UsbSda)
    }
}
#[doc = "Pin function select P0.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinsel1P0_28Enum {
    #[doc = "0: GPIO P0.28"]
    GpioP0 = 0,
    #[doc = "1: SCL0"]
    Scl0 = 1,
    #[doc = "2: USB_SCL"]
    UsbScl = 2,
}
impl From<PinconnectPinsel1P0_28Enum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinsel1P0_28Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinsel1P0_28Enum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinsel1P0_28Enum {}
#[doc = "Field `P0_28` reader - Pin function select P0.28."]
pub type P0_28R = crate::FieldReader<PinconnectPinsel1P0_28Enum>;
impl P0_28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinsel1P0_28Enum {
        match self.bits {
            0 => PinconnectPinsel1P0_28Enum::GpioP0,
            1 => PinconnectPinsel1P0_28Enum::Scl0,
            2 => PinconnectPinsel1P0_28Enum::UsbScl,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.28"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == PinconnectPinsel1P0_28Enum::GpioP0
    }
    #[doc = "SCL0"]
    #[inline(always)]
    pub fn is_scl0(&self) -> bool {
        *self == PinconnectPinsel1P0_28Enum::Scl0
    }
    #[doc = "USB_SCL"]
    #[inline(always)]
    pub fn is_usb_scl(&self) -> bool {
        *self == PinconnectPinsel1P0_28Enum::UsbScl
    }
}
#[doc = "Field `P0_28` writer - Pin function select P0.28."]
pub type P0_28W<'a, REG> = crate::FieldWriter<'a, REG, 2, PinconnectPinsel1P0_28Enum>;
impl<'a, REG> P0_28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.28"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_28Enum::GpioP0)
    }
    #[doc = "SCL0"]
    #[inline(always)]
    pub fn scl0(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_28Enum::Scl0)
    }
    #[doc = "USB_SCL"]
    #[inline(always)]
    pub fn usb_scl(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinsel1P0_28Enum::UsbScl)
    }
}
#[doc = "Pin function select P0.29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_29 {
    #[doc = "0: GPIO P0.29"]
    GpioP0 = 0,
    #[doc = "1: USB_D+"]
    UsbDp = 1,
    #[doc = "2: Reserved"]
    Reserved2 = 2,
    #[doc = "3: Reserved"]
    Reserved3 = 3,
}
impl From<P0_29> for u8 {
    #[inline(always)]
    fn from(variant: P0_29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_29 {
    type Ux = u8;
}
impl crate::IsEnum for P0_29 {}
#[doc = "Field `P0_29` reader - Pin function select P0.29"]
pub type P0_29R = crate::FieldReader<P0_29>;
impl P0_29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_29 {
        match self.bits {
            0 => P0_29::GpioP0,
            1 => P0_29::UsbDp,
            2 => P0_29::Reserved2,
            3 => P0_29::Reserved3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.29"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_29::GpioP0
    }
    #[doc = "USB_D+"]
    #[inline(always)]
    pub fn is_usb_dp(&self) -> bool {
        *self == P0_29::UsbDp
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved_2(&self) -> bool {
        *self == P0_29::Reserved2
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved_3(&self) -> bool {
        *self == P0_29::Reserved3
    }
}
#[doc = "Field `P0_29` writer - Pin function select P0.29"]
pub type P0_29W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_29, crate::Safe>;
impl<'a, REG> P0_29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.29"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_29::GpioP0)
    }
    #[doc = "USB_D+"]
    #[inline(always)]
    pub fn usb_dp(self) -> &'a mut crate::W<REG> {
        self.variant(P0_29::UsbDp)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved_2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_29::Reserved2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved_3(self) -> &'a mut crate::W<REG> {
        self.variant(P0_29::Reserved3)
    }
}
#[doc = "Pin function select P0.30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_30 {
    #[doc = "0: GPIO P0.30"]
    GpioP0 = 0,
    #[doc = "1: USB_D-"]
    UsbDm = 1,
    #[doc = "2: Reserved"]
    Reserved2 = 2,
    #[doc = "3: Reserved"]
    Reserved3 = 3,
}
impl From<P0_30> for u8 {
    #[inline(always)]
    fn from(variant: P0_30) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_30 {
    type Ux = u8;
}
impl crate::IsEnum for P0_30 {}
#[doc = "Field `P0_30` reader - Pin function select P0.30."]
pub type P0_30R = crate::FieldReader<P0_30>;
impl P0_30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_30 {
        match self.bits {
            0 => P0_30::GpioP0,
            1 => P0_30::UsbDm,
            2 => P0_30::Reserved2,
            3 => P0_30::Reserved3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.30"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_30::GpioP0
    }
    #[doc = "USB_D-"]
    #[inline(always)]
    pub fn is_usb_dm(&self) -> bool {
        *self == P0_30::UsbDm
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved_2(&self) -> bool {
        *self == P0_30::Reserved2
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved_3(&self) -> bool {
        *self == P0_30::Reserved3
    }
}
#[doc = "Field `P0_30` writer - Pin function select P0.30."]
pub type P0_30W<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_30, crate::Safe>;
impl<'a, REG> P0_30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.30"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_30::GpioP0)
    }
    #[doc = "USB_D-"]
    #[inline(always)]
    pub fn usb_dm(self) -> &'a mut crate::W<REG> {
        self.variant(P0_30::UsbDm)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved_2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_30::Reserved2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved_3(self) -> &'a mut crate::W<REG> {
        self.variant(P0_30::Reserved3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P0.16."]
    #[inline(always)]
    pub fn p0_16(&self) -> P0_16R {
        P0_16R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P0.17."]
    #[inline(always)]
    pub fn p0_17(&self) -> P0_17R {
        P0_17R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P0.18."]
    #[inline(always)]
    pub fn p0_18(&self) -> P0_18R {
        P0_18R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P019."]
    #[inline(always)]
    pub fn p0_19(&self) -> P0_19R {
        P0_19R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P0.20."]
    #[inline(always)]
    pub fn p0_20(&self) -> P0_20R {
        P0_20R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P0.21."]
    #[inline(always)]
    pub fn p0_21(&self) -> P0_21R {
        P0_21R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P022"]
    #[inline(always)]
    pub fn p0_22(&self) -> P0_22R {
        P0_22R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P023."]
    #[inline(always)]
    pub fn p0_23(&self) -> P0_23R {
        P0_23R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P0.24."]
    #[inline(always)]
    pub fn p0_24(&self) -> P0_24R {
        P0_24R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P0.25."]
    #[inline(always)]
    pub fn p0_25(&self) -> P0_25R {
        P0_25R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P0.26."]
    #[inline(always)]
    pub fn p0_26(&self) -> P0_26R {
        P0_26R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P0.27."]
    #[inline(always)]
    pub fn p0_27(&self) -> P0_27R {
        P0_27R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Pin function select P0.28."]
    #[inline(always)]
    pub fn p0_28(&self) -> P0_28R {
        P0_28R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P0.29"]
    #[inline(always)]
    pub fn p0_29(&self) -> P0_29R {
        P0_29R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Pin function select P0.30."]
    #[inline(always)]
    pub fn p0_30(&self) -> P0_30R {
        P0_30R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P0.16."]
    #[inline(always)]
    pub fn p0_16(&mut self) -> P0_16W<'_, Pinsel1Spec> {
        P0_16W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Pin function select P0.17."]
    #[inline(always)]
    pub fn p0_17(&mut self) -> P0_17W<'_, Pinsel1Spec> {
        P0_17W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Pin function select P0.18."]
    #[inline(always)]
    pub fn p0_18(&mut self) -> P0_18W<'_, Pinsel1Spec> {
        P0_18W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Pin function select P019."]
    #[inline(always)]
    pub fn p0_19(&mut self) -> P0_19W<'_, Pinsel1Spec> {
        P0_19W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin function select P0.20."]
    #[inline(always)]
    pub fn p0_20(&mut self) -> P0_20W<'_, Pinsel1Spec> {
        P0_20W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Pin function select P0.21."]
    #[inline(always)]
    pub fn p0_21(&mut self) -> P0_21W<'_, Pinsel1Spec> {
        P0_21W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Pin function select P022"]
    #[inline(always)]
    pub fn p0_22(&mut self) -> P0_22W<'_, Pinsel1Spec> {
        P0_22W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Pin function select P023."]
    #[inline(always)]
    pub fn p0_23(&mut self) -> P0_23W<'_, Pinsel1Spec> {
        P0_23W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Pin function select P0.24."]
    #[inline(always)]
    pub fn p0_24(&mut self) -> P0_24W<'_, Pinsel1Spec> {
        P0_24W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Pin function select P0.25."]
    #[inline(always)]
    pub fn p0_25(&mut self) -> P0_25W<'_, Pinsel1Spec> {
        P0_25W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Pin function select P0.26."]
    #[inline(always)]
    pub fn p0_26(&mut self) -> P0_26W<'_, Pinsel1Spec> {
        P0_26W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Pin function select P0.27."]
    #[inline(always)]
    pub fn p0_27(&mut self) -> P0_27W<'_, Pinsel1Spec> {
        P0_27W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Pin function select P0.28."]
    #[inline(always)]
    pub fn p0_28(&mut self) -> P0_28W<'_, Pinsel1Spec> {
        P0_28W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Pin function select P0.29"]
    #[inline(always)]
    pub fn p0_29(&mut self) -> P0_29W<'_, Pinsel1Spec> {
        P0_29W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Pin function select P0.30."]
    #[inline(always)]
    pub fn p0_30(&mut self) -> P0_30W<'_, Pinsel1Spec> {
        P0_30W::new(self, 28)
    }
}
#[doc = "Pin function select register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pinsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinsel1Spec;
impl crate::RegisterSpec for Pinsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel1::R`](R) reader structure"]
impl crate::Readable for Pinsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`pinsel1::W`](W) writer structure"]
impl crate::Writable for Pinsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINSEL1 to value 0"]
impl crate::Resettable for Pinsel1Spec {}
