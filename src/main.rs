use std::time::{Duration, Instant};

use ratatui::{
    crossterm::event::{self, Event, KeyCode,KeyEventKind}, layout::{self, Alignment, Constraint, Direction, Layout}, style::{Color, Modifier, Style}, symbols::block, widgets::{Block, Borders, Paragraph},
};


fn main()-> std::io::Result<()>{

    let mut terminal = ratatui::init();

    let mut inicio = Instant::now();
    let session = 1500;
    let mut paused = false;
    let mut acumulado: u64 = 0;
    loop{
        let transcurrido = if paused{
            acumulado  
        }else {
            acumulado + run_timer(inicio)
        };
        terminal.draw(|frame |{
            let restante = session - transcurrido;
            let texto = format!("{:02}:{:02}",restante / 60, restante % 60  );
            
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),// titulo
                    Constraint::Min(0), // timer 
                    Constraint::Length(1) // control
                ])
                .split(frame.area());

            let titulo = Paragraph::new(" Pomodorito ")
                    .alignment(Alignment::Center);

            let timer = Paragraph::new(texto)
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));
            
            let ayuda = Paragraph::new(" [p/pause]  [q/quit]")
                    .alignment(Alignment::Center);
            
            frame.render_widget(titulo, layout[0]);
            frame.render_widget(timer, layout[1]);
            frame.render_widget(ayuda, layout[2]);
            
        })?;

        if transcurrido >= session{
            break;
        }

        if event::poll(Duration::from_millis(200))?{
            if let Event::Key(k) = event::read()?{
                if k.kind == KeyEventKind::Press{
                    if k.code ==KeyCode::Char('q'){
                        break;
                    }else if k.code == KeyCode::Char('p'){
                        if paused{
                            inicio = Instant::now();
                            paused = false;
                        }else {
                            acumulado += run_timer(inicio);
                            paused = true;
                        }
                    }
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
