use std::thread::sleep;
use std::time::Duration;
#[allow(unused_imports)]
use fyrox::graph::prelude::*;
use fyrox::{
    core::{visitor::prelude::*, reflect::prelude::*, type_traits::prelude::*},
    event::Event, script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
    plugin::error::GameResult
};
use fyrox::core::algebra::Vector3;
use fyrox::keyboard::KeyCode;

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "dd0bed3c-cbfb-49f9-ad71-3c4a6ecb6e5b")]
#[visit(optional)]
pub struct RoleMove {
    // Add fields here.
    is_jump: bool,
    timer: u32,
}

impl ScriptTrait for RoleMove {
    fn on_init(&mut self, _context: &mut ScriptContext) -> GameResult {
        // Put initialization logic here.
        self.is_jump = false;
        Ok(())
    }

    fn on_start(&mut self, _context: &mut ScriptContext) -> GameResult {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
        Ok(())
    }

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) -> GameResult {
        // Put de-initialization logic here.
        Ok(())
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: &mut ScriptContext) -> GameResult {
        // Respond to OS events here.
        Ok(())
    }

    fn on_update(&mut self, context: &mut ScriptContext) -> GameResult {
        // Put object logic here.
        let transform = context.scene.graph[context.handle].local_transform_mut();
        if context.input_state.is_key_down(KeyCode::KeyW) || context.input_state.is_key_down(KeyCode::ArrowUp){
            transform.offset(Vector3::new(0f32, 0f32, 0.1));
        }
        if context.input_state.is_key_down(KeyCode::KeyS) || context.input_state.is_key_down(KeyCode::ArrowDown){
            transform.offset(Vector3::new(0f32, 0f32, -0.1));
        }
        if context.input_state.is_key_down(KeyCode::KeyA) || context.input_state.is_key_down(KeyCode::ArrowLeft){
            transform.offset(Vector3::new(0.1, 0f32, 0f32));
        }
        if context.input_state.is_key_down(KeyCode::KeyD) || context.input_state.is_key_down(KeyCode::ArrowRight){
            transform.offset(Vector3::new(-0.1, 0f32, 0f32));
        }
        if context.input_state.is_key_down(KeyCode::Space){
            if !self.is_jump {
                self.is_jump = true;
                transform.offset(Vector3::new(0f32, 1f32, 0f32));
                context.task_pool.spawn_script_task(
                    context.scene_handle,
                    context.handle,
                    context.script_index,
                    async move {
                        sleep(Duration::from_secs_f32(0.1));
                    },
                    |_result, _script: &mut RoleMove, ctx| {
                        ctx.scene.graph[ctx.handle].local_transform_mut().offset(Vector3::new(0f32, -1f32, 0f32));
                        Ok(())
                    }
                );
            }
        }else if self.is_jump{
            self.is_jump = false;
        }
        Ok(())
    }
}
    