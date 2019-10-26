/*! ****************************************************************************
\file             Echo.cpp
\author           Chyler Morrison
\par    Email:    contact\@chyler.info
\par    Project:  Audio Engine

\copyright        Copyright © 2019 Chyler Morrison
*******************************************************************************/

// Include Files                          //////////////////////////////////////

#include "Echo.hpp"

// Private Macros                         //////////////////////////////////////

// Private Enums                          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

// Private Function Declarations          //////////////////////////////////////

// Public Functions                       //////////////////////////////////////

namespace AudioEngine
{
namespace Modifier
{
	Echo::Echo(
		uint64_t sample_delay,
		Math_t decay_ratio
	) : ModifierBase(false),
		m_Echo(sample_delay, StereoData()), m_Ratio(decay_ratio)
	{
	}

	StereoData Echo::FilterSample(StereoData const & dry)
	{
		StereoData wet = m_Echo.front();
		m_Echo.pop_front();

		StereoData out = StereoData(
			SampleType(Math_t( Left(wet)) * m_Ratio + Math_t( Left(dry))),
			SampleType(Math_t(Right(wet)) * m_Ratio + Math_t(Right(dry)))
		);

		m_Echo.push_back(out);

		return out;
	}
} // namespace Modifier
} // namespace AudioEngine

// Private Functions                      //////////////////////////////////////