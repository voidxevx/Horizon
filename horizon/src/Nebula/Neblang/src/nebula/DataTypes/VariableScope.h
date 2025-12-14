#pragma once

#include "Data.h"

#include <vector>
#include <optional>

namespace neb::data
{

	class DataScope
	{
	public:
		DataScope(DataScope* lowerScope)
			: m_LowerScope(lowerScope)
		{}
		DataScope()
			: m_LowerScope(std::nullopt)
		{}
		~DataScope();

		void AddGarbageData(IDataInstance* data);
		/* - attempting to instantiate a variable with an id that already exists will push the data into garbage collection */
		void PushVariable(type::PropertyID id, IDataInstance* data);
		
		std::optional<const DataPointer> GetVariable(type::PropertyID id) const;
		/* - The passed DataPointer must match the type of the variable */
		void SetVariable(type::PropertyID id, const DataPointer& data);

	private:
		std::vector<IDataInstance*> m_GarbageData; // garbage data is data created during computations that needs be deallocated at some point.
		std::map<type::PropertyID, IDataInstance*> m_ScopeVariables;
		std::optional<DataScope*> m_LowerScope = std::nullopt;
	};

	struct ScopeStack
	{
		DataScope* CurrentScope;
		ScopeStack* LowerScope;
		ScopeStack*& Handle;

		ScopeStack(ScopeStack*& VariableHandle, ScopeStack* lowerScope)
			: LowerScope(lowerScope)
			, Handle(VariableHandle)
		{
			CurrentScope = new DataScope();
		}

		// if the scope gets deleted for unknown reasons it will delete current scope
		~ScopeStack();

		/* adds a new scope object on top of the current one
			- This will change the value of the variable to point to the new scope.
			- Allowing down allows the scope to access variables from lower scopes.
		 */
		void PushScope(bool hasDownAccess);

		/* Removes the top scope deallocating trash values and variables.
			- Updates the Handle to point to the lower scope.
		 */
		void PopScope();

		inline void 
		AddGarbageData(IDataInstance* data) 
		const
		{
			CurrentScope->AddGarbageData(data);
		}

		inline void 
		PushVariable(type::PropertyID id, IDataInstance* data)
		const
		{
			CurrentScope->PushVariable(id, data);
		}
	};

}