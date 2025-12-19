#pragma once

#include "../DataTypes/types.h"
#include "../systems/function.h"

#include <map>
#include <optional>
#include <set>

namespace neb
{

	class ArchetypeVTable
	{
	public:
		ArchetypeVTable(std::set<type::PropertyID> components)
			: m_Components(components)
		{}

		void
		AddFunction(type::PropertyID id, sys::Function method)
		{
			m_Functions[id] = method;
		}

		inline std::optional<sys::Function>
		GetFunction(const type::PropertyID& id) 
		const 
		{
			if (m_Functions.count(id) > 0)
				return m_Functions.at(id);
			else
				return std::nullopt;
		}

		inline const bool HasComponent(const type::PropertyID& component) const { return m_Components.contains(component); }

	private:
		std::set<type::PropertyID> m_Components;
		std::map<type::PropertyID, sys::Function> m_Functions;
	};

}