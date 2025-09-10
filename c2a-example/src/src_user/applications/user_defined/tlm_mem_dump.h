/**
 * @file
 * @brief  一般テレメでのmem dump
 *
 *         通常はMemoryDumpやFlashUtilityを使うべきだが，
 *         現在，WINGSでのdump系テレメが非対応なため，一般テレメでdumpするコマンドを提供する
 */
#ifndef TLM_MEM_DUMP_H_
#define TLM_MEM_DUMP_H_

#include <src_core/system/application_manager/app_info.h>
#include <src_core/tlm_cmd/common_cmd_packet.h>
#include <src_core/system/time_manager/obc_time.h>

#define APP_TMD_DUMP_SIZE   (64)        //!< 1 tlmでのdumpサイズ

typedef struct
{
  uint32_t start_addr;
  uint32_t size;
} TlmMemDumpMeta;

typedef struct
{
  TlmMemDumpMeta meta;
  uint8_t        data[APP_TMD_DUMP_SIZE];
} TlmMemDump;

extern const TlmMemDump* const tlm_mem_dump;

AppInfo APP_TMD_create_app(void);

CCP_CmdRet Cmd_APP_TMD_MEM_DUMP(const CommonCmdPacket* packet);

#endif
