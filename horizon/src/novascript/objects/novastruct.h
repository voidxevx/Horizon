#pragma once

#include "../util/types.h"
#include "novafunction.h"

#include <map>

namespace nova::obj
{

    class Structure
    {
    public:
        Structure() = default;

        inline void addProperty(propID propName, ObjectID object, ExposureType exposure) { m_Properties.emplace(propName, std::pair{object, exposure}); }
        inline void 
        addMethod(propID name, FunctionImplementation method) 
        { 
            if (m_Methods.count(name) > 0)
            {
                m_Methods.at(name).addImplementation(method);
            }
            else 
            {
                Function func{};
                func.addImplementation(method);
                m_Methods.emplace(name, func);
            }
        }

        // TODO: match methods
        // TODO: generate allocator

    private:
        std::map<propID, std::pair<ObjectID, ExposureType>> m_Properties;
        std::map<propID, Function> m_Methods;
    };

}