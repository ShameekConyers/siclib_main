#pragma once
#include <iostream>
#include "../memory.hpp"
#include "../common.hpp"

namespace sic
{


template<typename T, size_t CAPACITY = STATIC_CONTAINER_DEFAULT_SIZE>
struct ArrayUniqueOnStack {
  // types
  using size_type = size_t;
  using value_type = T;

  using Iterator = T*;
  using ConstIterator = const T*;

  // members
  T m_buffer[CAPACITY];
  size_type m_size;

  ArrayUniqueOnStack()
  {
    init_buffer();
  }

  ArrayUniqueOnStack(std::initializer_list<T> data)
  {
    init_buffer();
    build_from_iterator(data.begin(), data.end());
  }

  template<typename C, typename V = typename C::value_type>
  ArrayUniqueOnStack(const C& input_collection)
  {
    init_buffer();
    build_from_iterator(input_collection.cbegin(), input_collection.cend());
  }

  template<typename Iter>
  ArrayUniqueOnStack(Iter begin, Iter end)
  {
    init_buffer();
    build_from_iterator(begin, end);
  }

  void init_buffer()
  {
    m_size = 0;
  }

  size_type size()
  {
    return m_size;
  }

  template<typename Iter>
  void build_from_iterator(Iter begin, Iter end)
  {
    m_size = std::distance(begin, end);
    std::copy(begin, end, this->begin());
  }

  T& operator[](size_type index)
  {
    return m_buffer[index];
  }

  Iterator begin()
  {
    return m_buffer;
  }

  Iterator end()
  {
    return m_buffer + size();
  }

  ConstIterator cbegin() const
  {
    return m_buffer;
  }

  ConstIterator cend() const
  {
    return m_buffer + size();
  }

  void push_back(T item)
  {
    m_buffer[m_size] = item;
    ++m_size;
  }

  void emplace_back(T item)
  {
    push_back(item);
  }
};


template<typename T, size_t SMAX = STATIC_CONTAINER_DEFAULT_SIZE>
struct ArraySharedOnStack {
  // types
  using size_type = size_t;
  using value_type = T;

  using Iterator = T*;
  using ConstIterator = const T*;

  // members
  T m_buffer[SMAX];
  size_type m_size;

  ArraySharedOnStack()
  {
    init_buffer();
  }

  ArraySharedOnStack(std::initializer_list<T> data)
  {
    init_buffer();
    build_from_iterator(data.begin(), data.end());
  }

  template<typename C, typename V = typename C::value_type>
  ArraySharedOnStack(const C& input_collection)
  {
    init_buffer();
    build_from_iterator(input_collection.cbegin(), input_collection.cend());
  }

  template<typename Iter>
  ArraySharedOnStack(Iter begin, Iter end)
  {
    init_buffer();
    build_from_iterator(begin, end);
  }

  void init_buffer()
  {
    m_size = 0;
  }

  size_type size() const
  {
    return m_size;
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

  T& operator[](size_type index)
  {
    return m_buffer[index];
  }

  void push_back(T item)
  {
    m_buffer[m_size] = item;
    ++m_size;
  }

  void emplace_back(T item)
  {
    push_back(item);
  }
};


template<typename T, size_t CAPACITY>
struct ArrayUniqueOnHeap {
  // types

  // members
  T* m_buffer;
};

template<typename T, size_t CAPACITY>
struct ArraySharedOnHeap {
public:
  //types
  using Self = ArraySharedOnHeap;
  using value_type = T;
  using size_type = ssize_t;
  using difference_type = ptrdiff_t;

  using Iterator = T*;
  using ConstIterator = const T*;
  using ReverseIterator = std::reverse_iterator<Iterator>;
  using ConstReverseIterator = std::reverse_iterator<ConstIterator>;

  T* m_data;
  size_type m_size;
  size_type m_ref_count;

  ArraySharedOnHeap()
  {
    init_buffer();
  }

  ArraySharedOnHeap(std::initializer_list<T> data)
  {
    init_buffer();
    build_from_iterator(data.begin(), data.end());
  }

  template<typename C, typename V = typename C::value_type>
  ArraySharedOnHeap(const C& input_collection)
  {
    init_buffer();
    build_from_iterator(input_collection.cbegin(), input_collection.cend());
  }

  template<typename Iter>
  ArraySharedOnHeap(Iter begin, Iter end)
  {
    init_buffer();
    build_from_iterator(begin, end);
  }

  ArraySharedOnHeap(const ArraySharedOnHeap& other)
  {
    init_buffer();
    build_from_iterator(other.cbegin(), other.cend());
  }

  ArraySharedOnHeap& operator=(const ArraySharedOnHeap<T, CAPACITY>& other)
  {
    if (this == &other) return *this;
    free_buffer();
    init_buffer();
    build_from_iterator(other.cbegin(), other.cend());
    return *this;
  }

  ArraySharedOnHeap& operator=(ArraySharedOnHeap<T, CAPACITY>&& other)
  {
    if (this == &other) return *this;
    free_buffer();
    init_buffer();
    build_from_iterator(other.begin(), other.end());
    return *this;
  }

  ~ArraySharedOnHeap()
  {
    free_buffer();
  }

  void init_buffer()
  {
    m_ref_count = 0;
    m_size = 0;
    m_data = memory::allocate<T>(CAPACITY);
  }

  void free_buffer()
  {
    memory::free(m_data);
  }

  template<typename Iter>
  void build_from_iterator(Iter begin, Iter end)
  {
    m_size = std::distance(begin, end);
    std::copy(begin, end, this->begin());
  }

  size_type size() const
  {
    return m_size;
  }

  Iterator begin()
  {
    return m_data;
  }

  ConstIterator cbegin() const
  {
    return m_data;
  }

  ConstIterator end()
  {
    return begin() + size();
  }

  ConstIterator cend() const
  {
    return cbegin() + size();
  }

  T& operator[](size_type index)
  {
    return m_data[index];
  }

  template<typename... Args>
  void emplace_back(Args&&... args)
  {
    // T t{ std::forward < Args>(args)... };
    // m_data[m_size] = t;
    T* location = std::next(begin(), m_size);
    if constexpr (!std::is_trivially_destructible<T>::value) {
      location->~T();
    }
    new (location) T(std::forward<Args>(args)...);
    m_size++;
  }

  void push_back(T input)
  {
    m_data[m_size] = input;
    m_size++;
  }
};

}
