use super::{CoordTranslate, ReverseCoordTranslate};
use crate::drawing::backend::{BackendCoord, DrawingBackend, DrawingErrorKind};
use crate::style::ShapeStyle;

use std::ops::Range;

/// The trait that indicates we have a ordered and ranged value
/// Which is used to describe the axis
pub trait Ranged {
    /// The type of this value
    type ValueType;

    /// This function maps the value to i32, which is the drawing coordinate
    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32;

    /// This function gives the key points that we can draw a grid based on this
    fn key_points(&self, max_points: usize) -> Vec<Self::ValueType>;

    /// Get the range of this value
    fn range(&self) -> Range<Self::ValueType>;
}

/// The trait indicates the ranged value can be map reversely, which means
/// an pixel-based cooridinate is given, it's possible to figureout the underlying
/// logic value.
pub trait ReversableRanged: Ranged {
    fn unmap(&self, input: i32, limit: (i32, i32)) -> Option<Self::ValueType>;
}

/// The coordinate described by two ranged value
pub struct RangedCoord<X: Ranged, Y: Ranged> {
    logic_x: X,
    logic_y: Y,
    back_x: (i32, i32),
    back_y: (i32, i32),
}

impl<X: Ranged, Y: Ranged> RangedCoord<X, Y> {
    /// Create a new ranged value coordinate system
    pub fn new<IntoX: Into<X>, IntoY: Into<Y>>(
        logic_x: IntoX,
        logic_y: IntoY,
        actual: (Range<i32>, Range<i32>),
    ) -> Self {
        Self {
            logic_x: logic_x.into(),
            logic_y: logic_y.into(),
            back_x: (actual.0.start, actual.0.end),
            back_y: (actual.1.start, actual.1.end),
        }
    }

    /// Draw the mesh for the coordinate system
    pub fn draw_mesh<E, DrawMesh: FnMut(MeshLine<X, Y>) -> Result<(), E>>(
        &self,
        h_limit: usize,
        v_limit: usize,
        mut draw_mesh: DrawMesh,
    ) -> Result<(), E> {
        let (xkp, ykp) = (
            self.logic_x.key_points(v_limit),
            self.logic_y.key_points(h_limit),
        );

        for logic_x in xkp {
            let x = self.logic_x.map(&logic_x, self.back_x);
            draw_mesh(MeshLine::XMesh(
                (x, self.back_y.0),
                (x, self.back_y.1),
                &logic_x,
            ))?;
        }

        for logic_y in ykp {
            let y = self.logic_y.map(&logic_y, self.back_y);
            draw_mesh(MeshLine::YMesh(
                (self.back_x.0, y),
                (self.back_x.1, y),
                &logic_y,
            ))?;
        }

        Ok(())
    }

    /// Get the range of X axis
    pub fn get_x_range(&self) -> Range<X::ValueType> {
        self.logic_x.range()
    }

    /// Get the range of Y axis
    pub fn get_y_range(&self) -> Range<Y::ValueType> {
        self.logic_y.range()
    }
}

impl<X: Ranged, Y: Ranged> CoordTranslate for RangedCoord<X, Y> {
    type From = (X::ValueType, Y::ValueType);

    fn translate(&self, from: &Self::From) -> BackendCoord {
        (
            self.logic_x.map(&from.0, self.back_x),
            self.logic_y.map(&from.1, self.back_y),
        )
    }
}

impl<X: ReversableRanged, Y: ReversableRanged> ReverseCoordTranslate for RangedCoord<X, Y> {
    fn reverse_translate(&self, input: BackendCoord) -> Option<Self::From> {
        Some((
            self.logic_x.unmap(input.0, self.back_x)?,
            self.logic_y.unmap(input.1, self.back_y)?,
        ))
    }
}

/// Represent a coordinate mesh for the two ranged value coordinate system
pub enum MeshLine<'a, X: Ranged, Y: Ranged> {
    XMesh(BackendCoord, BackendCoord, &'a X::ValueType),
    YMesh(BackendCoord, BackendCoord, &'a Y::ValueType),
}

impl<'a, X: Ranged, Y: Ranged> MeshLine<'a, X, Y> {
    /// Draw a single mesh line onto the backend
    pub fn draw<DB: DrawingBackend>(
        &self,
        backend: &mut DB,
        style: &ShapeStyle,
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let (&left, &right) = match self {
            MeshLine::XMesh(a, b, _) => (a, b),
            MeshLine::YMesh(a, b, _) => (a, b),
        };
        backend.draw_line(left, right, &style.color)
    }
}

/// The trait indicates the coordinate is descrete, so that we can draw histogram on it
pub trait DescreteRanged
where
    Self: Ranged,
    Self::ValueType: Eq,
{
    /// Get the smallest value that is larger than the `this` value
    fn next_value(this: &Self::ValueType) -> Self::ValueType;
}

/// The trait for the type that can be converted into a ranged coordinate axis
pub trait AsRangedCoord: Sized {
    type CoordDescType: Ranged + From<Self>;
    type Value;
}

impl<T> AsRangedCoord for T
where
    T: Ranged,
    Range<T::ValueType>: Into<T>,
{
    type CoordDescType = T;
    type Value = T::ValueType;
}
