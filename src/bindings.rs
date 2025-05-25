#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gapcom_handle_t {
    _unused: [u8; 0],
}

#[doc = " @brief Callback installed by user, triggered when a specific type of message is received."]
pub type gapcom_callback_t = ::core::option::Option<
    unsafe extern "C" fn(handle: *mut gapcom_handle_t, proto_msg: *const cty::c_void)
>;

#[doc = " @brief Interface for an object able to send bytes through some opaque means.\n\n Libgapcom does not \"know\" how to send bytes, and leaves that task to the user.\n This implementation, in turn, is used to provide an implementation of TF_WriteImpl()\n to the TinyFrame library.\n\n This decoupling allows a flexible implementation and makes libgapcom only care\n about encoding/decoding message payloads.\n\n Please note there is no receive function in this interface. Indeed, while TinyFrame\n requires us to explain how to actually send bytes, libgapcom has no requirement\n on the receive side. In practice, the user should manage receiving bytes on her own,\n and call gapcom_accept() on the resulting data to trigger the TinyFrame state\n machine (decoding the frame and eventually invoking the user's callback)."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gapcom_sender_t {
    pub open: ::core::option::Option<
        unsafe extern "C" fn(self_: *mut gapcom_sender_t) -> cty::c_int
    >,
    pub close: ::core::option::Option<
        unsafe extern "C" fn(self_: *mut gapcom_sender_t) -> cty::c_int
    >,
    pub send: ::core::option::Option<
        unsafe extern "C" fn(self_: *mut gapcom_sender_t, buf: *const u8, len: usize) -> isize
    >,
}

pub const GAPCOM_MSG_MIN: gapcom_msg_t = 0;
pub const GAPCOM_MSG_PING_REQ: gapcom_msg_t = 0;
pub const GAPCOM_MSG_PING_RESP: gapcom_msg_t = 1;
pub const GAPCOM_MSG_SET_LOG_VERBOSITY_REQ: gapcom_msg_t = 2;
pub const GAPCOM_MSG_SET_LOG_VERBOSITY_RESP: gapcom_msg_t = 3;
pub const GAPCOM_MSG_SET_GYROSCOPE_REQ: gapcom_msg_t = 4;
pub const GAPCOM_MSG_SET_GYROSCOPE_RESP: gapcom_msg_t = 5;
pub const GAPCOM_MSG_SET_VERSION_REQ: gapcom_msg_t = 6;
pub const GAPCOM_MSG_SET_VERSION_RESP: gapcom_msg_t = 7;
pub const GAPCOM_MSG_GET_VERSION_REQ: gapcom_msg_t = 8;
pub const GAPCOM_MSG_GET_VERSION_RESP: gapcom_msg_t = 9;
pub const GAPCOM_MSG_SELFTEST_REQ: gapcom_msg_t = 10;
pub const GAPCOM_MSG_SELFTEST_RESP: gapcom_msg_t = 11;
pub const GAPCOM_MSG_POWER_SAVE_MODE_REQ: gapcom_msg_t = 12;
pub const GAPCOM_MSG_POWER_SAVE_MODE_RESP: gapcom_msg_t = 13;
pub const GAPCOM_MSG_MAX: gapcom_msg_t = 14;
#[doc = " @brief A message type in libgapcom.\n\n Messages come in three flavors:\n\n - Requests, sent by the client to the server\n - Responses, sent back by the server to the client\n - Events, spontaneously sent by the server to the client if the client has previously\n   subscribed to the appropriate event using a dedicated request"]
pub type gapcom_msg_t = cty::c_uint;

pub const GAP_OK: GAPErrorCode = 0;
pub const GAP_FEATURE_NOT_IMPLEMENTED: GAPErrorCode = 1;
pub const GAP_INVALID_LOG_VERBOSITY: GAPErrorCode = 2;
pub const GAP_INVALID_VERSION_FORMAT: GAPErrorCode = 3;

pub type GAPErrorCode = cty::c_uint;

