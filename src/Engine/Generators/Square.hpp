/*! ****************************************************************************
\file             Sine.hpp
\author           Chyler Morrison
\par    Email:    contact\@chyler.info
\par    Project:  Audio Engine

\copyright        Copyright © 2019 Chyler Morrison
*******************************************************************************/

#ifndef __SQUARE_HPP
#define __SQUARE_HPP

// Include Files                ////////////////////////////////////////////////

#include "../Engine.hpp"

#include "GeneratorBase.hpp"

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

// Public Enums                 ////////////////////////////////////////////////

// Public Objects               ////////////////////////////////////////////////

namespace AudioEngine
{
namespace Generator
{
	/*! ************************************************************************
	\brief
		Generates square wave data at the given frequency.
	***************************************************************************/
	class Square : public GeneratorBase
	{
	private:

		// Members              ///////////////////////

		Math_t m_Ind;
		Math_t m_Inv;

		MethodTable_t m_Table;

	public:

		// Con-/De- structors   ///////////////////////

		/*! ********************************************************************
		\brief
			Default destructor.
		***********************************************************************/
		virtual ~Square() = default;

		// Operators            ///////////////////////

		// Accossors/Mutators   ///////////////////////

		// Functions            ///////////////////////

		/*! ********************************************************************
		\brief
			Sends a single sample to Core::Driver for output to the OS.

		\return
			The stereo sample data.
		***********************************************************************/
		virtual StereoData_t SendSample(void);
		virtual void SendBlock(StereoData_t * buffer, uint64_t size);

		/*! ********************************************************************
		\brief
			Sets the frequency to a new value.

		\param freq
			The new frequency.
		***********************************************************************/
		void SetFrequency(Math_t freq);

		virtual MethodTable_t const & GetMethodTable() const;

	private:

		// Functions                  ///////////////////////

		/*! ********************************************************************
		\brief
			Creates an object that outputs a simple square wave without using
			inefficient functions like std::sin.

		\param freq
			The frequency for the square wav to output at.
		***********************************************************************/
		Square(Math_t freq);

	}; // class Square
} // namespace Generator
} // namespace AudioEngine

// Public Functions             ////////////////////////////////////////////////

#endif // __SQUARE_HPP
