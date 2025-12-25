#pragma once

#include <stdint.h>
#include <string>
#include <functional>

namespace nova
{
    typedef uint32_t propID;

    typedef uint64_t ObjectID;

    inline ObjectID 
    makeObjectID(propID moduleID, propID itemID) 
    {
        return ((uint64_t)moduleID << 32) | (uint64_t)itemID;
    }

    inline propID 
    ObjectID_getModule(ObjectID id)
    {
        return (uint32_t)(id >> 32) & 0xffffffff;
    }

    inline propID
    ObjectID_getItem(ObjectID id)
    {
        return (uint32_t)id & 0xffffffff;
    }



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