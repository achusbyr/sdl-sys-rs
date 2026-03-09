#include "include/SDL3/SDL.h"
#include "include/SDL3/SDL_main.h"

// Static wrappers

int _vfwscanf_l__extern(FILE *_File, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vfwscanf_l(_File, _Format, _Locale, _ArgList); }
int _vwscanf_l__extern(const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vwscanf_l(_Format, _Locale, _ArgList); }
int _vsnwscanf_l__extern(const wchar_t *_Src, size_t _MaxCount, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vsnwscanf_l(_Src, _MaxCount, _Format, _Locale, _ArgList); }
int _vswscanf_l__extern(const wchar_t *_Src, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vswscanf_l(_Src, _Format, _Locale, _ArgList); }
int _vfwprintf_p_l__extern(FILE *_File, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vfwprintf_p_l(_File, _Format, _Locale, _ArgList); }
int _vwprintf_p_l__extern(const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vwprintf_p_l(_Format, _Locale, _ArgList); }
int _vfwprintf_p__extern(FILE *_File, const wchar_t *_Format, va_list _ArgList) { return _vfwprintf_p(_File, _Format, _ArgList); }
int _vwprintf_p__extern(const wchar_t *_Format, va_list _ArgList) { return _vwprintf_p(_Format, _ArgList); }
int _vfwprintf_l__extern(FILE *_File, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vfwprintf_l(_File, _Format, _Locale, _ArgList); }
int _vwprintf_l__extern(const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vwprintf_l(_Format, _Locale, _ArgList); }
int _vswprintf_p_l__extern(wchar_t *_DstBuf, size_t _MaxCount, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vswprintf_p_l(_DstBuf, _MaxCount, _Format, _Locale, _ArgList); }
int _vswprintf_p__extern(wchar_t *_DstBuf, size_t _MaxCount, const wchar_t *_Format, va_list _ArgList) { return _vswprintf_p(_DstBuf, _MaxCount, _Format, _ArgList); }
int _vsnwprintf_l__extern(wchar_t *_DstBuf, size_t _MaxCount, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vsnwprintf_l(_DstBuf, _MaxCount, _Format, _Locale, _ArgList); }
int _vscwprintf_p_l__extern(const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vscwprintf_p_l(_Format, _Locale, _ArgList); }
int _vscwprintf_p__extern(const wchar_t *_Format, va_list _ArgList) { return _vscwprintf_p(_Format, _ArgList); }
int _vscwprintf_l__extern(const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vscwprintf_l(_Format, _Locale, _ArgList); }
int _vswprintf_c_l__extern(wchar_t *_DstBuf, size_t _MaxCount, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vswprintf_c_l(_DstBuf, _MaxCount, _Format, _Locale, _ArgList); }
int __vswprintf_l__extern(wchar_t *_DstBuf, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return __vswprintf_l(_DstBuf, _Format, _Locale, _ArgList); }
int _vswprintf_l__extern(wchar_t *_DstBuf, size_t _MaxCount, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vswprintf_l(_DstBuf, _MaxCount, _Format, _Locale, _ArgList); }
int mbsinit__extern(const mbstate_t *_P) { return mbsinit(_P); }
int _vfwscanf_s_l__extern(FILE *_File, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vfwscanf_s_l(_File, _Format, _Locale, _ArgList); }
int vfwscanf_s__extern(FILE *_File, const wchar_t *_Format, va_list _ArgList) { return vfwscanf_s(_File, _Format, _ArgList); }
int _vwscanf_s_l__extern(const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vwscanf_s_l(_Format, _Locale, _ArgList); }
int vwscanf_s__extern(const wchar_t *_Format, va_list _ArgList) { return vwscanf_s(_Format, _ArgList); }
int _vswscanf_s_l__extern(const wchar_t *_Src, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vswscanf_s_l(_Src, _Format, _Locale, _ArgList); }
int vswscanf_s__extern(const wchar_t *_Src, const wchar_t *_Format, va_list _ArgList) { return vswscanf_s(_Src, _Format, _ArgList); }
int _vsnwscanf_s_l__extern(const wchar_t *_Src, size_t _MaxCount, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vsnwscanf_s_l(_Src, _MaxCount, _Format, _Locale, _ArgList); }
int _vfwprintf_s_l__extern(FILE *_File, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vfwprintf_s_l(_File, _Format, _Locale, _ArgList); }
int _vwprintf_s_l__extern(const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vwprintf_s_l(_Format, _Locale, _ArgList); }
int vfwprintf_s__extern(FILE *_File, const wchar_t *_Format, va_list _ArgList) { return vfwprintf_s(_File, _Format, _ArgList); }
int vwprintf_s__extern(const wchar_t *_Format, va_list _ArgList) { return vwprintf_s(_Format, _ArgList); }
int _vswprintf_s_l__extern(wchar_t *_DstBuf, size_t _DstSize, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vswprintf_s_l(_DstBuf, _DstSize, _Format, _Locale, _ArgList); }
int vswprintf_s__extern(wchar_t *_DstBuf, size_t _DstSize, const wchar_t *_Format, va_list _ArgList) { return vswprintf_s(_DstBuf, _DstSize, _Format, _ArgList); }
int _vsnwprintf_s_l__extern(wchar_t *_DstBuf, size_t _DstSize, size_t _MaxCount, const wchar_t *_Format, _locale_t _Locale, va_list _ArgList) { return _vsnwprintf_s_l(_DstBuf, _DstSize, _MaxCount, _Format, _Locale, _ArgList); }
int _vsnwprintf_s__extern(wchar_t *_DstBuf, size_t _DstSize, size_t _MaxCount, const wchar_t *_Format, va_list _ArgList) { return _vsnwprintf_s(_DstBuf, _DstSize, _MaxCount, _Format, _ArgList); }
bool SDL_size_mul_check_overflow__extern(size_t a, size_t b, size_t *ret) { return SDL_size_mul_check_overflow(a, b, ret); }
bool SDL_size_mul_check_overflow_builtin__extern(size_t a, size_t b, size_t *ret) { return SDL_size_mul_check_overflow_builtin(a, b, ret); }
bool SDL_size_add_check_overflow__extern(size_t a, size_t b, size_t *ret) { return SDL_size_add_check_overflow(a, b, ret); }
bool SDL_size_add_check_overflow_builtin__extern(size_t a, size_t b, size_t *ret) { return SDL_size_add_check_overflow_builtin(a, b, ret); }
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
