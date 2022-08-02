use async_std::prelude::StreamExt;
use async_std::sync::{Arc, Mutex};
use async_std::task::{sleep, spawn};
use keyframe::ease;
use keyframe::functions::*;
use std::time::Duration;
use swayipc_async::{Connection, Event, EventType, Fallible, WindowChange, WindowEvent};
use tap::Tap;

const OPACITY_MAX: f32 = 1.0;
const OPACITY_MIN: f32 = 0.85;
const RISE_STEPS: i32 = 10;
const FALL_STEPS: i32 = 10;
const RISE_DURATION_MS: i32 = 10;
const FALL_DURATION_MS: i32 = 200;

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
	to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

async fn flash(connection: Arc<Mutex<Connection>>, id: i64) {
	let cmd_body: String = format!(r#"[con_id="{}"] opacity "#, id);

	// Rising edge
	for x in 1..=RISE_STEPS {
		{
			let opacity: f32 = ease(
				EaseOut,
				OPACITY_MAX,
				OPACITY_MIN,
				map_range((1., RISE_STEPS as f32), (0., 1.), x as f32),
			).tap_dbg(|x| eprintln!("RISE {}", x));
			let mut connection1 = connection.lock().await;
			connection1.run_command(String::new() + &cmd_body + &(opacity).to_string()).await;
		}
		sleep(Duration::from_millis((RISE_DURATION_MS / RISE_STEPS) as u64, )).await;
	}

	// Falling edge
	for x in 1..=FALL_STEPS {
		{
			let opacity: f32 = ease(
				EaseInOutQuad,
				OPACITY_MIN,
				OPACITY_MAX,
				map_range((1., FALL_STEPS as f32), (0., 1.), x as f32),
			).tap_dbg(|x| eprintln!("RISE {}", x));
			let mut connection1 = connection.lock().await;
			connection1.run_command(String::new() + &cmd_body + &(opacity).to_string()).await;
		}
		sleep(Duration::from_millis((FALL_DURATION_MS / FALL_STEPS) as u64, )).await;
	}
}

#[async_std::main]
async fn main() -> Fallible<()> {
	// Two separate connections must be made: https://github.com/JayceFayne/swayipc-rs/issues/25
	let mut events = Connection::new().await?.subscribe([EventType::Window]).await?;
	let connection = Arc::new(Mutex::new(Connection::new().await?));
	loop {
		if let Some(event) = events.next().await.transpose()? {
			match event {
				Event::Window(w) => {
					let WindowEvent { change, container, .. } = *w;
					if change == WindowChange::Focus {
						spawn(flash(
							connection.clone(),
							container.id.tap_dbg(|x| eprintln!("FOCUSED {:#?}", x)),
						));
					}
				}
				// We subscribe strictly to EventType::Window, so nothing else should come through.
				_ => unreachable!(),
			}
		}
	}
}
