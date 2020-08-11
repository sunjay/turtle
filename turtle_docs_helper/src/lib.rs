use std::path::Path; 
 


 /// Saves the image being drawn as an SVG and panics if an error occurs
 ///
 /// This is different from the `save_svg` method on `Drawing` and `AsyncDrawing`
 /// because this is only meant to be used for automation and may need to access
 /// internal APIs.
 pub trait SaveSvg {
    fn save_svg(&self, path: &Path);
}



/// Saves the currently drawn image to `docs/assets/images/docs/{output_name}`
pub fn save_docs_image<T: SaveSvg>(drawing: &T, output_name: &str) {
    println!("saving image");
    let svg_path = &Path::new("docs/assets/images/docs").join(output_name).with_extension("svg");
    drawing.save_svg(svg_path);
    assert_eq!(1,2);
}
