use crate::triangle::{
    TriangleKind,
    Triangle,
    Result
};

#[test]
#[should_panic]
fn invalid_triangle_1() {
    let invalid_triplet = (1, 10, 12);
    Triangle::from_triplet(invalid_triplet).unwrap();
}

#[test]
#[should_panic]
fn invalid_triangle_2() {
    let invalid_triplet = (3, 5, 13);
    Triangle::from_triplet(invalid_triplet).unwrap();
}

#[test]
#[should_panic]
fn invalid_triangle_3() {
    let invalid_triplet = (30, 2, 50);
    Triangle::from_triplet(invalid_triplet).unwrap();
}

#[test]
fn isosceles_triangles() -> Result<()> {
    let triplets = vec![
        (6, 2, 6),
        (5, 5, 9),
        (14, 9, 9),
    ];

    // Make triangles from triplets and fail if any of them is invalid
    let triangles = triplets.into_iter().map(Triangle::from_triplet).collect::<Result<Vec<_>>>()?;
    
    // Make sure all triangles are isosceles
    assert!(triangles.iter().all(|x| x.kind == TriangleKind::Isosceles));

    Ok(())
}

#[test]
#[should_panic]
fn not_isosceles_triangles() {
    let triplets = vec![
        (7, 2, 6),
        (5, 6, 9),
        (14, 8, 9),
    ];

    // Make triangles from triplets and fail if any of them is invalid
    let triangles = triplets.into_iter().map(Triangle::from_triplet).collect::<Result<Vec<_>>>().unwrap();
    
    // Make sure all triangles are isosceles
    assert!(triangles.iter().all(|tr| tr.kind == TriangleKind::Isosceles));
}

#[test]
fn equilateral_triangles() -> Result<()> {
    let triplets = vec![
        (2, 2, 2),
        (3, 3, 3),
        (4, 4, 4),
        (5, 5, 5),
        (6, 6, 6)
    ];
    
    // Make triangles from triplets and fail if any of them is invalid
    let triangles = triplets.into_iter().map(Triangle::from_triplet).collect::<Result<Vec<_>>>()?;
    
    assert!(triangles.iter().all(|tr| tr.kind == TriangleKind::Equilateral));

    Ok(())
}

#[test]
#[should_panic]
fn not_equilateral_triangles() {
    let triplets = vec![
        (3, 2, 2),
        (3, 4, 3),
        (4, 4, 5),
        (6, 5, 5),
        (6, 7, 6)
    ];
    
    // Make triangles from triplets and fail if any of them is invalid
    let triangles = triplets.into_iter().map(Triangle::from_triplet).collect::<Result<Vec<_>>>().unwrap();
    
    assert!(triangles.iter().all(|tr| tr.kind == TriangleKind::Equilateral));
}

#[test]
fn scalene_triangles() -> Result<()> {
    let triplets = vec![
        (2, 3, 4),
        (5, 6, 7),
        (6, 7, 8),
        (2, 5, 6),
        (3, 4, 6)
    ];
    
    // Make triangles from triplets and fail if any of them is invalid
    let triangles = triplets.into_iter().map(Triangle::from_triplet).collect::<Result<Vec<_>>>()?;
    
    assert!(triangles.iter().all(|tr| tr.kind == TriangleKind::Scalene));

    Ok(())
}
