#include "nameJudger.h"

#include <algorithm>
#include <cctype>

namespace nova::judge
{

    void
    judgeFunctionName(propID id, const gen::TokenPackage &package, bool isMethod, ExposureType exposure)
    {
        const std::string name = package.Identifiers.at(id);
        if (isMethod)
        {
            switch (exposure)
            {
            // public methods should be cammel case and shouldn't have symbols
            case ExposureType::Public: {
                if (!(name.at(0) != '_' && name.at(0) == std::tolower(name.at(0))))
                    printf("\033[34m* [NOVA][CPP] Public method names shouldn't contain symbols and should be lower cammel case. (Name: %s)\033[0m\n", name.c_str());
                break;
            }
            // private methods should be fully lower case and shouldnt include symbols
            case ExposureType::Private: {
                std::string name_lower = name;
                std::transform(name_lower.begin(), name_lower.end(), name_lower.begin(),
                    [](unsigned char c){ return std::tolower(c); });

                if (!(name.at(0) != '_' && name == name_lower) && name != "_alloc:")
                    printf("\033[34m* [NOVA][CPP] Private method names shouldn't contain symbols and should be fully lowercase. (Name: %s)\033[0m\n", name.c_str());
                break;
            }
            }
        }
        else
        {
            // public functions should upper cammel case and shouldn't contain symbols
            if (exposure == ExposureType::Public && !(name.at(0) != '_' && name.at(0) == std::toupper(name.at(0))))
                printf("\033[34m* [NOVA][CPP] Public functions shouldn't contain symbols and should be upper cammel case. (Name: %s)\033[0m\n", name.c_str());
        }
    }

    void 
    judgePropertyName(propID id, const gen::TokenPackage &package, ExposureType exposure)
    {
        const std::string name = package.Identifiers.at(id);
        switch (exposure)
        {
        // private properties should be prefixed with _$
        case ExposureType::Private: {
            if (!(name.at(0) == '_' && name.at(1) == '$'))
                printf("\033[34m* [NOVA][CPP] Private properties should be prefixed with _$ (Name: %s)\033[0m\n", name.c_str());
            break;
        }
        // public properties should be cammel case and not include symbols
        case ExposureType::Public: {
            if (!(name.at(0) != '_' && name.at(0) == std::tolower(name.at(0))))
                printf("\033[34m* [NOVA][CPP] Public properties shouldn't include symbols and should be lower cammel case. (Name: %s)\033[0m\n", name.c_str());
            break;
        }
        }
    }

    void
    judgeObjectName(propID id, const gen::TokenPackage &package, ObjectType type, ExposureType exposure)
    {
        const std::string name = package.Identifiers.at(id);
        char prefix = '_';
        switch (type)
        {
            case ObjectType::Archetype: prefix = 'A'; break;
            case ObjectType::Component: prefix = 'C'; break;
            case ObjectType::Entity: prefix = 'E'; break;
            case ObjectType::Interface: prefix = 'I'; break;
            case ObjectType::System: prefix = 'S'; break;
            case ObjectType::Trait: prefix = 'T'; break;

            case ObjectType::Structure: prefix = 'S'; break;
            case ObjectType::Filter: prefix = 'F'; break;
        }
        switch (exposure)
        {
        // private object should be prefixed with _L: 
        case ExposureType::Private: {
            if (!(name.at(0) == '_' && name.at(1) == prefix && name.at(2) == ':'))
                printf("\033[34m* [NOVA][CPP] Private objects should be prefixed with: _%c: (Name: %s)\033[0m\n", prefix, name.c_str());
            break;
        }
        // public sctuctures shouldn't contain symbols, should be upper cammel case and be prefixed
        case ExposureType::Public: {
            if (type == ObjectType::Structure || type == ObjectType::Filter)
            {
                if (!(name.at(0) != '_' && name.at(0) == std::toupper(name.at(0))))
                    printf("\033[34m* [NOVA][CPP] Public %s shouldn't contain symbols and should be upper cammel case. (Name: %s)\033[0m\n", (type == ObjectType::Structure ? "structures" : "filters"), name.c_str());
                break;
            }
            if (!(name.at(0) != '_' && name.at(0) == prefix && name.at(1) == std::toupper(name.at(1))))
                printf("\033[34m* [NOVA][CPP] Public objects shouldn't contain symbols, should be upper cammel case, and should be prefixed. (prefix: %c) (Name: %s)\033[0m\n", prefix, name.c_str());
            break;
        }
        }
    }
}