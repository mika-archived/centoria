use clap::App;

pub fn build_app() -> App<'static, 'static> {
  return App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!());
}
