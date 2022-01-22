#include <siclibcpp/containers.hpp>
#include "../common.hpp"
#include <vector>

TEST(VECTOR, GET_VECTOR_BIT_CHUNK_1)
{
  auto vec_size = sic::VECTOR_MAX_SIZE;
  auto chunk_size = sic::VECTOR_BIT_CHUNK_SIZE;

  auto num = 205;
  auto res = sic::get_vector_bit_chunk(205, 1024);
  EXPECT_EQ(res.next, 6);
  EXPECT_EQ(res.rest, 13);

}

TEST(VECTOR, VECTOR_1)
{
  std::vector<int> std_vector_mock(64);
  for (int i = 0; i < std_vector_mock.size(); i++) {
    std_vector_mock[i] = i * 2;
  }
  sic::Vector<int> sic_vector_init_list{ 1, 2, 4 };
  EXPECT_EQ(sic_vector_init_list[2], 4);
  EXPECT_EQ(sic_vector_init_list[1], 2);
  EXPECT_EQ(sic_vector_init_list[0], 1);

  sic::Vector<int> sic_vector_mock{ std_vector_mock };
  for (int i = 0; i < std_vector_mock.size(); i++) {
    EXPECT_EQ(sic_vector_mock[i], std_vector_mock[i]);
  }

}

TEST(VECTOR, VECTOR_2)
{
  std::vector<int> std_vector_mock(1025);
  for (int i = 0; i < std_vector_mock.size(); i++) {
    std_vector_mock[i] = i * 2;
  }

  sic::Vector<int> sic_vector_mock{ std_vector_mock };

  for (int i = 0; i < std_vector_mock.size(); i++) {
    EXPECT_EQ(sic_vector_mock[i], std_vector_mock[i]);
  }

}
