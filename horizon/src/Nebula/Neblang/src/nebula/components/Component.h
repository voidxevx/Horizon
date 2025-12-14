#pragma once

#include "../systems/function.h"

#include <vector>
#include <map>
#include <optional>

namespace neb
{

	class ComponentVTable
	{
	public:
		ComponentVTable(std::vector<std::pair<type::PropertyID, type::PropertyID>> properties)
		{
			for (const auto& prop : properties)
			{
				m_PropertyLocations[prop.first] = m_ComponentProperties.size();
				m_ComponentProperties.push_back(prop.second);
			}
		}

		~ComponentVTable()
		{
			for (auto& method : m_Methods)
				delete method.second;
		}

		void
		AddMethod(type::PropertyID id, function::IFunction* method)
		{
			m_Methods[id] = method;
		}

		inline std::optional<function::IFunction*> 
		GetMethod(const type::PropertyID& id)
		const 
		{
			if (m_Methods.count(id) > 0)
				return m_Methods.at(id);
			else
				return std::nullopt;
		}

		inline const size_t GetAllocationSize() const { return m_ComponentProperties.size(); }
		inline const std::vector <type::PropertyID > & GetProperties() const { return m_ComponentProperties; }
		const bool HasProperty(const type::PropertyID& prop) const { return m_PropertyLocations.count(prop) > 0; }

		const size_t GetPropertyLocation(const type::PropertyID& id) const { return m_PropertyLocations.at(id); }

	private:
		std::vector<type::PropertyID> m_ComponentProperties; // list of property types in the order that they are emplaced.
		std::map<type::PropertyID, size_t> m_PropertyLocations; // maps property ids to their location within the property list. this will be used to find where the property is located in virtual memory.

		std::map<type::PropertyID, function::IFunction*> m_Methods;
	};

}