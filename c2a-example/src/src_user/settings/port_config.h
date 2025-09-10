#ifndef PORT_CONFIG_H_
#define PORT_CONFIG_H_

// ============================================== //
// =            UART関連のポート設定             = //
// ============================================== //
#define  PORT_CH_UART_TEST                 (PORT_CH_LVTTL_AOBC)
#define  PORT_CH_LVTTL_AOBC                (0)  //!< USART1
#define  PORT_CH_RS422_SAMPLE1             (1)  //!< USART2
#define  PORT_CH_RS422_MOBC_EXT            (2)  //!< USART3
#define  PORT_CH_RS422_SAMPLE2             (3)  //!< UART4
#define  PORT_CH_RS422_SAMPLE3             (4)  //!< UART5

#define PORT_CH_RS422_SAMPLE4              (5) //!< UART7

#define  PORT_CH_MAX_UART_PORT             (6)  //!< UARTポート上限

// ============================================== //
// =            GPIO関連のポート設定               = //
// ============================================== //
#define   PORT_CH_GPIO_OUT_SAMPLE0         (0x0001)  //!< LVTTL OUT ch1
#define   PORT_CH_GPIO_OUT_SAMPLE1         (0x0002)  //!< LVTTL OUT ch2
#define   PORT_CH_GPIO_OUT_SAMPLE2         (0x0003)  //!< LVTTL OUT ch3
#define   PORT_CH_GPIO_OUT_SAMPLE3         (0x0004)  //!< LVTTL OUT ch4
#define   PORT_CH_GPIO_OUT_SAMPLE4         (0x0005)  //!< LVTTL OUT ch5
#define   PORT_CH_GPIO_OUT_SAMPLE5         (0x0006)  //!< LVTTL OUT ch6
#define   PORT_CH_GPIO_OUT_SAMPLE6         (0x0007)  //!< LVTTL OUT ch7
#define   PORT_CH_GPIO_OUT_SAMPLE7         (0x0008)  //!< LVTTL OUT ch8
#define   PORT_CH_GPIO_OUT_SAMPLE8         (0x0009)  //!< LVTTL OUT ch9

#define   PORT_CH_GPIO_IN_SAMPLE0          (0x0001)  //!< LVTTL IN ch1
#define   PORT_CH_GPIO_IN_SAMPLE1          (0x0002)  //!< LVTTL IN ch2
#define   PORT_CH_GPIO_IN_SAMPLE2          (0x0003)  //!< LVTTL IN ch3
#define   PORT_CH_GPIO_IN_SAMPLE3          (0x0004)  //!< LVTTL IN ch4
#define   PORT_CH_GPIO_IN_SAMPLE4          (0x0005)  //!< LVTTL IN ch5
#define   PORT_CH_GPIO_IN_SAMPLE5          (0x0006)  //!< LVTTL IN ch6

// ============================================== //
// =            ADC関連のポート設定                = //
// ============================================== //
#define   PORT_CH_ADC_SAMPLE0              (0x00)  //!< ADC ch1
#define   PORT_CH_ADC_SAMPLE1              (0x01)  //!< ADC ch2

#endif
