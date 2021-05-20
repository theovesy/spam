// Everything linked to outputing the image

pub fn upscale(&img: image::ImageBuffer<P, Container>, factor: u32) -> image::ImageBuffer<P, Container>
    where
        P: Pixel + 'static,
        P::Subpixel: 'static,
        Container: Deref<Target = [P::Subpixel]>
    {
    let dimensions = img.dimensions();
    let x = factor * dimensions[0];
    let y = factor * dimensions[1];

    image::imageops::resize(&img, x, y, image::imageops::Nearest)
}

pub fn save(&img: image::ImageBuffer<P, Container>, name: str, factor: u32)
    where
        P: Pixel + 'static,
        P::Subpixel: 'static,
        Container: Deref<Target = [P::Subpixel]>
    {
    let img = upscale(&img, factor);
    img.save(name).unwrap();
}

