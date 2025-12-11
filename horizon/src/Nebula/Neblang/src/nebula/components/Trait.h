#pragma once

#include "../systems/function.h"

#include <map>
#include <optional>
#include <set>

namespace neb::component
{

	struct TraitMethod
	{
		function::FunctionTemplate Template;
		std::map<PropertyID, function::IFunction*> Implementations;

		TraitMethod(function::FunctionTemplate templ)
			: Template(templ)
		{}

		~TraitMethod()
		{
			for (auto& impl : Implementations)
				delete impl.second;
		}
	};

	class TraitVTable
	{
	public:
		TraitVTable(std::set<PropertyID> requirements)
			: m_RequiredComponents(requirements)
		{}

		void
		AddMethod(PropertyID id, function::FunctionTemplate templ)
		{
			m_Methods[id] = TraitMethod(templ);
		}

		void 
		AddImplementation(PropertyID method, PropertyID entityClass, function::IFunction* impl)
		{
			if (m_Methods.count(method))
			{
				TraitMethod& traitMethod = m_Methods.at(method);
				traitMethod.Implementations[entityClass] = impl;
			}
		}

		inline std::optional<function::IFunction*>
		GetImplementation(const PropertyID& id, const PropertyID& entityClass)
		const
		{
			if (m_Methods.count(id) > 0)
				if (m_Methods.at(id).Implementations.count(entityClass) > 0)
					return m_Methods.at(id).Implementations.at(entityClass);
				else
					return std::nullopt;
			else
				return std::nullopt;
		}

		inline const bool RequiresComponent(const PropertyID& component) const { return m_RequiredComponents.contains(component); }

	private:
		std::map<PropertyID, TraitMethod> m_Methods;
		std::set<PropertyID> m_RequiredComponents;
	};

}