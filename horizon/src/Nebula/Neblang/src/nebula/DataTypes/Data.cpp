#include "Data.h"

namespace neb::data
{

	TypeRegistry* TypeRegistry::s_Instance = nullptr;
	std::hash<std::string> TypeRegistry::s_PropertyHasher = std::hash<std::string>();

	TypeRegistry::TypeRegistry()
	{
		s_Instance = this;
	}

	TypeRegistry::~TypeRegistry()
	{
		for (auto& type : m_DataTypes)
			delete type.second;
		m_DataTypes.clear();
	}

	void
	TypeRegistry::RegisterType(PropertyID id, IDataType* ptr)
	{
		if (m_DataTypes.count(id) > 0)
			m_DataTypes[id];
		m_DataTypes[id] = ptr;
	}

	IDataType::IDataType(PropertyID id)
		: m_TypeID(id)
	{
		TypeRegistry::Get()->RegisterType(id, this);
	}

}