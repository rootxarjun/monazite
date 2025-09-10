#ifndef RAMECC_H_
#define RAMECC_H_

#include <stdint.h>

/**
 * @brief  Memory scrubbing の回数を返す．
 * @note
 * @param  None.
 * @return Memory scrubbing の回数
 */
uint32_t RAMECC_get_scrubbing_loop(void);

/**
 * @brief  single error の数を返す．
 * @note
 * @param  None.
 * @return single error の数
 */
uint32_t RAMECC_get_single_error(void);

/**
 * @brief  double error の数を返す．
 * @note
 * @param  None.
 * @return double error の数
 */
uint32_t RAMECC_get_double_error(void);

/**
 * @brief  double error on byte write の数を返す．
 * @note
 * @param  None.
 * @return double error on byte write の数
 */
uint32_t RAMECC_get_double_error_on_byte_write(void);

/**
 * @brief  DTCM の single error の数を返す．
 * @note
 * @param  None.
 * @return DTCM single error の数
 */
uint32_t RAMECC_get_dtcm_single_error(void);

/**
 * @brief  DTCM の double error の数を返す．
 * @note
 * @param  None.
 * @return DTCM double error の数
 */
uint32_t RAMECC_get_dtcm_double_error(void);

/**
 * @brief  DTCM の double error on byte write の数を返す．
 * @note
 * @param  None.
 * @return DTCM double error on byte write の数
 */
uint32_t RAMECC_get_dtcm_double_error_on_byte_write(void);

/**
 * @brief  Memory scrubbing の間隔を返す．
 * @note
 * @param  None.
 * @return Memory scrubbing の間隔
 */
uint32_t RAMECC_get_scrubbing_interval(void);

/**
 * @brief  Memory scrubbing の間隔を設定する．
 * @note
 * @param  scrubbing_interval_tick: Memory scrubbing の間隔
 * @retval None
 */
void RAMECC_set_scrubbing_interval(uint32_t scrubbing_interval_tick);

#endif
