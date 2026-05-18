#include <stdint.h>

int _main() {
  uint32_t x = 0xF0u;
  return (x & 0x0Fu) != 0u;
}
