extern crate sdl2_engine_helpers;
use sdl2_engine_helpers::scene::{SceneStack, Scene, SceneChangeEvent, BoxedScene};

pub struct Data {
    pub value: u32
}

pub struct BorrowedData<'a> {
    pub data: &'a Data
}

pub struct DummyScene<'a> {
    pub stored: &'a mut Data
}

impl<'a> DummyScene<'a> {
    pub fn new<'b>(first_arg: &Data, second_arg: &'a mut Data) -> BoxedScene<'a, (), (), BorrowedData<'b>, ()> {
        Box::new(DummyScene {
            stored: second_arg
        })
    }
}

impl<'a, 'b> Scene<(), (), BorrowedData<'b>, ()> for DummyScene<'a> {

    fn render(&self, renderer: &mut (), engine_data: &BorrowedData, tick: u64) {
    }

    fn handle_event(&mut self, event: &(), renderer: &mut (), engine_data: &mut BorrowedData, tick: u64) {
    }

    fn think(&mut self, renderer: &mut (), engine_data: &mut BorrowedData, tick: u64) -> Option<SceneChangeEvent<()>> {
        None
    }
}
#[test]
fn correct_lifetimes_on_scene() {
    let d1 = Data {
        value: 1
    };
    let mut d2 = Data {
        value: 2
    };
    let d3 = Data {
        value: 3
    };
    let borrow = BorrowedData {
        data: &d3
    };
    let mut scene_stack = SceneStack::new();
    scene_stack.push(DummyScene::new(&d1, &mut d2));
    scene_stack.render(&mut (), &borrow, 0);
}