pub const GAP_LOG_DEBUG: GAPLogVerbosity = 0;
pub const GAP_LOG_INFO: GAPLogVerbosity = 1;
pub const GAP_LOG_WARNING: GAPLogVerbosity = 2;
pub const GAP_LOG_ERROR: GAPLogVerbosity = 3;
pub type GAPLogVerbosity = cty::c_uchar;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GAPSetLogVerbosityReq {
    pub verbosity: GAPLogVerbosity,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GAPSetGyroscopeReq {
    pub set: bool,
}

unsafe extern "C" {
    #[doc = " @brief Create a libgapcom session handle.\n\n A libgapcom session handle is an opaque object carrying session-specific metadata.\n It is required for calling every function of the libgapcom API.\n\n @return gapcom_handle_t* A new session handle"]
    pub fn gapcom_create() -> *mut gapcom_handle_t;
}

unsafe extern "C" {
    #[doc = " @brief Destroy a libgapcom session handle and associated ressources.\n\n @param handle Handle to the libgapcom session. Can be NULL, in which case nothing happens"]
    pub fn gapcom_destroy(handle: *mut gapcom_handle_t);
}

unsafe extern "C" {
    #[doc = " @brief Provision the libgapcom session with an implementation of the gapcom_sender_t interface.\n\n Calling this function is required before exchanging messages. There is no default\n implementation for sending bytes in libgapcom.\n\n @param handle Handle to the libgapcom session\n @param sender_impl Implementation of the gapcom_sender_t interface"]
    pub fn gapcom_set_sender_impl(handle: *mut gapcom_handle_t, sender_impl: *mut gapcom_sender_t);
}

unsafe extern "C" {
    #[doc = " @brief Install a user-given callback to be invoked when receiving a specific message type\n\n Please note the following design choices:\n\n - There is no send-time callack, only receive-time\n - Callbacks will be invoked with a generic pointer to the Protocol Buffer content of\n   the message. Callback code needs to cast the pointer explicitly to the expected type\n - There cannot be more than 1 callback installed for a given message type. Invoking this\n   function twice in a row with different callback functions will result in using the\n   one installed the latter.\n\n @param handle Handle to the libgapcom session\n @param callback User-given callback function\n @param msg_type Type of message for which the callback shall be invoked upon reception"]
    pub fn gapcom_install_callback(
        handle: *mut gapcom_handle_t,
        callback: gapcom_callback_t,
        msg_type: gapcom_msg_t
    );
}

unsafe extern "C" {
    #[doc = " @brief Remove a callback installed with gapcom_install_callback()\n\n After calling this function, no callback will be attached to the given message type.\n One can install a new callback like before, using gapcom_install_callback().\n\n @param handle Handle to the libgapcom session\n @param msg_type Type of message for which the callback shall be invoked upon reception"]
    pub fn gapcom_uninstall_callback(handle: *mut gapcom_handle_t, msg_type: gapcom_msg_t);
}

unsafe extern "C" {
    #[doc = " @brief Launch the libgapcom parser on data.\n\n The libgapcom parser relies on the TinyFrame state machine, which will:\n\n - Check the validity of the TinyFrame header\n - Check the validity of header CRC\n - Check the validity of payload CRC\n - Identify the message type and length\n - If some user-given callback has been installed for this message type, invoke it with\n   the decoded Protocol Buffer payload as an argument\n\n In practice, this function should be called on data collected by the user with whatever\n communication channel is currently being used: UART, UNIX socket, etc.\n\n @param handle Handle to the libgapcom session\n @param buf Data to parse\n @param len Length of given data"]
    pub fn gapcom_accept(handle: *mut gapcom_handle_t, buf: *const u8, len: usize);
}

unsafe extern "C" {
    pub fn gapcom_respond_ping(
        handle: *mut gapcom_handle_t,
        error_code: GAPErrorCode
    ) -> cty::c_int;
}

unsafe extern "C" {
    pub fn gapcom_respond_set_log_verbosity(
        handle: *mut gapcom_handle_t,
        error_code: GAPErrorCode
    ) -> cty::c_int;
}

unsafe extern "C" {
    pub fn gapcom_respond_set_gyroscope(
        handle: *mut gapcom_handle_t,
        error_code: GAPErrorCode
    ) -> cty::c_int;
}
