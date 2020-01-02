#ifndef BVH_ANIM_FWD_HPP
#define BVH_ANIM_FWD_HPP

#include <cstdint>

namespace bvh_anim {

enum class channel_type: uint8_t;

class channel;
class offset;
class joint;
class frame;

class frame_iterator;
class joint_iterator;
class channel_iterator;

class bvh;

}

#endif // BVH_ANIM_FWD_HPP
