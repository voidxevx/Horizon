#pragma once

#include "../systems/function.h"

#include <set>
#include <map>
#include <optional>

namespace neb
{

	struct TraitInterfaceMethod
	{
		function::FunctionTemplate Template;
		std::map<type::PropertyID, function::IFunction*> Implementations;
		std::set<type::PropertyID> Requirements;

		TraitInterfaceMethod(function::FunctionTemplate templ, std::set<type::PropertyID> requirements)
			: Template(templ)
			, Requirements(requirements)
		{}

		~TraitInterfaceMethod()
		{
			for (auto& impl : Implementations)
				delete impl.second;
		}
	};

	class TraitInterfaceVTable
	{
	public:
		TraitInterfaceVTable() = default;

		void 
		AddMethod(type::PropertyID id, function::FunctionTemplate templ, std::set<type::PropertyID> requirements)
		{
			m_Methods[id] = TraitInterfaceMethod(templ, requirements);
		}

		void 
		AddImplementation(type::PropertyID method, type::PropertyID entityClass, function::IFunction* impl)
		{
			if (m_Methods.count(method) > 0)
			{
				TraitInterfaceMethod& traitMethod = m_Methods.at(method);
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

		inline std::optional<const std::set<type::PropertyID>&>
		GetMethodRequirements(const type::PropertyID& methodID)
		const
		{
			if (m_Methods.count(methodID) > 0)
				return m_Methods.at(methodID).Requirements;
			return std::nullopt;
		}

	private:
		std::map<type::PropertyID, TraitInterfaceMethod> m_Methods;
	};

}