#ifndef GAPCOM_H
#define GAPCOM_H

#include <sys/types.h>
#include <stddef.h>
#include <stdint.h>

/* Forward declarations */
typedef struct gapcom_handle_t gapcom_handle_t;
typedef struct gapcom_sender_t gapcom_sender_t;

/**
 * @brief Callback installed by user, triggered when a specific type of message is received.
 */
typedef void (*gapcom_callback_t)(gapcom_handle_t *handle, const void *proto_msg);

/**
 * @brief Interface for an object able to send bytes through some opaque means.
 *
 * Libgapcom does not "know" how to send bytes, and leaves that task to the user.
 * This implementation, in turn, is used to provide an implementation of TF_WriteImpl()
 * to the TinyFrame library.
 *
 * This decoupling allows a flexible implementation and makes libgapcom only care
 * about encoding/decoding message payloads.
 *
 * Please note there is no receive function in this interface. Indeed, while TinyFrame
 * requires us to "explain" how to actually send bytes, libgapcom has no requirement
 * on the receive side. In practice, the user should manage receiving bytes on her own,
 * and call gapcom_accept() on the resulting data to trigger the TinyFrame state
 * machine (decoding the frame and eventually invoking the user's callback).
 */
typedef struct gapcom_sender_t {
    int (*open)(gapcom_sender_t *self);
    int (*close)(gapcom_sender_t *self);
    ssize_t (*send)(gapcom_sender_t *self, const uint8_t *buf, size_t len);
} gapcom_sender_t;

/**
 * @brief A message type in libgapcom.
 *
 * Messages come in three flavors:
 *
 * - Requests, sent by the client to the server
 * - Responses, sent back by the server to the client
 * - Events, spontaneously sent by the server to the client if the client has previously
 *   subscribed to the appropriate event using a dedicated request
 */
typedef enum gapcom_msg_t {
    GAPCOM_MSG_MIN = 0,
    GAPCOM_MSG_PING_REQ = GAPCOM_MSG_MIN,
    GAPCOM_MSG_PING_RESP,
    GAPCOM_MSG_SET_LOG_VERBOSITY_REQ,
    GAPCOM_MSG_SET_LOG_VERBOSITY_RESP,
    GAPCOM_MSG_SET_GYROSCOPE_REQ,
    GAPCOM_MSG_SET_GYROSCOPE_RESP,
    GAPCOM_MSG_SET_VERSION_REQ,
    GAPCOM_MSG_SET_VERSION_RESP,
    GAPCOM_MSG_GET_VERSION_REQ,
    GAPCOM_MSG_GET_VERSION_RESP,
    GAPCOM_MSG_SELFTEST_REQ,
    GAPCOM_MSG_SELFTEST_RESP,
    GAPCOM_MSG_POWER_SAVE_MODE_REQ,
    GAPCOM_MSG_POWER_SAVE_MODE_RESP,
    GAPCOM_MSG_MAX,
} gapcom_msg_t;
#define GAPCOM_MSG_COUNT   (GAPCOM_MSG_MAX - GAPCOM_MSG_MIN)

/**
 * @brief Maximum length (in bytes) of firmware version string. See proto/gap.options.
 */
#define GAPCOM_VERSION_MAX_LEN    11

/**
 * @brief Create a libgapcom session handle.
 *
 * A libgapcom session handle is an opaque object carrying session-specific metadata.
 * It is required for calling every function of the libgapcom API.
 *
 * @return gapcom_handle_t* A new session handle
 */
gapcom_handle_t *gapcom_create();
/**
 * @brief Destroy a libgapcom session handle and associated ressources.
 *
 * @param handle Handle to the libgapcom session. Can be NULL, in which case nothing happens
 */
void gapcom_destroy(gapcom_handle_t *handle);

/**
 * @brief Provision the libgapcom session with an implementation of the gapcom_sender_t interface.
 *
 * Calling this function is required before exchanging messages. There is no default
 * implementation for sending bytes in libgapcom.
 *
 * @param handle Handle to the libgapcom session
 * @param sender_impl Implementation of the gapcom_sender_t interface
 */
void gapcom_set_sender_impl(
    gapcom_handle_t *handle,
    gapcom_sender_t *sender_impl);

/**
 * @brief Install a user-given callback to be invoked when receiving a specific message type
 *
 * Please note the following design choices:
 *
 * - There is no send-time callack, only receive-time
 * - Callbacks will be invoked with a generic pointer to the Protocol Buffer content of
 *   the message. Callback code needs to cast the pointer explicitly to the expected type
 * - There cannot be more than 1 callback installed for a given message type. Invoking this
 *   function twice in a row with different callback functions will result in using the
 *   one installed the latter.
 *
 * @param handle Handle to the libgapcom session
 * @param callback User-given callback function
 * @param msg_type Type of message for which the callback shall be invoked upon reception
 */
void gapcom_install_callback(
    gapcom_handle_t *handle,
    gapcom_callback_t callback,
    gapcom_msg_t msg_type);
/**
 * @brief Remove a callback installed with gapcom_install_callback()
 *
 * After calling this function, no callback will be attached to the given message type.
 * One can install a new callback like before, using gapcom_install_callback().
 *
 * @param handle Handle to the libgapcom session
 * @param msg_type Type of message for which the callback shall be invoked upon reception
 */
void gapcom_uninstall_callback(
    gapcom_handle_t *handle,
    gapcom_msg_t msg_type);

/**
 * @brief Launch the libgapcom parser on data.
 *
 * The libgapcom parser relies on the TinyFrame state machine, which will:
 *
 * - Check the validity of the TinyFrame header
 * - Check the validity of header CRC
 * - Check the validity of payload CRC
 * - Identify the message type and length
 * - If some user-given callback has been installed for this message type, invoke it with
 *   the decoded Protocol Buffer payload as an argument
 *
 * In practice, this function should be called on data collected by the user with whatever
 * communication channel is currently being used: UART, UNIX socket, etc.
 *
 * @param handle Handle to the libgapcom session
 * @param buf Data to parse
 * @param len Length of given data
 */
void gapcom_accept(gapcom_handle_t *handle, const uint8_t *buf, size_t len);

#endif /* GAPCOM_H */
