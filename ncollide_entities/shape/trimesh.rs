//! 2d line strip, 3d triangle mesh, and nd subsimplex mesh.

use std::sync::Arc;
use na::{Translate, Dim, Pnt2, Pnt3};
use partitioning::BVT;
use bounding_volume::AABB;
use shape::{Triangle, BaseMesh};
use math::{Scalar, Point, Vect};

/// Shape commonly known as a 2d line strip or a 3d triangle mesh.
pub struct TriMesh<N, P, V> {
    mesh: BaseMesh<N, P, V, Pnt3<uint>, Triangle<P>>
}

impl<N, P, V> Clone for TriMesh<N, P, V>
    where N: Clone,
          P: Send + Sync + Clone,
          V: Send + Sync {
    fn clone(&self) -> TriMesh<N, P, V> {
        self.clone()
    }
}

impl<N, P, V> TriMesh<N, P, V>
    where N: Scalar,
          P: Point<N, V>,
          V: Translate<P> + Vect<N> {
    /// Builds a new mesh.
    pub fn new(vertices: Arc<Vec<P>>,
               indices:  Arc<Vec<Pnt3<uint>>>,
               uvs:      Option<Arc<Vec<Pnt2<N>>>>,
               normals:  Option<Arc<Vec<V>>>) // a loosening margin for the BVT.
               -> TriMesh<N, P, V> {
        TriMesh {
            mesh: BaseMesh::new(vertices, indices, uvs, normals)
        }
    }
}

impl<N, P, V> TriMesh<N, P, V> {
    /// The base representation of this mesh.
    #[inline]
    pub fn base_mesh(&self) -> &BaseMesh<N, P, V, Pnt3<uint>, Triangle<P>> {
        &self.mesh
    }

    /// The vertices of this mesh.
    #[inline]
    pub fn vertices(&self) -> &Arc<Vec<P>> {
        self.mesh.vertices()
    }

    /// Bounding volumes of the subsimplices.
    #[inline]
    pub fn bounding_volumes(&self) -> &[AABB<P>] {
        self.mesh.bounding_volumes()
    }

    /// The indices of this mesh.
    #[inline]
    pub fn indices(&self) -> &Arc<Vec<Pnt3<uint>>> {
        self.mesh.indices()
    }

    /// The texture coordinates of this mesh.
    #[inline]
    pub fn uvs(&self) -> &Option<Arc<Vec<Pnt2<N>>>> {
        self.mesh.uvs()
    }

    /// The normals of this mesh.
    #[inline]
    pub fn normals(&self) -> &Option<Arc<Vec<V>>> {
        self.mesh.normals()
    }

    /// The acceleration structure used for efficient collision detection and ray casting.
    #[inline]
    pub fn bvt(&self) -> &BVT<uint, AABB<P>> {
        self.mesh.bvt()
    }
}

impl<N, P: Send + Sync + Copy + Dim, V> TriMesh<N, P, V> {
    /// Gets the i-th mesh element.
    #[inline]
    pub fn triangle_at(&self, i: uint) -> Triangle<P> {
        self.mesh.element_at(i)
    }
}
