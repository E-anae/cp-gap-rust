use crate::{ bindings::*, logger::{ self, logger_instance, LogLevel }, mpu60x0 };

unsafe extern "C" fn ping_callback(handle: *mut gapcom_handle_t, _proto_msg: *const cty::c_void) {
    unsafe {
        gapcom_respond_ping(handle, GAP_OK);
    }
}

unsafe extern "C" fn set_log_verbosity_callback(
    handle: *mut gapcom_handle_t,
    proto_msg: *const cty::c_void
) {
    unsafe {
        let logger = logger::logger_instance();

        let level: u8 = *(proto_msg as *const u8);

        let level = match level {
            GAP_LOG_DEBUG => LogLevel::Debug,
            GAP_LOG_INFO => LogLevel::Info,
            GAP_LOG_WARNING => LogLevel::Warn,
            GAP_LOG_ERROR => LogLevel::Error,
            _ => {
                gapcom_respond_set_log_verbosity(handle, GAP_INVALID_LOG_VERBOSITY);
                return;
            }
        };

        logger.set_level(level);

        gapcom_respond_set_log_verbosity(handle, GAP_OK);
    }
}

unsafe extern "C" fn set_gyroscope_callback(
    handle: *mut gapcom_handle_t,
    proto_msg: *const cty::c_void
) {
    unsafe {
        let msg = &*(proto_msg as *const GAPSetGyroscopeReq);

        gapcom_respond_set_gyroscope(handle, GAP_OK);

        cortex_m::interrupt::free(|cs| {
            let mut gyro = loop {
                match mpu60x0::MPU.borrow(cs).try_borrow_mut() {
                    Ok(gyro) => {
                        break gyro;
                    }
                    Err(_) => {
                        continue;
                    }
                }
            };

            if let Some(gyro) = gyro.as_mut() {
                if msg.set {
                    match gyro.enable() {
                        Ok(_) => logger_instance().info("Gyroscope enabled"),
                        Err(_) => logger_instance().error("Failed to enable gyroscope"),
                    };
                } else {
                    match gyro.disable() {
                        Ok(_) => logger_instance().info("Gyroscope disabled"),
                        Err(_) =>
                            logger_instance().error(
                                "Failed to disable gyroscope (already disabled?)"
                            ),
                    };
                }
                GAP_OK
            } else {
                GAP_FEATURE_NOT_IMPLEMENTED
            }
        });
    }
}

pub fn init_gapcom_callback(gapcom: *mut gapcom_handle_t) {
    unsafe {
        gapcom_install_callback(gapcom, Some(ping_callback), GAPCOM_MSG_PING_REQ);
        gapcom_install_callback(
            gapcom,
            Some(set_log_verbosity_callback),
            GAPCOM_MSG_SET_LOG_VERBOSITY_REQ
        );
        gapcom_install_callback(gapcom, Some(set_gyroscope_callback), GAPCOM_MSG_SET_GYROSCOPE_REQ);
    }
}
