#pragma once

#include "Component.h"
#include "../DataTypes/Data.h"

#include <memory>

namespace neb::component
{


	class ComponentPool
	{
	public:
		ComponentPool(const ComponentVTable& component);

		const bool CreateComponent(PropertyID& owningEntity);
		const bool DestroyComponent(const PropertyID& owningEntity);

		std::optional<const data::DataPointer> GetComponentProperty(const PropertyID& owningEntity, const PropertyID& property) const;
		const bool SetComponentProperty(const PropertyID& owningEntity, const PropertyID& property, const data::DataPointer& value);

		inline const ComponentVTable& GetVTable() const { return m_Component; }

	private:
		std::vector<ComponentMemoryAllocator> m_Pool;
		std::map<PropertyID, size_t> m_ComponentOwnerships; // maps entity ids to component locations
		const ComponentVTable& m_Component;
	};

	struct ComponentMemoryAllocator
	{
		data::IDataInstance** Data;
		ComponentPool& OwningPool; // this doesn't need to be changed when setting as it should never be changed to a component outside of the pool
		PropertyID OwningEntity;

		ComponentMemoryAllocator(ComponentPool& owningPool, PropertyID& owningEntity);
		~ComponentMemoryAllocator();

		const data::DataPointer GetProperty(const size_t& location) const;
		void SetProperty(const size_t& location, const data::DataPointer& value);

		void
		operator=(const ComponentMemoryAllocator& other)
		{
			this->Data = other.Data;
			this->OwningEntity = other.OwningEntity;
		}
	};

}