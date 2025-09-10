/**
 * @file
 * @brief RAM ECC
 */
#ifndef CSRV_RAMECC_H_
#define CSRV_RAMECC_H_

#include "../../hal/ramecc.h"
#include <src_core/tlm_cmd/common_cmd_packet.h>
#include <src_core/system/application_manager/app_info.h>

typedef struct RAMECC_Driver RAMECC_Driver;

extern const RAMECC_Driver* const ramecc_driver;

#define RAMECC_MAX_TLM_NUM (256)

/**
 * @struct RAMECC_Info
 * @brief
 */
typedef struct {
  uint32_t scrubbing_interval;
  uint32_t scrubbing_loop;
  uint32_t single_error;
  uint32_t double_error;
  uint32_t double_error_on_byte_write;
  uint32_t dtcm_single_error;
  uint32_t dtcm_double_error;
  uint32_t dtcm_double_error_on_byte_write;
} RAMECC_Info;

/**
 * @struct RAMECC_Driver
 * @brief
 */
struct RAMECC_Driver {
  RAMECC_Info info;
};

// アプリケーション
AppInfo RAMECC_update(void);

CCP_CmdRet Cmd_RAMECC_SET_SCRUBBING_INTERVAL(const CommonCmdPacket *packet);

#endif
