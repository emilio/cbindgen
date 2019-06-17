#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

static const uint8_t WELL_THIS_TOO = 4;

enum class OnlyThisShouldBeGenerated : uint8_t {
  Foo,
  Bar,
};
