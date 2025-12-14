#pragma once

#include "../DataTypes/types.h"
#include "../systems/function.h"

#include <set>

namespace neb::entity
{

	class EntityVTable 
	{
	public:
		EntityVTable(std::set<PropertyID> components, std::set<PropertyID> archetypes, std::set<PropertyID> traits, function::IFunction* constructor)
			: m_Components(components)
			, m_Archetypes(archetypes)
			, m_Traits(traits)
			, m_Constructor(constructor)
		{}

	private:
		std::set<PropertyID> m_Components;
		std::set<PropertyID> m_Archetypes;
		std::set<PropertyID> m_Traits;
		function::IFunction* m_Constructor;
	};

}