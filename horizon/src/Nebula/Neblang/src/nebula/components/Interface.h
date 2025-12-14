#pragma once

#include "../DataTypes/types.h"
#include "../systems/function.h"

#include <set>
#include <map>
#include <optional>

namespace neb
{

	struct InterfaceFunction
	{
		std::set<type::PropertyID> RequiredComponents;
		function::IFunction* Function;

		InterfaceFunction(std::set<type::PropertyID> requirements, function::IFunction* function)
			: RequiredComponents(requirements)
			, Function(function)
		{}

		~InterfaceFunction()
		{
			delete Function;
			Function = nullptr;
		}
	};

	class InterfaceVtable
	{
	public:
		InterfaceVtable() = default;

		void
		AddFunction(type::PropertyID id, function::IFunction* method, std::set<type::PropertyID> requirements)
		{
			m_Functions[id] = InterfaceFunction{ requirements, method };
		}

		inline std::optional<InterfaceFunction>
		GetFunction(const type::PropertyID& id)
		{
			if (m_Functions.count(id) > 0)
				return m_Functions.at(id);
			else
				return std::nullopt;
		}

	private:
		std::map<type::PropertyID, InterfaceFunction> m_Functions;
	};

}