#ifndef NINJA_STORE_H_
#define NINJA_STORE_H_

extern "C" {
  char* Hello(char* name);
  void FreeString(char* s);
}

class LMDBStore {
public:
  LMDBStore();
  ~LMDBStore();
};

#endif  // NINJA_STORE_H_
