use bstr::ByteSlice;
use crate::Channel;
use mint::Vector3;
use smallvec::SmallVec;
use std::{fmt, mem};

/// An alias for the type used for the `Joint::name`.
///
/// This is a byte string which may be valid `utf8`.
pub type JointName = SmallVec<[u8; mem::size_of::<String>()]>;

/// A `Joint` in a bvh skeleton.
#[derive(Clone, PartialEq)]
pub struct Joint {
    /// Name of the `Joint`.
    pub name: JointName,
    /// Positional offset of this `Joint` relative to the parent.
    pub offset: Vector3<f32>,
    /// The channels applicable to this `Joint`.
    pub channels: SmallVec<[Channel; 6]>,
    /// End site offset.
    pub(crate) end_site: Option<Vector3<f32>>,
    /// The index of the parent `Joint` in the `Bvh::joints` array.
    pub(crate) parent_index: Option<usize>,
    /// The depth of the `Joint`.
    pub(crate) depth: usize,
}

impl Joint {
    /// Returns `true` if the `Joint` is a child `Joint`, or `false` if it isn't.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bvh_anim::bvh;
    /// let bvh = bvh! {
    ///     HIERARCHY
    ///     ROOT Hips
    ///     {
    ///         OFFSET 0.0 0.0 0.0
    ///         CHANNELS 6 Xposition Yposition Zposition Zrotation Xrotation Yrotation
    ///         End Site {
    ///             OFFSET 0.0 0.0 30.0
    ///         }
    ///     }
    ///     MOTION
    ///     Frames: 0
    ///     Frame Time: 0.0333333
    /// };
    ///
    /// let root = bvh.root_joint().unwrap();
    /// assert!(root.is_root());
    /// ```
    #[inline]
    pub fn is_root(&self) -> bool {
        self.parent_index.is_none()
    }

    /// Returns `true` if the `Joint` is an end `Joint` (i.e. has an end site),
    /// or `false` if it isn't.
    #[inline]
    pub fn is_end_joint(&self) -> bool {
        self.end_site.is_none()
    }

    /// Returns the index of the parent `Joint` in the `bvh`, or `None` if this
    /// is the root `Joint`.
    #[inline]
    pub fn parent_index(&self) -> Option<usize> {
        self.parent_index
    }

    /// Returns the `end_site` of the `Joint` if this is a leaf `Joint`, or `None`.
    #[inline]
    pub fn end_site(&self) -> Option<Vector3<f32>> {
        self.end_site
    }

    /// Returns the depth of this `Joint`. The root `Joint` has a depth of `0`,
    /// and then this value is incremented for each direct child.
    #[inline]
    pub fn depth(&self) -> usize {
        self.depth
    }
}

impl Default for Joint {
    #[inline]
    fn default() -> Self {
        Self {
            name: Default::default(),
            offset: From::from([0.0, 0.0, 0.0]),
            channels: Default::default(),
            end_site: Default::default(),
            parent_index: Default::default(),
            depth: Default::default(),
        }
    }
}

impl fmt::Debug for Joint {
    #[inline]
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmtr.debug_struct("Joint")
            .field("name", &self.name.as_slice().as_bstr())
            .field("offset", &self.offset)
            .field("channels", &&self.channels[..])
            .field("end_site", &self.end_site)
            .field("parent_index", &self.parent_index)
            .finish()
    }
}
