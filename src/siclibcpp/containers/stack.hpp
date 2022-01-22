#pragma once
// #include "array.hpp"
#include "../common.hpp"
#include "../memory.hpp"
#include "vector_mut.hpp"
#include <exception>
#include <stack>

namespace sic
{



template<typename T, size_t CAPACITY = STATIC_CONTAINER_DEFAULT_SIZE>
struct StackStatic {
  using value_type = T;
  using size_type = SIZE_TYPE_DEFAULT;

  using Iterator = T*;
  using ConstIterator = const T*;

  T m_buffer[CAPACITY];
  size_type m_size;

  StackStatic()
  {
    init_buffer();
  }

  StackStatic(std::initializer_list<T> input_collection)
  {
    init_buffer();
    build_from_iterator(input_collection.begin(), input_collection.end());
  }

  template<typename C, typename V = typename C::value_type>
  StackStatic(const C& input_collection)
  {
    init_buffer();
    build_from_iterator(input_collection.cbegin(), input_collection.cend());
  }

  template<typename Iter>
  StackStatic(Iter begin, Iter end)
  {
    init_buffer();
    build_from_iterator(begin, end);
  }

  template<typename Iter>
  void build_from_iterator(Iter begin, Iter end)
  {
    m_size = std::distance(begin, end);
    std::copy(begin, end, this->begin());
  }

  Iterator begin()
  {
    return m_buffer;
  }

  Iterator end()
  {
    return begin() + size();
  }

  ConstIterator cbegin() const
  {
    return m_buffer;
  }

  ConstIterator cend() const
  {
    return cbegin() + size();
  }

  size_type size() const
  {
    return m_size;
  }

  void init_buffer()
  {
    init_static_buffer();
  }

  void init_static_buffer()
  {
    m_size = 0;
  }
  void init_dynamic_buffer() = delete;

  void push(T item)
  {
    if (size() == CAPACITY) {
      throw std::runtime_error("Bad PUSH");
    }
    m_buffer[m_size] = item;
    m_size++;
  }
  void push_unchecked(T item) = delete;

  void pop()
  {
    if (size() == 0) {
      throw std::runtime_error("Bad POP");
    }
    m_size--;
  }
  void pop_unchecked(T item) = delete;

  T& top()
  {
    if (size() == 0) {
      throw std::runtime_error("Bad TOP");
    }
    return m_buffer[m_size - 1];
  };

  void top_unchecked(T item) = delete;
};

template<typename T>
using StackDynamic = std::stack<T>;

}
