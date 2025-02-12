#![no_std]
//! This crate provides a ILI9225 driver to connect to TFT displays.
//  width
// --------
// |      |
// |      |
// |      | height
// |      |
// |      |
// --------

pub mod instruction;
use crate::instruction::Instruction;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi;
pub struct ILI9225<SPI, RS, RST>
where
    SPI: spi::SpiDevice,
    RS: OutputPin,
    RST: OutputPin,
{
    /// SPI
    spi: SPI,
    /// Data/command pin.
    rs: RS,
    /// Reset pin.
    rst: Option<RST>,
    /// Whether the display is RGB (true) or BGR (false)
    rgb: bool,
    /// Whether the colours are inverted (true) or not (false)
    inverted: bool,
    /// Global image offset
    dx: u16,
    dy: u16,
    width: u32,
    height: u32,
}
/// Display orientation.
#[derive(Debug, Clone, Copy)]
#[repr(u16)]

pub enum Orientation {
    Portrait = 0x1030,
    Landscape = 0x1028,
    PortraitSwapped = 0x1000,
    LandscapeSwapped = 0x1018,
}

impl<SPI, RS, RST> ILI9225<SPI, RS, RST>
where
    SPI: spi::SpiDevice,
    RS: OutputPin,
    RST: OutputPin,
{
    /// Creates a new driver instance that uses hardware SPI.
    pub fn new(
        spi: SPI,
        rs: RS,
        rst: Option<RST>,
        rgb: bool,
        inverted: bool,
        width: u32,
        height: u32,
    ) -> Self {
        let display = ILI9225 {
            spi,
            rs,
            rst,
            rgb,
            inverted,
            dx: 0,
            dy: 0,
            width,
            height,
        };
        display
    }
    pub fn init<DELAY>(&mut self, delay: &mut DELAY) -> Result<(), ()>
    where
        DELAY: DelayNs,
    {
        self.hard_reset(delay)?;
        // Power-on sequence
        //
        self.write_command(Instruction::PowerCtrl1, &[0x0000])?; //Set SAP,DSTB,STB
        self.write_command(Instruction::PowerCtrl2, &[0x0000])?; // Set APON,PON,AON,VCI1EN,VC
        self.write_command(Instruction::PowerCtrl3, &[0x0000])?; // Set BT,DC1,DC2,DC3
        self.write_command(Instruction::PowerCtrl4, &[0x0000])?; // Set GVDD
        self.write_command(Instruction::PowerCtrl5, &[0x0000])?; // Set VCOMH/VCOML voltage
        delay.delay_ms(50);
        // Power-on sequence
        self.write_command(Instruction::PowerCtrl2, &[0x00, 0x18])?; // Set APON,PON,AON,VCI1EN,VC
        self.write_command(Instruction::PowerCtrl3, &[0x61, 0x21])?; // Set BT,DC1,DC2,DC3
        self.write_command(Instruction::PowerCtrl4, &[0x00, 0x6F])?; // Set GVDD  /*007F 0088 */
        self.write_command(Instruction::PowerCtrl5, &[0x49, 0x5F])?; // Set VCOMH/VCOML voltage
        self.write_command(Instruction::PowerCtrl1, &[0x08, 0x00])?; // Set SAP,DSTB,STB
        delay.delay_ms(50);
        self.write_command(Instruction::PowerCtrl2, &[0x10, 0x3B])?; // Set APON,PON,AON,VCI1EN,VC
        delay.delay_ms(50);
        self.write_command(Instruction::DriverOutputCtrl, &[0x01, 0x1C])?; // set the display line number and display direction
        self.write_command(Instruction::LcdAcDrivingCtrl, &[0x01, 0x00])?; // set 1 line inversion
        self.write_command(Instruction::EntryMode, &[0x10, 0x30])?; // Set GRAM write direction and BGR=1.
        self.write_command(Instruction::DispCtrl1, &[0x00, 0x00])?; // Display off
        self.write_command(Instruction::DispCtrl2, &[0x08, 0x08])?; // Set the back porch and front porch
        self.write_command(Instruction::FrameCycleCtrl, &[0x11, 0x00])?; // Set the clocks number per line
        self.write_command(Instruction::InterfaceCtrl, &[0x00, 0x00])?; // CPU interface
        self.write_command(Instruction::OscCtrl, &[0x0D, 0x01])?; // Set Osc
        self.write_command(Instruction::VciRecycling, &[0x00, 0x20])?; // Set VCI recycling
        self.write_command(Instruction::RamAddrSet1, &[0x00, 0x00])?; // RAM Address
        self.write_command(Instruction::RamAddRSet2, &[0x00, 0x00])?; // RAM Address
        delay.delay_ms(20);
        //-------------- Set GRAM area -----------------//
        self.write_command(Instruction::GateScanCtrl, &[0x00, 0x00])?;
        self.write_command(Instruction::VerticalScrollCtrl1, &[0x00, 0xDB])?;
        self.write_command(Instruction::VerticalScrollCtrl2, &[0x00, 0x00])?;
        self.write_command(Instruction::VerticalScrollCtrl3, &[0x00, 0x00])?; //0x0000
        self.write_command(Instruction::PartialDrivingPos1, &[0x00, 0xDB])?; //0x00DB
        self.write_command(Instruction::PartialDrivingPos2, &[0x00, 0x00])?; //0x0000
        self.write_command(Instruction::HorizontalWindowAddr1, &[0x00, 0xAF])?; //0x00AF
        self.write_command(Instruction::HorizontalWindowAddr2, &[0x00, 0x00])?; //0x0000
        self.write_command(Instruction::VerticalWindowAddr1, &[0x00, 0xDB])?; //0x00DB
        self.write_command(Instruction::VerticalWindowAddr2, &[0x00, 0x00])?; //0x0000
                                                                              // ----------- Adjust the Gamma Curve ----------//
        self.write_command(Instruction::GammaCtrl1, &[0x00, 0x00])?; //0x0000
        self.write_command(Instruction::GammaCtrl2, &[0x08, 0x08])?; //0x0808
        self.write_command(Instruction::GammaCtrl3, &[0x08, 0x0A])?; //0x080A
        self.write_command(Instruction::GammaCtrl4, &[0x00, 0x0A])?; //0x000A
        self.write_command(Instruction::GammaCtrl5, &[0x0A, 0x08])?; //0x0A08
        self.write_command(Instruction::GammaCtrl6, &[0x08, 0x08])?; //0x0808
        self.write_command(Instruction::GammaCtrl7, &[0x00, 0x00])?; //0x0000
        self.write_command(Instruction::GammaCtrl8, &[0x0A, 0x00])?; //0x0A00
        self.write_command(Instruction::GammaCtrl9, &[0x07, 0x10])?;
        self.write_command(Instruction::GammaCtrl10, &[0x07, 0x10])?;
        self.write_command(Instruction::DispCtrl1, &[0x00, 0x12])?;
        delay.delay_ms(50);
        self.write_command(Instruction::DispCtrl1, &[0x10, 0x17])?;
        Ok(())
    }
    pub fn hard_reset<DELAY>(&mut self, delay: &mut DELAY) -> Result<(), ()>
    where
        DELAY: DelayNs,
    {
        if let Some(rst) = &mut self.rst {
            rst.set_high().map_err(|_| ())?;
            delay.delay_ms(10);
            rst.set_low().map_err(|_| ())?;
            delay.delay_ms(10);
            rst.set_high().map_err(|_| ())?;
        }
        Ok(())
    }
    fn write_command(&mut self, command: Instruction, params: &[u8]) -> Result<(), ()> {
        // 发送命令给 LCD 控制器的功能。  它还具备发送命令参数的能力
        self.rs.set_low().map_err(|_| ())?;
        self.spi.write(&[command as u8]).map_err(|_| ())?;
        if !params.is_empty() {
            self.start_data()?;
            self.write_data(params)?;
        }
        Ok(())
    }
    fn start_data(&mut self) -> Result<(), ()> {
        self.rs.set_high().map_err(|_| ())
    }
    fn write_data(&mut self, data: &[u8]) -> Result<(), ()> {
        self.spi.write(data).map_err(|_| ())
    }
    /// Writes a data word to the display.
    fn write_word(&mut self, value: u16) -> Result<(), ()> {
        self.write_data(&value.to_be_bytes())
    }
    fn write_words_buffered(&mut self, words: impl IntoIterator<Item = u16>) -> Result<(), ()> {
        let mut buffer = [0; 32];
        let mut index = 0;
        for word in words {
            let as_bytes = word.to_be_bytes();
            buffer[index] = as_bytes[0];
            buffer[index + 1] = as_bytes[1];
            index += 2;
            if index >= buffer.len() {
                self.write_data(&buffer)?;
                index = 0;
            }
        }
        self.write_data(&buffer[0..index])
    }
    pub fn set_orientation(&mut self, orientation: &Orientation) -> Result<(), ()> {
        let mut value = *orientation as u16;

        // 处理颜色顺序（BGR/RGB）
        if !self.rgb {
            value |= 0x0800; // 设置Bit8
        }

        // ILI9225需要16位参数传输
        self.write_command(Instruction::EntryMode, &value.to_be_bytes())
    }

    // pub fn set_orientation(&mut self, orientation: &Orientation) -> Result<(), ()> {
    //     if self.rgb {
    //         self.write_command(Instruction::MADCTL, &[*orientation as u8])?;
    //     } else {
    //         self.write_command(Instruction::MADCTL, &[orientation as u8 | 0x08])?;
    //     }
    //     Ok(())
    // }
    /// Sets the global offset of the displayed image
    pub fn set_offset(&mut self, dx: u16, dy: u16) {
        self.dx = dx;
        self.dy = dy;
    }
    /// Sets the address window for the display.
    pub fn set_address_window(&mut self, sx: u16, sy: u16, ex: u16, ey: u16) -> Result<(), ()> {
        self.write_command(Instruction::GateScanCtrl, &[])?;
        self.write_command(Instruction::RamAddrSet1, &[])?;
        self.start_data()?;
        self.write_word(sx + self.dx)?;
        self.write_word(ex + self.dx)?;
        self.write_command(Instruction::RamAddRSet2, &[])?;
        self.start_data()?;
        self.write_word(sy + self.dy)?;
        self.write_word(ey + self.dy)
    }
    // Sets a pixel color at the given coords.
    pub fn set_pixel(&mut self, x: u16, y: u16, color: u16) -> Result<(), ()> {
        self.set_address_window(x, y, x, y)?;
        self.write_command(Instruction::GramDataReg, &[])?;
        self.start_data()?;
        self.write_word(color)
    }
    /// Writes pixel colors sequentially into the current drawing window
    pub fn write_pixels<P: IntoIterator<Item = u16>>(&mut self, colors: P) -> Result<(), ()> {
        self.write_command(Instruction::GramDataReg, &[])?;
        self.start_data()?;
        for color in colors {
            self.write_word(color)?;
        }
        Ok(())
    }
    pub fn write_pixels_buffered<P: IntoIterator<Item = u16>>(
        &mut self,
        colors: P,
    ) -> Result<(), ()> {
        self.write_command(Instruction::GramDataReg, &[])?;
        self.start_data()?;
        self.write_words_buffered(colors)
    }
    /// Sets pixel colors at the given drawing window
    pub fn set_pixels<P: IntoIterator<Item = u16>>(
        &mut self,
        sx: u16,
        sy: u16,
        ex: u16,
        ey: u16,
        colors: P,
    ) -> Result<(), ()> {
        self.set_address_window(sx, sy, ex, ey)?;
        self.write_pixels(colors)
    }
    pub fn set_pixels_buffered<P: IntoIterator<Item = u16>>(
        &mut self,
        sx: u16,
        sy: u16,
        ex: u16,
        ey: u16,
        colors: P,
    ) -> Result<(), ()> {
        self.set_address_window(sx, sy, ex, ey)?;
        self.write_pixels_buffered(colors)
    }
}

