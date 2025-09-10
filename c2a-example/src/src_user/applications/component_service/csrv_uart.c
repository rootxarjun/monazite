#pragma section REPRO
/**
 * @file
 * @brief Uart
 */
#include "csrv_uart.h"
#include <src_core/tlm_cmd/common_cmd_packet_util.h>
#include <src_core/library/endian.h>

static UART_Driver uart_driver_;
const UART_Driver* const uart_driver = &uart_driver_;

CCP_CmdRet Cmd_UART_REOPEN(const CommonCmdPacket* packet) {
  uint8_t channnel;
  uint32_t baud_rate;
  int ret;

  channnel = (uint8_t)CCP_get_param_head(packet)[0];
  ENDIAN_memcpy(&baud_rate, CCP_get_param_head(packet) + 1, sizeof(uint32_t));

  uart_driver_.driver.uart_config.ch = channnel;
  uart_driver_.driver.uart_config.baudrate = baud_rate;

  ret = UART_reopen(&uart_driver_.driver.uart_config, 0);
  return CCP_make_cmd_ret(CCP_EXEC_SUCCESS, (uint32_t)ret);
}

#pragma section
