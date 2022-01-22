#include <vector>

namespace sic
{

// /**
//  * @brief mutable optimized vector, lives on heap
//  *
//  */
// class VectorMut {

// };

template<typename T>
using VectorMut = std::vector<T>;

/**
 * @brief vector, initally lives on stack for small sizes
 *
 */
class VectorMutSmall {

};

/**
 * @brief stack vector that throws exception once reaches size
 *
 */
class VectorMutStatic {

};


}
