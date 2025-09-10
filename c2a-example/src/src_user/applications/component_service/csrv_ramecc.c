#pragma section REPRO
/**
 * @file
 * @brief RAM ECC
 */
#include "csrv_ramecc.h"
#include <src_core/tlm_cmd/common_cmd_packet_util.h>

static RESULT RAMECC_init_(void);
static RESULT RAMECC_update_(void);

static RAMECC_Driver ramecc_driver_;
const RAMECC_Driver* const ramecc_driver = &ramecc_driver_;

AppInfo RAMECC_update(void) {
  return AI_create_app_info("update_ramecc", RAMECC_init_, RAMECC_update_);
}

static RESULT RAMECC_init_(void) {
  return RESULT_OK;
}

static RESULT RAMECC_update_(void) {
  ramecc_driver_.info.scrubbing_interval = RAMECC_get_scrubbing_interval();
  ramecc_driver_.info.scrubbing_loop = RAMECC_get_scrubbing_loop();
  ramecc_driver_.info.single_error = RAMECC_get_single_error();
  ramecc_driver_.info.double_error = RAMECC_get_double_error();
  ramecc_driver_.info.double_error_on_byte_write = RAMECC_get_double_error_on_byte_write();
  ramecc_driver_.info.dtcm_single_error = RAMECC_get_dtcm_single_error();
  ramecc_driver_.info.dtcm_double_error = RAMECC_get_dtcm_double_error();
  ramecc_driver_.info.dtcm_double_error_on_byte_write = RAMECC_get_dtcm_double_error_on_byte_write();

  return RESULT_OK;
}

CCP_CmdRet Cmd_RAMECC_SET_SCRUBBING_INTERVAL(const CommonCmdPacket *packet)
{
  uint32_t scrubbing_interval_tick = CCP_get_param_from_packet(packet, 0, uint32_t);
  RAMECC_set_scrubbing_interval(scrubbing_interval_tick);
  return CCP_make_cmd_ret_without_err_code(CCP_EXEC_SUCCESS);
}

#pragma section
