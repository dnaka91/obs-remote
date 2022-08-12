use std::io::Cursor;

use image::{imageops::FilterType, DynamicImage, ImageOutputFormat, RgbaImage};
use obs::source::{Source, SourceType};
use tonic::{Request, Response, Status};

use self::screenshot_request::{resize::Filter, ImageFormat};
pub use self::sources_service_server::SourcesServiceServer;
use crate::precondition;

tonic::include_proto!("sources.v1");

pub struct SourcesService;

#[tonic::async_trait]
impl sources_service_server::SourcesService for SourcesService {
    async fn is_active(
        &self,
        request: Request<IsActiveRequest>,
    ) -> Result<Response<IsActiveResponse>, Status> {
        let IsActiveRequest { name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        let ty = source.ty();
        precondition!(
            ty == SourceType::Input || ty == SourceType::Scene,
            "`{}` is neither an input nor a scene",
            name
        );

        Ok(Response::new(IsActiveResponse {
            active: source.active(),
            showing: source.showing(),
        }))
    }

    async fn screenshot(
        &self,
        request: Request<ScreenshotRequest>,
    ) -> Result<Response<ScreenshotResponse>, Status> {
        let details = request.into_inner();
        let image = screenshot(&details).unwrap();

        Ok(Response::new(ScreenshotResponse {  image }))
    }

    async fn save_screenshot(
        &self,
        request: Request<SaveScreenshotRequest>,
    ) -> Result<Response<SaveScreenshotResponse>, Status> {
        let SaveScreenshotRequest { file_path, details } = request.into_inner();
        let details =
            details.ok_or_else(|| Status::failed_precondition("details must be specified"))?;
        let image = screenshot(&details).unwrap();

        std::fs::write(file_path, image).unwrap();

        Ok(Response::new(SaveScreenshotResponse {}))
    }
}

fn screenshot(details: &ScreenshotRequest) -> Option<Vec<u8>> {
    let source = Source::by_name(&details.name).unwrap();

    let image = take_source_screenshot(&source)?;
    let mut image = DynamicImage::ImageRgba8(image);

    if let Some(resize) = &details.resize {
        image = image.resize(
            resize.width,
            resize.height,
            match resize.filter() {
                Filter::Unspecified | Filter::Cubic => FilterType::CatmullRom,
                Filter::Nearest => FilterType::Nearest,
                Filter::Linear => FilterType::Triangle,
                Filter::Gaussian => FilterType::Gaussian,
                Filter::Lanczos3 => FilterType::Lanczos3,
            },
        );
    }

    let mut buf = Cursor::new(Vec::new());

    image
        .write_to(
            &mut buf,
            match details.format() {
                ImageFormat::Unspecified | ImageFormat::Png => ImageOutputFormat::Png,
                ImageFormat::Jpg => ImageOutputFormat::Jpeg(details.compression as _),
            },
        )
        .unwrap();

    Some(buf.into_inner())
}

fn take_source_screenshot(source: &Source<'_>) -> Option<RgbaImage> {
    use image::Rgba;
    use obs::{
        graphics,
        graphics::Vec4,
        gs::{self, BlendType, ClearFlags, ColorFormat, StageSurface, TexRender, ZstencilFormat},
    };

    let width = source.base_width();
    let height = source.base_height();

    let mut img = RgbaImage::new(width, height);

    graphics::scoped(|| {
        let tex_render = TexRender::create(ColorFormat::Rgba, ZstencilFormat::None);
        let stage_surface = StageSurface::create((width, height), ColorFormat::Rgba);

        tex_render.reset();

        tex_render.begin((width, height)).then(|| {
            gs::clear(ClearFlags::COLOR, Vec4::default(), 0.0, 0);
            gs::ortho(0.0, width as f32, 0.0, height as f32, -100.0, 100.0);

            gs::blend_state_push();
            gs::blend_function(BlendType::One, BlendType::Zero);

            source.inc_showing();
            source.video_render();
            source.dec_showing();

            gs::blend_state_pop();
            tex_render.end();

            gs::stage_texture(&stage_surface, &tex_render.get_texture());

            let mut y = 0;
            stage_surface.map(|row| {
                for (x, pixel) in row
                    .chunks_exact(4)
                    .enumerate()
                    .map(|(x, c)| (x as u32, Rgba([c[0], c[1], c[2], c[3]])))
                {
                    img.put_pixel(x, y, pixel);
                }
                y += 1;
            });

            img
        })
    })
}
