use rp2040_hal as hal;
use hal::pac;

/// Peripherals used by the OS
#[allow(non_snake_case)]
pub(crate) struct SystemPeripherals {
    #[cfg(feature = "stdio-usb")]
    pub PLL_USB: pac::PLL_USB,
    pub PPB: pac::PPB,
    pub PSM: pac::PSM,
    pub SIO: pac::SIO,
    #[cfg(feature = "stdio-uart")]
    UART0: pac::UART0,
    #[cfg(feature = "stdio-usb")]
    pub USBCTRL_DPRAM: pac::USBCTRL_DPRAM,
    #[cfg(feature = "stdio-usb")]
    pub USBCTRL_REGS: pac::USBCTRL_REGS
}

/// Re-exports RP2040 peripherals not used by the OS
#[allow(non_snake_case)]
pub struct Peripherals {
    pub ADC: pac::ADC,
    pub BUSCTRL: pac::BUSCTRL,
    pub CLOCKS: pac::CLOCKS,
    pub DMA: pac::DMA,
    pub I2C0: pac::I2C0,
    pub I2C1: pac::I2C1,
    pub IO_BANK0: pac::IO_BANK0,
    pub IO_QSPI: pac::IO_QSPI,
    pub PADS_BANK0: pac::PADS_BANK0,
    pub PADS_QSPI: pac::PADS_QSPI,
    pub PIO0: pac::PIO0,
    pub PIO1: pac::PIO1,
    pub PLL_SYS: pac::PLL_SYS,
    #[cfg(not(feature = "stdio-usb"))]
    pub PLL_USB: pac::PLL_USB,
    pub PWM: pac::PWM,
    pub RESETS: pac::RESETS,
    pub ROSC: pac::ROSC,
    pub RTC: pac::RTC,
    pub SPI0: pac::SPI0,
    pub SPI1: pac::SPI1,
    pub SYSCFG: pac::SYSCFG,
    pub SYSINFO: pac::SYSINFO,
    pub TBMAN: pac::TBMAN,
    pub TIMER: pac::TIMER,
    #[cfg(not(feature = "stdio-uart"))]
    pub UART0: pac::UART0,
    pub UART1: pac::UART1,
    #[cfg(not(feature = "stdio-usb"))]
    pub USBCTRL_DPRAM: pac::USBCTRL_DPRAM,
    #[cfg(not(feature = "stdio-usb"))]
    pub USBCTRL_REGS: pac::USBCTRL_REGS,
    pub VREG_AND_CHIP_RESET: pac::VREG_AND_CHIP_RESET,
    pub WATCHDOG: pac::WATCHDOG,
    pub XIP_CTRL: pac::XIP_CTRL,
    pub XIP_SSI: pac::XIP_SSI,
    pub XOSC: pac::XOSC
}

static mut PERIPHERALS: Option<Peripherals> = None;

impl Peripherals {
    pub fn take() -> Option<Self> {
        unsafe {
            let p = PERIPHERALS.take()?;
            PERIPHERALS = None;
            Some(p)
        }
    }
}

