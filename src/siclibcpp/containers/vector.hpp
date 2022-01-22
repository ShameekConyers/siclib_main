#pragma once

#include <stddef.h>
#include <variant>
#include <bit>
#include <vector>
#include <iostream>
#include <stack>

#include "../common.hpp"
#include "array.hpp"
#include "../math.hpp"
#include "stack.hpp"


namespace sic
{


const ufchar VECTOR_MAX_SIZE = 32;
const ufchar VECTOR_BIT_CHUNK_SIZE = log(VECTOR_MAX_SIZE, 2);
constexpr ufchar MAX_STACK_SIZE = 64;

using BitStack = StackStatic<ufchar, MAX_STACK_SIZE>;

// using BitStack = std::stack<ufchar>;

template<typename T>
using VectorBins = ArraySharedOnHeap<T, VECTOR_MAX_SIZE>;

struct VectorBitChunk {
  size_t next;
  size_t rest;
};

/**
 * @brief allows iterative generation of indexing bits for Vector
 *
 * @param chunk
 * @param capacity
 * @return VectorBitChunk
 */
VectorBitChunk inline get_vector_bit_chunk(size_t chunk, size_t capacity)
{
  if (LIKELY(capacity <= VECTOR_MAX_SIZE)) {
    return { chunk, 0 };
  }
  size_t i = capacity - 1;
  size_t j = i;
  j = j >> VECTOR_BIT_CHUNK_SIZE;

  unsigned int next_bit_mask = (i ^ j);
  unsigned int rest_bit_mask = j;

  auto next = (chunk & next_bit_mask);
  auto rest = chunk & rest_bit_mask;

  while ((next_bit_mask & 1) != 1) {
    next_bit_mask = next_bit_mask >> 1;
    next = next >> 1;
  }
  return { next, rest };
};


/**
 * @brief immutable optimized vector, copy on write & implemented as
 * a bitmapped vector trie.
 * @note only the leaf notes hold the actual data, otherwise it's pointers
 */
template<typename T>
class Vector {
public:
  using Self = Vector;
  using value_type = T;

  std::variant<
    VectorBins<T>,
    VectorBins<Vector<T>>
  > m_data;
  size_t m_size;
  size_t m_capacity;
  size_t m_is_leaf;

  Vector()
  {

  }

  Vector(std::initializer_list<T> data)
  {
    build_from_iterator(data.begin(), data.end());
  }

  template<typename Iter>
  Vector(Iter begin, Iter end)
  {
    build_from_iterator(begin, end);
  }

  template<typename C, typename V = typename C::value_type>
  Vector(const C& input_collection)
  {
    build_from_iterator(input_collection.cbegin(), input_collection.cend());
  }

  Vector(const Vector<T>& other) = default;
  Vector(const Vector<T>&& other) = default;
  Vector& operator=(const Vector<T>& other) = default;
  Vector& operator=(Vector<T>&& other) = default;


  template<typename Iter>
  void build_from_iterator(Iter begin, Iter end)
  {
    init_vector_fields(begin, end);

    if (is_leaf()) {
      m_data.template emplace<0>(begin, end);
    }
    else {
      m_data.template emplace<1>();
      VectorBins<Vector<T>>& tmp = std::get<1>(m_data);
      size_t next_vector_capacity = m_capacity / VECTOR_MAX_SIZE;

      size_t to_process = m_size;
      size_t last = std::min(to_process, next_vector_capacity);
      Iter cursor_begin = begin;
      Iter cursor_end = std::next(cursor_begin, last);

      while (to_process != 0) {
        // Vector<T> v{ cursor_begin, cursor_end };
        // tmp.push_back(v);
        tmp.emplace_back(cursor_begin, cursor_end);

        if (to_process < next_vector_capacity) break;
        to_process = to_process - next_vector_capacity;

        cursor_begin = std::next(cursor_begin, last);
        last = std::min(to_process, next_vector_capacity);
        cursor_end = std::next(cursor_end, last);
      }
    }
  }

  const T& operator[](size_t index)
  {
    if (is_leaf()) {
      return std::get<0>(m_data)[index];
    }
    else {
      BitStack result_stack = make_bit_stack(index);
      return index_with_bit_stack(result_stack);
    }
  }

  const T& index_with_bit_stack(BitStack& base_nums_stack)
  {
    auto index = base_nums_stack.top();
    base_nums_stack.pop();

    if (is_leaf()) {
      return std::get<0>(m_data)[index];
    }
    else {
      return std::get<1>(m_data)[index].index_with_bit_stack(base_nums_stack);
    }
  }

  bool is_leaf()
  {
    return m_is_leaf;
  }

  template<typename Iter>
  void init_vector_fields(Iter begin, Iter end)
  {
    m_size = std::distance(begin, end);
    init_leaf_status();
    auto num_levels = ceil(log((double)m_size, VECTOR_MAX_SIZE));
    m_capacity = power((size_t)VECTOR_MAX_SIZE, num_levels);
  }

  void init_leaf_status()
  {
    if (m_size <= VECTOR_MAX_SIZE) {
      m_is_leaf = true;
    }
    else {
      m_is_leaf = false;
    }
  }

  BitStack make_bit_stack(size_t num)
  {
    int base = VECTOR_MAX_SIZE;
    int s_size = log(m_capacity, base);
    BitStack res{};

    while (res.size() < s_size) {
      auto rem = num % base;
      res.push(rem);
      num = num / base;
    }

    return res;
  }

};


}
