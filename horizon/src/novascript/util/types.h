#pragma once

#include <stdint.h>
#include <string>
#include <functional>

namespace nova
{
    typedef uint64_t propID;

    struct ObjectID 
    {
        propID Name;
        propID Module;

        inline bool operator==(const ObjectID& other) { return this->Name == other.Name && this->Module == other.Module; }
    };

    struct Property
    {
        propID Name;
        ObjectID Type;
    };

    [[maybe_unused]]
    static std::hash<std::string> s_PropertyHasher;

    enum class ExposureType
    {
        Public,
        Private,
    };

    enum class ObjectType
    {
        Structure,
        Component,
        Interface,
        Archetype,
        Entity,
        System,
        Trait,
        Filter,
    };
}