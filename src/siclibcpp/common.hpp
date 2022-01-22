#pragma once
#include <initializer_list>
#include <cstdint>
#include <range/v3/all.hpp>

namespace sic
{

using ufshort = uint_fast16_t;
using ufchar = uint_fast8_t;
using SIZE_TYPE_DEFAULT = size_t;
inline const size_t STATIC_CONTAINER_DEFAULT_SIZE = 128;

// #define likely(x) __builtin_expect((x), 1)

template<typename T>
constexpr auto LIKELY(T t)
{
  return __builtin_expect(t, 1);
}

}
