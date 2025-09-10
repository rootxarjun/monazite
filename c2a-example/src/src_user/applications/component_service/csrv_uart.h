/**
 * @file
 * @brief UART
 */
#ifndef CSRV_UART_H_
#define CSRV_UART_H_

#include "../../../src_core/hal/uart.h"
#include <src_core/tlm_cmd/common_cmd_packet.h>

typedef struct UART_Driver UART_Driver;

extern const UART_Driver* const uart_driver;

#define UART_MAX_TLM_NUM (256)

/**
 * @struct UART_Info
 * @brief
 */
typedef struct {
  uint32_t baud_rate;
} UART_Info;

/**
 * @struct UART_Driver
 * @brief
 */
struct UART_Driver {
  struct {
    UART_Config uart_config;  //!< RS422 class
  } driver;
  UART_Info info;
};

// コマンド
CCP_CmdRet Cmd_UART_REOPEN(const CommonCmdPacket* packet);

#endif
