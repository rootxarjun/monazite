/**
 * @file
 * @brief Bootloader
 * @note
 */
#ifndef BTMGR_H_
#define BTMGR_H_

#include <stdint.h>

/**
 * @enum  BTMGR_BOOT_BANK
 * @brief ブートに利用するバンクを示す列挙型
 */
typedef enum
{
  BTMGR_INVALID_PARAM_ERR = -3,  //!< 不正なパラメータ
  BTMGR_OK                =  0,  //!< 正常終了
} BTMGR_BOOT_ERR_CODE;

/**
 * @enum  BTMGR_BOOT_BANK
 * @brief ブートに利用するバンクを示す列挙型
 */
typedef enum
{
  BTMGR_BANK_UNCHANGED = 0,         //!< 現在のバンクで起動する（next_boot_bank として使うときのみ）
  BTMGR_BANK_1 = 1,                 //!< Bank 1
  BTMGR_BANK_2 = 2,                 //!< Bank 2
} BTMGR_BOOT_BANK;

/**
 * @enum BTMGR_RESET_REASON
 * @brief リセット要因を示す列挙型
 */
typedef enum
{
  BTMGR_POWER_ON_RESET = 0, //!< パワーオンリセット
  BTMGR_PIN_RESET = 1, //!< ピンリセット
  BTMGR_BROWNOUT_RESET = 2, //!< ブラウンアウトリセット
  BTMGR_SYSTEM_RESET = 3, //!< CPU によって生成されたシステムリセット
  BTMGR_CPU_RESET = 4, //!< CPU リセット
  BTMGR_WINDOW_WATCHDOG_RESET = 5, //!< WWDG1 リセット
  BTMGR_INDEPENDENT_WATCHDOG_RESET = 6, //!< IWDG1 リセット
  BTMGR_GENERIC_WATCHDOG_RESET = 7, //!< ウォッチドッグリセット
  BTMGR_D1_EXITS_D_STANDBY_MODE = 8, //!< D1 が DStandby モードを終了した場合
  BTMGR_D2_EXITS_D_STANDBY_MODE = 9, //!< D2 が DStandby モードを終了した場合 
  BTMGR_D1_ENTERS_D_STANDBY_ERRONEOUSLY_OR_CPU_ENTERS_C_STOP_ERRONEOUSLY = 10, //!< D1 が誤って DStandby モードに移行した場合、または CPU が誤って CStop モードに移行した場合
  BTMGR_UNKNOWN = -1, //!< 不明
} BTMGR_RESET_REASON;

/**
 * @brief 現在のコードがどのバンクから起動しているかを取得する
 * @return BTMGR_BOOT_BANK
 */
BTMGR_BOOT_BANK BTMGR_get_current_boot_bank(void);

/**
 * @brief 次回の起動時にどのバンクから起動するかを取得する
 * @return BTMGR_BOOT_BANK
 */
BTMGR_BOOT_BANK BTMGR_get_next_boot_bank(void);

/**
 * @brief 次回の起動時にどのバンクから起動するかを設定する
 */
BTMGR_BOOT_ERR_CODE BTMGR_set_next_boot_bank(BTMGR_BOOT_BANK next_boot_bank);

/**
 * @brief STM のリセットステータスレジスタの生の値を取得する
 * @return リセットステータスレジスタの生の値
 */
uint32_t BTMGR_get_reset_flag(void);

/**
 * @brief STM のリセット要因を取得する
 * @return リセット要因
 */
BTMGR_RESET_REASON BTMGR_get_reset_reason(void);

/**
 * @brief マイコンをリセットする。これは電源の再投入を伴わないソフトウェアリセットである
 */
void BTMGR_system_reset(void);

#endif /* BTMGR_H_ */
