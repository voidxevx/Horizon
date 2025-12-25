#pragma once

#include "../util/types.h"
#include "../generation/token.h"

#include <vector>
#include <set>
#include <functional>
#include <variant>

namespace nova::obj
{

    class IFunctionImplementation
    {
    public:
        virtual ~IFunctionImplementation() = default;

        inline void setReturnType(ObjectID type) { m_ReturnType = type; }
        inline ObjectID getReturnType() const { return m_ReturnType; }

        inline void setInputs(std::vector<Property> inputs) { m_Inputs = inputs; }
        inline const std::vector<Property>& getInputs() const { return m_Inputs; }

        inline void setRequiredComponents(std::set<ObjectID> components) { m_RequiredComponents = components; }
        inline const std::set<ObjectID>& getRequiredComponents() const { return m_RequiredComponents; }

        inline void setExposureType(ExposureType type) { m_Exposure = type; }
        inline const ExposureType& getExposureType() const { return m_Exposure; }

    private:
        std::vector<Property> m_Inputs;
        std::set<ObjectID> m_RequiredComponents;
        ObjectID m_ReturnType;
        ExposureType m_Exposure;
    };

    class NativeFunctionImplementation : public IFunctionImplementation 
    {
    public:
        NativeFunctionImplementation(std::vector<Property> inputs, ObjectID retType, std::set<ObjectID> reqComponents, std::function<void()> lambda)
            : m_Function(lambda)
        {
            setReturnType(retType);
            setInputs(inputs);
            setRequiredComponents(reqComponents);
            setExposureType(ExposureType::Public);
        }
        virtual ~NativeFunctionImplementation() = default;

    private:
        std::function<void()> m_Function;
    };

    class UncompiledFunctionImplementation : public IFunctionImplementation
    {
    public:
        UncompiledFunctionImplementation(const gen::TokenPackage& package, size_t tokenStart, size_t tokenEnd)
            : m_Package(package)
            , m_TokenStart(tokenStart)
            , m_TokenEnd(tokenEnd)
        {}


        // TODO: compile into compiled implementation

    private:
        const gen::TokenPackage& m_Package;
        size_t m_TokenStart;
        size_t m_TokenEnd;
    };

    // TODO: compiled function implementation

    typedef std::variant<NativeFunctionImplementation, UncompiledFunctionImplementation /*, CompiledFunctionImplementation*/> FunctionImplementation;

    class Function
    {
    public:
        Function() = default;

        inline void addImplementation(FunctionImplementation impl) { m_Implementation.push_back(impl); }

        // TODO: match implementation function;

    private:
        std::vector<FunctionImplementation> m_Implementation;
    };

}