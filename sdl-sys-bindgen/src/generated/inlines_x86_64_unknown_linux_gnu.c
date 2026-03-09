#include "include/SDL3/SDL.h"
#include "include/SDL3/SDL_main.h"

// Static wrappers

bool SDL_size_mul_check_overflow__extern(size_t a, size_t b, size_t *ret) { return SDL_size_mul_check_overflow(a, b, ret); }
bool SDL_size_mul_check_overflow_builtin__extern(size_t a, size_t b, size_t *ret) { return SDL_size_mul_check_overflow_builtin(a, b, ret); }
bool SDL_size_add_check_overflow__extern(size_t a, size_t b, size_t *ret) { return SDL_size_add_check_overflow(a, b, ret); }
bool SDL_size_add_check_overflow_builtin__extern(size_t a, size_t b, size_t *ret) { return SDL_size_add_check_overflow_builtin(a, b, ret); }
__uint16_t __bswap_16__extern(__uint16_t __bsx) { return __bswap_16(__bsx); }
__uint32_t __bswap_32__extern(__uint32_t __bsx) { return __bswap_32(__bsx); }
__uint64_t __bswap_64__extern(__uint64_t __bsx) { return __bswap_64(__bsx); }
__uint16_t __uint16_identity__extern(__uint16_t __x) { return __uint16_identity(__x); }
__uint32_t __uint32_identity__extern(__uint32_t __x) { return __uint32_identity(__x); }
__uint64_t __uint64_identity__extern(__uint64_t __x) { return __uint64_identity(__x); }
float SDL_SwapFloat__extern(float x) { return SDL_SwapFloat(x); }
int SDL_MostSignificantBitIndex32__extern(Uint32 x) { return SDL_MostSignificantBitIndex32(x); }
bool SDL_HasExactlyOneBitSet32__extern(Uint32 x) { return SDL_HasExactlyOneBitSet32(x); }
void SDL_RectToFRect__extern(const SDL_Rect *rect, SDL_FRect *frect) { SDL_RectToFRect(rect, frect); }
bool SDL_PointInRect__extern(const SDL_Point *p, const SDL_Rect *r) { return SDL_PointInRect(p, r); }
bool SDL_RectEmpty__extern(const SDL_Rect *r) { return SDL_RectEmpty(r); }
bool SDL_RectsEqual__extern(const SDL_Rect *a, const SDL_Rect *b) { return SDL_RectsEqual(a, b); }
bool SDL_PointInRectFloat__extern(const SDL_FPoint *p, const SDL_FRect *r) { return SDL_PointInRectFloat(p, r); }
bool SDL_RectEmptyFloat__extern(const SDL_FRect *r) { return SDL_RectEmptyFloat(r); }
bool SDL_RectsEqualEpsilon__extern(const SDL_FRect *a, const SDL_FRect *b, float epsilon) { return SDL_RectsEqualEpsilon(a, b, epsilon); }
bool SDL_RectsEqualFloat__extern(const SDL_FRect *a, const SDL_FRect *b) { return SDL_RectsEqualFloat(a, b); }
