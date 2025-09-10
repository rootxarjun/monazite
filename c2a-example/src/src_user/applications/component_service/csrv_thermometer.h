/**
 * @file
 * @brief Thermometer
 */
#ifndef CSRV_THERMOMETER_H_
#define CSRV_THERMOMETER_H_

#include "../../hal/thermometer.h"
#include <src_core/system/application_manager/app_info.h>

typedef struct THERMOMETER_Driver THERMOMETER_Driver;

extern const THERMOMETER_Driver* const thermometer_driver;

#define THERMOMETER_MAX_TLM_NUM (256)

/**
 * @struct THERMOMETER_Info
 * @brief
 */
typedef struct {
  float temperature;
} THERMOMETER_Info;

/**
 * @struct THERMOMETER_Driver
 * @brief
 */
struct THERMOMETER_Driver {
  THERMOMETER_Info info;
};

// アプリケーション
AppInfo THERMOMETER_update(void);

#endif
