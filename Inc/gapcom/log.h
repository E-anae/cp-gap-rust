#ifndef GAPCOM_LOG_H
#define GAPCOM_LOG_H

#include <stdio.h>

#if __has_include("gap_config.h")
#include "gap_config.h"
#endif

#ifdef GAPCOM_LOGGING_BACKEND_NONE
#define log(Format, ...)
#define log_com(Format, ...)
#else /* GAPCOM_LOGGING_BACKEND_CONSOLE */
#ifndef GAP_COM_EOL
#pragma error GAP_COM_EOL must be defined to give End-Of-Line termination character set
#endif
#define log(Format, ...)        printf(Format GAP_COM_EOL __VA_OPT__(,) __VA_ARGS__)
#define log_com(Format, ...)    log("[GAPCOM] " Format, __VA_ARGS__)
#endif /* ! GAPCOM_LOGGING_BACKEND_CONSOLE */

#endif /* GAPCOM_LOG_H */
