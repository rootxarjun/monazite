/**
 * @file
 * @brief GPIO
 */
#ifndef CSRV_GPIO_H_
#define CSRV_GPIO_H_

#include "../../hal/gpio.h"
#include <src_core/tlm_cmd/common_cmd_packet.h>
#include <src_core/system/application_manager/app_info.h>

typedef struct GPIO_Driver GPIO_Driver;

#define GPIO_MAX_TLM_NUM (256)
#define GPIO_OUTPUT_PIN_NUM (9)
#define GPIO_INPUT_PIN_NUM (6)

/**
 * @struct GPIO_Info
 * @brief
 */
typedef struct {
  uint8_t GPIO_output_pin[GPIO_OUTPUT_PIN_NUM];
  uint8_t GPIO_input_pin[GPIO_INPUT_PIN_NUM];
} GPIO_Info;

/**
 * @struct GPIO_Driver
 * @brief
 */
struct GPIO_Driver {
  GPIO_Info info;
};

extern const GPIO_Driver* const gpio_driver;

// アプリケーション
AppInfo GPIO_update(void);

// command
CCP_CmdRet Cmd_GPIO_WRITE(const CommonCmdPacket* packet);
#endif
