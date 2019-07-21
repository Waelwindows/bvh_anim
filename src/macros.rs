use crate::{fraction_seconds_to_duration, joint::Joint, Bvh, Channel, ChannelType};

#[doc(hidden)]
#[macro_export]
macro_rules! match_channels {
    ($builder:ident; ) => {
    };
    ($builder:ident; Xposition $($rest:ident)*) => {
        $builder.push_channel($crate::ChannelType::PositionX);
        $crate::match_channels!($builder; $($rest)*);
    };
    ($builder:ident; Yposition $($rest:ident)*) => {
        $builder.push_channel($crate::ChannelType::PositionY);
        $crate::match_channels!($builder; $($rest)*);
    };
    ($builder:ident; Zposition $($rest:ident)*) => {
        $builder.push_channel($crate::ChannelType::PositionZ);
        $crate::match_channels!($builder; $($rest)*);
    };
    ($builder:ident; Xrotation $($rest:ident)*) => {
        $builder.push_channel($crate::ChannelType::RotationX);
        $crate::match_channels!($builder; $($rest)*);
    };
    ($builder:ident; Yrotation $($rest:ident)*) => {
        $builder.push_channel($crate::ChannelType::RotationY);
        $crate::match_channels!($builder; $($rest)*);
    };
    ($builder:ident; Zrotation $($rest:ident)*) => {
        $builder.push_channel($crate::ChannelType::RotationZ);
        $crate::match_channels!($builder; $($rest)*);
    };
    ($builder:expr; $($other:tt)*) => {
        compile_error!("Unknown tokens");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! parse_offset {
    ($builder:ident ($ofst_x:literal $ofst_y:literal $ofst_z:literal)) => {
        let offset = [f32::from($ofst_x), f32::from($ofst_y), f32::from($ofst_z)];
        $builder.push_joint_offset(offset.into(), false);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! parse_joints_internal {
    ($builder:ident (
        $(
            JOINT $joint_nm:ident
            {
                $( $children:tt )*
            }
        )*
    )) => {
        $(
            $builder.push_joint(stringify!($joint_nm));

            $builder.current_depth += 1;
            $crate::parse_joints_internal!($builder ( $( $children )* ));
            $builder.current_depth -= 1;
        )*
    };

    ($builder:ident (
        OFFSET $ofst_x:literal $ofst_y:literal $ofst_z:literal
        CHANNELS 0
        $($rest:tt)*
    )) => {
        $crate::parse_offset!($builder ($ofst_x $ofst_y $ofst_z));
        $crate::parse_joints_internal!($builder ( $($rest)* ));
    };

    ($builder:ident (
        OFFSET $ofst_x:literal $ofst_y:literal $ofst_z:literal
        CHANNELS 1 $ch0:ident
        $($rest:tt)*
    )) => {
        $crate::parse_offset!($builder ($ofst_x $ofst_y $ofst_z));
        $crate::match_channels!($builder ; $ch0);
        $crate::parse_joints_internal!($builder ( $($rest)* ));
    };

    ($builder:ident (
        OFFSET $ofst_x:literal $ofst_y:literal $ofst_z:literal
        CHANNELS 2 $ch0:ident $ch1:ident
        $($rest:tt)*
    )) => {
        $crate::parse_offset!($builder ($ofst_x $ofst_y $ofst_z));
        $crate::match_channels!($builder ; $ch0 $ch1);
        $crate::parse_joints_internal!($builder ( $($rest)* ));
    };

    ($builder:ident (
        OFFSET $ofst_x:literal $ofst_y:literal $ofst_z:literal
        CHANNELS 3 $ch0:ident $ch1:ident $ch2:ident
        $($rest:tt)*
    )) => {
        $crate::parse_offset!($builder ($ofst_x $ofst_y $ofst_z));
        $crate::match_channels!($builder ; $ch0 $ch1 $ch2);
        $crate::parse_joints_internal!($builder ( $($rest)* ));
    };

    ($builder:ident (
        OFFSET $ofst_x:literal $ofst_y:literal $ofst_z:literal
        CHANNELS 4 $ch0:ident $ch1:ident $ch2:ident $ch3:ident
        $($rest:tt)*
    )) => {
        bvh_anim$crate::parse_offset!($builder ($ofst_x $ofst_y $ofst_z));
        bvh_anim$crate::match_channels!($builder ; $ch0 $ch1 $ch2 $ch3);
        bvh_anim$crate::parse_joints_internal!($builder ( $($rest)* ));
    };

    ($builder:ident (
        OFFSET $ofst_x:literal $ofst_y:literal $ofst_z:literal
        CHANNELS 5
            $ch0:ident
            $ch1:ident
            $ch2:ident
            $ch3:ident
            $ch4:ident
        $($rest:tt)*
    )) => {
        bvh_anim::parse_offset!($builder ($ofst_x $ofst_y $ofst_z));
        bvh_anim::match_channels!($builder ; $ch0 $ch1 $ch2 $ch3 $ch4);
        bvh_anim::parse_joints_internal!($builder ( $($rest)* ));
    };

    ($builder:ident (
        OFFSET $ofst_x:literal $ofst_y:literal $ofst_z:literal
        CHANNELS 6
            $ch0:ident
            $ch1:ident
            $ch2:ident
            $ch3:ident
            $ch4:ident
            $ch5:ident
        $($rest:tt)*
    )) => {
        $crate::parse_offset!($builder ($ofst_x $ofst_y $ofst_z));
        $crate::match_channels!($builder ;
            $ch0 $ch1 $ch2 $ch3 $ch4 $ch5);
        $crate::parse_joints_internal!($builder ( $($rest)* ));
    };

    ($builder:ident (
        CHANNELS 7
            $ch0:ident
            $ch1:ident
            $ch2:ident
            $ch3:ident
            $ch4:ident
            $ch5:ident
            $ch6:ident
        $($rest:tt)*
    )) => {
        $crate::parse_offset!($builder ($ofst_x $ofst_y $ofst_z));
        $crate::match_channels!($builder ;
            $ch0 $ch1 $ch2 $ch3 $ch4 $ch5 $ch6);
        $crate::parse_joints_internal!($builder ( $($rest)* ));
    };

    ($builder:ident (
        OFFSET $ofst_x:literal $ofst_y:literal $ofst_z:literal
        CHANNELS 8
            $ch0:ident
            $ch1:ident
            $ch2:ident
            $ch3:ident
            $ch4:ident
            $ch5:ident
            $ch6:ident
            $ch7:ident
        $($rest:tt)*
    )) => {
        $crate::parse_offset!($builder ($ofst_x $ofst_y $ofst_z));
        $crate::match_channels!($builder ;
            $ch0 $ch1 $ch2 $ch3 $ch4 $ch5 $ch6 $ch7);
        $crate::parse_joints_internal!($builder ( $($rest)* ));
    };

    ($builder:ident (
        OFFSET $ofst_x:literal $ofst_y:literal $ofst_z:literal
        CHANNELS 9
            $ch0:ident
            $ch1:ident
            $ch2:ident
            $ch3:ident
            $ch4:ident
            $ch5:ident
            $ch6:ident
            $ch7:ident
            $ch8:ident
        $($rest:tt)*
    )) => {
        $crate::parse_offset!($builder ($ofst_x $ofst_y $ofst_z));
        $crate::match_channels!($builder ;
            $ch0 $ch1 $ch2 $ch3 $ch4 $ch5 $ch6 $ch7 $ch8);
        $crate::parse_joints_internal!($builder ( $($rest)* ));
    };

    ($builder:ident (
        OFFSET $ofst_x:literal $ofst_y:literal $ofst_z:literal
        CHANNELS $unsupported:literal $($rest:tt)*
    )) => {
        compile_error!("No more than 9 channels supported in CHANNELS statement");
    };

    ($builder:ident (
        End Site
        {
            OFFSET $end_x:literal $end_y:literal $end_z:literal
        }
    )) => {
        let offset = [
            f32::from($end_x),
            f32::from($end_y),
            f32::from($end_z),
        ];

        $builder.push_joint_offset(offset.into() , true);
    };
}

/// Create a new [`Bvh`][`Bvh`] object using a macro literal. Useful for
/// testing.
///
/// # Notes
///
/// If you have a very complex `Bvh` file with a large number of joints and frames, then
/// this macro will scale badly to it, and compilation time will suffer.
///
/// # Example
///
/// ```
/// # use bvh_anim::bvh;
/// let simple_skeleton = bvh! {
///     HIERARCHY
///     ROOT Base
///     {
///         OFFSET 0.0 0.0 0.0
///         CHANNELS 6 Xposition Yposition Zposition Zrotation Xrotation Yrotation
///         JOINT Middle1
///         {
///             OFFSET 0.0 0.0 15.0
///             CHANNELS 3 Zrotation Xrotation Yrotation
///             JOINT Tip1
///             {
///                 OFFSET 0.0 0.0 30.0
///                 CHANNELS 3 Zrotation Xrotation Yrotation
///                 End Site
///                 {
///                     OFFSET 0.0 0.0 45.0
///                 }
///             }
///         }
///         JOINT Middle2
///         {
///             OFFSET 0.0 15.0 0.0
///             CHANNELS 3 Zrotation Xrotation Yrotation
///             JOINT Tip2
///             {
///                 OFFSET 0.0 30.0 0.0
///                 CHANNELS 3 Zrotation Xrotation Yrotation
///                 End Site
///                 {
///                     OFFSET 0.0 45.0 0.0
///                 }
///             }
///         }
///     }
///
///     MOTION
///     Frames: 3
///     // Time in seconds.
///     Frame Time: 0.033333333333
///     0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0
///     1.0 1.0 1.0 1.0 1.0 1.0 1.0 1.0 1.0 1.0 1.0 1.0 1.0 1.0 1.0 1.0 1.0 1.0
///     2.0 2.0 2.0 2.0 2.0 2.0 2.0 2.0 2.0 2.0 2.0 2.0 2.0 2.0 2.0 2.0 2.0 2.0
/// };
/// ```
///
/// You can use the `bvh` macro to create empty `Bvh` instances:
///
/// ```
/// # use bvh_anim::bvh;
/// let empty = bvh!{};
///
/// let empty = bvh! {
///     HIERARCHY
///     MOTION
/// };
///
/// let empty = bvh! {
///     HIERARCHY
///     MOTION
///     Frames: 0
/// };
/// ```
///
/// [`bvh`]: struct.Bvh.html
#[macro_export]
macro_rules! bvh {
    () => {
        $crate::Bvh::default()
    };

    (
        HIERARCHY
        MOTION
    ) => {
        $crate::Bvh::default()
    };

    (
        HIERARCHY
        MOTION
        Frames: 0
    ) => {
        $crate::Bvh::default()
    };

    (
        HIERARCHY
        MOTION
        Frames: 0
        Frame Time: $frame_time:literal
    ) => {
        {
            let mut bvh = $crate::Bvh::default();
            bvh.set_frame_time(f64::from($frame_time));
            bvh
        }
    };

    (
        HIERARCHY
        ROOT $root_name:ident
        {
            $( $joints:tt )*
        }
        MOTION
        Frames: 0
        Frame Time: $frame_time:literal
    ) => {
        {
            use $crate::parse_joints_internal;

            let mut builder = $crate::BvhLiteralBuilder::default();
            builder.push_root(stringify!($root_name));

            builder.current_depth += 1;
            parse_joints_internal!(builder ($($joints)*));
            builder.current_depth -= 1;

            builder.set_num_frames(0);
            builder.set_frame_time(f64::from($frame_time));

            builder.bvh
        }
    };

    (
        HIERARCHY
        ROOT $root_name:ident
        {
            $( $joints:tt )*
        }
        MOTION
        Frames: $num_frames:literal
        Frame Time: $frame_time:literal
        $(
            $motion:literal
        )+
    ) => {
        {
            use $crate::parse_joints_internal;

            let mut builder = $crate::BvhLiteralBuilder::default();

            builder.push_root(stringify!($root_name));

            builder.current_depth += 1;
            parse_joints_internal!(builder ($($joints)*));
            builder.current_depth -= 1;

            builder.set_num_frames($num_frames as usize);
            builder.set_frame_time(f64::from($frame_time));

            builder.set_motion_values(vec![ $( f32::from($motion) ),+ ]);

            assert!(builder.check_valid_motion());

            builder.bvh
        }
    };
}

// @TODO: refactor this into a general `Builder` so that we have an
// easy interface to get other animation/skeleton data into a `bvh`
// file.
/// Helper struct to build a `Bvh` from the macro without exposing
/// too many internals.
#[doc(hidden)]
#[derive(Default)]
pub struct BvhLiteralBuilder {
    pub bvh: Bvh,
    pub current_channel_index: usize,
    pub current_depth: usize,
    pub current_index: usize,
    pub encountered_hierarchy: bool,
    pub encountered_root: bool,
    pub encountered_motion: bool,
    pub encountered_num_frames: bool,
    pub encountered_frame_time: bool,
    pub num_frames: usize,
}

#[doc(hidden)]
impl BvhLiteralBuilder {
    pub fn push_root(&mut self, name: &str) {
        let mut root = Joint::default();
        root.name = From::from(name.as_bytes());
        self.bvh.joints.push(root);
        self.current_index += 1;
    }

    pub fn push_joint(&mut self, name: &str) {
        // @TODO: make this shared
        #[inline]
        fn get_parent_index(joints: &[Joint], for_depth: usize) -> usize {
            joints
                .iter()
                .enumerate()
                .rev()
                .find(|(_, jd)| jd.depth == for_depth.saturating_sub(1))
                .map(|(i, _)| i)
                .unwrap_or(0)
        }

        let dpth = self.current_depth;
        let parent = get_parent_index(&self.bvh.joints[..], dpth);

        let mut joint = Joint::default();
        joint.name = From::from(name.as_bytes());
        joint.parent_index = Some(parent);
        joint.depth = dpth;

        self.bvh.joints.push(joint);

        self.current_index += 1;
    }

    pub fn push_channel(&mut self, channel: ChannelType) {
        let channel = Channel::new(channel, self.current_channel_index);
        self.last_joint().map(|joint| joint.channels.push(channel));
        self.current_channel_index += 1;
    }

    pub fn push_joint_offset(&mut self, offset: mint::Vector3<f32>, is_end_site: bool) {
        self.last_joint().map(|joint| {
            if is_end_site {
                joint.end_site = Some(offset);
            } else {
                joint.offset = offset;
            }
        });
    }

    #[inline]
    pub fn set_frame_time(&mut self, frame_time_secs: f64) {
        self.bvh
            .set_frame_time(fraction_seconds_to_duration(frame_time_secs));
    }

    #[inline]
    pub fn set_num_frames(&mut self, num_frames: usize) {
        self.num_frames = num_frames;
        self.bvh.num_channels = self.current_channel_index;
        self.bvh.num_frames = self.num_frames;
        self.bvh
            .motion_values
            .reserve(self.current_channel_index * self.num_frames);
    }

    #[inline]
    pub fn set_motion_values(&mut self, motion_values: Vec<f32>) {
        self.bvh.motion_values = motion_values;
    }

    #[inline]
    pub fn check_valid_motion(&self) -> bool {
        self.bvh.motion_values.len() == self.bvh.num_channels * self.bvh.num_frames
    }

    #[inline]
    fn last_joint(&mut self) -> Option<&mut Joint> {
        self.bvh.joints.last_mut()
    }
}

#[cfg(test)]
mod tests {
    // Needed for macros
    use std::time::Duration;

    #[test]
    fn macro_create() {
        let bvh = bvh! {
            HIERARCHY
            ROOT Base
            {
                OFFSET 0.0 0.0 0.0
                CHANNELS 6 Xposition Yposition Zposition Zrotation Xrotation Yrotation
                JOINT End
                {
                    OFFSET 0.0 0.0 15.0
                    CHANNELS 3 Zrotation Xrotation Yrotation
                    End Site
                    {
                        OFFSET 0.0 0.0 30.0
                    }
                }
            }

            MOTION
            Frames: 1
            Frame Time: 0.033333333333
            0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0
        };

        {
            use super::{ChannelType, Joint};
            use mint::Vector3;

            fn check_joint<V0: Into<Vector3<f32>>, V1: Into<Vector3<f32>>, O: Into<Option<V1>>>(
                joint: &Joint,
                expected_name: &[u8],
                expected_offset: V0,
                channels: &[ChannelType],
                end_site: O,
            ) {
                assert_eq!(&joint.name[..], expected_name);
                assert_eq!(joint.offset, expected_offset.into());
                for (chan, expected_chan) in joint
                    .channels
                    .iter()
                    .map(|c| c.channel_type())
                    .zip(channels.iter())
                {
                    assert_eq!(chan, *expected_chan);
                }
                let end_site = end_site.into().map(Into::into);
                assert_eq!(joint.end_site(), end_site);
            }

            let mut joints = bvh.joints();

            check_joint::<[_; 3], [f32; 3], _>(
                joints.next().unwrap(),
                b"Base",
                [0.0, 0.0, 0.0],
                &[
                    ChannelType::PositionX,
                    ChannelType::PositionY,
                    ChannelType::PositionZ,
                    ChannelType::RotationZ,
                    ChannelType::RotationX,
                    ChannelType::RotationY,
                ],
                None,
            );

            check_joint(
                joints.next().unwrap(),
                b"End",
                [0.0, 0.0, 15.0],
                &[
                    ChannelType::RotationZ,
                    ChannelType::RotationX,
                    ChannelType::RotationY,
                ],
                [0.0, 0.0, 30.0],
            );
        }

        assert_eq!(*bvh.frame_time(), Duration::from_nanos(33333333));

        let frames = bvh.frames();
        assert_eq!(frames.len(), 1);
        for frame in frames {
            assert_eq!(frame.len(), 9);
            for channel in frame.as_slice().iter() {
                assert_eq!(*channel, 0.0);
            }
        }
    }

    #[test]
    fn test_empty_create() {
        macro_rules! assert_empty {
            ($bvh:expr) => {
                assert!($bvh.joints().next().is_none());
                assert_eq!(*$bvh.frame_time(), Duration::default());
                assert!($bvh.frames().next().is_none());
            };
        }

        let empty = bvh!{};
        assert_empty!(empty);

        let empty_2 = bvh! {
            HIERARCHY
            MOTION
        };
        assert_empty!(empty_2);
    }
}
