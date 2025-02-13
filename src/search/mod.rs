


// pub struct CommandMatcher {
//     folder: PathBuf, 
//     injector: Injector<String>
// }

// impl CommandMatcher {
//     pub fn new(folder: PathBuf, injector: Injector<String>) -> Self {
//         Self { folder, injector }
//     }

//     pub async fn load_commands(&self) -> Result<(), Error> {
//         let mut entries = fs::read_dir(&self.folder).await?;

//         while let Ok(Some(entry)) = entries.next_entry().await {
//             let entry = entry.path();
//             let content = fs::read_to_string(&entry).await?;

//             self.injector.push(content, 
//                 |value: String, s: &mut Vec<Utf32String>| {
//                     let mut value = Utf32String::Ascii(value.clone().into());
//                     s.push(value);
//                 });
//         }

//         Ok(())
//     }
// }

// pub struct MatcherCommands {
//     nucleo: Nucleo<String>,
//     pattern: Option<String>,
// }

// // TODO: `Notify` will call the table to update the view.
// impl MatcherCommands {
//     pub fn new() -> Self {
//         let nucleo: Nucleo<String> = Nucleo::new(
//             Config::DEFAULT,
//             Arc::new(move || {}),
//             Some(3),
//             1,
//         );

//         Self { nucleo, pattern: None }
//     }
// }