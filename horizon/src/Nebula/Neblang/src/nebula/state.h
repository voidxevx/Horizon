#pragma once

namespace neb
{

	class State
	{
	public:
		State();
		~State();

		static State* Get() { return s_Instance; }
	private:
		static State* s_Instance;
	};

	extern "C"
	{
		void neb_init();
	}

}