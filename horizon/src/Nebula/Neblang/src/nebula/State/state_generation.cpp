#include "state.h"

#include <fstream>
#include <sstream>

#include <iostream>

namespace neb
{

	void 
	State::loadFile(type::PropertyID id)
	{
		if (m_Modules.count(id) > 0)
			if (std::holds_alternative<std::string>(m_Modules.at(id)))
			{ 
				const std::string path = std::get<std::string>(m_Modules.at(id));
				gen::TokenPackage package = tokenizeFile(path);
				ModulePackage modPackage = buildTokens(package);
				m_Modules.at(id) = modPackage;
			}
			else neberror_debug(NEB_ERROR_INFO, "Linked module was already loaded");
		else neberror(NEB_ERROR_WARNING, "Linked module not found");
	}

#define pushToken(t, v) { package.tokens.push_back(gen::Token{t, type::s_PropertyHasher(v)}); package.Constants[type::s_PropertyHasher(v)] = v; }
#define pushTokenT(t) package.tokens.push_back(gen::Token{t, std::nullopt})

	///////////////////
	// TOKEN PARSING //
	///////////////////


	gen::TokenPackage
	State::tokenizeFile(const std::string& filePath)
	{
		gen::TokenPackage package{};

		std::ifstream file{};
		file.open(filePath);
		neberror_assertion(file.is_open(), NEB_ERROR_CRITICAL, "Unable to open source file.");

		std::string buffer{};

		// consume lines until reaching the dependency section
		while (std::getline(file, buffer) && buffer != "_dep:");

		// load dependiancies
		while (std::getline(file, buffer) && buffer != "_code:")
		if (buffer.size() > 1)
		{
			type::PropertyID id = type::s_PropertyHasher(buffer);
			loadFile(id);
		}

		bool exclude = false;
		while (std::getline(file, buffer))
		{
			size_t index = 0;
			std::stringstream tBuffer{};

			bool skipLine = false;
			// iterate over each character in the current line.
			while (index < buffer.size() && !skipLine)
			{
				char c_char{ buffer[index] };

				if (exclude)
				{
					if (c_char == '*' && buffer[index + 1] == '/')
					{
						exclude = false;
						++index;
					}
					++index;
				}
				else if (isspace(c_char))
					++index;

				// Parse immediate numbers
				else if (isdigit(c_char))
				{
					bool floatingPoint = false;
					bool specificType = false;
					bool IsUnsigned = false;
					bool IsExtended = false;

					while (!isspace(c_char) && index < buffer.size())
					{
						if (specificType)
						{
							if (c_char == 'u')
								IsUnsigned = true;
							else if (c_char == 'l')
								IsExtended = true;
						}
						else
						{
							if (c_char == '.')
								floatingPoint = true;
							else if (c_char == ',')
								specificType = true;
							else
								tBuffer << c_char;
						}

						++index;
						c_char = buffer[index];
					}

					const std::string fullNum{ tBuffer.str() };
					tBuffer.str("");

					if (floatingPoint)
						if (IsExtended)
							pushToken(gen::TokenType::ImmediateDouble, fullNum)
						else
							pushToken(gen::TokenType::ImmediateFloat, fullNum)
					else
						if (IsExtended && IsUnsigned)
							pushToken(gen::TokenType::ImmediateUint64, fullNum)
						else if (IsExtended)
							pushToken(gen::TokenType::ImmediateInt64, fullNum)
						else if (IsUnsigned)
							pushToken(gen::TokenType::ImmediateInt32, fullNum)
				}

				// Keywords and identifiers
				else if (isalnum(c_char))
				{
					while (isalnum(c_char) && index < buffer.size())
					{
						tBuffer << c_char;
						++index;
						c_char = buffer[index];
					}

					const std::string str{ tBuffer.str() };
					tBuffer.str("");

					if (str == "global")
						pushTokenT(gen::TokenType::GlobalFunction);
					else if (str == "type")
						pushTokenT(gen::TokenType::CustomType);
					else if (str == "enum")
						pushTokenT(gen::TokenType::Enumeration);
					else if (str == "component")
						pushTokenT(gen::TokenType::ComponentObject);
					else if (str == "arch")
						pushTokenT(gen::TokenType::ArchetypeObject);
					else if (str == "interface")
						pushTokenT(gen::TokenType::InterfaceObject);
					else if (str == "trait")
						pushTokenT(gen::TokenType::TraitObject);
					else if (str == "entity")
						pushTokenT(gen::TokenType::EntityObject);
					else if (str == "system")
						pushTokenT(gen::TokenType::SystemObject);



					else if (str == "constructor")
						pushTokenT(gen::TokenType::ObjectConstructor);
					else if (str == "operator")
						pushTokenT(gen::TokenType::ObjectOperator);
					else if (str == "method")
						pushTokenT(gen::TokenType::ObjectMethod);
					else if (str == "function")
						pushTokenT(gen::TokenType::ObjectFunction);
					else if (str == "private")
						pushTokenT(gen::TokenType::ObjectPropertyPrivate);
					else if (str == "public")
						pushTokenT(gen::TokenType::ObjectPropertyPublic);
					else if (str == "on")
						pushTokenT(gen::TokenType::SystemDeligate);


					else if (str == "new")
						pushTokenT(gen::TokenType::SystemDeligateNew);
					else if (str == "destroy")
						pushTokenT(gen::TokenType::SystemDeligateDestroy);
					else if (str == "update")
						pushTokenT(gen::TokenType::SystemDeligateUpdate);
					else if (str == "attach")
						pushTokenT(gen::TokenType::ComponentDeligateAttachment);
					else if (str == "detach")
						pushTokenT(gen::TokenType::ComponentDeligateDetachment);


					else if (str == "return")
						pushTokenT(gen::TokenType::FunctionReturn);
					else if (str == "void")
						pushTokenT(gen::TokenType::VoidType);
					else
					{
						package.tokens.push_back(gen::Token{ gen::TokenType::Identifier, type::s_PropertyHasher(str) }); 
						package.Constants[type::s_PropertyHasher(str)] = str;
					}
				}

				// Symbols
				else
				{
					if (index + 1 < buffer.size() && !isalnum(buffer[index + 1]))
					{
						tBuffer << c_char << buffer[index + 1];
						const std::string c_2char = tBuffer.str();
						tBuffer.str("");

						if (c_2char == "/*")
						{
							exclude = true;
							index += 2;
							continue;
						}
						else if (c_2char == "//")
							skipLine = true;
						// ... other 2 char key symbols
					}

					if (c_char == '(')
						pushTokenT(gen::TokenType::ExpressionStart);
					else if (c_char == ')')
						pushTokenT(gen::TokenType::ExpressionEnd);
					else if (c_char == ',')
						pushTokenT(gen::TokenType::ExpressionBreak);
					else if (c_char == '{')
						pushTokenT(gen::TokenType::ScopeStart);
					else if (c_char == '}')
						pushTokenT(gen::TokenType::ScopeEnd);
					else if (c_char == ':')
						pushTokenT(gen::TokenType::TypeHint);
					else if (c_char == ';')
						pushTokenT(gen::TokenType::LineEnd);
					else if (c_char == '[')
						pushTokenT(gen::TokenType::ListStart);
					else if (c_char == ']')
						pushTokenT(gen::TokenType::ListEnd);
					else if (c_char == '"')
					{
						++index;
						c_char = buffer[index];
						while (c_char != '"')
						{
							tBuffer << c_char;
							++index;
							c_char = buffer[index];
						}

						pushToken(gen::TokenType::ImmediateString, tBuffer.str());
						tBuffer.str("");
					}

					++index;
				}
				
			}

		}

		file.close();
		return package;
	}






