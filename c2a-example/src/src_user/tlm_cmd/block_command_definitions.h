/**
 * @file
 * @brief  ブロックコマンド定義
 * @note   このコードは自動生成されています！
 */
#ifndef BLOCK_COMMAND_DEFINITIONS_H_
#define BLOCK_COMMAND_DEFINITIONS_H_

// 登録されるBlockCommandTableのblock番号を規定
typedef enum
{

  // Block Cmds for Mode Transition (シーケンスリスト)
  // ./src_user/settings/modes/transitions/ で定義
  BC_SL_START_UP_TO_INITIAL = 0,
  BC_SL_NOP  = 17,

  // Block Cmds for TaskList (タスクリスト) : 286-300
  // ./src_user/settings/modes/task_lists/ で定義
  BC_TL_START_UP = 20,
  BC_TL_INITIAL = 21,

  // Block Cmds for Elements (App Rotator, Combinar)
  // ./src_user/settings/modes/task_lists/elements/ で定義
  BC_AR_DEBUG_DISPLAY_INI = 30,
  BC_AR_DRIVERS_UPDATE_INI = 31,
  BC_AR_GS_RELATED_PROCESS = 32,
  BC_AC_TLM_CMD_HIRATE = 35,

  // ==== 各系領域 ====
  // ./C2A/tlm_cmd/normal_block_command_definition/で定義
  // イベントハンドラはBC_EH_など，接頭辞を適切につけること！

  // CDH:40-50
  BC_HK_CYCLIC_TLM = 40,
  BC_RESERVED_FOR_HK = 50,    // EM電気試験でのコマンドファイルとのバッティングを防ぐ

  // COMM:51-57

  // ==== 地上からupする“のみ”領域 ====
  // C2Aでは使用しない

  // ==== 追加領域 ====

  // Telemetry Manager
  BC_TLM_MGR_MASTER = 58,
  BC_TLM_MGR_DEPLOY = 59,
  BC_TLM_MGR_0 = 60,
  BC_TLM_MGR_1 = 61,
  BC_TLM_MGR_2 = 62,
  BC_TLM_MGR_3 = 63,
  BC_TLM_MGR_4 = 64,
  BC_TLM_MGR_5 = 65,
  BC_TLM_MGR_6 = 66,
  BC_TLM_MGR_7 = 67,
  BC_TLM_MGR_8 = 68,
  BC_TLM_MGR_9 = 69,

  // Test
  BC_TEST_EH_RESPOND = 70,
  BC_TEST_BCL = 71,

  // BCT MAX : 78

  BC_ID_MAX    // BCT 自体のサイズは BCT_MAX_BLOCKS で規定
} BC_DEFAULT_ID;

void BC_load_defaults(void);

#endif
