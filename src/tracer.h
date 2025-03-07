#ifndef NINJA_TRACER_H_
#define NINJA_TRACER_H_

#define FUNCTION_TRACER FunctionTracer functionTracer(__FILE__, __func__);

class FunctionTracer {
public:
  FunctionTracer(const char* fileName, const char* funcName);
  ~FunctionTracer();

private:
  const char* fileName;
  const char* functionName;
  int64_t timeDuration;
};

#endif  // NINJA_TRACER_H_
