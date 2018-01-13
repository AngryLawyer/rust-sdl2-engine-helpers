use sdl2::render::WindowCanvas;

pub type BoxedScene<'a, EventT, SceneChangeParamsT, EngineDataT> = Box<Scene<EventT, SceneChangeParamsT, EngineDataT> + 'a>;

pub trait Scene<EventT, SceneChangeParamsT, EngineDataT> {
    fn render(&mut self, renderer: &mut WindowCanvas, engine_data: &mut EngineDataT, tick: u64);
    fn handle_event(&mut self, event: &EventT, engine_data: &mut EngineDataT, tick: u64);
    fn think(&mut self, engine_data: &mut EngineDataT, tick: u64) -> Option<SceneChangeEvent<SceneChangeParamsT>>;
}

#[derive(Debug, Copy, Clone)]
pub enum SceneChangeEvent<T> {
    PushScene(T),
    SwapScene(T),
    PopScene,
}

pub struct SceneStack<'a, EventT, SceneChangeParamsT, EngineDataT> {
    scenes: Vec<BoxedScene<'a, EventT, SceneChangeParamsT, EngineDataT>>
}

impl<'a, EventT, SceneChangeParamsT, EngineDataT> SceneStack<'a, EventT, SceneChangeParamsT, EngineDataT> {

    pub fn new() -> SceneStack<'a, EventT, SceneChangeParamsT, EngineDataT> {
        SceneStack {
            scenes: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.len() == 0
    }

    pub fn push(&mut self, scene: BoxedScene<'a, EventT, SceneChangeParamsT, EngineDataT>) {
        self.scenes.push(scene)
    }

    pub fn swap(&mut self, scene: BoxedScene<'a, EventT, SceneChangeParamsT, EngineDataT>) -> Option<BoxedScene<'a, EventT, SceneChangeParamsT, EngineDataT>> {
        let old_scene = self.scenes.pop();
        self.scenes.push(scene);
        old_scene
    }

    pub fn pop(&mut self) -> Option<BoxedScene<'a, EventT, SceneChangeParamsT, EngineDataT>> {
        self.scenes.pop()
    }

    pub fn render(&mut self, renderer: &mut WindowCanvas, engine_data: &mut EngineDataT, tick: u64) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(mut scene) => {
                scene.render(renderer, engine_data, tick);
                self.scenes.push(scene);
            },
            None => ()
        }
    }

    pub fn handle_event(&mut self, event: &EventT, engine_data: &mut EngineDataT, tick: u64) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(mut scene) => {
                scene.handle_event(event, engine_data, tick);
                self.scenes.push(scene);
            },
            None => ()
        }
    }

    pub fn think(&mut self, engine_data: &mut EngineDataT, tick: u64)  -> Option<SceneChangeEvent<SceneChangeParamsT>> {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(mut scene) => {
                let event = scene.think(engine_data, tick);
                self.scenes.push(scene);
                event
            },
            None => None
        }
    }
}
