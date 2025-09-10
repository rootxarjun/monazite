/**
 * @file
 * @brief Internal Flash
 * @note 
 */
#ifndef IFLASH_H_
#define IFLASH_H_

#include <stdint.h>

typedef enum {
  IFLASH_ERR_OTHER = -12,
  IFLASH_ERR_READ_SECURE = -11,
  IFLASH_ERR_READ_PROTECTION = -10,
  IFLASH_ERR_ECC_DOUBLE_DETECTION = -9,
  IFLASH_ERR_OPERATION = -8,
  IFLASH_ERR_INCONSISTENCY = -7,
  IFLASH_ERR_STROBE = -6,
  IFLASH_ERR_PROGRAMMING_SEQUENCE = -5,
  IFLASH_ERR_WRITE_PROTECTION = -4,
  IFLASH_ERR_OUT_OF_BOUNDS = -3,
  IFLASH_ERR_NOT_ALIGNED = -2,
  IFLASH_ERR_BUSY = -1,
  IFLASH_ERR_OK = 0,
} IFLASH_ERR_CODE;

IFLASH_ERR_CODE IFLASH_get_status(void);
IFLASH_ERR_CODE IFLASH_erase(void);
IFLASH_ERR_CODE IFLASH_program(uint32_t offset, const uint8_t *data_v, uint32_t len);

#endif /* IFLASH_H_ */
