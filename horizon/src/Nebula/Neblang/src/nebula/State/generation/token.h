#pragma once

#include <optional>
#include <map>
#include <vector>
#include "../../DataTypes/types.h"

namespace neb::gen
{
	enum class TokenType
	{
		/////////////
		// OBJECTS //
		/////////////
		GlobalFunction,  // global
		CustomType,      // type
		Enumeration,     // enum
		ComponentObject, // component
		ArchetypeObject, // arch
		InterfaceObject, // interface
		TraitObject,     // trait
		EntityObject,    // entity
		SystemObject,    // system

		///////////////////////
		// OBJECT PROPERTIES //
		///////////////////////
		ObjectConstructor,     // constructor
		ObjectOperator,        // operator
		ObjectMethod,          // method
		ObjectFunction,        // function
		ObjectPropertyPrivate, // private
		ObjectPropertyPublic,  // public
		SystemDeligate,        // on
		
		//////////
		// MISC //
		//////////
		ExpressionStart, // (
		ExpressionEnd,   // )
		ExpressionBreak,  // ,
		ScopeStart,      // {
		ScopeEnd,        // }
		TypeHint,        // :
		LineEnd,         // ;
		ListStart,       // [
		ListEnd,         // ]

		SystemDeligateNew,           // new
		SystemDeligateDestroy,       // destroy
		SystemDeligateUpdate,        // update
		ComponentDeligateAttachment, // attach
		ComponentDeligateDetachment, // detach

		FunctionReturn, // return
		VoidType,       // void

		ImmediateFloat,
		ImmediateDouble,
		ImmediateInt32,
		ImmediateUint32,
		ImmediateInt64,
		ImmediateUint64,
		ImmediateString,

		Identifier // any non-keyword

	};

	struct Token
	{
		TokenType type;
		std::optional<type::PropertyID> Value;
	};

	struct TokenPackage
	{
		std::map<type::PropertyID, std::string> Constants;
		std::vector<Token> tokens;
	};
}