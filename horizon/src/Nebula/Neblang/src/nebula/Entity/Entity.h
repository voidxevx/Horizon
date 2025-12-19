#pragma once

#include "../DataTypes/types.h"
#include "../systems/function.h"

#include <set>

namespace neb::entity
{

	class EntityVTable 
	{
	public:
		EntityVTable(std::set<type::PropertyID> components, std::set<type::PropertyID> archetypes, std::set<type::PropertyID> traits, sys::Function constructor)
			: m_Components(components)
			, m_Archetypes(archetypes)
			, m_Traits(traits)
			, m_Constructor(constructor)
		{}

	private:
		std::set<type::PropertyID> m_Components;
		std::set<type::PropertyID> m_Archetypes;
		std::set<type::PropertyID> m_Traits;
		sys::Function m_Constructor;
	};

}