fn split_peripherals(p: pac::Peripherals) -> (Peripherals, SystemPeripherals) {
    let safe_peripherals = Peripherals {
        ADC: p.ADC,
        BUSCTRL: p.BUSCTRL,
        CLOCKS: p.CLOCKS,
        DMA: p.DMA,
        I2C0: p.I2C0,
        I2C1: p.I2C1,
        IO_BANK0: p.IO_BANK0,
        IO_QSPI: p.IO_QSPI,
        PADS_BANK0: p.PADS_BANK0,
        PADS_QSPI: p.PADS_QSPI,
        PIO0: p.PIO0,
        PIO1: p.PIO1,
        PLL_SYS: p.PLL_SYS,
        #[cfg(not(feature = "stdio-usb"))]
        PLL_USB: p.PLL_USB,
        PWM: p.PWM,
        RESETS: p.RESETS,
        ROSC: p.ROSC,
        RTC: p.RTC,
        SPI0: p.SPI0,
        SPI1: p.SPI1,
        SYSCFG: p.SYSCFG,
        SYSINFO: p.SYSINFO,
        TBMAN: p.TBMAN,
        TIMER: p.TIMER,
        #[cfg(not(feature = "stdio-uart"))]
        UART0: p.UART0,
        UART1: p.UART1,
        #[cfg(not(feature = "stdio-usb"))]
        USBCTRL_DPRAM: p.USBCTRL_DPRAM,
        #[cfg(not(feature = "stdio-usb"))]
        USBCTRL_REGS: p.USBCTRL_REGS,
        VREG_AND_CHIP_RESET: p.VREG_AND_CHIP_RESET,
        WATCHDOG: p.WATCHDOG,
        XIP_CTRL: p.XIP_CTRL,
        XIP_SSI: p.XIP_SSI,
        XOSC: p.XOSC
    };

    let system_peripherals = SystemPeripherals {
        #[cfg(feature = "stdio-usb")]
        PLL_USB: p.PLL_USB,
        PPB: p.PPB,
        PSM: p.PSM,
        SIO: p.SIO,
        #[cfg(feature = "stdio-uart")]
        UART0: p.UART0,
        #[cfg(feature = "stdio-usb")]
        USBCTRL_DPRAM: p.USBCTRL_DPRAM,
        #[cfg(feature = "stdio-usb")]
        USBCTRL_REGS: p.USBCTRL_REGS
    };

    (safe_peripherals, system_peripherals)
}

/// SIO peripherals used by the OS
#[allow(dead_code)]
pub(crate) struct SystemSio {
    pub hwdivider: hal::sio::HwDivider,
    pub fifo: hal::sio::SioFifo
}

/// Re-exports RP2040 Single-cycle IO (SIO) peripherals not used by the OS
pub struct Sio {
    pub gpio_bank0: hal::sio::SioGpioBank0,
    pub gpio_qspi: hal::sio::SioGpioQspi,
    pub interp0: hal::sio::Interp0,
    pub interp1: hal::sio::Interp1
}

static mut SIO: Option<Sio> = None;

impl Sio {
    pub fn take() -> Option<Self> {
        unsafe {
            let sio = SIO.take()?;
            SIO = None;
            Some(sio)
        }
    }
}

fn split_sio(sio: hal::Sio) -> (Sio, SystemSio) {
    let safe_sio = Sio {
        gpio_bank0: sio.gpio_bank0,
        gpio_qspi: sio.gpio_qspi,
        interp0: sio.interp0,
        interp1: sio.interp1
    };

    let system_sio = SystemSio {
        hwdivider: sio.hwdivider,
        fifo: sio.fifo
    };

    (safe_sio, system_sio)
}

/// Peripherals used by the OS
#[allow(non_snake_case, dead_code)]
pub(crate) struct SystemPeripheralsWithoutSio {
    #[cfg(feature = "stdio-usb")]
    pub PLL_USB: pac::PLL_USB,
    pub PPB: pac::PPB,
    pub PSM: pac::PSM,
    #[cfg(feature = "stdio-uart")]
    UART0: pac::UART0,
    #[cfg(feature = "stdio-usb")]
    pub USBCTRL_DPRAM: pac::USBCTRL_DPRAM,
    #[cfg(feature = "stdio-usb")]
    pub USBCTRL_REGS: pac::USBCTRL_REGS
}

pub(crate) fn init(p: pac::Peripherals) -> (SystemPeripheralsWithoutSio, SystemSio) {
    let (safe_peripherals, system_peripherals) = split_peripherals(p);
    unsafe { PERIPHERALS = Some(safe_peripherals) };

    let system_peripherals_without_sio = SystemPeripheralsWithoutSio {
        #[cfg(feature = "stdio-usb")]
        PLL_USB: system_peripherals.PLL_USB,
        PPB: system_peripherals.PPB,
        PSM: system_peripherals.PSM,
        #[cfg(feature = "stdio-uart")]
        UART0: system_peripherals.UART0,
        #[cfg(feature = "stdio-usb")]
        USBCTRL_DPRAM: system_peripherals.USBCTRL_DPRAM,
        #[cfg(feature = "stdio-usb")]
        USBCTRL_REGS: system_peripherals.USBCTRL_REGS
    };

    let (safe_sio, system_sio) = split_sio(hal::Sio::new(system_peripherals.SIO));
    unsafe { SIO = Some(safe_sio) };

    (system_peripherals_without_sio, system_sio)
}