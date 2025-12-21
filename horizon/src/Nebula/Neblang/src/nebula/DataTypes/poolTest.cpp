#include "poolTest.h"

#include <iostream>

void test_fn()
{
	PrimitiveDataInstance inst(-32);
	std::cout << "inst as f32: " << (float)inst << "\n";
	std::cout << "inst as f64: " << (double)inst << "\n";
	std::cout << "inst as i32: " << (int32_t)inst << "\n";
	std::cout << "inst as i64: " << (int64_t)inst << "\n";
	std::cout << "inst as u32: " << (uint32_t)inst << "\n";
	std::cout << "inst as u64: " << (uint64_t)inst << "\n";
}
