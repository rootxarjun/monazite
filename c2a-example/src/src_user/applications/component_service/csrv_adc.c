#pragma section REPRO
/**
 * @file
 * @brief A/D Conveter
 */
#include "csrv_adc.h"

static RESULT ADC_init_(void);
static RESULT ADC_update_(void);

static ADC_Driver adc_driver_;
const ADC_Driver* const adc_driver = &adc_driver_;

AppInfo ADC_update(void) {
  return AI_create_app_info("update_adc", ADC_init_, ADC_update_);
}

static RESULT ADC_init_(void) {
  return RESULT_OK;
}

static RESULT ADC_update_(void) {
  adc_driver_.info.ch1 = ADC_get_value(0);
  adc_driver_.info.ch2 = ADC_get_value(1);
  adc_driver_.info.ch3 = ADC_get_value(2);

  adc_driver_.info.half_of_max = ADC_get_test_value(0);
  adc_driver_.info.full_of_max = ADC_get_test_value(1);

  return RESULT_OK;
}

#pragma section
