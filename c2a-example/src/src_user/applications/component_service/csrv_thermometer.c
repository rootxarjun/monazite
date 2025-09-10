#pragma section REPRO
/**
 * @file
 * @brief Thermometer
 */
#include "csrv_thermometer.h"
#include <src_core/tlm_cmd/common_cmd_packet_util.h>

static RESULT THERMOMETER_init_(void);
static RESULT THERMOMETER_update_(void);

static THERMOMETER_Driver thermometer_driver_;
const THERMOMETER_Driver* const thermometer_driver = &thermometer_driver_;

AppInfo THERMOMETER_update(void) {
  return AI_create_app_info("update_thermometer", THERMOMETER_init_, THERMOMETER_update_);
}

static RESULT THERMOMETER_init_(void) {
  return RESULT_OK;
}

static RESULT THERMOMETER_update_(void) {
  thermometer_driver_.info.temperature = THERMOMETER_get_value();

  return RESULT_OK;
}

#pragma section
