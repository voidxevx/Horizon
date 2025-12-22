#pragma once

#include <stdint.h>
#include <xhash>
#include <string>

namespace nova
{
    typedef uint64_t propID;

    struct Property
    {
        propID Name;
        propID Type;
    };

    static std::hash<std::string> s_PropertyHasher;
}