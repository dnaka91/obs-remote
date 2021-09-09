use image::{DynamicImage, RgbaImage};
use obs::source::{Source, SourceType};
use tonic::{Request, Response, Status};

pub use self::sources_server::SourcesServer;
use crate::precondition;

tonic::include_proto!("obs_remote.v5.sources");

pub struct SourcesService;

#[tonic::async_trait]
impl sources_server::Sources for SourcesService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_active(&self, request: Request<String>) -> Result<Response<IsActiveReply>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        let ty = source.ty();
        precondition!(
            ty == SourceType::Input || ty == SourceType::Scene,
            "`{}` is neither an input nor a scene",
            name
        );

        Ok(Response::new(IsActiveReply {
            active: source.active(),
            showing: source.showing(),
        }))
    }

    async fn screenshot(
        &self,
        request: Request<ScreenshotRequest>,
    ) -> Result<Response<Vec<u8>>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn save_screenshot(
        &self,
        request: Request<SaveScreenshotRequest>,
    ) -> Result<Response<()>, Status> {
        let source = Source::by_name("OBWS-TEST-Text").unwrap();
        let image = take_source_screenshot(&source).unwrap();

        DynamicImage::ImageRgba8(image)
            .save("/Users/dominik.nakamura/Projects/.private/public/obs-remote/test.png")
            .unwrap();

        Ok(Response::new(()))
    }
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
            gs::ortho(0.0, width as f32, 0.0, height as f32, -100.0, -100.0);

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
