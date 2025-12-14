#pragma once

#include "DataTypes/types.h"

#include <map>
#include <optional>

namespace neb
{

	class State
	{
	public:
		State();
		~State();

		static State* Get() { return s_Instance; }
		void LinkModule(const char* name, const char* filePath);

	private:
		static State* s_Instance;

		/*
		 * Maps the id of the module to either its file path if not yet built, or nothing if already built.
		 */
		std::map<type::PropertyID, std::optional<const char*>> m_Modules;
	};

	// rust linkages
	extern "C"
	{
		void neb_init();
		void neb_link_module(const char*, const char*);
	}

}