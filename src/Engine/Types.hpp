/*! ****************************************************************************
\file             Types.hpp
\author           Chyler Morrison
\par    Email:    contact\@chyler.info
\par    Project:  Audio Engine

\copyright        Copyright © 2019 Chyler Morrison
*******************************************************************************/

#ifndef __TYPES_HPP
#define __TYPES_HPP

// Include Files                ////////////////////////////////////////////////

#include <cstdint>

#include <functional>
#include <memory>
#include <tuple>

// Public Macros                ////////////////////////////////////////////////

// Forward References           ////////////////////////////////////////////////

// Public Types                 ////////////////////////////////////////////////

namespace AudioEngine
{
	using Math_t = double;//FixedPoint::FixedPoint<15,16>;
	using SampleType = Math_t;//FixedPoint::Audio16Bit_t;
	TYPEDEF_SHARED(SampleType);
	using StereoData = std::tuple<SampleType,SampleType>;
	TYPEDEF_SHARED(StereoData);
	using Track_t = std::vector<StereoData>;

	using AudioCallback_t = std::function<StereoData(void)>;

	using Void_fn = std::function<void(void*)>;
	using MethodTable_t = std::unordered_map<std::string, Void_fn>;
} // namespace AudioEngine

#endif  // __TYPES_HPP
