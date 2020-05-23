pub mod blur {
    extern crate image;
    use std::convert::TryInto;

    pub fn gaussian_blur_asymmetric(
        image: image::RgbImage,
        blur_radius_horizontal: f32,
        blur_radius_vertical: f32,
    ) -> Option<image::RgbImage> {
        let (width, height) = image.dimensions();
        let mut data = unflatten(&image.into_raw());
        fastblur::gaussian_blur_asymmetric(
            &mut data,
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            blur_radius_horizontal,
            blur_radius_vertical,
        );
        image::RgbImage::from_raw(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            flatten(&data),
        )
    }

    fn flatten(data: &Vec<[u8; 3]>) -> Vec<u8> {
        let mut a = vec![];
        for rgb in data.into_iter() {
            a.push(rgb[0]);
            a.push(rgb[1]);
            a.push(rgb[2]);
        }
        a
    }

    fn unflatten(data: &Vec<u8>) -> Vec<[u8;3]> {
        let iter = data.chunks(3);
        let mut a = vec![];
        for rgb in iter {
            // unwrap unwrap unwrap
            a.push([rgb[0], rgb[1], rgb[2]]);
        }
        a
    }
}