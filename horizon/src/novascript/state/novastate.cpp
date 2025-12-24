#include "novastate.h"

#include <stdio.h>
#include <fstream>
#include <cassert>
#include <sstream>

namespace nova
{

    std::shared_ptr<State>
    new_novastate()
    {
        return std::make_shared<State>(State{});
    }

    void
    State::linkModule(const std::string& moduleName, const std::string& filePath)
    {
        propID id = s_PropertyHasher(moduleName);
        if (m_Modules.count(id) == 0)
            m_Modules[id] = filePath;
    }

    void
    State::loadModule(const std::string& moduleName)
    {
        propID id = s_PropertyHasher(moduleName);
        loadModule(id);
    }

    void
    State::loadModule(propID moduleID)
    {
        if (m_Modules.count(moduleID) > 0)
        {
            if (std::holds_alternative<std::string>(m_Modules.at(moduleID)))
            {
                const std::string path = std::get<std::string>(m_Modules.at(moduleID));
                const gen::TokenPackage package = tokenizeFile(path);
                ModulePackage mPackage = buildTokens(package, moduleID);
                m_Modules[moduleID] = mPackage;
            }
        }
        else
        {
            printf("\033[33m** [NOVA][CPP] Unable to load module, module id not found (Module ID: %llu).\033[0m\n", (unsigned long long)moduleID);
            if (moduleID == s_PropertyHasher("root"))
                printf("\033[31m^^ [NOVA][CPP] Module failed to load was root\033[34m\t<<<\t(make sure that your root.ns file starts with the header _mod: root)\033[0m\n");
        }
    }











    ///////////////////////
    // FILE TOKENIZATION //
    ///////////////////////

    const gen::TokenPackage
    State::tokenizeFile(const std::string& filePath)
    {
        gen::TokenPackage package{};

        std::ifstream file{};
        file.open(filePath);
        assert(file.is_open() && "*** [NOVA][CPP] Unable to open nova source file!");

        std::string buffer{};

        // iterate up to the dependencies list.
        while (std::getline(file, buffer) && buffer != "_dep:");

        // load dependant modules
        while (std::getline(file, buffer) && buffer != "_code:")
        if (buffer.size() > 1)
            loadModule(buffer);

        bool exclude = false;
        while (std::getline(file, buffer))
        {
            size_t index = 0;
            std::stringstream tBuffer{};

            bool skipLine = false;
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

                else if (isdigit(c_char))
                {
                    // TODO: immediate numbers
                    ++index;
                }

                // keyword & identifiers
                else if (isalnum(c_char) || c_char == '_')
                {
                    bool allow_symbols = c_char == '_';
                    while((isalnum(c_char) || (allow_symbols && !isspace(c_char))) && index < buffer.size())
                    {
                        tBuffer << c_char;
                        ++index;
                        c_char = buffer[index];
                    }

                    const std::string str{ tBuffer.str() };
                    tBuffer.str("");

                    if (gen::s_TokenMap.count(str) > 0)
                        package.Tokens.push_back(gen::Token(gen::s_TokenMap[str]));
                    else
                    {
                        propID id = s_PropertyHasher(str);
                        package.Identifiers[id] = str;
                        package.Tokens.push_back(gen::Token(gen::TokenType::Identifier, id));
                    }
                }

                // symbols
                else 
                {
                    if (index + 1 < buffer.size() && !isalnum(buffer[index + 1]))
                    {
                        tBuffer << c_char << buffer[index + 1];
                        const std::string c_2char = tBuffer.str();
                        tBuffer.str("");

                        if (gen::s_TokenMap.count(c_2char) > 0)
                        {
                            package.Tokens.push_back(gen::Token(gen::s_TokenMap[c_2char]));
                            index += 2;
                            continue;
                        }
                        else if (c_2char == "/*")
                        {
                            exclude =  true;
                            index += 2;
                            continue;
                        }
                        else if (c_2char == "//")
                        {
                            skipLine = true;
                            index += 2;
                            continue;
                        }
                    }

                    const std::string as_str = std::string(1, c_char);
                    if (gen::s_TokenMap.count(as_str) > 0)
                        package.Tokens.push_back(gen::Token(gen::s_TokenMap[as_str]));
                    // else if (c_char == '"')

                    ++index;
                }
            }
        }

