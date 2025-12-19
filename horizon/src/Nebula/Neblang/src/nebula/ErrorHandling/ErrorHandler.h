#pragma once

#include <iostream>
#include <cassert>

namespace neb::debug
{
	static const char* s_DebugPipe = "unitialized error";
	static bool s_DebugMode = false;
#define NEB_ERROR_CRITICAL 0
#define NEB_ERROR_WARNING  1
#define NEB_ERROR_INFO     2

#define NEB_ERROR_CPP      0

	extern "C"
	{
		void print_error(int32_t origin, int32_t type, const char* message, bool debugOnly, bool condition);
		void set_error_pipe(const char* pipe);
		void enable_debug_mode();
	}

#define neberror(t, m) neb::debug::print_error(0, t, m, false, false)
#define neberror_debug(t, m) neb::debug::print_error(0, t, m, true, false)
#define neberror_assertion(c, t, m) neb::debug::print_error(0, t, m, false, c)
#define neberror_assertion_debug(c, t, m) neb::debug::print_error(0, t, m, true, c)

}