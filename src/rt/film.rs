use ::rt::*;
use palette::pixel::Srgb;

#[derive(Clone, Debug)]
pub struct Photon {
    pub place: Pt2,
    pub light: Color
}

pub trait Film {
    fn size(&self) -> (usize, usize);

    fn raster_to_relative(&self, raster: Pt2) -> Pt2 {
        let (xmax, ymax) = self.size();
        Point2 {
            x: raster.x / xmax as f32,
            y: raster.y / ymax as f32
        }
    }

    fn relative_to_raster(&self, point: Pt2) -> Pt2 {
        let (xmax, ymax) = self.size();
        Point2 {
            x: point.x * xmax as f32,
            y: point.y * ymax as f32
        }
    }

    fn push_photon(&mut self, Photon);

    fn write_image(&self) -> Vec<[u8; 3]>;
}

pub struct SimpleFilm {
    size: (usize, usize),
    bins: Vec<(Color, usize)>
}

impl SimpleFilm {
    fn new(x: usize, y: usize) -> SimpleFilm {
        SimpleFilm {
            size: (x, y),
            bins: vec![(BLACK, 0); x * y]
        }
    }

    fn relative_to_index(&self, point: Pt2) -> usize {
        let raster = self.relative_to_raster(point);
        let (xmax, _) = self.size();
        let x_idx = raster.x.floor() as usize;
        let y_idx = raster.y.floor() as usize;
        x_idx * xmax + y_idx
    }
}

impl Film for SimpleFilm {
    fn size(&self) -> (usize, usize) {
        self.size
    }

    fn push_photon(&mut self, photon: Photon) {
        let index = self.relative_to_index(photon.place);
        let (light, count) = self.bins[index];
        self.bins[index] = (light + photon.light, count + 1)
    }

    fn write_image(&self) -> Vec<[u8; 3]> {
        let lights = self.bins.iter().map(|&(light, count)|
            if count == 0 {
                BLACK
            } else {
                light / count as f32
            });
        lights.map(|light| Srgb::from(light).to_pixel::<[u8; 3]>()).collect()
    }
}