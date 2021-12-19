use crate::rotation::{RotationMatrix, ORIENTATIONS};
use crate::vector::Vector3d;
use std::collections::HashSet;
use std::str::FromStr;

pub fn from_lines(
    lines: impl Iterator<Item = String>,
) -> Result<Vec<PointCloud>, Box<dyn std::error::Error>> {
    let mut point_clouds = Vec::new();
    let mut collected_points = Vec::new();

    for line in lines {
        if line.starts_with("---") {
            continue;
        }

        if line.is_empty() {
            point_clouds.push(PointCloud {
                points: collected_points.clone(),
            });
            collected_points.clear();
        } else {
            collected_points.push(Vector3d::from_str(line.as_str())?);
        }
    }

    // Add any trailing lines
    if !collected_points.is_empty() {
        point_clouds.push(PointCloud {
            points: collected_points.clone(),
        });
    }

    Ok(point_clouds)
}

#[derive(Debug, Eq, PartialEq)]
pub struct PointCloud {
    points: Vec<Vector3d>,
}

impl PointCloud {
    pub fn translate(&self, delta: Vector3d) -> Self {
        PointCloud {
            points: self.points.iter().map(|&point| point + delta).collect(),
        }
    }

    pub fn rotate(&self, rotation: &RotationMatrix) -> Self {
        PointCloud {
            points: self
                .points
                .iter()
                .map(|point| point.rotate(rotation))
                .collect(),
        }
    }

    pub fn overlap_transform(
        &self,
        other: &PointCloud,
        n: usize,
    ) -> Option<(RotationMatrix, Vector3d)> {
        let other_point_set = other.points.iter().collect();

        for orientation in ORIENTATIONS {
            let rotated = self.rotate(&orientation);

            // For each point in this cloud in this orientation, hypothesize that the point is the
            // same as each point in the other cloud and see if we find alignment.
            for &local_point in &rotated.points {
                for &other_point in &other.points {
                    let offset = other_point - local_point;
                    let translated = rotated.translate(offset);

                    let translated_set: HashSet<&Vector3d> = translated.points.iter().collect();
                    let common_points = translated_set.intersection(&other_point_set);

                    if common_points.count() >= n {
                        return Some((orientation, offset));
                    }
                }
            }
        }

        None
    }

    pub fn common_points(
        &self,
        other: &PointCloud,
        rotation: &RotationMatrix,
        translation: Vector3d,
    ) -> HashSet<Vector3d> {
        let transformed_set: HashSet<Vector3d> = self
            .rotate(rotation)
            .translate(translation)
            .points
            .into_iter()
            .collect();

        transformed_set
            .intersection(&other.points.iter().cloned().collect())
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::point_cloud::{from_lines, PointCloud};
    use crate::rotation::ORIENTATIONS;
    use crate::vector::Vector3d;
    use indoc::indoc;
    use std::collections::HashSet;

    const TEST_SCANNER_STRING: &str = indoc! {"
        --- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
    "};

    #[test]
    fn test_from_lines() {
        assert_eq!(5, from_lines(TEST_SCANNER_STRING.lines()).unwrap().len());
    }

    #[test]
    fn test_translate() {
        assert_eq!(
            PointCloud {
                points: vec![
                    Vector3d::new(2, 3, 4),
                    Vector3d::new(3, 4, 5),
                    Vector3d::new(4, 5, 6),
                ],
            },
            PointCloud {
                points: vec![
                    Vector3d::new(1, 1, 1),
                    Vector3d::new(2, 2, 2),
                    Vector3d::new(3, 3, 3),
                ],
            }
            .translate(Vector3d::new(1, 2, 3))
        )
    }

    #[test]
    fn test_rotate() {
        assert_eq!(
            PointCloud {
                points: vec![
                    Vector3d::new(0, 1, 0),
                    Vector3d::new(-1, 0, 0),
                    Vector3d::new(0, 0, 1),
                ],
            },
            PointCloud {
                points: vec![
                    Vector3d::new(1, 0, 0),
                    Vector3d::new(0, 1, 0),
                    Vector3d::new(0, 0, 1),
                ],
            }
            .rotate(&ORIENTATIONS[1])
        )
    }

    #[test]
    fn test_overlap_transform_2d() {
        let scanner_0_cloud = PointCloud {
            points: vec![
                Vector3d::new(0, 2, 0),
                Vector3d::new(4, 1, 0),
                Vector3d::new(3, 3, 0),
            ],
        };

        let scanner_1_cloud = PointCloud {
            points: vec![
                Vector3d::new(-1, -1, 0),
                Vector3d::new(-5, 0, 0),
                Vector3d::new(-2, 1, 0),
            ],
        };

        let (rotation, translation) = scanner_1_cloud
            .overlap_transform(&scanner_0_cloud, 3)
            .unwrap();

        // In this example, no rotation should be required
        assert_eq!(ORIENTATIONS[0], rotation);
        assert_eq!(Vector3d::new(5, 2, 0), translation);

        assert!(scanner_1_cloud
            .overlap_transform(&scanner_0_cloud, 4)
            .is_none());
    }

    #[test]
    fn test_overlap_transform_3d() {
        let point_clouds = from_lines(TEST_SCANNER_STRING.lines()).unwrap();

        let expected_common_points: HashSet<Vector3d> = vec![
            Vector3d::new(-618, -824, -621),
            Vector3d::new(-537, -823, -458),
            Vector3d::new(-447, -329, 318),
            Vector3d::new(404, -588, -901),
            Vector3d::new(544, -627, -890),
            Vector3d::new(528, -643, 409),
            Vector3d::new(-661, -816, -575),
            Vector3d::new(390, -675, -793),
            Vector3d::new(423, -701, 434),
            Vector3d::new(-345, -311, 381),
            Vector3d::new(459, -707, 401),
            Vector3d::new(-485, -357, 347),
        ]
        .into_iter()
        .collect();

        let (rotation, translation) = point_clouds[1]
            .overlap_transform(&point_clouds[0], 12)
            .unwrap();

        assert_eq!(Vector3d::new(68, -1246, -43), translation);
        assert_eq!(
            expected_common_points,
            point_clouds[1].common_points(&point_clouds[0], &rotation, translation)
        );
    }
}
