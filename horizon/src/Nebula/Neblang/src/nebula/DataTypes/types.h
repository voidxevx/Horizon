#pragma once

#include <xhash>
#include <string>

namespace neb::type
{

	typedef size_t PropertyID;
	static std::hash<std::string> s_PropertyHasher;

}