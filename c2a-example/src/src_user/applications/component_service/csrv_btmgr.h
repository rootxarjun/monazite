/**
 * @file
 * @brief Bootloader Configuration
 */
#ifndef CSRV_BTMGR_H_
#define CSRV_BTMGR_H_

#include "../../hal/btmgr.h"
#include <src_core/tlm_cmd/common_cmd_packet.h>

// command
CCP_CmdRet Cmd_BTMGR_SET_NEXT_BOOT_BANK(const CommonCmdPacket* packet);
CCP_CmdRet Cmd_BTMGR_SYSTEM_RESET(const CommonCmdPacket* packet);

#endif /* CSRV_BTMGR_H_ */
