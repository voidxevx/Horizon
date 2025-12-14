#pragma once

#include "../systems/function.h"

#include <map>
#include <optional>
#include <set>

namespace neb
{

	struct TraitMethod
	{
		function::FunctionTemplate Template;
		std::map<type::PropertyID, function::IFunction*> Implementations;

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
	};

}