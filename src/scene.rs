pub type BoxedScene<'a, EventT, RendererT, EngineDataT, SceneChangeDataT> = Box<Scene<EventT, RendererT, EngineDataT, SceneChangeDataT> + 'a>;
pub type SceneChangeCallback<'a, EventT, RendererT, EngineDataT, SceneChangeDataT> = Fn(&SceneChangeDataT, &mut RendererT, &mut EngineDataT) -> BoxedScene<'a, EventT, RendererT, EngineDataT, SceneChangeDataT>;

pub trait Scene<EventT, RendererT, EngineDataT, SceneChangeDataT> {
    fn render(&self, renderer: &mut RendererT, engine_data: &EngineDataT, tick: u64);
    fn handle_event(&mut self, event: &EventT, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64);
    fn think(&mut self, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64) -> Option<SceneChangeEvent<SceneChangeDataT>>;
}

pub enum SceneChangeEvent<SceneChangeDataT> {
    PushScene(SceneChangeDataT),
    SwapScene(SceneChangeDataT),
    PopScene,
}

pub struct SceneStack<'a, EventT, RendererT, EngineDataT, SceneChangeDataT> {
    scenes: Vec<BoxedScene<'a, EventT, RendererT, EngineDataT, SceneChangeDataT>>
}

impl<'a, EventT, RendererT, EngineDataT, SceneChangeDataT> SceneStack<'a, EventT, RendererT, EngineDataT, SceneChangeDataT> {

    pub fn new() -> SceneStack<'a, EngineDataT, EventT, RendererT, SceneChangeDataT> {
        SceneStack {
            scenes: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.len() == 0
    }

    pub fn push(&mut self, scene: BoxedScene<'a, EventT, RendererT, EngineDataT, SceneChangeDataT>) {
        self.scenes.push(scene)
    }

    pub fn swap(&mut self, scene: BoxedScene<'a, EventT, RendererT, EngineDataT, SceneChangeDataT>) -> Option<BoxedScene<'a, EventT, RendererT, EngineDataT, SceneChangeDataT>> {
        let old_scene = self.scenes.pop();
        self.scenes.push(scene);
        old_scene
    }

    pub fn pop(&mut self) -> Option<BoxedScene<'a, EventT, RendererT, EngineDataT, SceneChangeDataT>> {
        self.scenes.pop()
    }

    pub fn render(&mut self, renderer: &mut RendererT, engine_data: &EngineDataT, tick: u64) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(scene) => {
                scene.render(renderer, engine_data, tick);
                self.scenes.push(scene);
            },
            None => ()
        }
    }

    pub fn handle_event(&mut self, event: &EventT, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(mut scene) => {
                scene.handle_event(event, renderer, engine_data, tick);
                self.scenes.push(scene);
            },
            None => ()
        };
    }

    pub fn think(&mut self, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64, scene_change_handler: &SceneChangeCallback<'a, EventT, RendererT, EngineDataT, SceneChangeDataT>) {
        let maybe_last_scene = self.scenes.pop();
        let event = match maybe_last_scene {
            Some(mut scene) => {
                let event = scene.think(renderer, engine_data, tick);
                self.scenes.push(scene);
                event
            },
            None => None
        };

        match event {
            Some(SceneChangeEvent::PushScene(ref data)) => {
                self.push(scene_change_handler(data, renderer, engine_data));
            },
            Some(SceneChangeEvent::SwapScene(ref data)) => {
                self.swap(scene_change_handler(data, renderer, engine_data));
            },
            Some(SceneChangeEvent::PopScene) => {
                self.pop();
            },
            None => ()
        };
    }
}