	////////////////////
	// OBJECT PARSING //
	////////////////////
#define push c_token = package.tokens[++index]

	ModulePackage
	State::buildTokens(const gen::TokenPackage& package)
	{
		size_t index{};
		gen::Token c_token = package.tokens[index];

		std::map<type::PropertyID, sys::Function> globalFunctions;

		while (index < package.tokens.size())
		{

			/*
			 * Global Functions
			 * signature:
			 * global <name>([...])(: <output>) {}
			 */
			if (c_token.type == gen::TokenType::GlobalFunction)
			{
				push;
				auto func = buildFunction(index, package, globalFunctions);
				if (func.has_value())
					globalFunctions[func.value().first] = func.value().second;
			}

			else
				neberror(NEB_ERROR_WARNING, "Unexpected token, expected object");

			push;
		}

		return ModulePackage{
			globalFunctions
		};
	}



	const std::optional<std::pair<type::PropertyID, sys::Function>>
	State::buildFunction(size_t& index, const gen::TokenPackage& package, std::map<type::PropertyID, sys::Function>& scope)
	const
	{
		gen::Token c_token = package.tokens[index];
		if (c_token.type == gen::TokenType::Identifier)
		{
			// Get Function id and create uncompiled function if not already.
			type::PropertyID funcID = c_token.Value.value();
			sys::Function func;
			bool alreadyExists = false;
			if (scope.count(funcID) > 0)
			{ 
				func = scope.at(funcID); // this should never fail since the scope shouldn't be compiled before implementations are added.
				alreadyExists = true;
			}
			else
				func = sys::Function{};

			sys::UncompiledFunctionImplementation impl{};

			// Get Function inputs
			push;
			std::vector<type::Property> inputs;
			type::PropertyID returnType;
			if (c_token.type == gen::TokenType::ListStart)
			{
				push;
				while (c_token.type != gen::TokenType::ListEnd)
				{
					if (c_token.type == gen::TokenType::ExpressionBreak)
						push;
					auto input = buildProperty(index, package);
					if (input.has_value())
						inputs.push_back(input.value());
					push;
				}

				push;
			}

			impl.setInputs(inputs);

			// Get output
			returnType = 0;
			if (c_token.type == gen::TokenType::TypeHint)
			{
				push;
				if (c_token.type == gen::TokenType::Identifier)
					returnType = c_token.Value.value();
				else if (c_token.type != gen::TokenType::VoidType)
					neberror(NEB_ERROR_WARNING, "Unexpected token following function or method output value, expected an identifier or void. (defaults to void)");
				push;
			}

			impl.setReturnType(returnType);


			// get uncompiled tokens;
			std::vector<gen::Token> tokens;
			int scopes = 1;
			if (c_token.type == gen::TokenType::ScopeStart)
			{
				push;
				while (index < package.tokens.size())
				{
					if (c_token.type == gen::TokenType::ScopeStart)
						++scopes;
					else if (c_token.type == gen::TokenType::ScopeEnd)
					{
						--scopes;
						if (scopes == 0)
							break;
					}

					tokens.push_back(c_token);
					push;
				}
			}
			else
			{
				neberror(NEB_ERROR_WARNING, "Unexpected token following function or method signature, expected scope");
				return std::nullopt;
			}

			impl.setTokens(tokens);

			// add the implemenation
			func.addImplementation(impl);

			if (!alreadyExists)
				return std::make_pair(funcID, func);
			else
				return std::nullopt;
		}
		else
		{
			neberror(NEB_ERROR_WARNING, "Unexpected token in function or method, expected identifier");
			return std::nullopt;
		}

	}


	const std::optional<type::Property>
	State::buildProperty(size_t& index, const gen::TokenPackage& package)
	const
	{
		if (
			package.tokens[index].type == gen::TokenType::Identifier &&
			package.tokens[index + 1].type == gen::TokenType::TypeHint &&
			package.tokens[index + 2].type == gen::TokenType::Identifier
		)
		{
			const type::PropertyID id = package.tokens[index].Value.value();
			const type::PropertyID type = package.tokens[index + 2].Value.value();
			index += 2;

			return type::Property{ id, type };
		}

		neberror(NEB_ERROR_WARNING, "Error parsing property, expected pattern: <name>: <type>");
		++index;
		return std::nullopt;
	}



#undef push

}