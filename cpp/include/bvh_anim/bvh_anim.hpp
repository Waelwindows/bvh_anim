#ifndef BVH_ANIM_HPP
#define BVH_ANIM_HPP

#include "bvh_anim/bvh_anim.h"
#include "bvh_anim/bvh_anim_fwd.hpp"

#include <string_view>
#include <optional>
#include <iterator>
#include <chrono>
#include <iosfwd>

namespace bvh_anim {

enum class channel_type: uint8_t {
    x_position,
    y_position,
    z_position,
    x_rotation,
    y_rotation,
    z_rotation,
};

class channel final {
public:
    constexpr channel() = default;
    constexpr channel(channel_type ty, size_t idx);
    constexpr channel(const bvh_Channel& channel);

    constexpr channel(const channel&) = default;
    constexpr channel(channel&&) = default;
    ~channel() = default;

    channel_type type = channel_type::x_position;
    size_t index = 0;
};

class offset final {
public:
    constexpr offset() = default;
    constexpr offset(float x_, float y_, float z_): x(x_), y(y_), z(z_) {}
    constexpr offset(const bvh_Offset& ofst):
        x(ofst.offset_x),
        y(ofst.offset_y),
        z(ofst.offset_z) {}

    constexpr offset(const offset&) = default;
    constexpr offset(offset&&) = default;

    float x = 0.0f, y = 0.0f, z = 0.0f;
};

class joint final {
public:
    joint(const joint&) = delete;
    joint(joint&&) = delete;
    ~joint() = default;

    constexpr bool is_end_joint() const;
    channel_iterator channels() const;
    std::string_view name() const;
    offset offset() const;
    std::optional<typename bvh_anim::offset> end_site() const;
    joint_iterator children() const;

private:
    friend class bvh;
    constexpr joint(bvh_Joint& joint): m_joint(joint) {}

    const bvh_Joint m_joint;
};

class frame final {
public:
    const float* begin() const;
    const float* end() const;

    const float* cbegin() const { return begin(); }
    const float* cend() const { return end(); }

    size_t length() const;

private:
    friend class frame_iterator;
    constexpr frame(const bvh_BvhFile& bvh, size_t frame_num):
        m_bvh(bvh), m_frame_number(frame_num) {}

    const bvh_BvhFile& m_bvh;
    size_t m_frame_number = 0;
};

class frame_iterator final {
public:
    frame_iterator& operator++();
    frame_iterator operator++(int);

    bool operator==(const frame_iterator& i) const;
    bool operator!=(const frame_iterator& i) const {
        return !operator==(i);
    }

    frame operator*() const;

private:
    friend class bvh;
    frame_iterator(const bvh_BvhFile& bvh):
        frame_idx(0), num_frames(bvh.bvh_num_frames), m_bvh(bvh) {}

    size_t frame_idx = 0, num_frames = 0;
    const bvh_BvhFile& m_bvh;
};

class channel_iterator final {
public:
    channel_iterator& operator++();
    channel_iterator operator++(int);

    bool operator==(const channel_iterator& i) const;
    bool operator!=(const channel_iterator& i) const {
        return !operator==(i);
    }

    channel operator*() const;

private:
    friend class joint;

    channel_iterator() = delete;
    channel_iterator(
        const bvh_Channel* channels,
        size_t num_channels
    ):
        m_channels(channels),
        m_num_channels(num_channels)
    {
    }

    const bvh_Channel* m_channels = nullptr;
    size_t m_num_channels = 0;
    size_t m_channel_index = 0;
};

class joint_iterator final {
public:
    joint_iterator& operator++();
    joint_iterator operator++(int);

    bool operator==(const joint_iterator& i) const;
    bool operator!=(const joint_iterator& i) const {
        return !operator==(i);
    }

    joint operator*() const;
private:
    friend class joint;
};

class bvh final {
public:
    constexpr bvh() = default;
    constexpr bvh(bvh_BvhFile&& bvh): m_bvh(bvh) {}

    bvh(const bvh&);
    bvh(bvh&&);

    bvh(const std::string& bvh_string);
    bvh(std::istream& istream);
    ~bvh();

    std::optional<bvh_anim::joint> root_joint() const;

    frame_iterator frames() const {
        return frame_iterator(m_bvh);
    }

    std::chrono::duration<double> frame_time() const {
        return std::chrono::duration<double>(m_bvh.bvh_frame_time);
    }

private:
    bvh_BvhFile m_bvh = {0};
};

}

namespace std {

template<>
struct iterator_traits<bvh_anim::channel_iterator> {
public:
    using difference_type = void;
    using value_type = bvh_anim::channel;
    using pointer = void;
    using iterator_category = std::forward_iterator_tag;
};

template<>
struct iterator_traits<bvh_anim::joint_iterator> {
public:
    using difference_type = void;
    using value_type = bvh_anim::joint;
    using pointer = void;
    using iterator_category = std::forward_iterator_tag;
};

template<>
struct iterator_traits<bvh_anim::frame_iterator> {
public:
    using difference_type = void;
    using value_type = bvh_anim::frame;
    using pointer = void;
    using iterator_category = std::forward_iterator_tag;
};

}

#endif // BVH_ANIM_HPP