#[cfg(feature = "graphics")]
extern crate embedded_graphics_core;
#[cfg(feature = "graphics")]
use self::embedded_graphics_core::{
    draw_target::DrawTarget,
    pixelcolor::{
        raw::{RawData, RawU16},
        Rgb565,
    },
    prelude::*,
    primitives::Rectangle,
};

#[cfg(feature = "graphics")]
impl<SPI, RS, RST> DrawTarget for ILI9225<SPI, RS, RST>
where
    SPI: spi::SpiDevice,
    RS: OutputPin,
    RST: OutputPin,
{
    type Error = ();
    type Color = Rgb565;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Only draw pixels that would be on screen
            if coord.x >= 0
                && coord.y >= 0
                && coord.x < self.width as i32
                && coord.y < self.height as i32
            {
                self.set_pixel(
                    coord.x as u16,
                    coord.y as u16,
                    RawU16::from(color).into_inner(),
                )?;
            }
        }

        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        // Clamp area to drawable part of the display target
        let drawable_area = area.intersection(&Rectangle::new(Point::zero(), self.size()));

        if drawable_area.size != Size::zero() {
            self.set_pixels_buffered(
                drawable_area.top_left.x as u16,
                drawable_area.top_left.y as u16,
                (drawable_area.top_left.x + (drawable_area.size.width - 1) as i32) as u16,
                (drawable_area.top_left.y + (drawable_area.size.height - 1) as i32) as u16,
                area.points()
                    .zip(colors)
                    .filter(|(pos, _color)| drawable_area.contains(*pos))
                    .map(|(_pos, color)| RawU16::from(color).into_inner()),
            )?;
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.set_pixels_buffered(
            0,
            0,
            self.width as u16 - 1,
            self.height as u16 - 1,
            core::iter::repeat(RawU16::from(color).into_inner())
                .take((self.width * self.height) as usize),
        )
    }
}

#[cfg(feature = "graphics")]
impl<SPI, RS, RST> OriginDimensions for ILI9225<SPI, RS, RST>
where
    SPI: spi::SpiDevice,
    RS: OutputPin,
    RST: OutputPin,
{
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}
