pub type BoxedScene<SceneChangeParamsT, EngineDataT, EventT, RendererT> = Box<Scene<SceneChangeParamsT, EngineDataT, EventT, RendererT> + 'static>;

pub trait Scene<SceneChangeParamsT, EngineDataT, EventT, RendererT> {
    fn render(&self, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64);
    fn think(&self, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64);
    fn handle_event(&mut self, event: &EventT, renderer: &mut RendererT, &mut EngineDataT) -> Option<SceneChangeEvent<SceneChangeParamsT>>;
}

pub enum SceneChangeEvent<T> {
    PushScene(T),
    SwapScene(T),
    PopScene,
}

pub struct SceneStack<SceneChangeParamsT, EngineDataT, EventT, RendererT> {
    scenes: Vec<BoxedScene<SceneChangeParamsT, EngineDataT, EventT, RendererT>>
}

impl<SceneChangeParamsT, EngineDataT, EventT, RendererT> SceneStack<SceneChangeParamsT, EngineDataT, EventT, RendererT> {

    pub fn new() -> SceneStack<SceneChangeParamsT, EngineDataT, EventT, RendererT> {
        SceneStack {
            scenes: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.len() == 0
    }

    pub fn push(&mut self, scene: BoxedScene<SceneChangeParamsT, EngineDataT, EventT, RendererT>) {
        self.scenes.push(scene)
    }

    pub fn swap(&mut self, scene: BoxedScene<SceneChangeParamsT, EngineDataT, EventT, RendererT>) -> Option<BoxedScene<SceneChangeParamsT, EngineDataT, EventT, RendererT>> {
        let old_scene = self.scenes.pop();
        self.scenes.push(scene);
        old_scene
    }

    pub fn pop(&mut self) -> Option<BoxedScene<SceneChangeParamsT, EngineDataT, EventT, RendererT>> {
        self.scenes.pop()
    }

    pub fn think(&mut self, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(scene) => {
                scene.think(renderer, engine_data, tick);
                self.scenes.push(scene);
            },
            None => ()
        }
    }

    pub fn render(&mut self, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(scene) => {
                scene.render(renderer, engine_data, tick);
                self.scenes.push(scene);
            },
            None => ()
        }
    }

    pub fn handle_event(&mut self, event: &EventT, renderer: &mut RendererT, engine_data: &mut EngineDataT) -> Option<SceneChangeEvent<SceneChangeParamsT>> {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(mut scene) => {
                let event = scene.handle_event(event, renderer, engine_data);
                self.scenes.push(scene);
                event
            },
            None => None
        }
    }
}
