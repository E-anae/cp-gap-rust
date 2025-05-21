#ifndef GAPCOM_SERVER_H
#define GAPCOM_SERVER_H

#include "gap.pb.h"

#include "gapcom/gapcom.h"

/**
 * @brief Send a Ping response to the client.
 *
 * @param handle Handle to libgapcom session
 * @param error_code GAP_OK if operation went fine, or some specific error code if an error occurred
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_respond_ping(
    gapcom_handle_t *handle,
    GAPErrorCode error_code);

/**
 * @brief Send a Set Log Verbosity response to the client.
 *
 * @param handle Handle to libgapcom session
 * @param error_code GAP_OK if operation went fine, or some specific error code if an error occurred
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_respond_set_log_verbosity(
    gapcom_handle_t *handle,
    GAPErrorCode error_code);

/**
 * @brief Send a Set Gyroscope response to the client.
 *
 * @param handle Handle to libgapcom session
 * @param error_code GAP_OK if operation went fine, or a specific error status otherwise. In case of error, the client should ignore tag_detected
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_respond_set_gyroscope(
    gapcom_handle_t *handle,
    GAPErrorCode error_code);

/**
 * @brief Send a Set Version response to the client.
 *
 * @param handle Handle to the libgapcom session
 * @param error_code GAP_OK if operation went fine, or a specific error status otherwise. In case of error, the client should ignore tag_detected
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_respond_set_version(
    gapcom_handle_t *handle,
    GAPErrorCode error_code);

/**
 * @brief Send a Get Version response to the client.
 *
 * @param handle Handle to the libgapcom session
 * @param error_code GAP_OK if operation went fine, or a specific error status otherwise. In case of error, the client should ignore tag_detected
 * @param version Version retransmitted by the server
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_respond_get_version(
    gapcom_handle_t *handle,
    GAPErrorCode error_code,
    const char *version);

/**
 * @brief Send a Selftest response to the client.
 *
 * @param handle Handle to the libgapcom session
 * @param error_code GAP_OK if operation went fine, or a specific error status otherwise
 * @param flash_test_code Result of the flash memory test
 * @param debuguart_test_code Result of the debug UART test
 * @param gyroscope_test_code Result of the 3-axis gyroscope test
 * @param led_test_code Result of the LED test
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_respond_selftest(
    gapcom_handle_t *handle,
    GAPErrorCode error_code,
    GAPSelftestCode flash_test_code,
    GAPSelftestCode debuguart_test_code,
    GAPSelftestCode gyroscope_test_code,
    GAPSelftestCode led_test_code);

/**
 * @brief Send a Power Save Mode response to the client.
 *
 * @param handle Handle to the libgapcom session
 * @param error_code GAP_OK if operation went fine, or a specific error status otherwise
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_respond_power_save_mode(
    gapcom_handle_t *handle,
    GAPErrorCode error_code);

#endif /* GAPCOM_SERVER_H */
