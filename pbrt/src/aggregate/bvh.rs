use std::convert::TryFrom;
use std::ops::Range;
use std::mem;
use std::sync::{ Arc, Mutex };
use std::sync::atomic::{ AtomicUsize, Ordering };

use bitwise::morton;
use cgmath::prelude::*;
use itertools;
use rayon::prelude::*;

use crate::prelude::*;
use crate::interaction::SurfaceInteraction;
use crate::primitive::Primitive;
use super::Aggregate;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SplitMethod {
    SAH,
    HLBVH,
    Middle,
    EqualCounts,
}

#[derive(Copy, Clone, Debug)]
struct BvhPrimitiveInfo {
    num: usize,
    bounds: Bounds3f,
    centroid: Point3f,
}

impl BvhPrimitiveInfo {
    fn new(num: usize, bounds: Bounds3f) -> Self {
        let centroid = bounds.min * float(0.5) + (bounds.max * float(0.5)).into_vector();

        Self {
            num,
            bounds,
            centroid,
        }
    }
}

enum BvhBuildNode {
    Leaf {
        bounds: Bounds3f,
        first_primitive_offset: usize,
        num_primitives: usize,
    },
    Interior {
        bounds: Bounds3f,
        split_axis: Dim,
        children: [Arc<BvhBuildNode>; 2],
    },
}

impl BvhBuildNode {
    fn bounds(&self) -> Bounds3f {
        match self {
            BvhBuildNode::Leaf { bounds, .. } => *bounds,
            BvhBuildNode::Interior { bounds, .. } => *bounds,
        }
    }

    fn init_leaf(first: usize, n: usize, bounds: Bounds3f) -> Self {
        BvhBuildNode::Leaf {
            bounds,
            first_primitive_offset: first,
            num_primitives: n,
        }
    }

    fn init_interior(axis: Dim, c0: Arc<Self>, c1: Arc<Self>) -> Self {
        BvhBuildNode::Interior {
            bounds: c0.bounds().union(c1.bounds()),
            split_axis: axis,
            children: [c0, c1],
        }
    }
}

const MAX_PRIMITIVES_IN_NODE: usize = 255;

#[derive(Copy, Clone, Debug)]
struct MortonPrimitive {
    index: usize,
    code: u32,
}

const MORTON_SCALE: u32 = 1 << 10;

#[derive(Copy, Clone, Debug)]
enum LinearBvhNodeOffset {
    /// The number and offset of the primitives of the leaf node.
    Leaf(usize, usize),
    /// The offset of the second child of the interior node.
    Interior(Dim, usize),
}

#[repr(align(32))]
#[derive(Copy, Clone, Debug)]
struct LinearBvhNode {
    bounds: Bounds3f,
    offset: LinearBvhNodeOffset,
}

#[derive(Debug)]
pub struct BvhAccel {
    primitives: Vec<Arc<dyn Primitive + Send>>,
    nodes: Vec<LinearBvhNode>,
    split_method: SplitMethod,
}

impl BvhAccel {
    pub fn new(mut primitives: Vec<Arc<dyn Primitive + Send>>, split_method: SplitMethod) -> Self {
        assert!(!primitives.is_empty());

        // build BVH from primitives
        // init primitive_info array
        let mut primitive_info = Vec::with_capacity(primitives.len());
        for (i, prim) in primitives.iter().enumerate() {
            primitive_info.push(BvhPrimitiveInfo::new(i, prim.world_bound()));
        }

        // build BVH tree from primitive_info
        let arena = ();
        let mut total = 0;
        let mut ordered_primitives = Vec::with_capacity(primitives.len());

        let root = if split_method == SplitMethod::HLBVH {
            hlbvh_build(
                &primitives,
                &primitive_info,
                &mut total,
                &mut ordered_primitives,
                &arena
            )
        } else {
            recursive_build(
                &primitives,
                &primitive_info,
                split_method,
                0..primitives.len(),
                &mut total,
                &mut ordered_primitives,
                &arena
            )
        };

        mem::swap(&mut primitives, &mut ordered_primitives);

        // compute representation of depth-first traversal of BVH tree
        let mut nodes = vec![None; total];
        let mut offset = 0;
        flatten_bvh_tree(&mut nodes, root, &mut offset, total);

        let nodes = nodes.into_iter()
            .map(|n| n.unwrap())
            .collect();

        Self {
            primitives,
            split_method,
            nodes,
        }
    }
}

