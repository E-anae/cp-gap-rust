use crate::{ bindings::*, logger::{ self, LogLevel }, gyro, power };

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

        // Runs in interrupt context: only record the request, the main loop applies it.
        if gyro::is_installed() {
            gyro::request(msg.set);
            gapcom_respond_set_gyroscope(handle, GAP_OK);
        } else {
            gapcom_respond_set_gyroscope(handle, GAP_FEATURE_NOT_IMPLEMENTED);
        }
    }
}

unsafe extern "C" fn power_save_mode_callback(
    handle: *mut gapcom_handle_t,
    proto_msg: *const cty::c_void
) {
    unsafe {
        let msg = &*(proto_msg as *const GAPPowerSaveModeReq);

        power::set_requested(msg.save_power);

        gapcom_respond_power_save_mode(handle, GAP_OK);
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
        gapcom_install_callback(
            gapcom,
            Some(power_save_mode_callback),
            GAPCOM_MSG_POWER_SAVE_MODE_REQ
        );
    }
}
