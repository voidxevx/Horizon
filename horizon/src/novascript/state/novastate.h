#pragma once

#include "../generation/token.h"

#include <map>
#include <variant>
#include <memory>

namespace nova 
{

    struct ModulePackage
    {
        
    };

    class State
    {
        using ModuleState = std::variant<std::string, ModulePackage>;
    public:
        State();

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

        // Loaded modules
        std::map<propID, ModuleState> m_Modules;
    };

    std::shared_ptr<State> new_novastate();

}