use lcd_pcf8574::{ErrorHandling, Pcf8574};

/// HD44780 LCD display, using the PCF8574 I2C expander boards

pub struct Display {
    display: lcd::Display<Pcf8574>,
}

impl Display {
    pub fn new() -> Self {
        const I2C_BUS: u8 = 1;
        const I2C_ADDR: u16 = 0x27;

        let mut dev = Pcf8574::new(I2C_BUS, I2C_ADDR).unwrap();
        dev.on_error(ErrorHandling::Panic);

        let mut display = lcd::Display::new(dev);
        display.display(
            lcd::DisplayMode::DisplayOn,
            lcd::DisplayCursor::CursorOff,
            lcd::DisplayBlink::BlinkOff,
        );

        Self { display }
    }

    pub fn print(&mut self, line_1: &str, line_2: &str, line_3: &str, line_4: &str) {
        self.display.clear();
        self.display.home();

        self.display.print(line_1);
        self.display.position(0, 1);
        self.display.print(line_2);
        self.display.position(0, 2);
        self.display.print(line_3);
        self.display.position(0, 3);
        self.display.print(line_4);
    }
}
