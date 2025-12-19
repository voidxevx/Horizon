#include "ErrorHandler.h"

namespace neb::debug
{

	extern "C"
	{
		void print_error(int32_t origin, int32_t type, const char* message, bool debugOnly, bool condition)
		{
			if (!condition && (s_DebugMode || !debugOnly))
			{
				std::cout
					<< (type == 0 ? "\033[31m" : type == 1 ? "\033[33m" : "\033[34m")
					<< (origin == 0 ? "[CPP]" : "[RS]")
					<< "[Neblang] "
					<< (type == 0 ? "CRITICAL" : type == 1 ? "WARNING" : "INFO")
					<< " - " << s_DebugPipe << ", "
					<< message
					<< "\n\033[0m";
			}
			if (type == 0)
				assert(condition && "Critical error created within neblang error handling");
		}

		void set_error_pipe(const char* pipe)
		{
			s_DebugPipe = pipe;
		}

		void enable_debug_mode()
		{
			s_DebugMode = true;
		}
	}

}