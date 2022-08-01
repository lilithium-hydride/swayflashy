use std::time::Duration;
use async_std::io::{prelude::WriteExt, stdin, stdout};
use async_std::prelude::StreamExt;
use async_std::task::sleep;
use swayipc_async::{Connection, Event, EventType, Fallible, WindowChange, WindowEvent};


async fn flash(connection: &mut Connection, id: i64) {
	let cmd_body: String = format!(r#"[con_id="{}"] opacity "#, id);
	for x in (1..10) {
		connection.run_command(String::new() + &cmd_body + &(x as f32/10.0 + 0.5).to_string()).await;
		sleep(Duration::from_millis(50)).await;
	}
	println!("flashed");
}

#[async_std::main]
async fn main() -> Fallible<()> {
	let mut connection = Connection::new().await?;
	let mut events = connection.subscribe([EventType::Window]).await?;
	let mut connection = Connection::new().await?;
	loop {
		println!(".");
		for event in events.next().await.transpose()? {
			match event {
				Event::Window(w) => {
					if let WindowEvent { change, container, .. } = *w {
						if change == WindowChange::Focus {
							println!("{:#?}", container);
							flash(&mut connection, container.id).await;
						}
					}
				}
				_ => unreachable!()
			}
		}
		// for outcome in connection.run_command(&command_text).await? {
		// 	if let Err(error) = outcome {
		// 		println!("failure '{}'", error);
		// 	} else {
		// 		println!("success");
		// 	}
		// }
	}
	Ok(())
}