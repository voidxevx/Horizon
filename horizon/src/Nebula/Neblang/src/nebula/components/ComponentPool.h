#pragma once

#include "Component.h"
#include "../DataTypes/Data.h"

#include <memory>

namespace neb
{

	struct ComponentMemoryAllocator
	{
		data::IDataInstance** Data;
		type::PropertyID OwningEntity;
		size_t AllocationSize;

		ComponentMemoryAllocator(type::PropertyID& owningEntity, const size_t allocationSize, const std::vector<type::PropertyID>& properties);
		~ComponentMemoryAllocator();

		const data::DataPointer GetProperty(const size_t& location) const;
		void SetProperty(const size_t& location, const data::DataPointer& value);

		void
		operator=(const ComponentMemoryAllocator& other)
		{
			this->Data = other.Data;
			this->OwningEntity = other.OwningEntity;
			this->AllocationSize = other.AllocationSize;
		}
	};

	class ComponentPool
	{
	public:
		ComponentPool(const ComponentVTable& component);

		const bool CreateComponent(type::PropertyID& owningEntity);
		const bool DestroyComponent(const type::PropertyID& owningEntity);

		std::optional<const data::DataPointer> GetComponentProperty(const type::PropertyID& owningEntity, const type::PropertyID& property) const;
		const bool SetComponentProperty(const type::PropertyID& owningEntity, const type::PropertyID& property, const data::DataPointer& value);

		inline const ComponentVTable& GetVTable() const { return m_Component; }

	private:
		std::vector<ComponentMemoryAllocator> m_Pool;
		std::map<type::PropertyID, size_t> m_ComponentOwnerships; // maps entity ids to component locations
		const ComponentVTable& m_Component;
	};



}