#pragma once

#include "../DataTypes/types.h"
#include "../systems/function.h"

#include <map>
#include <optional>
#include <set>

namespace neb::component
{

	class ArchetypeVTable
	{
	public:
		ArchetypeVTable(std::set<PropertyID> components)
			: m_Components(components)
		{}

		void
		AddFunction(PropertyID id, function::IFunction* method)
		{
			m_Functions[id] = method;
		}

		inline std::optional<function::IFunction*> 
		GetFunction(const PropertyID& id) 
		const 
		{
			if (m_Functions.count(id) > 0)
				return m_Functions.at(id);
			else
				return std::nullopt;
		}

		inline const bool HasComponent(const PropertyID& component) const { return m_Components.contains(component); }

	private:
		std::set<PropertyID> m_Components;
		std::map<PropertyID, function::IFunction*> m_Functions;
	};

}