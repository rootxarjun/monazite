/**
 * @file
 * @brief CCSDS API 依存の関数を宣言
 */
#ifndef CCSDS_USER_H_
#define CCSDS_USER_H_

#include <stdint.h>
#include <src_core/hal/ccsds.h>

#define CCSDS_FIFO_SIZE (8) // 現在使っている CCSDS API の設計上決まっている

/**
 * @enum  CCSDS_ERR_CODE
 * @brief CCSDS IF 関数の返り値
 */
typedef enum
{
  CCSDS_ERR_TX_NO_BUFFER = -6,
  CCSDS_ERR_TX_INVALID = -5,
  CCSDS_ERR_TX_SIZE_ERR = -4,
  CCSDS_ERR_RX_4KBPS = -3, //!< 4kbps に訂正出来ない BCH エラー
  CCSDS_ERR_RX_1KBPS = -2, //!< 1kbps に訂正出来ない BCH エラー
  CCSDS_ERR_PARAM_ERR = -1,
  CCSDS_ERR_OK = 0 //!< OK は 0 を踏襲
} CCSDS_ERR_CODE;

/**
 * @brief CCSDS TX の残り buffer をカウントするAPI を呼びだす
 * @return 残り buffer 数
 */
uint8_t CCSDS_get_buffer_num(void);

/**
 * @brief 本来は CCSDS のレートを設定する関数。example/mobc の gs.c との互換性を保つために残しているが、monazite では何もしない
 * @param[in] ui_rate: 40000000u をこれで割ったレートが設定される. 0xFF 以下である必要がある (超えていたら 0xFF 扱いになる)
 * @param[in] config: CCSDS_Config
 */
void CCSDS_set_rate(uint32_t ui_rate, CCSDS_Config *config);

/**
 * @struct CCSDS_RxStats
 * @brief CCSDS の受信統計情報
 */
typedef struct
{
  uint32_t corrupted_frames;            //!< 訂正不能なフレーム数
  uint32_t overflowed_frames;           //!< 終了シーケンスが見つからなかったフレーム数
  uint32_t found_starts;                //!< フレームの開始シーケンスが見つかった回数
  uint32_t skipped_frames;              //!< 受信バッファが溢れたためにスキップされたフレーム数
  uint32_t last_frame_corrected_errors; //!< 最後に受信されたフレームにおける訂正エラー数
} CCSDS_RxStats;

/**
 * @brief CCSDS の統計情報を取得する API を呼び出す
 * @param[out] stats: CCSDS_Stats
 */
void CCSDS_get_rx_stats(CCSDS_RxStats *rx_stats);

/**
 * @brief ダウンリンクの AOS Transfer Frame に用いる SCID をセットする
 * @param scid AOS Transfer Frame に用いる SCID
 */
void CCSDS_set_aos_scid(uint8_t scid);

#endif
