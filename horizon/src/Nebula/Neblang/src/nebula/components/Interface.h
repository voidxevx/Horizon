#pragma once

#include "../DataTypes/types.h"
#include "../systems/function.h"

#include <set>
#include <map>
#include <optional>

namespace neb::component
{

	struct InterfaceFunction
	{
		std::set<PropertyID> RequiredComponents;
		function::IFunction* Function;

		InterfaceFunction(std::set<PropertyID> requirements, function::IFunction* function)
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
		AddFunction(PropertyID id, function::IFunction* method, std::set<PropertyID> requirements)
		{
			m_Functions[id] = InterfaceFunction{ requirements, method };
		}

		inline std::optional<InterfaceFunction>
		GetFunction(const PropertyID& id)
		{
			if (m_Functions.count(id) > 0)
				return m_Functions.at(id);
			else
				return std::nullopt;
		}

	private:
		std::map<PropertyID, InterfaceFunction> m_Functions;
	};

}