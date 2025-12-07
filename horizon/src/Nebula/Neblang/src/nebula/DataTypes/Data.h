#pragma once
/*
	Neblang data

	Data Registry Class:
		Only one should be created.
		Globally accessable.
		Contains an instance of every data type available.
		Manages deligation of type casting and method calling.

	Data Type Class:
		Holds essentially the virtual table for the data type.
		Has the implementations for the types methods and operators.

	Data Instance:
		An instance of a data type that stores the actual value of the data.
		also holds a pointer to the data type class that created it.
*/

#include "types.h"

#include <map>
#include <string>
#include <functional>

namespace neb::data
{
	// Data Type interface.
	// inherited by primitive data types and custom data types so they are interoperable
	class IDataType
	{
	public:
		IDataType(PropertyID id);
		virtual ~IDataType() = default;
	private:
		PropertyID m_TypeID;
		// TODO: map method ID to implementations
	};

	// this container lets the values in the DataPointer remain constant
	struct DataPointerConatiner
	{
		DataPointer& ptr;
	};

	struct DataPointer
	{
		const void* const Data;
		const PropertyID Type;

		DataPointer(const void* const data, const PropertyID type)
			: Data(data)
			, Type(type)
		{}
	};

	class IDataInstance
	{
	public:
		IDataInstance(PropertyID id, const IDataType* const& type)
			: m_TypeID(id)
			, m_TypeVTable(type)
		{}
		virtual ~IDataInstance() = default;

		/* Creates a DataPointer that holds a pointer to the data stored by this instance */
		virtual DataPointer MakePointer() const = 0;
		/* Changes the data in this instance to a new value matching the data in the pointer. 
			- this function assumes that the type of the data pointer is the same as the data instance. 
		*/
		virtual void SetFromPointer(const DataPointer&) = 0;

	protected:
		inline const IDataType* const& GetTypeVTable() const { return m_TypeVTable; }

	private:
		const PropertyID m_TypeID;
		const IDataType* const& m_TypeVTable;
	};

	class TypeRegistry
	{
	public:
		TypeRegistry();
		~TypeRegistry();

		/* adds a type to the registry */
		void RegisterType(PropertyID id, IDataType* ptr);

		static std::hash<std::string> s_PropertyHasher;
		static TypeRegistry* Get() { return s_Instance; }
	private:
		std::map<PropertyID, IDataType*> m_DataTypes;
		static TypeRegistry* s_Instance;
	};


}