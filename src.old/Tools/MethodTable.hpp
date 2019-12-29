/*! ****************************************************************************
\file             MethodTable.hpp
\author           Chyler Morrison
\par    Email:    contact\@chyler.info
\par    Project:  Audio Engine

\copyright        Copyright © 2019 Chyler Morrison
*******************************************************************************/

#ifndef __OCAE_METHODTABLE_HPP
#define __OCAE_METHODTABLE_HPP

// Include Files                ////////////////////////////////////////////////

#include <string>
#include <tuple>
#include <type_traits>
#include <unordered_map>
#include <utility>
#include <vector>

#include "../Engine.hpp"

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

// Public Enums                 ////////////////////////////////////////////////

// Public Objects               ////////////////////////////////////////////////

namespace OCAE
{
namespace Tools
{
	/*! ************************************************************************
	\brief
		The purpose of this class is to create a simple interface for calling
		methods from an object of an unknown type.

		For example, within OCAE you have a Sine object currently represented by
		a GeneratorBasePtr object. To call the Sine method to set the frequency
		you would utilize this class in the following manner:

	\details

		\code{.cpp}
			GeneratorBasePtr obj = CreateSine(440);

			Math_t new_freq = 880;
			obj->CallMethod("SetFrequency", OCAE_METHOD_PARAM(new_freq));
			obj->CallMethod("GetFrequency", OCAE_METHOD_RET(new_freq));
		\endcode

		Here, the `OCAE_METHOD_RET()` and `OCAE_METHOD_PARAM()` macros ensure that the
		values passed to the function will have the proper types, guaranteeing
		they are handled properly. See the macros' documentation and definition
		in Macro.hpp for more info.

		It is recommended to construct the method table with the default
		constructor, and then set the methods for the class in a fashion like:

		\code{.cpp}
			Foo:Foo() : MethodTable(), // ...
			{
				RegisterMethods(CreateMethodList());

					// or

				RegisterMethod("method1", [this](void *){ method1(); });
				RegisterMethod("method2", [this](void * p){
					method2(
						std::get<0>(
							*reinterpret_cast<OCAE_METHOD_PARAM_T(int)>(p)
						)
					);
				});
				// ...
			}

			Tools::MethodTable::MethodList_t Foo::CreateMethodList()
			{
					// Returns initializer list that constructs a MethodList_t
				return {
					std::make_tuple(
						std::string("method1"),
						Tools::MethodTable::Void_fn(
							[this](void *){ method1(); }
						)
					),
					std::make_tuple(
						std::string("method2"),
						Tools::MethodTable::Void_fn(
							[this](void *){
								method2(
									std::get<0>(
										*reinterpret_cast<OCAE_METHOD_PARAM_T(int)>(p)
									)
								);
							}
						)
					),
					// ...
				};
			}
		\endcode

		Here, `OCAE_METHOD_PARAM_T()` is a macro that helps ensure that the type
		being casted to is in the correct format.

		The user creating the derived classes will need to ensure that it
		properly registers all the methods they want to be accessible through
		this class in the constructors of the derived classes, including
		grandchildren classes.
	***************************************************************************/
	class MethodTable
	{
	public:

			/// Alias for a void-returning function that takes a void pointer
		using Void_fn = std::function<void(void*)>;
			/// Alias for the mapping of method names to the method
		using MethodTable_t = std::unordered_map<MethodTable *, std::unordered_map<std::string, Void_fn>>;
			/// Alias for the list of method names and their associated methods
		using MethodList_t = std::vector<std::tuple<std::string, Void_fn>>;

	protected:

		// Members              ///////////////////////

			/// Object mapping a string to a function
		static MethodTable_t s_Table;

	public:

		// Con-/De- structors   ///////////////////////

		/*! ********************************************************************
		\brief
			Default constructor.
		***********************************************************************/
		MethodTable();

		/*! ********************************************************************
		\brief
			Consturctor.

		\param list
			List of tuples for mapping a string to a function to initialize the
			internal method table.
		***********************************************************************/
		MethodTable(MethodList_t const & list);

		/*! ********************************************************************
		\brief
			Default destructor.
		***********************************************************************/
		virtual ~MethodTable() = default;

		// Operators            ///////////////////////

		/*! ********************************************************************
		\brief
			Calls a method.

			If the provided function name does not exist within the map an
			exception will be thrown by std::unordered_map and the user will
			need to handle it if desired.

			If the method is to return a value, the first parameter must be a
			reference to a variable that will store the returned value.

		\tparam Args
			The arguments' types of the given method.

		\param fn
			The name of the method. If a function matching this name is
			registered with the table, an exception will be thrown by
			std::unordered_map and the user will need to handle it if desired.

		\param args
			The parameters for the method.
		***********************************************************************/
		template<typename... Args>
		void CallMethod(std::string const & fn, Args&&... args)
		{
			using tuple = std::tuple<Args...>;
			tuple params(std::forward<Args>(args)...);
			s_Table.at(this).at(fn)(reinterpret_cast<void*>(&params));
		}

		// Accossors/Mutators   ///////////////////////

		// Functions            ///////////////////////

	protected:

		// Functions            ///////////////////////

		/*! ********************************************************************
		\brief
			Registers a single method and its name within the internal method
			table.
		
		\param fn_name
			The name of the function.

		\param fn_obj
			The callable function object.
		***********************************************************************/
		void RegisterMethod(std::string const & fn_name, Void_fn const & fn_obj);

		/*! ********************************************************************
		\brief
			Registers a list of methods and their names within the internal
			method table.

		\param list
			A list of methods and names to be added.
		***********************************************************************/
		void RegisterMethods(MethodList_t const & list);

		/*! ********************************************************************
		\brief
			Creates a vector containing the names of functions, and the callable
			functions themselves.

			See Tools::MethodTable documentation on more info about this system.

		\return
			The vector containing callable functions and their names as strings.
		***********************************************************************/
		virtual MethodList_t CreateMethodList() = 0;
	}; // class MethodTable
} // namespace Tools
} // namespace OCAE

// Public Functions             ////////////////////////////////////////////////

#endif // __OCAE_METHODTABLE_HPP