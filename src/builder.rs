#![allow(unused)]

//! Defines a `Builder` struct used to build a `Bvh` dynamically.

use crate::{joint::JointName, Bvh, Channel, ChannelType};
use bstr::{BStr, ByteSlice};
use mint::Vector3;
use smallvec::SmallVec;
use std::{fmt, time::Duration};

use crate::joint::Joint;

/// The `Builder`.
#[derive(Default)]
pub struct Builder {
    _priv: (),
}

impl fmt::Debug for Builder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Builder { .. }")
    }
}

fn collect_channels(channels: &[ChannelType], num_channels: &mut usize) -> SmallVec<[Channel; 6]> {
    let out_channels = channels
        .iter()
        .enumerate()
        .map(|(motion_index, &channel_type)| Channel {
            channel_type,
            motion_index: motion_index + *num_channels,
        })
        .collect::<SmallVec<[Channel; 6]>>();

    *num_channels += out_channels.len();
    out_channels
}

impl Builder {
    /// Start to create a new `Bvh` with a root joint.
    pub fn with_root_joint(
        name: &BStr,
        offset: Vector3<f32>,
        channels: &[ChannelType],
    ) -> JointsBuilder {
        let mut num_channels = 0;
        let channels = collect_channels(channels, &mut num_channels);
        let root_joint = new_joint(None, name, offset, channels, 0);

        JointsBuilder {
            joints: vec![root_joint],
            num_channels,
        }
    }
}

/// The `JointsBuilder`.
pub struct JointsBuilder {
    joints: Vec<Joint>,
    num_channels: usize,
}

impl fmt::Debug for JointsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JointsBuilder { .. }")
    }
}

fn new_joint(
    parent_index: Option<usize>,
    name: &BStr,
    offset: Vector3<f32>,
    channels: SmallVec<[Channel; 6]>,
    depth: usize,
) -> Joint {
    Joint {
        name: JointName::from(name.as_bytes()),
        offset,
        channels,
        depth,
        parent_index,
        ..Default::default()
    }
}

impl JointsBuilder {
    /// Push a `Joint`, setting the parent to be the previous joint.
    pub fn push_child(
        mut self,
        depth: usize,
        name: &BStr,
        offset: Vector3<f32>,
        channels: &[ChannelType],
    ) -> Self {
        let parent = self.joints.len() - 1;
        self.push_child_with_parent(depth, name, offset, channels, parent)
    }

    /// Push a `Joint` with `parent_idx` being the index of the parent,
    /// # Panics
    ///
    /// Panics if `parent_idx` is out of bounds.

    pub fn push_child_with_parent(
        mut self,
        depth: usize,
        name: &BStr,
        offset: Vector3<f32>,
        channels: &[ChannelType],
        parent_idx: usize,
    ) -> Self {
        let channels = collect_channels(channels, &mut self.num_channels);
        assert!(parent_idx < self.joints.len());
        self.joints
            .push(new_joint(Some(parent_idx), name, offset, channels, depth));
        self
    }

    /// Cap the last pushed `Joint` with an `End Site`.
    pub fn push_end(mut self, offset: Vector3<f32>) -> Self {
        self.joints.last_mut().unwrap().end_site = Some(offset);
        self
    }

    /// Begin the `MOTION` section.
    #[inline]
    pub fn with_motion(self, num_frames: usize, frame_time: Duration) -> MotionBuilder {
        let num_channels = self.num_channels;
        MotionBuilder {
            joints_builder: self,
            frame_time,
            num_frames,
            motion_values: Vec::with_capacity(num_frames * num_channels),
        }
    }
}

/// The `MotionBuilder`.
pub struct MotionBuilder {
    joints_builder: JointsBuilder,
    frame_time: Duration,
    num_frames: usize,
    motion_values: Vec<f32>,
}

impl fmt::Debug for MotionBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MotionBuilder { .. }")
    }
}

impl MotionBuilder {
    /// Push a frame of motion values.
    pub fn push_frame(mut self, frame: &[f32]) -> Self {
        assert_eq!(frame.len(), self.joints_builder.num_channels);
        self.motion_values.extend(frame);
        self
    }

    /// Build the `Bvh`.
    pub fn build(self) -> Result<Bvh, ()> {
        let mut bvh = Bvh::default();

        bvh.joints = self.joints_builder.joints;
        bvh.set_frame_time(self.frame_time);
        bvh.num_frames = self.num_frames;
        bvh.num_channels = self.joints_builder.num_channels;
        bvh.motion_values = self.motion_values;

        Ok(bvh)
    }
}

#[cfg(test)]
mod tests {
    #[ignore]
    #[test]
    fn builder_create() {
        // let bvh = Builder::with_root_joint();
    }
}
