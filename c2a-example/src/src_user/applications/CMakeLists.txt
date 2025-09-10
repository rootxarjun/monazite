cmake_minimum_required(VERSION 3.13)

project(C2A_USER_APPS)

set(C2A_SRCS
  app_registry.c
  component_service/csrv_adc.c
  component_service/csrv_aobc.c
  component_service/csrv_btmgr.c
  component_service/csrv_gpio.c
  component_service/csrv_ramecc.c
  component_service/csrv_gs.c
  component_service/csrv_iflash.c
  component_service/csrv_thermometer.c
  component_service/csrv_uart.c
  component_service/csrv_uart_test.c
  user_defined/debug_apps.c
  user_defined/tlm_mem_dump.c
)

if(C2A_BUILD_AS_CXX)
  set_source_files_properties(${C2A_SRCS} PROPERTIES LANGUAGE CXX)  # C++
endif()

add_library(${PROJECT_NAME} OBJECT ${C2A_SRCS})

include(${C2A_USER_DIR}/common.cmake)
