#pragma once

#include "../generation/token.h"
#include "../objects/novafunction.h"
#include "../objects/novastruct.h"

#include <map>
#include <variant>
#include <memory>
#include <optional>

namespace nova 
{

    struct ModulePackage
    {
        std::map<propID, obj::Function> m_GlobalFunctions;
        std::map<propID, obj::Structure> m_Structures;
    };

    class State final
    {
        using ModuleState = std::variant<std::string, ModulePackage>;
    public:
        State() = default;

        //////////////////
        // FILE LINKAGE //
        //////////////////

        void linkModule(const std::string& moduleName, const std::string& filePath);
        void loadModule(const std::string& moduleName);
        void loadModule(propID moduleID);


    private:

        /////////////////////////////
        // SOURCE FILE COMPILATION //
        /////////////////////////////
        
        const gen::TokenPackage tokenizeFile(const std::string& filePath);
        ModulePackage buildTokens(const gen::TokenPackage &package, const propID thisModule);

        // OBJECT BUILDING

        std::optional<obj::UncompiledFunctionImplementation> parseFunction(propID& id, const gen::TokenPackage &package, size_t& index, const propID thisModule, ExposureType exposure);
        std::optional<Property> parseProperty(const gen::TokenPackage &package, size_t &index, propID thisModule);
        std::optional<ObjectID> parseObjectID(const gen::TokenPackage&, size_t& index, propID thisModule);
        std::optional<obj::Structure> parseStructure(propID& id, const gen::TokenPackage& package, size_t& index, const propID thisModule, ExposureType exposure);

    private:
        // Loaded modules
        std::map<propID, ModuleState> m_Modules;
        std::map<propID, std::string> m_ModuleNames;
    };

    std::shared_ptr<State> new_novastate();

}