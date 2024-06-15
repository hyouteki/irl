#ifndef HELPER_HPP_
#define HELPER_HPP_

#define STRINGIFY(x) #x
#define TOSTRING(x) STRINGIFY(x)

#define Func_Loc (std::string(__FILE__) + ":"  + TOSTRING(__LINE__) + \
				  ":0: function: " + __FUNCTION__)

#endif // HELPER_HPP_
