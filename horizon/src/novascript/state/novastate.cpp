#include "novastate.h"

#include "../generation/nameJudger.h"

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
        {
            m_Modules[id] = filePath;
            m_ModuleNames[id] = moduleName;
        }
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
            printf("\033[33m** [NOVA][CPP] Unable to load module, module id not found (Module: %s).\033[0m\n", m_ModuleNames[moduleID].c_str());
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

                    while ((isalnum(c_char) || c_char == '_' || (allow_symbols && !isspace(c_char))) && index < buffer.size())
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
                propID funcID {};
                std::optional<obj::UncompiledFunctionImplementation> impl = parseFunction(funcID, package, index, thisModule, c_exposure);
                if (impl.has_value())
                {
                    judge::judgeFunctionName(funcID, package, false, c_exposure);
                    if (mPackage.m_GlobalFunctions.count(funcID) > 0)
                        mPackage.m_GlobalFunctions[funcID].addImplementation(impl.value());
                    else
                    {
                        obj::Function func = obj::Function{};
                        func.addImplementation(impl.value());
                        mPackage.m_GlobalFunctions.emplace(funcID, func);
                    }
                    c_exposure = ExposureType::Private;
                    continue;
                }
                else
                    printf("\033[33m^^ [NOVA][CPP] Error while parsing function implementation. (module: %s)\033[0m\n", m_ModuleNames[thisModule].c_str());
            }

            else if (c_token.Type == gen::TokenType::Structure)
            {
                next;
                propID structID{};
                std::optional<obj::Structure> stru = parseStructure(structID, package, index, thisModule, c_exposure);
                if (stru.has_value())
                {
                    if (mPackage.m_Structures.count(structID) > 0)
                        printf("\033[33m** [NOVA][CPP] Defined structure already exists in this module. (struct: %s, module: %s)\033[0m\n", package.Identifiers.at(structID).c_str(), package.Identifiers.at(thisModule).c_str());
                    else
                        mPackage.m_Structures.emplace(structID, stru.value());
                }
                else
                    printf("\033[33m^^ [NOVA][CPP] Error while parsing structure. (module: %s)\033[0m\n", m_ModuleNames[thisModule].c_str());
            }


            c_exposure = ExposureType::Private;
            ++index;
        }

        if (package.Tokens.back().Type != gen::TokenType::ModuleEnd)
            printf("\033[33m** [NOVA][CPP] Module %s does not end with module terminator (_;) module should end with a terminator to avoid issues during parsing.\033[0m\n", m_ModuleNames[thisModule].c_str());

        return mPackage;
    }

    std::optional<obj::UncompiledFunctionImplementation>
    State::parseFunction(propID& id, const gen::TokenPackage &package, size_t &index, const propID thisModule, ExposureType exposure)
    {
        gen::Token c_token = package.Tokens[index];
        if (c_token.Value.has_value() && c_token.Type == gen::TokenType::Identifier)
        {
            propID funcID = c_token.Value.value();
            id = funcID;
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
            std::set<ObjectID> components;
            if (c_token.Type == gen::TokenType::ListStart)
            {
                next; // to first token of component object
                while (c_token.Type != gen::TokenType::ListEnd && index < package.Tokens.size())
                {
                    // skip on token if it is a break
                    if (c_token.Type == gen::TokenType::ExpressionBreak) next;

                    std::optional<ObjectID> objID = parseObjectID(package, index, thisModule);
                    if (objID.has_value())
                        components.insert(objID.value());
                    else
                    {
                        printf("\033[33m^^ [NOVA][CPP] Error while parsing object for function component requirements. (function: %s)\033[0m\n", package.Identifiers.at(funcID).c_str());
                        return std::nullopt;
                    }
                    next;
                }
                next; // to return type, implementation, or endline.
            }

            ObjectID returnType = 0;
            if (c_token.Type == gen::TokenType::ReturnTypeHint)
            {
                next; // to type
                if (c_token.Type == gen::TokenType::Identifier && c_token.Value.has_value())
                {
                    std::optional<ObjectID> obj = parseObjectID(package, index, thisModule);
                    if (obj.has_value())
                    {
                        returnType = obj.value();
                    }
                    else
                    {
                        printf("\033[33m^^ [NOVA][CPP] Error while parsing object for function return type. (function: %s)\033[0m\n", package.Identifiers.at(funcID).c_str());
                    }
                }
                else
                {
                    printf("\033[33m** [NOVA][CPP] Unexpected token following function type hint. (Found token: %s)\n", gen::diagnoseToken(c_token.Type).c_str());
                    printf("\033[34m^^ Expected signature pattern:\n>>>\t\t\t%s function %s(...)[...] -> <type or void> {...}\033[0m\n", (exposure == ExposureType::Public ? "public" : "private"), package.Identifiers.at(funcID).c_str());
                    return std::nullopt;
                }
                next; // to implementation or endline
            }

            if (c_token.Type == gen::TokenType::LineEnd)
            {
                printf("\033[34m* [NOVA][CPP] TODO: Unimplemented function: %s (module: %s)\033[0m\n", package.Identifiers.at(funcID).c_str(), m_ModuleNames[thisModule].c_str());
                obj::UncompiledFunctionImplementation impl = obj::UncompiledFunctionImplementation(package, 0, 0);
                impl.setExposureType(exposure);
                impl.setInputs(inputs);
                impl.setRequiredComponents(components);
                impl.setReturnType(returnType);
                next; // push to next token
                return impl;
            }
            else if (c_token.Type == gen::TokenType::ScopeStart)
            {
                next; // to first token
                size_t start = index;
                int scopes = 1;
                while (scopes != 0 && index < package.Tokens.size())
                {
                    if (c_token.Type == gen::TokenType::ScopeStart)
                        ++scopes;
                    else if (c_token.Type == gen::TokenType::ScopeEnd)
                        --scopes;
                    next; // push to next token after the scope end.
                }
                size_t end = index - 2; // subtract the current token and the scope end.

                obj::UncompiledFunctionImplementation impl = obj::UncompiledFunctionImplementation(package, start, end);
                impl.setExposureType(exposure);
                impl.setInputs(inputs);
                impl.setRequiredComponents(components);
                impl.setReturnType(returnType);
                return impl;
            }
            else 
            {
                printf("\033[33m** [NOVA][CPP] Unexpected token following function signature. Expected scope or line end found: %s. (function: %s)\033[0m\n", gen::diagnoseToken(c_token.Type).c_str(), package.Identifiers.at(funcID).c_str());
                return std::nullopt;
            }

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

        printf("\033[33m** [NOVA][CPP] Error while parsing property signature\n");
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
            else if (package.Tokens[index + 1].Type == gen::TokenType::InstanceAccess)
                printf("\033[34m* [NOVA][CPP] A instance access (:) was found where a static access (::) could be, an error will likely follow.\033[0m\n");

                return makeObjectID(moduleID, objectID);
        }

        _parseObjectIDError:
        printf("\033[33m** [NOVA][CPP] Error while parsing object identifier\n");
        printf("\033[34m^^ [NOVA][CPP] Expected signature:\n>>\t\t\t<module>::<object> or <object>\033[0m\n");
        return std::nullopt;
    }

    std::optional<obj::Structure>
    State::parseStructure(propID &id, const gen::TokenPackage &package, size_t &index, const propID thisModule, ExposureType exposure)
    {
        gen::Token c_token = package.Tokens[index];
        if (!(c_token.Value.has_value() || c_token.Type == gen::TokenType::Identifier))
        {
            printf("\033[33m** [NOVA][CPP] Unexpected token found in structure header. found token: %s\n", gen::diagnoseToken(c_token.Type).c_str());
            printf("\033[34m^^ [NOVA][CPP] Expected header pattern:\n>>>\t\t\t%s struct <identifier> {...}\033[0m\n", (exposure == ExposureType::Public ? "public" : "private"));
            return std::nullopt;
        }

        propID structID = c_token.Value.value();
        judge::judgeObjectName(structID, package, ObjectType::Structure, exposure);
        id = structID;
        next;

        // unimplemented warning
        if (c_token.Type == gen::TokenType::LineEnd)
        {
            printf("\033[34m* [NOVA][CPP] TODO: Unimplemented structure: %s (module: %s)\033[0m\n", package.Identifiers.at(structID).c_str(), m_ModuleNames[thisModule].c_str());
            return obj::Structure{};
        }
        else if (c_token.Type == gen::TokenType::ScopeStart)
        {
            obj::Structure stru{};
            next;
            ExposureType c_exposure = ExposureType::Private;
            bool hasConstructor = false;
            unsigned int propCount{};

            while (c_token.Type != gen::TokenType::ScopeEnd && index < package.Tokens.size())
            {
                if (c_token.Type == gen::TokenType::ObjectExposurePrivate)
                {
                    c_exposure = ExposureType::Private;
                    next;
                    continue;
                }
                else if (c_token.Type == gen::TokenType::ObjectExposurePublic)
                {
                    c_exposure = ExposureType::Public;
                    next;
                    continue;
                }

                // properties
                else if (c_token.Type == gen::TokenType::Identifier)
                {
                    std::optional<Property> prop = parseProperty(package, index, thisModule);
                    if (prop.has_value())
                    {
                        Property prop_v = prop.value();
                        judge::judgePropertyName(prop_v.Name, package, c_exposure);
                        stru.addProperty(prop_v.Name, prop_v.Type, c_exposure);
                        ++propCount;
                    }
                    else 
                    {
                        printf("\033[33m^^ [NOVA][CPP] Error while parsing property for structure: %s\033[0m\n", package.Identifiers.at(structID).c_str());
                        return std::nullopt;
                    }
                }

                else if (c_token.Type == gen::TokenType::MethodType)
                {
                    next; // to identifier
                    propID methodID{};
                    std::optional<obj::UncompiledFunctionImplementation> method = parseFunction(methodID, package, index, thisModule, c_exposure);
                    if (method.has_value())
                    {
                        judge::judgeFunctionName(methodID, package, true, c_exposure);
                        if (method.value().getRequiredComponents().size() > 0)
                            printf("\033[34m* [NOVA][CPP] Entities cannot be passed into methods. The required components list will be ignored. (method: %s, struct: %s)\033[0m\n", package.Identifiers.at(methodID).c_str(), package.Identifiers.at(structID).c_str());
                        stru.addMethod(methodID, method.value());
                        c_token = package.Tokens[index];
                        if (!hasConstructor && methodID == (propID)s_PropertyHasher("_alloc:") && c_exposure == ExposureType::Private)
                            hasConstructor = true;
                        continue;
                    }
                    else
                    {
                        printf("\033[33m^^ [NOVA][CPP] Error while parsing method for structure: %s\033[0m\n", package.Identifiers.at(structID).c_str());
                        return std::nullopt;
                    }
                }

                else 
                {
                    printf("\033[33m** [NOVA][CPP] Unexpected object found in structure body. (structure: %s) Token found: %s\033[0m\n", package.Identifiers.at(structID).c_str(), gen::diagnoseToken(c_token.Type).c_str());
                    if (c_token.Type == gen::TokenType::FunctionType)
                        printf("\033[34m^^ [NOVA][CPP] structures cannot implement functions try replacing function with method.\033[0m\n");
                }
                
                c_exposure = ExposureType::Private;
                next;
            }

            if (!hasConstructor)
                printf("\033[34m* [NOVA][CPP] Structure does not implement any constructor. Add a private method called _alloc: to define the structure. (structure: %s)\033[0m\n", package.Identifiers.at(structID).c_str());

            if (propCount == 0)
                printf("\033[34m* [NOVA][CPP] Structure contains no properties. (structure: %s)\033[0m\n", package.Identifiers.at(structID).c_str());

            return stru;
        }
        else 
        {
            printf("\033[33m** [NOVA][CPP] Unexpected token following structure header. Expected Scope start or line end. found token: %s (structure: %s)\033[0m\n", gen::diagnoseToken(c_token.Type).c_str(), package.Identifiers.at(structID).c_str());
            return std::nullopt;
        }

    }


}