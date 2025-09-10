/**
 * @file
 * @brief Internal Flash
 */
#include "csrv_iflash.h"
#include <stddef.h> // for NULL
#include <src_core/library/print.h>
#include <src_core/tlm_cmd/common_cmd_packet_util.h>
#include <src_core/tlm_cmd/packet_handler.h>
#include <src_core/library/endian.h>
#include <src_core/library/result.h>

static RESULT IFLASH_init_(void);
static RESULT IFLASH_update_(void);

static IFLASH_Driver iflash_driver_;
const IFLASH_Driver* const iflash_driver = &iflash_driver_;

AppInfo IFLASH_update(void) {
  return AI_create_app_info("update_iflash", IFLASH_init_, IFLASH_update_);
}

static RESULT IFLASH_init_(void) {
  return RESULT_OK;
}

static RESULT IFLASH_update_(void) {
  iflash_driver_.info.status = IFLASH_get_status();
  return RESULT_OK;
}

CCP_CmdRet Cmd_IFLASH_ERASE(const CommonCmdPacket* packet)
{
  (void)packet;

  IFLASH_erase();
  return CCP_make_cmd_ret_without_err_code(CCP_EXEC_SUCCESS);
}

CCP_CmdRet Cmd_IFLASH_PROGRAM(const CommonCmdPacket* packet)
{
  const uint8_t* param = CCP_get_param_head(packet);
  uint32_t offset;
  int ret;

  ENDIAN_memcpy(&offset, param, 4);
  ret = -IFLASH_program(offset, param + 4, CCP_get_param_len(packet) - 4);
  return CCP_make_cmd_ret(CCP_EXEC_SUCCESS, (uint32_t)ret);
}
