#include <chrono>
#include <iostream>

#include "tracer.h"

using namespace std;

FunctionTracer::FunctionTracer(const char* file, const char* function) {
  fileName = file;
  functionName = function;

  auto start = std::chrono::high_resolution_clock::now();
  auto start_ns = start.time_since_epoch();
  timeDuration = start_ns.count();

  std::cout << fileName << ": " << functionName << ": enter" << std::endl;
}

FunctionTracer::~FunctionTracer() {
  auto end = std::chrono::high_resolution_clock::now();
  auto end_ns = end.time_since_epoch();
  timeDuration = end_ns.count() - timeDuration;

  std::cout << fileName << ": " << functionName << ": exit (" << timeDuration << "ns)" << std::endl;
}
