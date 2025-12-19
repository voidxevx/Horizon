#pragma once

#include <xhash>
#include <string>

namespace neb::type
{

	typedef size_t PropertyID;

	struct Property
	{
		PropertyID NameID;
		PropertyID typeID;
	};

	static std::hash<std::string> s_PropertyHasher;

}