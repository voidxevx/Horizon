#pragma once

#include "../systems/function.h"

#include <map>
#include <optional>
#include <set>

namespace neb
{
	// TODO: traits store functions differently than normal functions
	/*struct TraitMethod
	{
		std::map<type::PropertyID, sys::functionImpl> Implementations;

		TraitMethod(std::vector<type::Property> inputs, type::PropertyID retType)
			: Inputs(inputs)
			, ReturnType(retType)
		{}

	};

	class TraitVTable
	{
	public:
		TraitVTable(std::set<type::PropertyID> requirements)
			: m_RequiredComponents(requirements)
		{}

		void
		AddMethod(type::PropertyID id, function::FunctionTemplate templ)
		{
			m_Methods[id] = TraitMethod(templ);
		}

		void 
		AddImplementation(type::PropertyID method, type::PropertyID entityClass, function::IFunction* impl)
		{
			if (m_Methods.count(method) > 0)
			{
				TraitMethod& traitMethod = m_Methods.at(method);
				traitMethod.Implementations[entityClass] = impl;
			}
		}

		inline std::optional<function::IFunction*>
		GetImplementation(const type::PropertyID& id, const type::PropertyID& entityClass)
		const
		{
			if (m_Methods.count(id) > 0 && m_Methods.at(id).Implementations.count(entityClass) > 0)
				return m_Methods.at(id).Implementations.at(entityClass);
			return std::nullopt;
		}

		inline const bool RequiresComponent(const type::PropertyID& component) const { return m_RequiredComponents.contains(component); }

	private:
		std::map<type::PropertyID, TraitMethod> m_Methods;
		std::set<type::PropertyID> m_RequiredComponents;
	};*/

}