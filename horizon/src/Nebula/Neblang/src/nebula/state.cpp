#include "state.h"
#include <iostream>

namespace neb
{
	State* State::s_Instance = nullptr;

	State::State()
	{
		s_Instance = this;
	}

	State::~State()
	{

	}

	extern "C"
	{
		void neb_init()
		{
			new State();
		}
	}

}