use egui::FullOutput;
use wgpu::{
    CommandEncoderDescriptor,
    TextureViewDescriptor,
    LoadOp, StoreOp,
    Operations, 
    RenderPassDescriptor, RenderPassColorAttachment,
};
use super::{
    WindowEventError,
};
use crate::{
    aliases::{
        EGUIScreenDescriptor
    },
    modules::{
        graphics::{
            graphics_data::{
                graphics_backend_data::{
                    wgpu_data::WGPUData,
                    egui_data::EGUIData,
                },
            },
        },
    },
};

pub struct DrawPhaseContext<'c> {
    pub wgpu_data: &'c WGPUData,
    pub egui_data: &'c mut EGUIData,
}

pub fn draw_phase(
    full_output: FullOutput,
    draw_phase_context: DrawPhaseContext,
) -> Result<(), WindowEventError> { 
    let surface_texture = draw_phase_context.wgpu_data
        .surface
        .get_current_texture()?;

    let window_surface_view = surface_texture
        .texture
        .create_view(&TextureViewDescriptor::default()
    );

    let command_encoder_descriptor = CommandEncoderDescriptor {
        label: Some("Main comand encoder")
    };

    let mut encoder = draw_phase_context
        .wgpu_data
        .device
        .create_command_encoder(&command_encoder_descriptor);
   
    let platform_output = full_output.platform_output;

    let screen_descriptor = EGUIScreenDescriptor { 
        size_in_pixels: [
            draw_phase_context.wgpu_data.surface_configuration.width, 
            draw_phase_context.wgpu_data.surface_configuration.height
        ], 
        pixels_per_point: draw_phase_context.wgpu_data.window.scale_factor() as f32,  
    };

    draw_phase_context
        .egui_data
        .egui_winit_state
        .egui_ctx()
        .set_pixels_per_point(screen_descriptor.pixels_per_point);

    draw_phase_context
        .egui_data
        .egui_winit_state
        .handle_platform_output(
            &draw_phase_context.wgpu_data.window, 
            platform_output
        );

    let paint_jobs = draw_phase_context
        .egui_data
        .egui_winit_state
        .egui_ctx()
        .tessellate(
            full_output.shapes, 
            draw_phase_context.egui_data.egui_winit_state.egui_ctx().pixels_per_point()
        );

    for (id, image_delta) in &full_output.textures_delta.set {
        draw_phase_context
            .egui_data
            .egui_renderer
            .update_texture(
                &draw_phase_context.wgpu_data.device, 
                &draw_phase_context.wgpu_data.queue, 
                *id, 
                image_delta
            );    
    }

    let egui_commands_buffers = draw_phase_context
        .egui_data
        .egui_renderer
        .update_buffers(
            &draw_phase_context.wgpu_data.device, 
            &draw_phase_context.wgpu_data.queue, 
            &mut encoder, 
            &paint_jobs, 
            &screen_descriptor
        );

    draw_phase_context.wgpu_data.queue.submit(egui_commands_buffers);

    let render_pass_descriptor = RenderPassDescriptor {
        color_attachments: &[Some(RenderPassColorAttachment {
                view: &window_surface_view,
                resolve_target: None, 
                ops: Operations { 
                    load: LoadOp::Load, 
                    store: StoreOp::Store 
                },
                depth_slice: None,
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        label: Some("egui render pass"),
        occlusion_query_set: None,
    };

    let render_pass = encoder.begin_render_pass(&render_pass_descriptor);

    draw_phase_context
        .egui_data
        .egui_renderer
        .render(
            &mut render_pass.forget_lifetime(), 
            &paint_jobs, 
            &screen_descriptor
        );

    for x in &full_output.textures_delta.free {
        draw_phase_context
            .egui_data
            .egui_renderer
            .free_texture(x);
    }

    draw_phase_context
        .wgpu_data
        .queue
        .submit(Some(encoder.finish()));
    
    surface_texture.present();

    Ok(())
}
