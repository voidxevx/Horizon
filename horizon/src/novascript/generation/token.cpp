#include "token.h"

namespace nova::gen
{
    std::string
    diagnoseToken(const TokenType token)
    {
        switch (token)
        {
        case TokenType::Structure:
            return "struct";
        case TokenType::Component:
            return "component";
        case TokenType::Interface:
            return "interface";
        case TokenType::Archetype:
            return "archetype";
        case TokenType::Trait:
            return "trait";
        case TokenType::Entity:
            return "entity";
        case TokenType::System:
            return "system";
        case TokenType::Filter:
            return "filter tag";
        case TokenType::ArchetypeInclusion:
            return "archetypes tag";
        case TokenType::ComponentInclusion:
            return "components tag";
        case TokenType::SystemIncludion:
            return "systems tag";
        case TokenType::FunctionType:
            return "function";
        case TokenType::MethodType:
            return "method";
        case TokenType::EventType:
            return "event";
        case TokenType::ObjectExposurePublic:
            return "public";
        case TokenType::ObjectExposurePrivate:
            return "private";
        case TokenType::ExpressionStart:
            return "expression start (";
        case TokenType::ExpressionEnd:
            return "expression end )";
        case TokenType::ExpressionBreak:
            return "expression break ,";
        case TokenType::ScopeStart:
            return "scope start {";
        case TokenType::ScopeEnd:
            return "scope end }";
        case TokenType::ListStart:
            return "list start [";
        case TokenType::ListEnd:
            return "list end ]";
        case TokenType::InstanceAccess:
            return "instance access :";
        case TokenType::StaticAccess:
            return "static access ::";
        case TokenType::ReturnTypeHint:
            return "type hint ->";
        case TokenType::LineEnd:
            return "line end";
        case TokenType::OperatorAssign:
            return "assignment operator";
        case TokenType::OperatorAdd:
            return "addition operator";
        case TokenType::OperatorSubtract:
            return "subtraction operator";
        case TokenType::OperatorMultiply:
            return "multiply operator";
        case TokenType::OperatorDivide:
            return "division operator";
        case TokenType::OperatorCreate:
            return "create";
        case TokenType::OperatorDestroy:
            return "destroy";
        case TokenType::OperatorAttach:
            return "attach";
        case TokenType::OperatorDetach:
            return "detach";
        case TokenType::OperatorEquals:
            return "equals operator";
        case TokenType::OperatorGreater:
            return "greater than operator";
        case TokenType::OperatorLess:
            return "less than operator";
        case TokenType::OperatorGreaterEquals:
            return "greater equals operator";
        case TokenType::OperatorLessEquals:
            return "less equals operator";
        case TokenType::OperatorAnd:
            return "and operator";
        case TokenType::OperatorOr:
            return "or operator";
        case TokenType::OperatorNot:
            return "not operator";
        case TokenType::OperatorLeftShift:
            return "left shift operator";
        case TokenType::OperatorRightShift:
            return "right shift operator";
        case TokenType::Identifier:
            return "identifier";
        case TokenType::ModuleEnd:
            return "module end";
        default:
            break;
        }
        return "unknown token";
    }
}