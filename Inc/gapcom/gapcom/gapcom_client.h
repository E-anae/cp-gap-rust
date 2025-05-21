#ifndef GAPCOM_CLIENT_H
#define GAPCOM_CLIENT_H

#include "gapcom/gapcom.h"
#include <gap.pb.h>

/**
 * @brief Send a Ping request to the server. This operation cannot fail.
 *
 * @param handle Handle to libgapcom session
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_request_ping(gapcom_handle_t *handle);

/**
 * @brief Send a Set Log Verbosity request to the server.
 *
 * @param handle Handle to libgapcom session
 * @param verbosity Log verbosity level to set
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_request_set_log_verbosity(
    gapcom_handle_t *handle,
    GAPLogVerbosity verbosity);

/**
 * @brief Send a Set Gyroscope request to the server.
 *
 * @param handle Handle to libgapcom session
 * @param set True to enable the gyroscope reading, false to disable it
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_request_set_gyroscope(
    gapcom_handle_t *handle,
    bool set);

/**
 * @brief Send a Set Version request to the server.
 *
 * @param handle Handle to the libgapcom session
 * @param fw_version Version to set in the server
 * @return int int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_request_set_version(
    gapcom_handle_t *handle,
    const char *version);

/**
 * @brief Send a Get Version request to the server.
 *
 * @param handle Handle to the libgapcom session
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_request_get_version(gapcom_handle_t *handle);

/**
 * @brief Send a Selftest request to the server.
 *
 * @param handle Handle to the libgapcom session
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_request_selftest(gapcom_handle_t *handle);

/**
 * @brief Send a Power Save request to the server.
 *
 * @param handle Handle to the libgapcom session
 * @param save_power True to ask the server to enter power saving mode, false otherwise
 * @return int -1 if the message could not be sent, otherwise the number of bytes sent
 */
int gapcom_request_power_save_mode(
    gapcom_handle_t *handle,
    bool save_power);

#endif /* GAPCOM_CLIENT_H */
