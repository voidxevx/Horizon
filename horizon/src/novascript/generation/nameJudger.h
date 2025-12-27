#pragma once

#include "token.h"

namespace nova::judge
{

    void judgeFunctionName(propID id, const gen::TokenPackage &package, bool isMethod, ExposureType exposure);
    void judgePropertyName(propID id, const gen::TokenPackage& package, ExposureType exposure);
    void judgeObjectName(propID id, const gen::TokenPackage &package, ObjectType type, ExposureType exposure);
}