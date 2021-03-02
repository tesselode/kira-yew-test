use kira::{manager::AudioManager, sequence::Sequence};
use wasm_bindgen::prelude::*;
use web_sys::console;
use yew::prelude::*;

struct Model {
	link: ComponentLink<Self>,
	value: i64,
}

enum Msg {
	AddOne,
}

impl Component for Model {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { link, value: 0 }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::AddOne => self.value += 1,
		}
		let mut manager = AudioManager::new(Default::default()).unwrap();
		manager
			.start_sequence(Sequence::<()>::new(Default::default()), Default::default())
			.unwrap();
		alert("test");
		console::log_1(&"freeing unused resources".into());
		manager.free_unused_resources();
		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		// Should only return "true" if new properties are different to
		// previously received properties.
		// This component has no properties so we will always return "false".
		false
	}

	fn view(&self) -> Html {
		html! {
			<div>
				<button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
				<p>{ self.value }</p>
			</div>
		}
	}
}

#[wasm_bindgen]
extern "C" {
	fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
	App::<Model>::new().mount_to_body();
}
