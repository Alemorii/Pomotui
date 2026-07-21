use std::time::{Duration, Instant};

use ratatui::{
    crossterm::event::{self, Event, KeyCode,KeyEventKind},
    layout::{Alignment, Layout, Constraint, Direction},
    style::{Style, Color, Modifier},
    widgets::{Paragraph, Block, Borders},
};
use tui_big_text::{BigText, PixelSize};


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
            let widget = Paragraph::new(texto)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title(" Pomodorito [p/pause] [q/quit]"));
            //let big_text = BigText::builder()
            //    .pixel_size(PixelSize::Full)
            //    .style(Style::default().fg(Color::Cyan))
            //    .lines(vec![texto.into()])
            //    .build();
            frame.render_widget(widget, frame.area());
            
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
