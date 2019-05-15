/*! ****************************************************************************
\file             Vocoder.cpp
\author           Chyler Morrison
\par    Email:    contact\@chyler.info
\par    Project:  AudioEngine

\copyright        Copyright © 2018 Chyler
*******************************************************************************/

// Include Files                          //////////////////////////////////////

#include "../Generators/Sine.hpp"
#include "../Generators/Square.hpp"
#include "Envelope.hpp"
#include "BandPass.hpp"
#include "Vocoder.hpp"

#include <Trace/Trace.hpp>

// Private Macros                         //////////////////////////////////////

// Private Enums                          //////////////////////////////////////

// Private Objects                        //////////////////////////////////////

float freq[] = {/*55,68.75,82.5,110,137.5,165,220,275,330,440,550,660,880,1100,1320,1760,2200,2640,2520,4400,5280*/
                220,440,660,880};

// Private Function Declarations          //////////////////////////////////////

// Public Functions                       //////////////////////////////////////

namespace AudioEngine
{
namespace Modifier
{

  Vocoder::Vocoder(std::shared_ptr<Core::Node> const & base_input, int N) :
    m_CentralFrequencies(), m_BandCount(N), m_Mu(1.f),
    m_Sound(new Core::Sound())
  {
    auto l_BP = BPSetup();
    auto l_Env = EnvSetup();
    auto l_Osc = OscSetup();

    m_Sound->AddNode(base_input, 0);

    for(uint32_t i = 0; i < m_BandCount; ++i)
    {
      std::shared_ptr<Core::Node> bp(
        std::make_shared<Core::Node>(
          std::make_shared<Generator::Base>(true),
          l_BP[i]
        )
      );
      std::shared_ptr<Core::Node> mod(
        std::make_shared<Core::Node>(l_Osc[i], l_Env[i])
      );

      base_input->AddTarget(*bp);
      bp->AddTarget(*mod);

      m_Sound->AddNode(bp, 1);
      m_Sound->AddNode(mod, 2, true);
    }
  }

  Vocoder::operator std::shared_ptr<Core::Sound>()
  {
    return m_Sound;
  }

  void Vocoder::SetOffset(float p)
  {
    m_Mu = std::powf(2, p/1200.f);

    uint32_t counter = 0;

    auto & graph = m_Sound->GetGraph();
    for(auto & node : graph[2])
    {
      dynamic_cast<Carrier_t *>(node->GetGenerator().get())->
        SetFrequency(/*m_CentralFrequencies*/freq[++counter]*m_Mu);
    }
  }

} // namespace Modifier
} // namespace AudioEngine

// Private Functions                      //////////////////////////////////////

namespace AudioEngine
{
namespace Modifier
{

  std::vector<std::shared_ptr<Modifier::Base>> Vocoder::BPSetup()
  {
    std::vector<double> l_Freq(m_BandCount+1, 0.0);

    l_Freq[0] = 80;
    l_Freq[m_BandCount] = 4'000;

    double delta = ((std::log(l_Freq[m_BandCount])/std::log(10)) - (std::log(l_Freq[0])/std::log(10))) / m_BandCount;

    for(uint32_t i = 1; i < m_BandCount; ++i)
    {
      l_Freq[i] = l_Freq[0] * std::pow(10, i*delta);
    }

    std::vector<std::shared_ptr<Modifier::Base>> l_BP;

    double l_Q = std::sqrt(l_Freq[1]*l_Freq[0]) / (l_Freq[1] - l_Freq[0]);

    for(uint32_t i = 0; i < m_BandCount; ++i)
    {
      m_CentralFrequencies.push_back(std::sqrt(float(l_Freq[i] * l_Freq[i+1])));
      l_BP.push_back(
        std::make_shared<BandPass>(m_CentralFrequencies.back(), float(l_Q))
      );
    }

    return l_BP;
  }

  std::vector<std::shared_ptr<Modifier::Base>> Vocoder::EnvSetup()
  {
    std::vector<std::shared_ptr<Modifier::Base>> l_Env;

    for(uint32_t i = 0; i < m_BandCount; ++i)
    {
      l_Env.push_back(std::make_shared<EnvelopeFollower>(20.f, 20'000.f));
    }

    return l_Env;
  }

  std::vector<std::shared_ptr<Generator::Base>> Vocoder::OscSetup()
  {
    std::vector<std::shared_ptr<Generator::Base>> l_Osc;

    for(uint32_t i = 0; i < m_BandCount; ++i)
    {
      l_Osc.push_back(
        std::make_shared<Carrier_t>(/*m_CentralFrequencies*/freq[i] * m_Mu)
      );
    }

    return l_Osc;
  }

} // namespace Modifier
} // namespace AudioEngine