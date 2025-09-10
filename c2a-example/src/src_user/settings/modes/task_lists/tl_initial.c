#pragma section REPRO
#include "tl_initial.h"

#include "../../../applications/app_registry.h"
#include "../../../tlm_cmd/block_command_definitions.h"

#include <src_core/tlm_cmd/block_command_loader.h>

void BCL_load_tl_initial(void)
{
  BCL_tool_register_combine( 0, BC_AR_GS_RELATED_PROCESS);
  BCL_tool_register_app    ( 8, AR_TLC_DISPATCHER_GS);
  BCL_tool_register_combine(10, BC_AC_TLM_CMD_HIRATE);
  BCL_tool_register_rotate (30, BC_AR_DRIVERS_UPDATE_INI);
  BCL_tool_register_app    (40, AR_EVENT_UTILITY);
  BCL_tool_register_app    (50, AR_CSRV_AOBC_CDIS);
  BCL_tool_register_app    (60, AR_CSRV_ADC);
  BCL_tool_register_app    (65, AR_CSRV_THERMOMETER);
  BCL_tool_register_app    (70, AR_CSRV_GPIO);
  BCL_tool_register_app    (75, AR_CSRV_RAMECC);
  BCL_tool_register_app    (80, AR_CSRV_UART);
  BCL_tool_register_app    (85, AR_CSRV_IFLASH);
  BCL_tool_register_rotate (95, BC_AR_DEBUG_DISPLAY_INI);
}

#pragma section
