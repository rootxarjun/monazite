// src/src_user/Settings/port_config.h
local PORT_CONFIG = {
  PORT_CH_RS422_MOBC_REPRO: 0,
  PORT_CH_RS422_MOBC_EXT: 1,
  PORT_CH_RS422_XTX: 2,
  PORT_CH_RS422_MIS_IF: 3,
  PORT_CH_RS422_SCOBC: 4,
  PORT_CH_RS422_PCU_PIC2: 5,
  PORT_CH_RS422_PCU_PIC1: 6,
  PORT_CH_RS422_RESERVE: 7,
  PORT_CH_LVTTL_AOBC: 8,
  PORT_CH_LVTTL_RESERVE: 9,
  PORT_CH_LVTTL_STX: 10,
  PORT_CH_LVTTL_TOBC: 11,
  PORT_CH_LVTTL_LOBC: 12,
};

local uart_spaghetti = std.foldl(function(a, b) a + b, [
  {
    plugs+: {
      // SILS 側の kble socket を開く
      ['sils_' + portName]: 'ws://localhost:9696/channels/%s' % PORT_CONFIG[portName],
      // PC 側のシリアルポートを開く
      ['pc_' + portName]: 'ws://%s:9600/open?baudrate=115200&port=%s' % [
        std.extVar('KBLE_SERIALPORT_ADDR'),
        std.extVar(portName),
      ],
    },
    links+: {
      // 双方向に接続
      ['sils_' + portName]: 'pc_' + portName,
      ['pc_' + portName]: 'sils_' + portName,
    },
  }
  for portName in std.objectFields(PORT_CONFIG)
  if std.extVar(portName) != '' // 空文字列の場合は接続しない
], {});

uart_spaghetti + {
  plugs+: {
    tmtc_c2a: 'ws://localhost:8910',
    sils_ccsds: 'ws://localhost:22545',
  },
  links+: {
    tmtc_c2a: 'sils_ccsds',  // CMD
    sils_ccsds: 'tmtc_c2a',  // TLM
  },
}
