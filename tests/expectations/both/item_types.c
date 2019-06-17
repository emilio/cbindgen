#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define WELL_THIS_TOO 4

enum OnlyThisShouldBeGenerated {
  Foo,
  Bar,
};
typedef uint8_t OnlyThisShouldBeGenerated;