        file.close();
        return package;
    }










    ////////////////////////
    // STRUCTURE BUILDING //
    ////////////////////////

    #define next c_token = package.Tokens[++index]
    
    ModulePackage
    State::buildTokens(const gen::TokenPackage& package, const propID thisModule)
    {
        ModulePackage mPackage{};
        size_t index{};
        // empty module
        if (package.Tokens.size() == 0) return mPackage;
        gen::Token c_token = package.Tokens[index];
        ExposureType c_exposure = ExposureType::Private;

        while (index < package.Tokens.size())
        {
            c_token = package.Tokens[index];
            if (c_token.Type == gen::TokenType::ObjectExposurePublic)
            {
                c_exposure = ExposureType::Public;
                next;
                continue;
            }
            else if (c_token.Type == gen::TokenType::ObjectExposurePrivate)
            {
                c_exposure = ExposureType::Private;
                next;
                continue;
            }

            else if (c_token.Type == gen::TokenType::FunctionType)
            {
                next; // to function id
                // TODO: parse function implementation
                std::optional<obj::UncompiledFunctionImplementation> impl = parseFunction(package, index, thisModule, c_exposure);
                if (impl.has_value())
                {
                    // TODO: create function if not already created + add the implementation
                }
                else
                    printf("\033[33m^^ [NOVA][CPP] Error while parsing function implementation. (In module: %llu)\033[0m\n", (unsigned long long)thisModule);
            }


            c_exposure = ExposureType::Private;
            ++index;
        }

        return mPackage;
    }

    std::optional<obj::UncompiledFunctionImplementation>
    State::parseFunction(const gen::TokenPackage &package, size_t &index, const propID thisModule, ExposureType exposure)
    {
        gen::Token c_token = package.Tokens[index];
        if (c_token.Value.has_value())
        {
            propID funcID = c_token.Value.value();
            next; // to either input list, component list, return type, implementation, or line end if unimplemented (will generate info warning).

            // input list
            std::vector<Property> inputs;
            if (c_token.Type == gen::TokenType::ExpressionStart)
            {
                next; // to first token of input type.
                while (c_token.Type != gen::TokenType::ExpressionEnd && index < package.Tokens.size())
                {
                    // skip on token if it is a break
                    if (c_token.Type == gen::TokenType::ExpressionBreak) next;

                    std::optional<Property> prop = parseProperty(package, index, thisModule);
                    if (prop.has_value())
                        inputs.push_back(prop.value());
                    else 
                    {  
                        printf("\033[33m^^ [NOVA][CPP] Error while parsing property for function input. (function: %s)\033[0m\n", package.Identifiers.at(funcID).c_str());
                        return std::nullopt;
                    }
                    next;
                }
                next; // to components list, return type, implementation, or line end.
            }

            // components list
            std::vector<ObjectID> components;
            if (c_token.Type == gen::TokenType::ListStart)
            {
                next; // to first token of component object
                while (c_token.Type != gen::TokenType::ListEnd && index < package.Tokens.size())
                {
                    // skip on token if it is a break
                    if (c_token.Type == gen::TokenType::ExpressionBreak) next;

                    std::optional<ObjectID> objID = parseObjectID(package, index, thisModule);
                    if (objID.has_value())
                        components.push_back(objID.value());
                    else
                    {
                        printf("\033[33m^^ [NOVA][CPP] Error while parsing object for function component requirements. (function: %s)\033[0m\n", package.Identifiers.at(funcID).c_str());
                        return std::nullopt;
                    }
                    next;
                }
                next;
            }

            return std::nullopt;
        }
        else 
        {
            printf("\033[33m** [NOVA][CPP] Unexpected token found in function signature. found token: %s\n", gen::diagnoseToken(c_token.Type).c_str());
            printf("\033[34m^^ [NOVA][CPP] Expected signature patern:\n>>\t\t\t%s function <identifier>(inputs...)[components...] {...}\033[0m\n", (exposure == ExposureType::Public ? "public" : "private"));
            return std::nullopt;
        }
        
    }

    std::optional<Property>
    State::parseProperty(const gen::TokenPackage &package, size_t &index, propID thisModule)
    {
        gen::Token c_token = package.Tokens[index];
        if (c_token.Value.has_value() && c_token.Type == gen::TokenType::Identifier)
        {
            gen::Token c_token = package.Tokens[index];
            std::optional<ObjectID> obj = parseObjectID(package, index, thisModule);
            if (!obj.has_value())
            {
                printf("\033[33m^^ [NOVA][CPP] Error while parsing object identifier in property.\033[0m\n");
                return std::nullopt;
            }

            next; // to name
            propID propName;
            if (c_token.Value.has_value() && c_token.Type == gen::TokenType::Identifier)
            {
                propName = c_token.Value.value();
                Property prop{propName, obj.value()};
                return prop;
            }
        }

        printf("\033[33m** [NOVA][CPP] Error while parsing signature\n");
        printf("\033[34m^^ [NOVA][CPP] Expected signatures:\n>>\t\t\t<module>::<object> <name> or <object> <name>\033[0m\n");
        return std::nullopt;
    }

    std::optional<ObjectID>
    State::parseObjectID(const gen::TokenPackage &package, size_t &index, propID thisModule)
    {
        gen::Token c_token = package.Tokens[index];
        if (c_token.Value.has_value() && c_token.Type == gen::TokenType::Identifier)
        {
            propID moduleID = thisModule;
            propID objectID = c_token.Value.value();
            if (package.Tokens[index + 1].Type == gen::TokenType::StaticAccess)
            {
                moduleID = objectID;
                next; // to static access
                next; // to object id
                if (c_token.Value.has_value() && c_token.Type == gen::TokenType::Identifier)
                    objectID = c_token.Value.value();
                else goto _parseObjectIDError;
            }

            return ObjectID{ objectID, moduleID };
        }

        _parseObjectIDError:
        printf("\033[33m** [NOVA][CPP] Error while parsing object identifier\n");
        printf("\033[34m^^ [NOVA][CPP] Expected signature:\n>>\t\t\t<module>::<object> or <object>\033[0m\n");
        return std::nullopt;
    }
}