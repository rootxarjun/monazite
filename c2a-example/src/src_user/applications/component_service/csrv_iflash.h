/**
 * @file
 * @brief Internal Flash
 */
#ifndef CSRV_IFLASH_H_
#define CSRV_IFLASH_H_

#include "../../hal/iflash.h"
#include <src_core/tlm_cmd/common_cmd_packet.h>
#include <src_core/system/application_manager/app_info.h>

/**
 * @struct IFLASH_Info
 * @brief
 */
typedef struct {
  IFLASH_ERR_CODE status;
} IFLASH_Info;

/**
 * @struct IFLASH_Driver
 * @brief
 */
typedef struct {
  IFLASH_Info info;
} IFLASH_Driver;

extern const IFLASH_Driver* const iflash_driver;

// command
CCP_CmdRet Cmd_IFLASH_ERASE(const CommonCmdPacket* packet);
CCP_CmdRet Cmd_IFLASH_PROGRAM(const CommonCmdPacket* packet);

// アプリケーション
AppInfo IFLASH_update(void);

#endif /* CSRV_IFLASH_H_ */