impl Aggregate for BvhAccel { }

impl Primitive for BvhAccel {
    fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction<'_>> {
        let mut isect = None;
        let inv_dir = ray.direction.map(|d| float(1.0) / d);
        let dir_is_neg = [inv_dir.x < 0.0, inv_dir.y < 0.0, inv_dir.z < 0.0];

        // follow ray through the BVH nodes to find primitive intersections
        let mut to_visit_offset = 0;
        let mut current_node_index = 0;
        let mut nodes_to_visit = [0; 64];

        loop {
            let node = &self.nodes[current_node_index];
            // check ray against BVH node
            if node.bounds.intersect_p_precomputed(*ray, inv_dir, dir_is_neg) {
                match node.offset {
                    LinearBvhNodeOffset::Leaf(n_primitives, primitives_offset) => {
                        assert!(n_primitives > 0);
                        // intersect ray with primitives
                        for p in &self.primitives[primitives_offset..(primitives_offset + n_primitives)] {
                            if let Some(i) = p.intersect(ray) {
                                isect = Some(i);
                            }
                        }

                        if to_visit_offset == 0 {
                            break;
                        }

                        to_visit_offset -= 1;
                        current_node_index = nodes_to_visit[to_visit_offset];
                    },
                    LinearBvhNodeOffset::Interior(dir, child_offset) => {
                        // put far BVH node on the nodes_to_visit stack, advance to near node
                        if dir_is_neg[dir as usize] {
                            nodes_to_visit[to_visit_offset] = current_node_index + 1;
                            to_visit_offset += 1;
                            current_node_index = child_offset;
                        } else {
                            nodes_to_visit[to_visit_offset] = child_offset;
                            to_visit_offset += 1;
                            current_node_index += 1;
                        }
                    },
                }
            } else {
                if to_visit_offset == 0 {
                    break;
                }

                to_visit_offset -= 1;
                current_node_index = nodes_to_visit[to_visit_offset];
            }
        }

        isect
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        let inv_dir = ray.direction.map(|d| float(1.0) / d);
        let dir_is_neg = [inv_dir.x < 0.0, inv_dir.y < 0.0, inv_dir.z < 0.0];

        // follow ray through the BVH nodes to find primitive intersections
        let mut to_visit_offset = 0;
        let mut current_node_index = 0;
        let mut nodes_to_visit = [0; 64];

        loop {
            let node = &self.nodes[current_node_index];
            // check ray against BVH node
            if node.bounds.intersect_p_precomputed(*ray, inv_dir, dir_is_neg) {
                match node.offset {
                    LinearBvhNodeOffset::Leaf(n_primitives, primitives_offset) => {
                        assert!(n_primitives > 0);
                        // intersect ray with primitives
                        for p in &self.primitives[primitives_offset..(primitives_offset + n_primitives)] {
                            if p.intersect_p(ray) {
                                return true;
                            }
                        }

                        if to_visit_offset == 0 {
                            break;
                        }

                        to_visit_offset -= 1;
                        current_node_index = nodes_to_visit[to_visit_offset];
                    },
                    LinearBvhNodeOffset::Interior(dir, child_offset) => {
                        // put far BVH node on the nodes_to_visit stack, advance to near node
                        if dir_is_neg[dir as usize] {
                            nodes_to_visit[to_visit_offset] = current_node_index + 1;
                            to_visit_offset += 1;
                            current_node_index = child_offset;
                        } else {
                            nodes_to_visit[to_visit_offset] = child_offset;
                            to_visit_offset += 1;
                            current_node_index += 1;
                        }
                    },
                }
            } else {
                if to_visit_offset == 0 {
                    break;
                }

                to_visit_offset -= 1;
                current_node_index = nodes_to_visit[to_visit_offset];
            }
        }

