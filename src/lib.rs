mod test_component;

use kira::{manager::AudioManager, sequence::Sequence};
use test_component::TestComponent;
use wasm_bindgen::prelude::*;
use web_sys::console;
use yew::prelude::*;

struct Model {
	link: ComponentLink<Self>,
	show_test_component: bool,
}

enum Msg {
	ToggleTestComponent,
}

impl Component for Model {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			show_test_component: false,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::ToggleTestComponent => {
				self.show_test_component = !self.show_test_component;
				true
			}
		}
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		// Should only return "true" if new properties are different to
		// previously received properties.
		// This component has no properties so we will always return "false".
		false
	}

	fn view(&self) -> Html {
		let test_component = if self.show_test_component {
			html! {
				<TestComponent />
			}
		} else {
			html! {}
		};

		html! {
			<div>
				<button onclick=self.link.callback(|_| Msg::ToggleTestComponent)>{ "toggle test component" }</button>
				{ test_component }
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
