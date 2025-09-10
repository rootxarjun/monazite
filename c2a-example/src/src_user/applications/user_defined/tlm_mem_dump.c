#pragma section REPRO
/**
 * @file
 * @brief  一般テレメでのmem dump
 */
#include "tlm_mem_dump.h"

#include <string.h> // for memcpy
#include <src_core/library/endian.h>
#include <src_core/tlm_cmd/common_cmd_packet_util.h>

static RESULT APP_TMD_init_(void);

static TlmMemDump tlm_mem_dump_;
const TlmMemDump* const tlm_mem_dump = &tlm_mem_dump_;


AppInfo APP_TMD_create_app(void)
{
  return AI_create_app_info("tlm_mem_dump", APP_TMD_init_, NULL);
}


static RESULT APP_TMD_init_(void)
{
  tlm_mem_dump_.meta.start_addr = 0;
  tlm_mem_dump_.meta.size = 0;
  memset(tlm_mem_dump_.data, 0, sizeof(tlm_mem_dump_.data));

  return RESULT_OK;
}


CCP_CmdRet Cmd_APP_TMD_MEM_DUMP(const CommonCmdPacket* packet)
{
  const uint8_t* param = CCP_get_param_head(packet);
  uint32_t start_addr, size;

  // パラメータを読み出し
  ENDIAN_memcpy(&start_addr, param, 4);
  ENDIAN_memcpy(&size, param + 4, 4);

  if (size > sizeof(tlm_mem_dump_.data)) {
    return CCP_make_cmd_ret_without_err_code(CCP_EXEC_ILLEGAL_PARAMETER);
  }

  tlm_mem_dump_.meta.start_addr = start_addr;
  tlm_mem_dump_.meta.size = size;
  memcpy(tlm_mem_dump_.data, (uint8_t*)start_addr, size);

  return CCP_make_cmd_ret_without_err_code(CCP_EXEC_SUCCESS);
}

#pragma section
