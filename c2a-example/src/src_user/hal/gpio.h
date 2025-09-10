/**
 * @file
 * @brief GPIO(General Purpose Input/Output)のラッパークラスです。
 * @note GPIOラッパーは、
 *       GPIOのインターフェースを実現し、
 *       GPIOポートの初期化、データ送信、データ受信を行う、
 *       GPIOラッパーのクラスです。
 *       個々の機器のインターフェースモジュールに継承させて使用します。
 */
#ifndef GPIO_HAL_H_
#define GPIO_HAL_H_

#include <stdint.h>

/**
 * @brief GPIOの方向を定義するためのenum
 * @note  1以外は入力として運用するのが安全なので、そう使ってほしい
 * @note  uint8_tを想定
 */
typedef enum
{
  GPIO_OUTPUT = 1,   //!< 出力
  GPIO_INPUT  = 0    //!< 入力
} GPIO_DIRECTION;

/**
 * @brief GPIOのHigh/Lowを定義するためのenum
 * @note  High/Lowの定義はPICポートでの論理に従う
 * @note  uint8_tを想定
 */
typedef enum
{
  GPIO_HIGH = 1,   //!< High
  GPIO_LOW  = 0    //!< Low
} GPIO_HL;

/**
 * @brief GPIOのHigh/Lowを定義するためのenum
 * @note  High/Lowの定義はPICポートでの論理に従う
 * @note  int8_tを想定
 */
typedef enum
{
  GPIO_UNKNOWN_ERR   = -14, //!< 原因不明
  GPIO_LOGIC_ERR     = -3,  //!< 論理指定異常(HIGHでもLOWでも無い)
  GPIO_DIRECTION_ERR = -2,  //!< 方向指定異常
  GPIO_PORT_ERR      = -1,  //!< ポート異常
  GPIO_OK            =  0   //!< OKは0を踏襲
} GPIO_ERR_CODE;

/**
 * @brief  GPIO出力ポートの0,1操作
 * @note   内部で各H/Wに依存した出力関数を読み出し、出力設定をする
 * @param  port : 制御するポート番号
 * @param  output : HIGHかLOWかを選択する
 * @return GPIO_ERR_CODE
 */
int GPIO_set_output(const uint8_t port, const GPIO_HL output);

/**
 * @brief  GPIO入力ポートの0,1読み出し
 * @note   内部でChipKitの入力読み出し関数を読み出し、入力値を返している
 * @param  port  : 制御するポート番号
 * @return 負    : GPIO_ERR_CODE
 * @return 負以外 : GPIO_HL
 */
int GPIO_get_input(const uint8_t port);

/**
 * @brief  GPIO出力ポートの状態の0,1読み出し
 * @note   
 * @param  port  : 制御するポート番号
 * @return 負    : GPIO_ERR_CODE
 * @return 負以外 : GPIO_HL
 */
int GPIO_get_output(const uint8_t port);

#endif
