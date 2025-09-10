{
  plugs: {
    tmtc_c2a: 'ws://localhost:8910',
    PORT_CH_RS422_MOBC_EXT: 'ws://%s:9600/open?baudrate=115200&port=%s' % [
      std.extVar('KBLE_SERIALPORT_ADDR'),
      std.extVar('PORT_CH_RS422_MOBC_EXT'),
    ],

    // EXT PORT(UART) に流れてくる Transfer Frame は切れ目が不明になっているため、
    // tfsync を用いて切れ目を検出し、Trasfer Frame 単位に分割する。
    tfsync: 'exec:kble-c2a tfsync',
  },
  links: {
    // CMD
    tmtc_c2a: 'PORT_CH_RS422_MOBC_EXT',
    // TLM
    PORT_CH_RS422_MOBC_EXT: 'tfsync',
    tfsync: 'tmtc_c2a',
  },
}
