use core::{ cell::RefCell, fmt::Write };
use cortex_m::interrupt::Mutex;
use stm32f4xx_hal::{ pac::USART1, serial::Tx };

static LOGGER: Mutex<RefCell<Option<Logger>>> = Mutex::new(RefCell::new(None));

pub fn init_logger(usart1_tx: Tx<USART1>) {
    cortex_m::interrupt::free(|cs| {
        let logger = Logger::new(usart1_tx);
        LOGGER.borrow(cs).replace(Some(logger));
    });
}

pub fn logger_instance() -> &'static mut Logger {
    cortex_m::interrupt::free(|cs| {
        LOGGER.borrow(cs)
            .borrow_mut()
            .as_mut()
            .map(|logger| unsafe { &mut *(logger as *mut _) })
            .expect("Logger not initialized")
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

pub struct Logger {
    tx: Tx<USART1>,
    level: LogLevel,
}

impl Logger {
    pub fn new(tx: Tx<USART1>) -> Self {
        Logger {
            tx,
            level: LogLevel::Info,
        }
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    pub fn log(&mut self, level: LogLevel, message: &str) {
        if (level as u8) <= (self.level as u8) {
            let prefix = match level {
                LogLevel::Error => "[ERROR] ",
                LogLevel::Warn => "[WARN] ",
                LogLevel::Info => "[INFO] ",
                LogLevel::Debug => "[DEBUG] ",
            };

            let _ = self.tx.write_str(prefix);
            let _ = self.tx.write_str(message);
            let _ = self.tx.write_str("\r\n");
        }
    }

    pub fn error(&mut self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    pub fn _warn(&mut self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    pub fn info(&mut self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn debug(&mut self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
}
