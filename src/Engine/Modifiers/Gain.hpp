/*! ****************************************************************************
\file             Gain.hpp
\author           Chyler Morrison
\par    Email:    contact\@chyler.info
\par    Project:  Audio Engine

\copyright        Copyright © 2019 Chyler Morrison
*******************************************************************************/

#ifndef __OCAE_GAIN_HPP
#define __OCAE_GAIN_HPP

// Include Files                ////////////////////////////////////////////////

#include "../Engine.hpp"

#include "ModifierBase.hpp"

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

// Public Enums                 ////////////////////////////////////////////////

// Public Objects               ////////////////////////////////////////////////

namespace OCAE
{
namespace Modifier
{
	/*! ************************************************************************
	\brief
		Simple gain filter for amplifying the input signal. The gain value can
		be negative allowing for inverting the input signal.
	***************************************************************************/
	class Gain : public ModifierBase
	{
	private:

		// Members              ///////////////////////

			/// The gain
		Math_t m_Gain;

	public:

		// Con-/De- structors   ///////////////////////

		/*! ********************************************************************
		\brief
			Copy constructor. Deleted.

		\param other
			The other object to be copied.
		***********************************************************************/
		Gain(Gain const & other) = delete;

		/*! ********************************************************************
		\brief
			Default move constructor.

		\param other
			The other object to be moved.
		***********************************************************************/
		Gain(Gain && other) = default;

		/*! ********************************************************************
		\brief
			Destructor.
		***********************************************************************/
		virtual ~Gain() = default;

		// Operators            ///////////////////////

		/*! ********************************************************************
		\brief
			Copy assignment operator. Deleted.

		\param rhs
			The object to be copied.

		\return
			*this.
		***********************************************************************/
		Gain & operator=(Gain const & rhs) = delete;

		/*! ********************************************************************
		\brief
			Default move assignment operator.

		\param rhs
			The object to be moved.

		\return
			*this.
		***********************************************************************/
		Gain & operator=(Gain && rhs) = default;

		// Accossors/Mutators   ///////////////////////

		/*! ********************************************************************
		\brief
			Sets the gain for the filter.

		\param gain
			The new gain. Can be negative.
		***********************************************************************/
		void SetGain(Math_t gain);

		/*! ********************************************************************
		\brief
			Returns the current gain for the filter.

		\return
			The gain of the filter.
		***********************************************************************/
		Math_t GetGain() const;

		// Functions            ///////////////////////

		/*! ********************************************************************
		\brief
			Takes input sample and filters it, returning the result.

		\param input
			The input sample.

		\return
			The filtered sample.
		***********************************************************************/
		virtual StereoData Process(StereoData const & input);

		/*! ********************************************************************
		\brief
			Returns boolean for if the object calling this function is a
			ModifierBase or not.

		\return
			False.
		***********************************************************************/
		virtual bool IsBase() { return false; };

			/// Add the factory as a friend so it can construct Gain objects
		friend class ModifierFactory;

	protected:

		// Functions                  ///////////////////////

		/*! ********************************************************************
		\brief
			Constructor.

		\param gain
			The gain to apply to the input data. Can be negative allowing for
			inverting the signal.
		***********************************************************************/
		Gain(Math_t gain = OCAE_DEFAULT_GAIN);

		/*! ********************************************************************
		\brief
			Creates a vector containing the names of functions, and the callable
			functions themselves.

			See Tools::MethodTable documentation on more info about this system.

		\return
			The vector containing callable functions and their names as strings.
		***********************************************************************/
		virtual Tools::MethodTable::MethodList_t CreateMethodList();
	}; // class Gain

		/// Alias for a std::shared_ptr instantiated with the Gain class
	OCAE_TYPEDEF_SHARED(Gain);
} // namespace Modifier
} // namespace OCAE

// Public Functions             ////////////////////////////////////////////////

#endif // __OCAE_GAIN_HPP
