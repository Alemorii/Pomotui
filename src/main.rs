//use std::io;
//use std::thread::sleep;
use std::time::{Duration, Instant};
use ratatui::crossterm::event::{self, Event, KeyCode};


fn main()-> std::io::Result<()>{

    let mut terminal = ratatui::init();
    let inicio = Instant::now();
    let session = 1500;
    loop{
        let transcurrido = run_timer(inicio);
        terminal.draw(|frame |{
            let restante = session - transcurrido;
            let texto = format!("{:02}:{:02}",restante / 60, restante % 60  );
            frame.render_widget(texto, frame.area());
        })?;

        if transcurrido >= session{
            break;
        }
        if event::poll(Duration::from_millis(200))?{
            if let Event::Key(k) = event::read()?{
                if k.code == KeyCode::Char('q'){
                    break;
                }
            }
        }
    }
    ratatui::restore();
    Ok(())
}


fn run_timer(inicio: Instant)-> u64{
    inicio.elapsed().as_secs()
}
