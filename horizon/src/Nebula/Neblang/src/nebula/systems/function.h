#pragma once

#include "../DataTypes/types.h"
#include "../State/generation/token.h"

#include <vector>
#include <functional>
#include <variant>

namespace neb::sys
{
	
	class IFunctionImplementation
	{
	public:
		virtual ~IFunctionImplementation() = default;

		inline void setReturnType(type::PropertyID type) { m_ReturnType = type; }
		inline const type::PropertyID getReturnType() const { return m_ReturnType; }

		inline void setInputs(std::vector<type::Property> inputs) { m_Inputs = inputs; }
		inline const std::vector<type::Property>& getInputs() const { return m_Inputs; }

	private:
		std::vector<type::Property> m_Inputs;
		type::PropertyID m_ReturnType = 0;
	};

	class NativeFunctionImplementation : public IFunctionImplementation
	{
	public:
		NativeFunctionImplementation(std::vector<type::Property> inputs, type::PropertyID retType, std::function<void()> lambda)
			: m_Function(lambda)
		{
			setReturnType(retType);
			setInputs(inputs);
		}
		virtual ~NativeFunctionImplementation() = default;

	private:
		std::function<void()> m_Function;
	};

	class UncompiledFunctionImplementation : public IFunctionImplementation
	{
	public:
		UncompiledFunctionImplementation() = default;
		virtual ~UncompiledFunctionImplementation() = default;

		inline void setTokens(std::vector<gen::Token> tokens) { m_Tokens = tokens; }

	private:
		std::vector<gen::Token> m_Tokens;
	};

	// TODO: compiled implementation

	typedef std::variant<NativeFunctionImplementation, UncompiledFunctionImplementation> functionImpl;

	class Function
	{
	public:
		Function() = default;

		inline void addImplementation(functionImpl impl) { m_Implementations.push_back(impl); };

		// TODO: match implementations

	private:
		std::vector<functionImpl> m_Implementations;
	};

}