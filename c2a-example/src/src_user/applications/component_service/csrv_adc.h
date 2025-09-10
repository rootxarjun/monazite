/**
 * @file
 * @brief A/D Converter
 */
#ifndef CSRV_ADC_H_
#define CSRV_ADC_H_

#include "../../hal/adc.h"
#include <src_core/system/application_manager/app_info.h>

typedef struct ADC_Driver ADC_Driver;

extern const ADC_Driver* const adc_driver;

#define ADC_MAX_TLM_NUM (256)

/**
 * @struct ADC_Info
 * @brief
 */
typedef struct {
  int16_t ch1;
  int16_t ch2;
  int16_t ch3;
  int16_t half_of_max;
  int16_t full_of_max;
} ADC_Info;

/**
 * @struct ADC_Driver
 * @brief
 */
struct ADC_Driver {
  ADC_Info info;
};

// アプリケーション
AppInfo ADC_update(void);

#endif
