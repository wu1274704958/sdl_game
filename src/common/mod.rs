
#[macro_export]
macro_rules! create_texture {
    ($path:expr,$tc:ident) => {{
        let surface =  match Surface::from_file($path) {
            Ok(sur) => sur,
            Err(err) => panic!("failed to load {} surface err : {}", $path,err)
        };
        let texture = match  $tc.create_texture_from_surface(surface) {
            Ok(texture) => texture,
            Err(e) => panic!("failed to create {} texture : {}", $path,e)
        };
        texture
    }}
}