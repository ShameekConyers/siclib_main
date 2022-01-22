#pragma once
#include <concepts>
#include <iostream>
#include <cmath>

namespace sic
{

template<typename T>
concept Integral = std::is_integral<T>::value;

template<typename T>
concept Number = std::is_fundamental<T>::value;

template<Integral T>
constexpr bool is_power_of_two(T num);


template<typename T, typename U>
constexpr T power(T base, U exponent);


template<typename T, typename U>
T log(T target_num, U base);

}



namespace sic
{

template<Integral T>
constexpr bool is_power_of_two(T num)
{
  return ((num > 0) && ((num & (num - 1)) == 0));
}

template<typename T, typename U>
constexpr T power(T base, U exponent)
{
  if (exponent == 0) {
    return 1;
  }
  else if (exponent < 0) {
    base = 1 / base;
    exponent = -exponent;
  }

  return base * power(base, exponent - 1);


}

template<typename T, typename U>
T log(T target_num, U base)
{
  return std::log(target_num) / std::log(base);
}

}
