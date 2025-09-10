/**
 * @file
 * @brief Bootloader Configuration
 */
#include "csrv_btmgr.h"
#include "../../hal/btmgr.h"
#include <stddef.h> // for NULL
#include <src_core/library/endian.h>
#include <src_core/library/print.h>
#include <src_core/tlm_cmd/common_cmd_packet_util.h>

CCP_CmdRet Cmd_BTMGR_SET_NEXT_BOOT_BANK(const CommonCmdPacket* packet)
{
  BTMGR_BOOT_BANK next_boot_bank;
  next_boot_bank = CCP_get_param_from_packet(packet, 0, uint32_t);
  BTMGR_set_next_boot_bank(next_boot_bank);
  return CCP_make_cmd_ret_without_err_code(CCP_EXEC_SUCCESS);
}

CCP_CmdRet Cmd_BTMGR_SYSTEM_RESET(const CommonCmdPacket* packet)
{
  BTMGR_system_reset();
  return CCP_make_cmd_ret_without_err_code(CCP_EXEC_SUCCESS);
}
