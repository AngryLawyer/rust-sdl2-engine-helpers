pub type BoxedScene<EngineDataT, EventT, RendererT> = Box<Scene<EngineDataT, EventT, RendererT> + 'static>;
pub type SceneChangeCallback<EngineDataT, EventT, RendererT> = Box<Fn() -> BoxedScene<EngineDataT, EventT, RendererT>>;

pub trait Scene<EngineDataT, EventT, RendererT> {
    fn render(&self, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64);
    fn handle_event(&mut self, event: &EventT, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64) -> Option<SceneChangeEvent<EngineDataT, EventT, RendererT>>;
}

pub enum SceneChangeEvent<EngineDataT, EventT, RendererT> {
    PushScene(SceneChangeCallback<EngineDataT, EventT, RendererT>),
    SwapScene(SceneChangeCallback<EngineDataT, EventT, RendererT>),
    PopScene,
}

pub struct SceneStack<EngineDataT, EventT, RendererT> {
    scenes: Vec<BoxedScene<EngineDataT, EventT, RendererT>>
}

impl<EngineDataT, EventT, RendererT> SceneStack<EngineDataT, EventT, RendererT> {

    pub fn new() -> SceneStack<EngineDataT, EventT, RendererT> {
        SceneStack {
            scenes: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.len() == 0
    }

    pub fn push(&mut self, scene: BoxedScene<EngineDataT, EventT, RendererT>) {
        self.scenes.push(scene)
    }

    pub fn swap(&mut self, scene: BoxedScene<EngineDataT, EventT, RendererT>) -> Option<BoxedScene<EngineDataT, EventT, RendererT>> {
        let old_scene = self.scenes.pop();
        self.scenes.push(scene);
        old_scene
    }

    pub fn pop(&mut self) -> Option<BoxedScene<EngineDataT, EventT, RendererT>> {
        self.scenes.pop()
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

    pub fn handle_event(&mut self, event: &EventT, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64) {
        let maybe_last_scene = self.scenes.pop();
        let event = match maybe_last_scene {
            Some(mut scene) => {
                let event = scene.handle_event(event, renderer, engine_data, tick);
                self.scenes.push(scene);
                event
            },
            None => None
        };

        match event {
            Some(SceneChangeEvent::PushScene(callback)) => {
                self.push(callback());
            },
            Some(SceneChangeEvent::SwapScene(callback)) => {
                self.swap(callback());
            },
            Some(SceneChangeEvent::PopScene) => {
                self.pop();
            },
            None => ()
        };
    }
}
