#pragma once

#include "../DataTypes/types.h"

#include <vector>
#include <functional>
#include <utility>

namespace neb::function
{

	struct FunctionTemplate
	{
		std::vector<PropertyID> InputTypes;
		PropertyID OutputType;
	};

	class IFunction
	{
	public:
		virtual ~IFunction() = default;

		inline const PropertyID GetID() const { return m_FunctionID; }

	protected:
		IFunction(PropertyID id)
			: m_FunctionID(id)
		{}

	private:
		PropertyID m_FunctionID;
	};

	class NativeFunction : public IFunction
	{
	public:
		NativeFunction(PropertyID id)
			:IFunction(id)
		{}
		virtual ~NativeFunction() = default;

		void AddImplementation(FunctionTemplate templ, std::function<int()> lambda)
		{
			m_Implementations.push_back(std::make_pair(templ, lambda));
		}

	private:
		std::vector<std::pair<FunctionTemplate, std::function<int()>>> m_Implementations;
	};

	// TODO: LocalFunctions -> need byte nodes first

}