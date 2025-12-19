#pragma once

#include "../DataTypes/types.h"
#include "generation/token.h"
#include "../ErrorHandling/ErrorHandler.h"
#include "../systems/function.h"

#include <map>
#include <optional>
#include <variant>

namespace neb
{

	struct ModulePackage
	{
		std::map<type::PropertyID, sys::Function> GlobalFunctions;
	};

	class State
	{
	public:
		State();
		~State();

		//////////
		// UTIL //
		//////////

		static State* Get() { return s_Instance; }

		/////////////////////
		// HANDOFF LINKAGE //
		/////////////////////

		void LinkModule(const char* name, const char* filePath);

		////////////////
		// GENERATION //
		////////////////

		void loadFile(type::PropertyID id);
		gen::TokenPackage tokenizeFile(const std::string& filePath);

		ModulePackage buildTokens(const gen::TokenPackage& package);
		const std::optional<std::pair<type::PropertyID, sys::Function>> buildFunction(size_t& index, const gen::TokenPackage& package, std::map<type::PropertyID, sys::Function>& scope) const;
		const std::optional<type::Property> buildProperty(size_t& index, const gen::TokenPackage& package) const;

	private:
		static State* s_Instance;
		std::map<type::PropertyID, std::variant<std::string, ModulePackage>> m_Modules;
	};

	// rust linkages
	extern "C"
	{
		void neb_init();
		void neb_link_module(const char*, const char*);
	}

}