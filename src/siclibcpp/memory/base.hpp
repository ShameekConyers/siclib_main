#pragma once

#include <memory>
#include <new>

namespace sic
{

namespace memory
{


// inline const double CAPACITY_MULTIPLIER = 2.0;


inline size_t grow_capacity(
  size_t capacity,
  double capacity_multiplier = 2.0)
{
  return capacity < 8 ? 8 : capacity * capacity_multiplier;
}

template<typename T>
T* allocate(size_t count = 1)
{
  // return new T[count];

  size_t meta_size = sizeof(size_t);
  size_t data_size = sizeof(T) * count;
  size_t* head = reinterpret_cast<size_t*>(std::malloc(meta_size + data_size));
  head[0] = count;
  T* ptr = reinterpret_cast<T*>(head + 1);
  T* cursor = ptr;
  for (size_t i = 0; i < count; ++i) {
    new (cursor) T();
    ++cursor;
  }

  return ptr;
}


template<typename T>
void free(T* ptr)
{
  // delete[] ptr;

  size_t* head = reinterpret_cast<size_t*>(ptr) - 1;
  size_t count = head[0];
  T* cursor = ptr;
  for (size_t i = 0; i < count; ++i) {
    cursor->~T();
    ++cursor;
  }

  std::free(head);
}

template<typename T>
T* reallocate(T* old_ptr, size_t new_count)
{

  size_t* old_head = reinterpret_cast<size_t*>(old_ptr) - 1;
  size_t old_count = old_head[0];
  size_t meta_size = sizeof(size_t);
  size_t data_size = sizeof(T) * new_count;
  size_t* head;
  T* ptr;
  T* old_cursor;
  T* cursor;

  if constexpr (std::is_trivially_copyable<T>::value) {
    head = reinterpret_cast<size_t*>(std::realloc(old_head, meta_size + data_size));
    head[0] = new_count;
    ptr = reinterpret_cast<T*>(head + 1);

    old_cursor = old_ptr;
    cursor = ptr;
  }
  else {
    size_t* head = reinterpret_cast<size_t*>(std::malloc(meta_size + data_size));
    head[0] = new_count;
    T* ptr = reinterpret_cast<T*>(head + 1);

    T* old_cursor = old_ptr;
    T* cursor = ptr;

    for (size_t i = 0; i < old_count; i++) {
      new (cursor) T(std::move(old_cursor[0]));
      ++cursor;
      ++old_cursor;
    }

    free(old_head);
  }

  for (size_t i = old_count; i < new_count; i++) {
    new (cursor) T();
    ++cursor;
  }

  return ptr;
}

}// memory
}
