#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

int32_t sum(int32_t a, int32_t b);

int32_t subtract(int32_t a, int32_t b);

int32_t multiply(int32_t a, int32_t b);

int32_t divide(int32_t a, int32_t b);

} // extern "C"
