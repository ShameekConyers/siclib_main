#include <siclibcpp/containers.hpp>
#include "../common.hpp"


struct ArrayOnStack_TEST : public ::testing::Test {

  void SetUp() override
  {
    for (int i = 0; i < 64; i++) {
      m_test_vector.push_back(i * 5);
    }
  }

  void test_default_init()
  {
    EXPECT_EQ(m_uarr_1.size(), 0);
    EXPECT_EQ(m_sarr_1.size(), 0);

  }

  void test_build_from_init_list()
  {
    sic::ArraySharedOnStack<int> sarr{ 2, 4, 10 };
    sic::ArrayUniqueOnStack<int> uarr{ 2, 4, 10 };

    std::vector<int> v{ 2 , 4, 10 };

    for (int i = 0; i < sarr.size(); i++) {
      ASSERT_EQ(sarr[i], v[i]);
      ASSERT_EQ(uarr[i], v[i]);
    }
  }

  void test_build_from_vector()
  {
    sic::ArraySharedOnStack<int> sarr{ m_test_vector };
    sic::ArrayUniqueOnStack<int> uarr{ m_test_vector };

    for (int i = 0; i < sarr.size(); i++) {
      ASSERT_EQ(sarr[i], m_test_vector[i]);
      ASSERT_EQ(uarr[i], m_test_vector[i]);
    }

    ASSERT_EQ(sarr.size(), uarr.size());
    ASSERT_EQ(sarr.size(), m_test_vector.size());
  }

  void test_assignment_operator()
  {
    sic::ArraySharedOnStack<int> sarr = m_test_vector;
    sic::ArrayUniqueOnStack<int> uarr = m_test_vector;

    for (int i = 0; i < sarr.size(); i++) {
      ASSERT_EQ(sarr[i], m_test_vector[i]);
      ASSERT_EQ(uarr[i], m_test_vector[i]);
    }

  }



  sic::ArrayUniqueOnStack<int> m_uarr_1;
  sic::ArrayUniqueOnStack<int> m_uarr_2;
  sic::ArrayUniqueOnStack<int> m_uarr_3;

  sic::ArraySharedOnStack<int> m_sarr_1;
  sic::ArraySharedOnStack<int> m_sarr_2;
  sic::ArraySharedOnStack<int> m_sarr_3;

  std::vector<int> m_test_vector;
};


TEST_F(ArrayOnStack_TEST, init)
{
  test_default_init();
  test_build_from_init_list();
  test_build_from_vector();
  test_assignment_operator();
}


TEST(ARRAY, ARRAY_SHARED_HEAP)
{
  std::vector<int> std_vector_mock(4201);
  for (int i = 0; i < std_vector_mock.size(); i++) {
    std_vector_mock[i] = i * 3;
  }

  sic::ArraySharedOnHeap<int, 6000> sic_array_shared_mock{ std_vector_mock };

  for (int i = 0; i < std_vector_mock.size(); i++) {
    EXPECT_EQ(std_vector_mock[i], sic_array_shared_mock[i]);
  }

  auto other = sic_array_shared_mock;
}