        false
    }

    fn world_bound(&self) -> Bounds3<Float> {
        self.nodes[0].bounds
    }
}

fn flatten_bvh_tree(nodes: &mut Vec<Option<LinearBvhNode>>, node: Arc<BvhBuildNode>, offset: &mut usize, total: usize) -> usize {
    let my_offset = *offset;
    *offset += 1;

    let linear_node = match &*node {
        BvhBuildNode::Leaf { bounds, first_primitive_offset, num_primitives } => {
            assert!(*num_primitives > 0);

            Some(LinearBvhNode {
                bounds: *bounds,
                offset: LinearBvhNodeOffset::Leaf(*num_primitives, *first_primitive_offset),
            })
        },
        BvhBuildNode::Interior { bounds, split_axis, children } => {
            // create interior flattened BVH node

            flatten_bvh_tree(nodes, children[0].clone(), offset, total);
            let offset = flatten_bvh_tree(nodes, children[1].clone(), offset, total);

            Some(LinearBvhNode {
                bounds: *bounds,
                offset: LinearBvhNodeOffset::Interior(*split_axis, offset),
            })
        },
    };

    nodes[my_offset] = linear_node;

    my_offset
}

fn hlbvh_build(primitives: &'a [Arc<dyn Primitive + Send>], primitive_info: &[BvhPrimitiveInfo], total_nodes: &mut usize, ordered_primitives: &'a mut Vec<Arc<dyn Primitive + Send>>, arena: &()) -> Arc<BvhBuildNode> {
    // compute bounding box of all primitive centroids
    let mut bounds = Bounds3f::empty();
    for p in primitive_info.iter() {
        bounds = bounds.union_p(p.centroid);
    }

    // compute morton indices of primitives
    let mut morton_prims = primitive_info.par_chunks(512)
        .map(|p| p.iter().map(|p| {
            let centroid = bounds.offset(p.centroid);
            let centroid = centroid.map(|c| c.raw() as u32);
            let centroid = centroid * MORTON_SCALE;

            MortonPrimitive {
                index: p.num,
                code: morton::encode_3d(centroid.x, centroid.y, centroid.z),
            }
        }).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();

    // radix sort morton indices
    radix_sort(&mut morton_prims);

    // create LBVH treelets at the bottom of the BVH
    struct LbvhTreelet {
        start: usize,
        n: usize,
        nodes: Vec<Arc<BvhBuildNode>>,
    }

    let mut treelets_to_build = vec![];
    let mut start = 0;
    for end in 1..=morton_prims.len() {
        let mask = 0b0011_1111_1111_1100_0000_0000_0000_0000;
        if end == morton_prims.len() ||
            (morton_prims[start].code & mask) !=
            (morton_prims[end].code & mask) {
            // add entry to `treelets_to_build` for this treelet
            let n_prims = end - start;
            let max_nodes = 2 * n_prims;
            let nodes = Vec::with_capacity(max_nodes);
            treelets_to_build.push(LbvhTreelet {
                start,
                n: n_prims,
                nodes,
            });

            start = end;
        }
    }

    // create treelets in parallel
    let total = AtomicUsize::new(0);
    let ordered_offset = AtomicUsize::new(0);

    #[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
    fn emit_lbvh(build_nodes: &mut Vec<Arc<BvhBuildNode>>, primitive_info: &[BvhPrimitiveInfo], morton_prims: &[MortonPrimitive], n_primitives: usize, total_nodes: &mut usize, ordered_primitives: Arc<Mutex<&mut Vec<Option<Arc<dyn Primitive + Send>>>>>, primitives: &[Arc<dyn Primitive + Send>], offset: &AtomicUsize, bit_index: i32) -> Arc<BvhBuildNode> {
        if bit_index == -1 || n_primitives < MAX_PRIMITIVES_IN_NODE {
            // create and return leaf node for treelet
            *total_nodes += 1;
            let mut bounds = Bounds3f::empty();
            let first_primitive_offset = offset.fetch_add(n_primitives, Ordering::SeqCst);

            let mut ordered_primitives = ordered_primitives.lock().unwrap();
            for i in 0..n_primitives {
                let idx = morton_prims[i].index;
                ordered_primitives[first_primitive_offset + i] = Some(primitives[idx].clone());
                bounds = bounds.union(primitive_info[idx].bounds);
            }

            let node = BvhBuildNode::init_leaf(first_primitive_offset, n_primitives, bounds);
            let node = Arc::new(node);
            build_nodes.push(node.clone());
            node
        } else {
            let mask = 1 << bit_index;

            // advance to next subtree level if there's no split for this bit
            if (morton_prims[0].code & mask) == (morton_prims[n_primitives - 1].code & mask) {
                return emit_lbvh(build_nodes, primitive_info, morton_prims, n_primitives, total_nodes, ordered_primitives, primitives, offset, bit_index - 1);
            }

            // find split point for this dimension
            let mut search_start = 0;
            let mut search_end = n_primitives - 1;
            while search_start + 1 != search_end {
                let mid = (search_start + search_end) / 2;

                if (morton_prims[search_start].code & mask) == (morton_prims[mid].code & mask) {
                    search_start = mid;
                } else {
                    search_end = mid;
                }
            }
            let split_offset = search_end;

            // create and return interior node
            *total_nodes += 1;

            let axis = match Dim::try_from(bit_index as usize % 3) {
                Ok(a) => a,
                Err(_) => unreachable!(),
            };

            let node = BvhBuildNode::init_interior(
                axis,
                emit_lbvh(
                    build_nodes, primitive_info,
                    morton_prims, split_offset, total_nodes,
                    ordered_primitives.clone(), primitives, offset,
                    bit_index - 1),
                emit_lbvh(
                    build_nodes, primitive_info,
                    &morton_prims[split_offset..], n_primitives - split_offset, total_nodes,
                    ordered_primitives, primitives, offset,
                    bit_index - 1),
            );
            let node = Arc::new(node);

            build_nodes.push(node.clone());

            node
        }
    }

    let mut op = vec![None; primitives.len()];
    let op = Arc::new(Mutex::new(&mut op));

    treelets_to_build.par_iter_mut()
        .for_each(|tr| {
            let mut created = 0;
            // 29 is the index of the 30th bit, 12 is how many bits we used to cluster the primitives
            let first_bit_index = 29 - 12;
            emit_lbvh(
                &mut tr.nodes,
                primitive_info,
                &morton_prims[tr.start..(tr.start + tr.n)],
                tr.n,
                &mut created,
                op.clone(),
                primitives,
                &ordered_offset,
                first_bit_index,
            );
            total.fetch_add(created, Ordering::SeqCst);
        });
    *total_nodes += total.load(Ordering::SeqCst);

    let op = Arc::try_unwrap(op).unwrap();
    let op = op.into_inner().unwrap();
    *ordered_primitives = op.clone().into_iter().map(|p| p.unwrap()).collect();

    fn build_upper_sah(treelet_roots: &mut [Arc<BvhBuildNode>], start: usize, end: usize, total_nodes: &mut usize, arena: &()) -> Arc<BvhBuildNode> {
        assert!(start < end);

        let n_nodes = end - start;
        if n_nodes == 1 {
            return treelet_roots[start].clone();
        }

        *total_nodes += 1;

        // compute bounds of all nodes
        let mut bounds = Bounds3f::empty();
        for treelet in &treelet_roots[start..end] {
            bounds = bounds.union(treelet.bounds());
        }

        // compute bounds of centroids and choose split dimensio
        let mut centroid_bounds = Bounds3f::empty();
        for treelet in &treelet_roots[start..end] {
            let centroid = (treelet.bounds().min + treelet.bounds().max.into_vector()) * float(0.5);
            centroid_bounds = centroid_bounds.union_p(centroid);
        }
        let dim = centroid_bounds.maximum_extent();

        assert_ne!(centroid_bounds.min[dim as usize], centroid_bounds.max[dim as usize]);

        // init BucketInfo for HLBVH SAH partition buckets
        const N_BUCKETS: usize = 12;

        #[derive(Copy, Clone, Debug)]
        struct BucketInfo {
            count: i32,
            bounds: Bounds3f,
        }

        let mut buckets = [BucketInfo { count: 0, bounds: Bounds3f::empty() }; N_BUCKETS];
        for treelet in &treelet_roots[start..end] {
            let centroid = (treelet.bounds().min[dim as usize] + treelet.bounds().max[dim as usize]) * float(0.5);
            let mut b = ((centroid - centroid_bounds.min[dim as usize]) / (centroid_bounds.max[dim as usize] - centroid_bounds.min[dim as usize])).raw() as usize * N_BUCKETS;

            if b == N_BUCKETS {
                b = N_BUCKETS - 1;
            }

            assert!(b > 0);
            assert!(b < N_BUCKETS);

            buckets[b].count += 1;
            buckets[b].bounds = buckets[b].bounds.union(treelet.bounds());
        }

        // compute costs for splitting after each bucket
        // traversal cost is set to 8x less than intersection cost
        // because the virtual dispatch used for intersections
        // intersection cost is set to 1 for ease of calculations
        // Ray -> Aggregate -> Shape -> Primitive
        let traversal_cost = float(0.125);
        let mut cost = [float(0.0); N_BUCKETS - 1];
        for i in 0..(N_BUCKETS - 1) {
            let mut b0 = Bounds3f::empty();
            let mut b1 = Bounds3f::empty();

            let mut count0 = 0;
            let mut count1 = 0;

            for bucket in &buckets[0..=i] {
                b0 = b0.union(bucket.bounds);
                count0 += bucket.count;
            }

            for bucket in &buckets[(i + 1)..N_BUCKETS] {
                b1 = b1.union(bucket.bounds);
                count1 += bucket.count;
            }

            cost[i] = traversal_cost + (
                float(count0) * b0.surface_area() +
                float(count1) * b1.surface_area()
            ) / bounds.surface_area();
        }

        // find bucket to split at where it'll minimise SAH metric
        let mut min_cost = cost[0];
        let mut min_cost_split_bucket = 0;
        for (i, cost) in cost.iter().enumerate().skip(1) {
            if *cost < min_cost {
                min_cost = *cost;
                min_cost_split_bucket = i;
            }
        }

        // split nodes and create interior nodes
        let mid = itertools::partition(&mut treelet_roots[start..end], |node| {
            let centroid = (node.bounds().min[dim as usize] + node.bounds().max[dim as usize]) * float(0.5);
            let mut b = ((centroid - centroid_bounds.min[dim as usize]) / (centroid_bounds.max[dim as usize] - centroid_bounds.min[dim as usize])).raw() as usize / N_BUCKETS;

            if b == N_BUCKETS {
                b = N_BUCKETS - 1;
            }

            assert!(b > 0);
            assert!(b < N_BUCKETS);

            b <= min_cost_split_bucket
        });

        assert!(mid > start);
        assert!(mid < end);

        let node = BvhBuildNode::init_interior(
            dim,
            build_upper_sah(treelet_roots, start, mid, total_nodes, arena),
            build_upper_sah(treelet_roots, mid, end, total_nodes, arena),
        );

        Arc::new(node)
    }

    // create SAH BVH from LBVH treelets
    let mut finished_treelets = treelets_to_build.into_iter()
        .flat_map(|t| t.nodes)
        .collect::<Vec<_>>();

    let len = finished_treelets.len();
    build_upper_sah(&mut finished_treelets, 0, len, total_nodes, arena)
}

fn recursive_build(primitives: &'a [Arc<dyn Primitive + Send>], primitive_info: &[BvhPrimitiveInfo], split_method: SplitMethod, range: Range<usize>, total_nodes: &mut usize, ordered_primitives: &'a mut Vec<Arc<dyn Primitive + Send>>, arena: &()) -> Arc<BvhBuildNode> {
    assert!(!primitive_info.is_empty());

    let Range { start, end } = range;

    *total_nodes += 1;

    // compute bounds of all primitives in Bvh node
    let mut bounds = primitive_info[0].bounds;
    for p in primitive_info.iter().skip(1) {
        bounds = bounds.union(p.bounds);
    }

    let n_primitives = end - start;

    let mut create_leaf = || {
        let first_offset = ordered_primitives.len();
        for primitive_info in &primitive_info[start..end] {
            let num = primitive_info.num;
            ordered_primitives.push(primitives[num].clone());
        }
        Arc::new(BvhBuildNode::init_leaf(first_offset, n_primitives, bounds))
    };

    if n_primitives == 1 {
        // create leaf node
        create_leaf()
    } else {
        // compute bound of primitive centroids, choose split dimension `dim`
        let mut centroid = primitive_info[start].bounds;
        for p in primitive_info[start..end].iter().skip(1) {
            centroid = centroid.union_p(p.centroid);
        }
        let dim = centroid.maximum_extent();

        // partition primitives into two sets and build children
        let mut mid = (start + end) / 2;

        if centroid.max[dim as usize] == centroid.min[dim as usize] {
            // if all the centroid points are at the same position
            // (the bounds has zero volume)
            // then we can't split it effectively, so just make a leaf node
            // this is an unusual case!
            create_leaf()
        } else {
            // partition primitives based on `split_method`
            let mut primitive_info = primitive_info[start..end].to_vec();
            match split_method {
                SplitMethod::Middle => {
                    let p_mid: Float = (centroid.min[dim as usize] + centroid.max[dim as usize]) / float(2.0);
                    mid = itertools::partition(&mut primitive_info, |p| p.centroid[dim as usize] < p_mid);
                    if mid == start || mid == end {
                        // if the primitives all have overlapping boxes,
                        // then this splitting method doesn't work properly
                        // so use `EqualCounts`
                        partition_by(&mut primitive_info, |p| p.centroid[dim as usize]);
                    }
                },
                SplitMethod::EqualCounts => {
                    partition_by(&mut primitive_info, |p| p.centroid[dim as usize]);
                },
                SplitMethod::SAH => {
                    if n_primitives <= 2 {
                        // splitting small amounts using SAH isn't worth the cost
                        // so just use `EqualCounts`
                        partition_by(&mut primitive_info, |p| p.centroid[dim as usize]);
                    } else {
                        // allocate `BucketInfo` for SAH partition buckets
                        const N_BUCKETS: usize = 12;

                        #[derive(Copy, Clone, Debug)]
                        struct BucketInfo {
                            count: i32,
                            bounds: Bounds3f,
                        }

                        let mut buckets: [BucketInfo; N_BUCKETS] = [
                            BucketInfo { count: 0, bounds: Bounds3f::empty() };
                            N_BUCKETS
                        ];

                        // init `BucketInfo`
                        for prim in &primitive_info[start..end] {
                            let b = centroid.offset(prim.centroid)[dim as usize] * float(N_BUCKETS);
                            let mut b = b.raw() as usize;

                            if b == N_BUCKETS {
                                b = N_BUCKETS - 1;
                            }

                            buckets[b].count += 1;
                            buckets[b].bounds = buckets[b].bounds.union(prim.bounds);
                        }

                        // compute costs for splitting after each bucket
                        // traversal cost is set to 8x less than intersection cost
                        // because the virtual dispatch used for intersections
                        // intersection cost is set to 1 for ease of calculations
                        // Ray -> Aggregate -> Shape -> Primitive
                        let traversal_cost = float(1.0);
                        let mut cost = [float(0.0); N_BUCKETS - 1];
                        for i in 0..(N_BUCKETS - 1) {
                            let mut b0 = Bounds3f::empty();
                            let mut b1 = Bounds3f::empty();

                            let mut count0 = 0;
                            let mut count1 = 0;

                            for bucket in &buckets[0..=i] {
                                b0 = b0.union(bucket.bounds);
                                count0 += bucket.count;
                            }

                            for bucket in &buckets[(i + 1)..N_BUCKETS] {
                                b1 = b1.union(bucket.bounds);
                                count1 += bucket.count;
                            }

                            cost[i] = traversal_cost + (
                                float(count0) * b0.surface_area() +
                                float(count1) * b1.surface_area()
                            ) / bounds.surface_area();
                        }

                        // find bucket to split at where it'll minimise SAH metric
                        let mut min_cost = cost[0];
                        let mut min_cost_split_bucket = 0;
                        for (i, cost) in cost.iter().enumerate().skip(1) {
                            if *cost < min_cost {
                                min_cost = *cost;
                                min_cost_split_bucket = i;
                            }
                        }

                        // create leaf or split primitives at this bucket
                        let leaf_cost = float(n_primitives);
                        if n_primitives > MAX_PRIMITIVES_IN_NODE || min_cost < leaf_cost {
                            mid = itertools::partition(&mut primitive_info, |p| {
                                let b = float(N_BUCKETS) * centroid.offset(p.centroid)[dim as usize];
                                let mut b = b.raw() as usize;

                                if b == N_BUCKETS {
                                    b = N_BUCKETS - 1;
                                }

                                b <= min_cost_split_bucket
                            });
                        } else {
                            create_leaf();
                        }
                    }
                },
                SplitMethod::HLBVH => unreachable!(),
            }

            Arc::new(BvhBuildNode::init_interior(dim,
                recursive_build(primitives, &primitive_info, split_method, (range.start)..mid, total_nodes, ordered_primitives, arena),
                recursive_build(primitives, &primitive_info, split_method, mid..(range.end), total_nodes, ordered_primitives, arena),
            ))
        }
    }
}

fn radix_sort(v: &mut Vec<MortonPrimitive>) {
    const BITS_PER_PASS: u32 = 6;
    const N_BITS: u32 = 30;
    const N_PASSES: u32 = N_BITS / BITS_PER_PASS;

    let mut temp = vec![MortonPrimitive { index: 0, code: 0 }; v.len()];

    for pass in 0..N_PASSES {
        // perform one pass of radix sort,
        // sorting `BITS_PER_PASS` bits
        let low_bit = pass * BITS_PER_PASS;

        // set in and out vec pointers for radix sort pass
        let (v_in, v_out) = if pass & 1 != 0 {
            (&mut temp, &mut *v)
        } else {
            (&mut *v, &mut temp)
        };

        // count number of zero bits in array for current radix sort bit
        const N_BUCKETS: u32 = 1 << BITS_PER_PASS;
        const BITMASK: u32 = (1 << BITS_PER_PASS) - 1;
        let mut bucket_count = [0u32; N_BUCKETS as usize];

        for mp in &*v_in {
            let bucket = (mp.code >> low_bit) & BITMASK;
            bucket_count[bucket as usize] += 1;
        }

        // compute starting index in output array for each bucket
        let mut out_index = [0u32; N_BUCKETS as usize];

        for i in 1..N_BUCKETS as usize {
            out_index[i] = out_index[i - 1] + bucket_count[i - 1];
        }

        // store sorted values in output array
        for mp in v_in {
            let bucket = (mp.code >> low_bit) & BITMASK;
            v_out[out_index[bucket as usize] as usize] = *mp;
            out_index[bucket as usize] += 1;
        }
    }

    // copy from `temp` if needed
    if N_PASSES & 1 != 0 {
        mem::swap(v, &mut temp);
    }
}
