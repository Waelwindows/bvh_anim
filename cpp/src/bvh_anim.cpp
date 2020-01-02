#include "bvh_anim/bvh_anim.hpp"

#include <iostream>

using namespace bvh_anim;

constexpr channel::channel(bvh_anim::channel_type ty, size_t idx):
    type(ty),
    index(idx)
{
}

constexpr channel::channel(const bvh_Channel& channel): index(channel.channel_index) {
    switch(channel.channel_type) {
    case X_POSITION:
        type = channel_type::x_position;
        break;
    case Y_POSITION:
        type = channel_type::y_position;
        break;
    case Z_POSITION:
        type = channel_type::z_position;
        break;
    case X_ROTATION:
        type = channel_type::x_rotation;
        break;
    case Y_ROTATION:
        type = channel_type::y_rotation;
        break;
    case Z_ROTATION:
        type = channel_type::z_rotation;
        break;
    default:
        break;
    }
}

std::string_view joint::name() const {
    return std::string_view(m_joint.joint_name);
}

offset joint::offset() const {
    return bvh_anim::offset(m_joint.joint_offset);
}

std::optional<typename bvh_anim::offset> joint::end_site() const {
    if (m_joint.joint_has_end_site == 0) {
        return std::nullopt;
    } else {
        return std::make_optional(bvh_anim::offset(m_joint.joint_end_site));
    }
}

constexpr bool joint::is_end_joint() const {
    return m_joint.joint_has_end_site != 0;
}

channel_iterator joint::channels() const {
    return channel_iterator(m_joint.joint_channels, m_joint.joint_num_channels);
}

channel_iterator& channel_iterator::operator++() {
    if (m_channel_index <= m_num_channels) {
        m_channel_index++;
    }

    return *this;
}

channel_iterator channel_iterator::operator++(int) {
    if (m_channel_index <= m_num_channels) {
        m_channel_index++;
    }

    return *this;
}

channel channel_iterator::operator*() const {
    const auto& chnl = m_channels[m_channel_index];
    return channel(chnl);
}
