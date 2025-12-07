#include "test.h"

#include <iostream>

extern "C"
{
	void cpp_print()
	{
		std::cout << "Hello, cpp\n";
	}

	int32_t square_add(int32_t a, int32_t b)
	{
		return a * a + b;
	}
}