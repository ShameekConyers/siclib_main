#include <siclibcpp/containers.hpp>
#include "../common.hpp"


struct ContainerBase_TEST {

};

struct StackDyanamic_TEST : ::testing::Test {

  void SetUp() override
  {

  }

  sic::StackDynamic<int> m_stack;
};


struct StaticStack_TEST : ::testing::Test {

  void SetUp() override
  {
    for (int i = 0; i < 50; i++) {
      m_test_vector.push_back(i * 3);
    }


  }

  void test_init()
  {
    ASSERT_EQ(m_static_stack_0.size(), 0);
  }

  void test_push()
  {
    for (int i = 0; i < m_test_vector.size(); ++i) {
      m_static_stack_1.push(m_test_vector[i]);
      ASSERT_EQ(m_static_stack_1.top(), m_test_vector[i]);
      ASSERT_EQ(m_static_stack_1.size(), i + 1);
    }
  }

  void test_pop_after_push()
  {
    for (int i = m_test_vector.size() - 1; i >= 0; --i) {
      ASSERT_EQ(m_static_stack_1.top(), m_test_vector[i]);
      m_static_stack_1.pop();
      ASSERT_EQ(m_static_stack_1.size(), i);
    }
  }

  sic::StackStatic<int> m_static_stack_0;
  sic::StackStatic<int> m_static_stack_1;

  std::vector<int> m_test_vector;
};


TEST_F(StaticStack_TEST, Stack)
{
  test_init();
  test_push();
  test_pop_after_push();
}
