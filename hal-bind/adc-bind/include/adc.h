/**
 * @file
 * @brief ADC(A/D Converter)のラッパークラスです。
 * @note
 */
#ifndef ADC_H_
#define ADC_H_

#include <stdint.h>

/**
 * @enum  ADC_ERR_CODE
 * @brief ADCのエラーコードを定義するenum
 * @note  int16_tを想定
 */
typedef enum
{
  ADC_UNKNOWN_ERR = -14, //!< 原因不明
  ADC_CHANNEL_ERR = -1,  //!< チャンネル異常
  ADC_OK          =  0,  //!< OKは0を踏襲
} ADC_ERR_CODE;

/**
 * @brief  ADC を初期化する
 * @return 0     : 正常終了
 * @return 0以外 : ADC_CHANNEL_ERR
 */
ADC_ERR_CODE ADC_initialize(void);

/**
 * @brief  ADC の値を取得する
 * @note   0 -> 12bit::MAX の 1/2, 1 -> 12bit::MAX
 * @param  ch: チャンネル番号
 * @return 0以上: ADC の値
 * @return 負   : ADC_ERR_CODE
 */
int16_t ADC_get_value(const uint8_t ch);

/**
 * @brief  自己診断のための ADC の値を取得する
 * @note   0 -> 12bit::MAX の 1/2, 1 -> 12bit::MAX
 * @param  ch: チャンネル番号
 * @return 0以上: ADC の値
 * @return 負   : ADC_ERR_CODE
 */
int16_t ADC_get_test_value(const uint8_t ch);
#endif
