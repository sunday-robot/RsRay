use crate::types::BvhTrait;
use crate::types::Hittable;
use crate::types::BvhLeaf;
use crate::types::BvhNode;
use crate::types::Aabb;
use crate::types::aabb;
use crate::utils::rand::rand_i32;

pub fn create(objects: Vec<Box<dyn Hittable>>, exposure_time: f64) -> Box<dyn Hittable> {
    let bvh = create_core(objects, exposure_time, 0, objects.len());
}

fn create_core(mut objects: Vec<Box<dyn Hittable>>, exposure_time: f64, start: usize, end: usize) -> Box<dyn BvhTrait> {
    let object_span = end - start;

    // if object_span <= 0
    // {
    //     throw new ArgumentException("empty list.");
    // }

    if object_span == 1 {
        let o = objects[start];
        return Box::new(BvhLeaf::new(o.bounding_box(exposure_time), o));
    } else {
        let comparator = get_comparer_randomly();
        let objects_sub = &objects[start..end];
        objects_sub.sort_by(|a,b| comparator(*a, *b));
        let mid = (start + end) / 2;
        let left = create_core(objects, exposure_time, start, mid);
        let right = create_core(objects, exposure_time, mid, end);
        let aabb = aabb::surrounding_aabb(
            left.bounding_box(exposure_time),
            right.bounding_box(exposure_time),
        );
        return Box::new(BvhNode::new(aabb, left, right));
    }
}

fn box_compare_x(a:Box<dyn Hittable>, b:Box<dyn Hittable>) -> std::cmp::Ordering
{
    let (box_a, box_b) = box_compare_sub(a, b);
    return box_b.min.x.partial_cmp(&box_a.min.x).unwrap();
}

fn box_compare_y(a:Box<dyn Hittable>, b:Box<dyn Hittable>) -> std::cmp::Ordering
{
    let (box_a, box_b) = box_compare_sub(a, b);
    return box_b.min.y.partial_cmp(&box_a.min.y).unwrap();
}

fn box_compare_z(a:Box<dyn Hittable>, b:Box<dyn Hittable>) -> std::cmp::Ordering
{
    let (box_a, box_b) = box_compare_sub(a, b);
    return box_b.min.z.partial_cmp(&box_a.min.z).unwrap();
}

// static readonly IComparer<Hittable> _boxCompareY = Comparer<Hittable>.Create((a, b) =>
// {
//     BoxCompareSub(a, b, out var boxA, out var boxB);
//     return boxB.Min.Y.CompareTo(boxA.Min.Y);
// });

// static readonly IComparer<Hittable> _boxCompareZ = Comparer<Hittable>.Create((a, b) =>
// {
//     BoxCompareSub(a, b, out var boxA, out var boxB);
//     return boxB.Min.Z.CompareTo(boxA.Min.Z);
// });

fn get_comparer_randomly()->fn (Box<dyn Hittable>, Box<dyn Hittable>)->std::cmp::Ordering
{
   return match rand_i32(3) {
        0 => box_compare_x,
        1 => box_compare_y,
        _ => box_compare_z,
    };
}

fn box_compare_sub(a:Box<dyn  Hittable>, b:Box<dyn Hittable>) -> (Aabb, Aabb)
{
    (a.bounding_box(0.0), b.bounding_box(0.0))
}
