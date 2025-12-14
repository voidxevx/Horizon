#include "VariableScope.h"

namespace neb::data
{

	DataScope::~DataScope()
	{
		for (auto& garbage : m_GarbageData)
		{
			delete garbage;
			garbage = nullptr;
		}

		for (auto& var : m_ScopeVariables)
			delete var.second;

		m_GarbageData.clear();
		m_ScopeVariables.clear();
	}

	void
	DataScope::AddGarbageData(IDataInstance* data)
	{
		m_GarbageData.push_back(data);
	}

	void 
	DataScope::PushVariable(type::PropertyID id, IDataInstance* data)
	{
		if (m_ScopeVariables.count(id) == 0)
			m_ScopeVariables[id] = data;
		else
			AddGarbageData(data);
	}

	std::optional<const DataPointer>
	DataScope::GetVariable(type::PropertyID id)
	const
	{
		if (m_ScopeVariables.count(id) > 0)
			return m_ScopeVariables.at(id)->MakePointer();
		else if (m_LowerScope.has_value())
			return m_LowerScope.value()->GetVariable(id);
		else
			return std::nullopt;
	}

	void
	DataScope::SetVariable(type::PropertyID id, const DataPointer& data)
	{
		if (m_ScopeVariables.count(id) > 0)
			m_ScopeVariables[id]->SetFromPointer(data);
	}







	void
	ScopeStack::PushScope(bool hasDownAccess)
	{
		ScopeStack* next = new ScopeStack(Handle, (hasDownAccess ? this : nullptr));
		Handle = next;
	}

	ScopeStack::~ScopeStack()
	{
		if (CurrentScope)
			delete CurrentScope;
	}

	void
	ScopeStack::PopScope()
	{
		Handle = LowerScope;
		delete CurrentScope;
		CurrentScope = nullptr;
		delete this;
	}

}