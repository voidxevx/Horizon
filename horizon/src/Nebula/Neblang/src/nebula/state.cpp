#include "state.h"

#include <assert.h>
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

	void
	State::LinkModule(const char* name, const char* filePath)
	{
		type::PropertyID id = (type::PropertyID)type::s_PropertyHasher(name);
		if (m_Modules.count(id) == 0)
		{
			m_Modules[id] = filePath;
		}
		else
			std::cout << "\033[33m[CPP][Neblang] WARNING - Failed to link module, Module under the same alias was already linked.\033[0m\n";
	}

	extern "C"
	{
		void 
		neb_init()
		{
			new State();
		}

		void
		neb_link_module(const char* name, const char* path)
		{
			assert(State::Get() && "\033[31m[CPP][Nebalang] CRITICAL - Failed to link module, State is not initialized or was destroyed.\033[0m");
			State::Get()->LinkModule(name, path);
		}
	}

}