use core::fmt::{Debug, Display, Formatter};
use embedded_hal_async::spi::SpiBusRead;
use embedded_hal_async::spi::SpiBusWrite;
use hal::clock::Clocks;
use hal::dma::{ChannelRx, ChannelTx, DmaPriority};
use hal::gdma::{Channel0, Channel0RxImpl, Channel0TxImpl, Gdma, SuitablePeripheral0};
use hal::peripherals::SPI2;
use hal::prelude::_esp_hal_spi_dma_WithDmaSpi2;
use hal::prelude::_fugit_RateExtU32;
use hal::spi::dma::SpiDma;
use hal::spi::{Spi, SpiMode};
use hal::system::PeripheralClockControl;
use hal::IO;

pub struct Acc {
    val: u16,
}

impl Acc {
    const MAX_G: f32 = 2.0;
    const MIN_G: f32 = -2.0;

    #[allow(unused)]
    pub fn raw(&self) -> u16 {
        self.val
    }

    pub fn acc(&self) -> f32 {
        Acc::MIN_G + (Acc::MAX_G - Acc::MIN_G) * (self.val as f32 / u16::MAX as f32)
    }
}

impl Debug for Acc {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl Display for Acc {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.acc())
    }
}

impl From<u16> for Acc {
    fn from(value: u16) -> Self {
        Acc { val: value }
    }
}

pub struct Lis2dh<'a> {
    spi: SpiDma<
        'a,
        hal::peripherals::SPI2,
        ChannelTx<'a, Channel0TxImpl, Channel0>,
        ChannelRx<'a, Channel0RxImpl, Channel0>,
        SuitablePeripheral0,
    >,
}

fn array_to_u16(array: &[u8]) -> u16 {
    ((array[1] as u16) << 8) | (array[0] as u16)
}

impl<'a> Lis2dh<'a> {
    pub fn new(
        io: IO,
        dma: Gdma,
        spi: SPI2,
        clock_control: &mut PeripheralClockControl,
        clocks: &Clocks,
        frequency_khz: u32,
        tx_descriptors: &'static mut [u32],
        rx_descriptors: &'static mut [u32],
    ) -> Self {
        hal::interrupt::enable(
            hal::peripherals::Interrupt::DMA_CH0,
            hal::interrupt::Priority::Priority1,
        )
        .unwrap();

        let sck = io.pins.gpio6;
        let miso = io.pins.gpio2;
        let mosi = io.pins.gpio7;
        let cs = io.pins.gpio10;

        let dma_channel = dma.channel0;

        let spi = Spi::new(
            spi,
            sck,
            mosi,
            miso,
            cs,
            frequency_khz.kHz(),
            SpiMode::Mode0,
            clock_control,
            clocks,
        )
        .with_dma(dma_channel.configure(
            false,
            tx_descriptors,
            rx_descriptors,
            DmaPriority::Priority0,
        ));

        Self { spi }
    }

    #[allow(unused)]
    async fn write(&mut self, address: u8, tx_buffer: &[u8]) {
        let address = [(address & 0x3F) | (0b01 << 6)];

        SpiBusWrite::write(&mut self.spi, &address).await.unwrap();
        SpiBusWrite::write(&mut self.spi, &tx_buffer).await.unwrap();
    }

    async fn read<const N: usize>(&mut self, address: u8) -> [u8; N] {
        let address = [(address & 0x3F) | (0b11 << 6)];
        let mut rx_buffer = [0; N];

        SpiBusWrite::write(&mut self.spi, &address).await.unwrap();
        SpiBusRead::read(&mut self.spi, &mut rx_buffer)
            .await
            .unwrap();

        rx_buffer
    }

    pub async fn who_i_am(&mut self) -> u8 {
        let result = self.read::<1>(0x0F).await;
        result[0]
    }

    #[allow(unused)]
    pub async fn read_x(&mut self) -> Acc {
        let result = self.read::<2>(0x28).await;
        Acc::from(array_to_u16(&result))
    }

    #[allow(unused)]
    pub async fn read_y(&mut self) -> Acc {
        let result = self.read::<2>(0x2A).await;
        Acc::from(array_to_u16(&result))
    }

    #[allow(unused)]
    pub async fn read_z(&mut self) -> Acc {
        let result = self.read::<2>(0x2C).await;
        Acc::from(array_to_u16(&result))
    }

    pub async fn read_all(&mut self) -> (Acc, Acc, Acc) {
        let result = self.read::<6>(0x28).await;
        (
            Acc::from(array_to_u16(&result[..2])),
            Acc::from(array_to_u16(&result[2..4])),
            Acc::from(array_to_u16(&result[4..])),
        )
    }
}
