#include "state.h"

#include "Debug/ErrorHandler.h"

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
			neberror_debug(NEB_ERROR_WARNING, "Module under the same alias was already linked");
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
			neberror_assertion(!State::Get(), NEB_ERROR_CRITICAL, "State is not initialized or was destroyed.");
			State::Get()->LinkModule(name, path);
		}
	}

}