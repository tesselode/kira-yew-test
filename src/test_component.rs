use kira::{manager::AudioManager, sequence::Sequence};
use yew::prelude::*;

pub struct TestComponent {
	manager: AudioManager,
}

impl Component for TestComponent {
	type Message = ();

	type Properties = ();

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		let mut manager = AudioManager::new(Default::default()).unwrap();
		manager
			.start_sequence(Sequence::<()>::new(Default::default()), Default::default())
			.unwrap();
		Self { manager }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<div>{ "hi!" }</div>
		}
	}
}
