pub type BoxedScene<EventT, RendererT, EngineDataT> = Box<Scene<EventT, RendererT, EngineDataT> + 'static>;
pub type SceneChangeCallback<EventT, RendererT, EngineDataT> = Box<Fn(&mut RendererT, &mut EngineDataT) -> BoxedScene<EventT, RendererT, EngineDataT>>;

pub trait Scene<EventT, RendererT, EngineDataT> {
    fn render(&self, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64);
    fn handle_event(&mut self, event: &EventT, renderer: &mut RendererT, engine_data: &mut EngineDataT, tick: u64) -> Option<SceneChangeEvent<EventT, RendererT, EngineDataT>>;
}

pub enum SceneChangeEvent<EventT, RendererT, EngineDataT> {
    PushScene(SceneChangeCallback<EventT, RendererT, EngineDataT>),
    SwapScene(SceneChangeCallback<EventT, RendererT, EngineDataT>),
    PopScene,
}

pub struct SceneStack<EventT, RendererT, EngineDataT> {
    scenes: Vec<BoxedScene<EventT, RendererT, EngineDataT>>
}

impl<EventT, RendererT, EngineDataT> SceneStack<EventT, RendererT, EngineDataT> {

    pub fn new() -> SceneStack<EngineDataT, EventT, RendererT> {
        SceneStack {
            scenes: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.len() == 0
    }

    pub fn push(&mut self, scene: BoxedScene<EventT, RendererT, EngineDataT>) {
        self.scenes.push(scene)
    }

    pub fn swap(&mut self, scene: BoxedScene<EventT, RendererT, EngineDataT>) -> Option<BoxedScene<EventT, RendererT, EngineDataT>> {
        let old_scene = self.scenes.pop();
        self.scenes.push(scene);
        old_scene
    }

    pub fn pop(&mut self) -> Option<BoxedScene<EventT, RendererT, EngineDataT>> {
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
                self.push(callback(renderer, engine_data));
            },
            Some(SceneChangeEvent::SwapScene(callback)) => {
                self.swap(callback(renderer, engine_data));
            },
            Some(SceneChangeEvent::PopScene) => {
                self.pop();
            },
            None => ()
        };
    }
}
