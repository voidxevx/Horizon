#pragma once

#include <stdint.h>
#include <string>
#include <functional>

namespace nova
{
    typedef uint64_t propID;

    struct Property
    {
        propID Name;
        propID Type;
    };

    [[maybe_unused]]
    static std::hash<std::string> s_PropertyHasher;
}