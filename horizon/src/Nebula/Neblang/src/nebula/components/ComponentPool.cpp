#include "ComponentPool.h"
#include "../DataTypes/types.h"

namespace neb
{

	ComponentMemoryAllocator::ComponentMemoryAllocator(type::PropertyID& owningEntity, const size_t allocationSize, const std::vector<type::PropertyID>& properties)
		: OwningEntity(owningEntity)
		, AllocationSize(allocationSize)
	{
		Data = (data::IDataInstance**)malloc(allocationSize * sizeof(data::IDataInstance*));
		const data::TypeRegistry const* reg = data::TypeRegistry::Get();
		size_t i{};
		for (const auto& prop : properties)
		{
			Data[i] = reg->NullDecl(prop);
			++i;
		}
	}

	ComponentMemoryAllocator::~ComponentMemoryAllocator()
	{
		for (size_t i{}; i < AllocationSize; ++i)
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
	ComponentPool::CreateComponent(type::PropertyID& owningEntity)
	{
		if (m_ComponentOwnerships.count(owningEntity) == 0)
		{
			m_ComponentOwnerships[owningEntity] = m_Pool.size();
			m_Pool.push_back(ComponentMemoryAllocator{ owningEntity, m_Component.GetAllocationSize(), m_Component.GetProperties() });
			return true;
		}
		else
			return false;
	}

	const bool
	ComponentPool::DestroyComponent(const type::PropertyID& owningEntity)
	{
		if (m_ComponentOwnerships.count(owningEntity) > 0)
		{
			/*
			 * Swap the top and select component data locations
			 * then pop the data from the top of the vector removing the data
			 */

			// get top component
			ComponentMemoryAllocator& topAllocator = m_Pool.back();
			const type::PropertyID& topEntity = topAllocator.OwningEntity;

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
	ComponentPool::GetComponentProperty(const type::PropertyID& owningEntity, const type::PropertyID& property)
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
	ComponentPool::SetComponentProperty(const type::PropertyID& owningEntity, const type::PropertyID& property, const data::DataPointer& value)
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