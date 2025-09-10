#pragma section REPRO
/**
 * @file
 * @brief GPIO
 */
#include "csrv_gpio.h"
#include <src_core/tlm_cmd/common_cmd_packet_util.h>

static RESULT GPIO_init_(void);
static void CSRV_GPIO_init(void);
static RESULT GPIO_update_(void);

static GPIO_Driver gpio_driver_;
const GPIO_Driver* const gpio_driver = &gpio_driver_;

AppInfo GPIO_update(void) {
  return AI_create_app_info("update_gpio", GPIO_init_, GPIO_update_);
}

static RESULT GPIO_init_(void) {
  return RESULT_OK;
}

static RESULT GPIO_update_(void) {
  // output pins
  gpio_driver_.info.GPIO_output_pin[0] = GPIO_get_output(0);
  gpio_driver_.info.GPIO_output_pin[1] = GPIO_get_output(1);
  gpio_driver_.info.GPIO_output_pin[2] = GPIO_get_output(2);
  gpio_driver_.info.GPIO_output_pin[3] = GPIO_get_output(3);
  gpio_driver_.info.GPIO_output_pin[4] = GPIO_get_output(4);
  gpio_driver_.info.GPIO_output_pin[5] = GPIO_get_output(5);
  gpio_driver_.info.GPIO_output_pin[6] = GPIO_get_output(6);
  gpio_driver_.info.GPIO_output_pin[7] = GPIO_get_output(7);
  gpio_driver_.info.GPIO_output_pin[8] = GPIO_get_output(8);

  // input pins
  gpio_driver_.info.GPIO_input_pin[0] = GPIO_get_input(0);
  gpio_driver_.info.GPIO_input_pin[1] = GPIO_get_input(1);
  gpio_driver_.info.GPIO_input_pin[2] = GPIO_get_input(2);
  gpio_driver_.info.GPIO_input_pin[3] = GPIO_get_input(3);
  gpio_driver_.info.GPIO_input_pin[4] = GPIO_get_input(4);
  gpio_driver_.info.GPIO_input_pin[5] = GPIO_get_input(5);

  return RESULT_OK;
}

CCP_CmdRet Cmd_GPIO_WRITE(const CommonCmdPacket* packet)
{
  int ret;

  uint8_t pin_number = CCP_get_param_head(packet)[0];
  uint8_t value = CCP_get_param_head(packet)[1];

  ret = GPIO_set_output(pin_number, value);
  return CCP_make_cmd_ret(CCP_EXEC_SUCCESS, (uint32_t)ret);
}

#pragma section
