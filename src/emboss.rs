pub mod emboss {
    pub fn apply_bump_map(
        image: image::RgbImage,
        bump: image::RgbImage,
        azimuth: f64,
        elevation: f64,
    ) -> Option<image::RgbImage> {
        let (width, height) = image.dimensions();
        if (width, height) != bump.dimensions() {
            return None;
        }

        let mut dst = image::RgbImage::new(width, height).into_raw();

        emboss(
            azimuth,
            elevation,
            3,
            &bump.into_raw(),
            &image.into_raw(),
            &mut dst,
            width as usize,
            height as usize,
        );

        image::RgbImage::from_raw(width, height, dst)
    }

    /*
     * ANSI C code from the article
     * "Fast Embossing Effects on Raster Image Data"
     * by John Schlag, jfs@kerner.com
     * in "Graphics Gems IV", Academic Press, 1994
     *
     *
     * Emboss - shade 24-bit pixels using a single distant light source.
     * Normals are obtained by differentiating a monochrome 'bump' image.
     * The unary case ('texture' == NULL) uses the shading result as output.
     * The binary case multiples the optional 'texture' image by the shade.
     * Images are in row major order with interleaved color components (rgbrgb...).
     * E.g., component c of pixel x,y of 'dst' is dst[3*(y*x_size + x) + c].
     * */
    pub fn emboss(
        azimuth: f64,
        elevation: f64, /* light source direction */
        width45: u16,   /* filter width */
        bump: &Vec<u8>, /* monochrome bump image */
        texture: &Vec<u8>,
        dst: &mut Vec<u8>, /* texture & output images */
        x_size: usize,
        y_size: usize,
    ) {
        let pixel_scale = 255.9;
        let mut n_x;
        let mut n_y;
        let mut shade: u8;

        /*
         * compute the light vector from the input parameters.
         * normalize the length to pixel_scale for fast shading calculation.
         */
        let l_x = (azimuth.cos() * elevation.cos() * pixel_scale) as i32;
        let l_y = (azimuth.sin() * elevation.cos() * pixel_scale) as i32;
        let l_z = (elevation.sin() * pixel_scale) as i32;

        /*
         * constant z component of image surface normal - this depends on the
         * image slope we wish to associate with an angle of 45 degrees, which
         * depends on the width of the filter used to produce the source image.
         */
        let n_z = (6 * 255) / (width45 as i32);
        let n_z2 = n_z * n_z;
        let n_zl_z = n_z * l_z;
        let mut n_dot_l: i32;

        /* optimization for vertical normals: L.[0 0 1] */
        let background = l_z as u8;

        let mut dst_offset = 0;
        let mut texture_offset = 0;
        let mut bump_offset: usize = 0;

        /* mung pixels, avoiding edge pixels */
        dst_offset += x_size * 3;
        texture_offset += x_size * 3;

        let mut s1_minus_offsetted_bump: usize;
        let mut s2_minus_offsetted_bump;
        let mut s3_minus_offsetted_bump;

        for y in 1..y_size - 1 {
            println!("{}", y);
            s1_minus_offsetted_bump = 1;
            s2_minus_offsetted_bump = s1_minus_offsetted_bump + x_size;
            s3_minus_offsetted_bump = s2_minus_offsetted_bump + x_size;
            dst_offset += 3;
            texture_offset += 3;

            for _ in 1..x_size - 1 {
                /*
                 * compute the normal from the bump map. the type of the expression
                 * before the cast is compiler dependent. in some cases the sum is
                 * unsigned, in others it is signed. ergo, cast to signed.
                 */

                n_x = (bump[bump_offset + s1_minus_offsetted_bump - 1]
                    + bump[bump_offset + s2_minus_offsetted_bump - 1]
                    + bump[bump_offset + s3_minus_offsetted_bump - 1]
                    - bump[bump_offset + s1_minus_offsetted_bump + 1]
                    - bump[bump_offset + s2_minus_offsetted_bump + 1]
                    - bump[bump_offset + s3_minus_offsetted_bump + 1]) as i32;
                n_y = (bump[bump_offset + s3_minus_offsetted_bump - 1]
                    + bump[bump_offset + s3_minus_offsetted_bump + 0]
                    + bump[bump_offset + s3_minus_offsetted_bump + 1]
                    - bump[bump_offset + s1_minus_offsetted_bump - 1]
                    - bump[bump_offset + s1_minus_offsetted_bump + 0]
                    - bump[bump_offset + s1_minus_offsetted_bump + 1]) as i32;

                /* shade with distant light source */
                if n_x == 0 && n_y == 0 {
                    shade = background;
                } else if {
                    n_dot_l = n_x * l_x + n_y * l_y + n_zl_z;
                    n_dot_l < 0
                } {
                    shade = 0;
                } else {
                    shade =
                        ((n_dot_l as f64) / ((n_x * n_x + n_y * n_y + n_z2) as f64).sqrt()) as u8;
                }

                /* do something with the shading result */

                dst[dst_offset] = ((texture[texture_offset] as i32 * shade as i32) >> 8) as u8;
                dst_offset += 1;
                texture_offset += 1;

                dst[dst_offset] = ((texture[texture_offset] as i32 * shade as i32) >> 8) as u8;
                dst_offset += 1;
                texture_offset += 1;

                dst[dst_offset] = ((texture[texture_offset] as i32 * shade as i32) >> 8) as u8;
                dst_offset += 1;
                texture_offset += 1;

                s1_minus_offsetted_bump += 1;
                s2_minus_offsetted_bump += 1;
                s3_minus_offsetted_bump += 1;
            }

            texture_offset += 3;

            bump_offset += x_size;
            dst_offset += 3;
        }
    }
}
