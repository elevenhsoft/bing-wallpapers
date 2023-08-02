use std::path::PathBuf;
use wallpaper;

use iced::{executor, Alignment, Length};
use iced::{Application, Element, Command, Theme};
use iced::widget::{column, row, image, text, button};
use crate::api::response::Images;

#[derive(Debug, Clone)]
pub enum Buttons {
    PreviosPressed,
    NextPressed,
    DownloadPressed,
    SetAsPressed,
}
impl Application for Images {

    type Executor = executor::Default;
    type Flags = ();
    type Message = Buttons;
    type Theme = Theme;

    fn new(_flags: ()) -> (Images, Command<Self::Message>) {
        let images = Images::new().unwrap();
        let current = 0;
        (Images { json: images.json, current }, Command::none())
    }
    
    fn title(&self) -> String {
        let win_title = format!("{0}{1}", "Bing Wallpapers: ", self.json.images[self.current as usize].copyright);
        String::from(win_title)
    }

    fn view(&self) -> Element<Buttons> {

        let curr_endpoint = &self.json.images[self.current as usize].url;
        let img = image::Handle::from_memory(Images::fetch_image(Images::image_url(curr_endpoint.to_string())).unwrap());

        column![
            row![
                image::viewer(img)
                    .width(Length::Fixed(800.0))
            ],
            row![
                column![
                    row![
                        text(&self.json.images[self.current as usize].copyright)
                    ]
                        .padding(20)
                        .align_items(Alignment::Center),
                    row![
                        column![
                            button(text("< Previous").horizontal_alignment(iced::alignment::Horizontal::Center))
                                .on_press(Buttons::PreviosPressed)
                                .width(Length::Fill)
                        ]
                            .width(Length::Fill)
                            .padding(3),
                        column![
                            button(text("Next >").horizontal_alignment(iced::alignment::Horizontal::Center))
                                .on_press(Buttons::NextPressed)
                                .width(Length::Fill)
                        ]
                            .width(Length::Fill)
                            .padding(3),
                    ],
                    row![
                        column![
                            button(text("Download and save to Pictures").horizontal_alignment(iced::alignment::Horizontal::Center))
                                .on_press(Buttons::DownloadPressed)
                                .width(Length::Fill)
                        ]
                            .padding(3)
                    ],
                    row![
                        column![
                            button(text("Set as wallpaper").horizontal_alignment(iced::alignment::Horizontal::Center))
                                .on_press(Buttons::SetAsPressed)
                                .width(Length::Fill)
                        ]
                            .padding(3)
                    ]
                ]
                    .align_items(Alignment::Center)
            ]
        ]
            .into()

    }

    fn update(&mut self, button: Self::Message) -> Command<Buttons> {
        match button {
            Buttons::NextPressed => {
                if (self.current as usize) < self.json.images.len() - 1 {
                    self.current += 1;
                }

                Command::none()
            }

            Buttons::PreviosPressed => {
                if self.current > 0 {
                    self.current -= 1;
                }

                Command::none()
            }

            Buttons::DownloadPressed => {
                self.download_image();

                Command::none()
            }

            Buttons::SetAsPressed => {
                self.download_image();
                set_wallpaper(self.image_path());

                Command::none()
            }
        }
    }
}

fn set_wallpaper(path: PathBuf) {
    wallpaper::set_from_path(path.to_str().unwrap()).unwrap();
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
} 
