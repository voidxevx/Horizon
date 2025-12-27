#pragma once

#include "../util/types.h"

#include <map>
#include <string>
#include <vector>
#include <optional>

namespace nova::gen
{

    enum class TokenType
    {
        //////////////////
        // OBJECT TYPES //
        //////////////////
        Structure, // struct
        Component, // component
        Interface, // interface
        Archetype, // archetype
        Trait,     // trait
        Entity,    // entity
        System,    // system
        Filter,    // _filter:

        // object inclusion tags
        ArchetypeInclusion, // _archetypes:
        ComponentInclusion, // _components:
        SystemIncludion,    // _systems:

        //////////////////////
        // SUBROUTINE TYPES //
        //////////////////////
        FunctionType,    // function
        MethodType,      // method
        EventType,       // event

        //////////////////////////
        // OBJECT EXPOSURE TYPE //
        //////////////////////////
        ObjectExposurePublic,  // public
        ObjectExposurePrivate, // private

        /////////////
        // SYMBOLS //
        /////////////
        ExpressionStart, // (
        ExpressionEnd,   // )
        ExpressionBreak, // ,
        ScopeStart,      // {
        ScopeEnd,        // }
        ListStart,       // [
        ListEnd,         // ]
        InstanceAccess,  // :
        StaticAccess,    // ::
        ReturnTypeHint,  // ->
        LineEnd,         // ;

        ///////////////
        // OPERATORS //
        ///////////////
        OperatorAssign,        // =
        OperatorAdd,           // +
        OperatorSubtract,      // -
        OperatorMultiply,      // *
        OperatorDivide,        // /
        OperatorCreate,        // create
        OperatorDestroy,       // destroy
        OperatorAttach,        // attach
        OperatorDetach,        // detach
        OperatorEquals,        // ==
        OperatorGreater,       // >
        OperatorLess,          // <
        OperatorGreaterEquals, // >=
        OperatorLessEquals,    // <=
        OperatorAnd,           // &&
        OperatorOr,            // ||
        OperatorNot,           // !
        OperatorLeftShift,     // <<
        OperatorRightShift,    // >>

        ModuleEnd, // _;

        Identifier, // non keyword
    };

    static std::map<std::string, TokenType> s_TokenMap = {
        {"struct", TokenType::Structure},
        {"component", TokenType::Component},
        {"interface", TokenType::Interface},
        {"archetype", TokenType::Archetype},
        {"trait", TokenType::Trait},
        {"entity", TokenType::Entity},
        {"system", TokenType::System},
        {"_filter:", TokenType::Filter},

        {"_archetypes:", TokenType::ArchetypeInclusion},
        {"_components:", TokenType::ComponentInclusion},
        {"_systems:", TokenType::SystemIncludion},

        {"function", TokenType::FunctionType},
        {"method", TokenType::MethodType},
        {"event", TokenType::EventType},

        {"public", TokenType::ObjectExposurePublic},
        {"private", TokenType::ObjectExposurePrivate},

        {"(", TokenType::ExpressionStart},
        {")", TokenType::ExpressionEnd},
        {",", TokenType::ExpressionBreak},
        {"{", TokenType::ScopeStart},
        {"}", TokenType::ScopeEnd},
        {"[", TokenType::ListStart},
        {"]", TokenType::ListEnd},
        {":", TokenType::InstanceAccess},
        {"::", TokenType::StaticAccess},
        {"->", TokenType::ReturnTypeHint},
        {";", TokenType::LineEnd},

        {"=", TokenType::OperatorAssign},
        {"+", TokenType::OperatorAdd},
        {"-", TokenType::OperatorSubtract},
        {"*", TokenType::OperatorMultiply},
        {"/", TokenType::OperatorDivide},
        {"create", TokenType::OperatorCreate},
        {"destroy", TokenType::OperatorDestroy},
        {"attach", TokenType::OperatorAttach},
        {"detach", TokenType::OperatorDetach},
        {"==", TokenType::OperatorEquals},
        {">", TokenType::OperatorGreater},
        {"<", TokenType::OperatorLess},
        {">=", TokenType::OperatorGreaterEquals},
        {"<=", TokenType::OperatorLessEquals},
        {"&&", TokenType::OperatorAnd},
        {"||", TokenType::OperatorOr},
        {"!", TokenType::OperatorNot},
        {"<<", TokenType::OperatorLeftShift},
        {">>", TokenType::OperatorRightShift},

        {"_;", TokenType::ModuleEnd},
    };

    struct Token
    {
        TokenType Type;
        std::optional<propID> Value;
        
        Token(TokenType type)
            : Type(type)
            , Value(std::nullopt)
        {}

        Token(TokenType type, propID value)
            : Type(type)
            , Value(value)
        {}
    };

    struct TokenPackage
    {
        std::map<propID, std::string> Identifiers;
        std::vector<Token> Tokens;
    };

    std::string diagnoseToken(const TokenType token);
}