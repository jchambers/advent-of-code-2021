use self::Pixel::{Dark, Light};
use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let (algorithm, image) =
            algorithm_and_image_from_str(std::fs::read_to_string(path)?.as_str()).unwrap();

        println!(
            "Light pixels in enhanced image after 2 steps: {}",
            image.enhance(&algorithm, 2).light_pixel_count()
        );

        println!(
            "Light pixels in enhanced image after 50 steps: {}",
            image.enhance(&algorithm, 50).light_pixel_count()
        );

        Ok(())
    } else {
        Err("Usage: day20 INPUT_FILE_PATH".into())
    }
}

fn algorithm_and_image_from_str(
    string: &str,
) -> Result<(EnhancementAlgorithm, Image), Box<dyn error::Error>> {
    let mut pieces = string.splitn(2, "\n\n");

    let enhancement_algorithm = EnhancementAlgorithm::from_str(pieces.next().unwrap())?;
    let image = Image::from_str(pieces.next().unwrap())?;

    Ok((enhancement_algorithm, image))
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pixel {
    Light,
    Dark,
}

struct EnhancementAlgorithm {
    rules: [Pixel; 512],
}

impl EnhancementAlgorithm {
    const MAX: usize = 511;

    pub fn pixel(&self, index: usize) -> Pixel {
        self.rules[index]
    }
}

impl FromStr for EnhancementAlgorithm {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let rules = string
            .chars()
            .map(|c| match c {
                '#' => Light,
                '.' => Dark,
                _ => unreachable!(),
            })
            .collect::<Vec<Pixel>>()
            .try_into()
            .unwrap();

        Ok(EnhancementAlgorithm { rules })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Image {
    width: usize,
    height: usize,

    pixels: Vec<Pixel>,
    infinite_pixel: Pixel,
}

impl Image {
    pub fn enhance(&self, enhancement_algorithm: &EnhancementAlgorithm, levels: usize) -> Self {
        let mut enhanced = self.clone();

        for _ in 0..levels {
            enhanced = enhanced.next(enhancement_algorithm);
        }

        enhanced
    }

    fn next(&self, enhancement_algorithm: &EnhancementAlgorithm) -> Self {
        let mut enhanced_pixels = vec![Dark; (self.width + 2) * (self.height + 2)];

        let enhanced_width = self.width + 2;
        let enhanced_height = self.height + 2;

        for y in 0..enhanced_height {
            for x in 0..enhanced_width {
                // Note that x and y here correspond to positions in the enhanced image, which will
                // be offset by 1 in each direction within this image (i.e. we're adding a border of
                // 1 pixel the whole way around).
                enhanced_pixels[(y * enhanced_width) + x] = enhancement_algorithm
                    .pixel(self.enhancement_index(x as isize - 1, y as isize - 1));
            }
        }

        let enhanced_infinite_pixel = match self.infinite_pixel {
            Light => enhancement_algorithm.pixel(EnhancementAlgorithm::MAX),
            Dark => enhancement_algorithm.pixel(0),
        };

        Image {
            width: enhanced_width,
            height: enhanced_height,
            pixels: enhanced_pixels,
            infinite_pixel: enhanced_infinite_pixel,
        }
    }

    pub fn light_pixel_count(&self) -> usize {
        match self.infinite_pixel {
            Light => usize::MAX,
            Dark => self.pixels.iter().filter(|pixel| pixel == &&Light).count(),
        }
    }

    fn pixel(&self, x: isize, y: isize) -> Pixel {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            self.infinite_pixel
        } else {
            self.pixels[(y as usize * self.width) + x as usize]
        }
    }

    fn enhancement_index(&self, x: isize, y: isize) -> usize {
        let mut index = 0;

        for y_n in y - 1..=y + 1 {
            for x_n in x - 1..=x + 1 {
                index <<= 1;

                if self.pixel(x_n, y_n) == Light {
                    index |= 1;
                }
            }
        }

        index
    }
}

impl FromStr for Image {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let pixels: Vec<Pixel> = string
            .chars()
            .filter_map(|c| match c {
                '#' => Some(Light),
                '.' => Some(Dark),
                _ => None,
            })
            .collect();

        let width = string.find('\n').unwrap();

        if pixels.len() % width == 0 {
            let height = pixels.len() / width;

            Ok(Image {
                width,
                height,
                pixels,
                infinite_pixel: Dark,
            })
        } else {
            Err("Bad image string length".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_IMAGE_STRING: &str = indoc! {"
        #..#.
        #....
        ##..#
        ..#..
        ..###
    "};

    const TEST_ALGORITHM_STRING: &str =
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...\
        ####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#.\
        .#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#\
        ..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.#\
        #...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####\
        ..#...#.#.#...##..#.#..###..#####........#..####......#..#";

    #[test]
    fn test_enhancement_algorithm_from_string() {
        let enhancement_algorithm = EnhancementAlgorithm::from_str(TEST_ALGORITHM_STRING).unwrap();

        assert_eq!(Dark, enhancement_algorithm.rules[0]);
        assert_eq!(Light, enhancement_algorithm.rules[10]);
        assert_eq!(Light, enhancement_algorithm.rules[20]);
        assert_eq!(Light, enhancement_algorithm.rules[30]);
        assert_eq!(Light, enhancement_algorithm.rules[40]);
        assert_eq!(Light, enhancement_algorithm.rules[50]);
        assert_eq!(Dark, enhancement_algorithm.rules[60]);
        assert_eq!(Dark, enhancement_algorithm.rules[70]);

        assert_eq!(Light, enhancement_algorithm.rules[34]);
    }

    #[test]
    fn test_image_from_string() {
        let image = Image::from_str(TEST_IMAGE_STRING).unwrap();

        assert_eq!(5, image.width);
        assert_eq!(5, image.height);
        assert_eq!(Dark, image.infinite_pixel);

        assert!(image
            .pixels
            .starts_with(&[Light, Dark, Dark, Light, Dark, Light, Dark]));
    }

    #[test]
    fn test_pixel() {
        let image = Image::from_str(TEST_IMAGE_STRING).unwrap();

        assert_eq!(Dark, image.pixel(-1, 0));
        assert_eq!(Dark, image.pixel(0, -1));
        assert_eq!(Dark, image.pixel(isize::MAX, 0));
        assert_eq!(Dark, image.pixel(0, isize::MAX));

        assert_eq!(Light, image.pixel(0, 0));
        assert_eq!(Dark, image.pixel(1, 0));
        assert_eq!(Light, image.pixel(0, 1));
    }

    #[test]
    fn test_enhancement_index() {
        let image = Image::from_str(TEST_IMAGE_STRING).unwrap();

        assert_eq!(34, image.enhancement_index(2, 2));
    }

    #[test]
    fn test_enhance() {
        let enhancement_algorithm = EnhancementAlgorithm::from_str(TEST_ALGORITHM_STRING).unwrap();
        let enhanced_image = Image::from_str(TEST_IMAGE_STRING)
            .unwrap()
            .enhance(&enhancement_algorithm, 1);

        let expected_enhanced_image = Image::from_str(indoc! {"
            .##.##.
            #..#.#.
            ##.#..#
            ####..#
            .#..##.
            ..##..#
            ...#.#.
        "})
        .unwrap();

        assert_eq!(expected_enhanced_image, enhanced_image);
    }

    #[test]
    fn test_light_pixel_count() {
        let enhancement_algorithm = EnhancementAlgorithm::from_str(TEST_ALGORITHM_STRING).unwrap();
        let image = Image::from_str(TEST_IMAGE_STRING).unwrap();

        assert_eq!(35, image.enhance(&enhancement_algorithm, 2).light_pixel_count());
        assert_eq!(3351, image.enhance(&enhancement_algorithm, 50).light_pixel_count());
    }
}
