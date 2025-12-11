#include "ComponentPool.h"

namespace neb::component
{
	ComponentMemoryAllocator::ComponentMemoryAllocator(ComponentPool& owningPool, PropertyID& owningEntity)
		: OwningEntity(owningEntity)
		, OwningPool(owningPool)
	{
		const ComponentVTable& vtable = OwningPool.GetVTable();
		size_t allocationSize = vtable.GetAllocationSize();
		Data = (data::IDataInstance**)malloc(allocationSize * sizeof(data::IDataInstance*));
		const data::TypeRegistry const* reg = data::TypeRegistry::Get();
		size_t i{};
		for (const auto& prop : vtable.GetProperties())
		{
			Data[i] = reg->NullDecl(prop);
			++i;
		}
	}

	ComponentMemoryAllocator::~ComponentMemoryAllocator()
	{
		size_t allocationSize = OwningPool.GetVTable().GetAllocationSize();
		for (size_t i{}; i < allocationSize; ++i)
		{
			delete Data[i];
			Data[i] = nullptr;
		}
		free(Data);
	}

	const data::DataPointer
	ComponentMemoryAllocator::GetProperty(const size_t& location)
	const
	{
		return Data[location]->MakePointer();
	}

	void
	ComponentMemoryAllocator::SetProperty(const size_t& location, const data::DataPointer& value)
	{
		Data[location]->SetFromPointer(value);
	}


	ComponentPool::ComponentPool(const ComponentVTable& component)
		: m_Component(component)
	{}

	const bool 
	ComponentPool::CreateComponent(PropertyID& owningEntity)
	{
		if (m_ComponentOwnerships.count(owningEntity) == 0)
		{
			m_ComponentOwnerships[owningEntity] = m_Pool.size();
			m_Pool.push_back(ComponentMemoryAllocator(*this, owningEntity));
			return true;
		}
		else
			return false;
	}

	const bool
	ComponentPool::DestroyComponent(const PropertyID& owningEntity)
	{
		if (m_ComponentOwnerships.count(owningEntity) > 0)
		{
			/*
			 * Swap the top and select component data locations
			 * then pop the data from the top of the vector removing the data
			 */

			// get top component
			ComponentMemoryAllocator& topAllocator = m_Pool.back();
			const PropertyID& topEntity = topAllocator.OwningEntity;

			// get the removed data
			const size_t trashEntityLocation = m_ComponentOwnerships.at(owningEntity);
			ComponentMemoryAllocator& trashAllocator = m_Pool[trashEntityLocation];

			// swap top and select
			std::swap(m_Pool.back(), m_Pool.at(trashEntityLocation));

			// pop, deleting the data
			m_Pool.pop_back();

			// update ownership locations
			m_ComponentOwnerships[topEntity] = trashEntityLocation;
			m_ComponentOwnerships.erase(owningEntity);

			return true;
		}
		else
			return false;
	}


	std::optional<const data::DataPointer>
	ComponentPool::GetComponentProperty(const PropertyID& owningEntity, const PropertyID& property)
	const
	{
		if (m_ComponentOwnerships.count(owningEntity) > 0 && m_Component.HasProperty(property))
		{
			const ComponentMemoryAllocator& mem = m_Pool[m_ComponentOwnerships.at(owningEntity)];
			return mem.GetProperty(m_Component.GetPropertyLocation(property));
		}
		else 
			return std::nullopt;
	}

	const bool
	ComponentPool::SetComponentProperty(const PropertyID& owningEntity, const PropertyID& property, const data::DataPointer& value)
	{
		if (m_ComponentOwnerships.count(owningEntity) > 0 && m_Component.HasProperty(property))
		{
			ComponentMemoryAllocator& mem = m_Pool[m_ComponentOwnerships.at(owningEntity)];
			mem.SetProperty(m_Component.GetPropertyLocation(property), value);
			return true;
		}

		return false;
	}

}