#include <iostream>

#include "store.h"

using namespace std;

LMDBStore::LMDBStore() {
  char* result = Hello(const_cast<char*>("World"));
  std::cout << result << std::endl;
  FreeString(result);
}

LMDBStore::~LMDBStore() {
}
