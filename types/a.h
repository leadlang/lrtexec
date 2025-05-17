#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uint16_t VERSION = 0;

struct CVariable {
  void *data;
  void (*_drop)(void*);
  CVariable (*_clone)(void*);
};

template<typename T>
struct Maybe {
  enum class Tag {
    Some,
    None,
  };

  struct Some_Body {
    T _0;
  };

  Tag tag;
  union {
    Some_Body some;
  };
};

struct FnStack {
  /// Return value (identifier in MemoryMap)
  Maybe<CVariable> ret;
  uint16_t itself;
  uint16_t r1;
  uint16_t r2;
  uint16_t r3;
  uint16_t r4;
  uint16_t r5;
  uint16_t r6;
  uint16_t r7;
  uint16_t r8;
};

template<typename T>
struct Wrapper {
  T _0;
};

struct VariableData {
  enum class Tag {
    Constant,
    Raw,
  };

  struct Constant_Body {
    RStr _0;
  };

  struct Raw_Body {
    CVariable _0;
  };

  Tag tag;
  union {
    Constant_Body constant;
    Raw_Body raw;
  };
};

struct MemoryMap {
  RHashMap<uint16_t, VariableData> variables;
};

struct CommonString {
  char *ptr;
};

extern "C" {

FnStack create();

void setOutput(FnStack *self, CVariable data);

CVariable into_common_u8(Wrapper<uint8_t> value);

/// This creates a Clone of the inner value
uint8_t get_common_u8(CVariable self);

CVariable into_common_u16(Wrapper<uint16_t> value);

/// This creates a Clone of the inner value
uint16_t get_common_u16(CVariable self);

CVariable into_common_u32(Wrapper<uint32_t> value);

/// This creates a Clone of the inner value
uint32_t get_common_u32(CVariable self);

CVariable into_common_u64(Wrapper<uint64_t> value);

/// This creates a Clone of the inner value
uint64_t get_common_u64(CVariable self);

CVariable into_common_usize(Wrapper<uintptr_t> value);

/// This creates a Clone of the inner value
uintptr_t get_common_usize(CVariable self);

CVariable into_common_i8(Wrapper<int8_t> value);

/// This creates a Clone of the inner value
int8_t get_common_i8(CVariable self);

CVariable into_common_i16(Wrapper<int16_t> value);

/// This creates a Clone of the inner value
int16_t get_common_i16(CVariable self);

CVariable into_common_i32(Wrapper<int32_t> value);

/// This creates a Clone of the inner value
int32_t get_common_i32(CVariable self);

CVariable into_common_i64(Wrapper<int64_t> value);

/// This creates a Clone of the inner value
int64_t get_common_i64(CVariable self);

CVariable into_common_f32(Wrapper<float> value);

/// This creates a Clone of the inner value
float get_common_f32(CVariable self);

CVariable into_common_f64(Wrapper<double> value);

/// This creates a Clone of the inner value
double get_common_f64(CVariable self);

CVariable into_common_isize(Wrapper<intptr_t> value);

/// This creates a Clone of the inner value
intptr_t get_common_isize(CVariable self);

CVariable into_common_bool(Wrapper<bool> value);

/// This creates a Clone of the inner value
bool get_common_bool(CVariable self);

MemoryMap create_map();

CommonString create(char *data, CVariable (*_clone)(void *data), void (*_drop)(void *data));

}  // extern "C"
