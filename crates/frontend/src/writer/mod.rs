use gpui::{App, Global, Task};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::{io::Write, path::Path, sync::Arc, time::Duration};
use wayclick_schema::Settings;

pub mod config;

pub(crate) fn init_writers(cx: &mut App, path: &Path) {
    Settings::init(cx, path);
}

struct WriterHolder<Writer>
where
    Writer: std::fmt::Debug + Serialize + for<'de> Deserialize<'de> + Default + Clone + Save,
{
    data: Writer,
    write_task: Option<Task<()>>,
    path: Arc<Path>,
}

impl<Writer> Global for WriterHolder<Writer> where
    Writer:
        std::fmt::Debug + Serialize + for<'de> Deserialize<'de> + Default + Clone + Save + 'static
{
}

#[allow(dead_code)]
pub trait Writer
where
    Self:
        std::fmt::Debug + Clone + Save + Serialize + for<'de> Deserialize<'de> + Default + 'static,
{
    fn init(cx: &mut App, path: &Path) {
        let path: Arc<Path> = path.join(format!("{}.json", Self::get_name())).into();
        let mut data = try_read_json::<Self>(&path);
        data.post_load();
        cx.set_global(WriterHolder {
            data,
            write_task: None,
            path,
        });
    }

    /// Same as [Writer::get] but returns a clone of the data instead.
    fn get_copy(cx: &App) -> Self {
        cx.global::<WriterHolder<Self>>().data.clone()
    }

    fn get(cx: &App) -> &Self {
        &cx.global::<WriterHolder<Self>>().data
    }

    fn force_save(cx: &mut App) {
        cx.global_mut::<WriterHolder<Self>>().write_to_disk();
    }

    fn get_mut(cx: &mut App) -> &mut Self {
        if cx.global::<WriterHolder<Self>>().write_task.is_none() {
            let task = cx.spawn(async |app| {
                app.background_executor()
                    .timer(Duration::from_secs(1))
                    .await;
                app.update_global::<WriterHolder<Self>, _>(|holder, _| {
                    println!("Writing {} to disk!", Self::get_name());
                    holder.write_to_disk();
                });
            });

            let holder = cx.global_mut::<WriterHolder<Self>>();
            holder.write_task = Some(task);
            &mut holder.data
        } else {
            &mut cx.global_mut::<WriterHolder<Self>>().data
        }
    }

    fn get_name() -> &'static str {
        "Data"
    }
}

pub trait Save {
    /// Run the code before saving to disk.
    fn pre_save(&mut self) {}

    /// Run this code after loading the data.
    fn post_load(&mut self) {}
}

impl<Writer> WriterHolder<Writer>
where
    Writer: std::fmt::Debug + Serialize + for<'de> Deserialize<'de> + Default + Clone + Save,
{
    fn write_to_disk(&mut self) {
        self.write_task = None;
        let mut config = self.data.clone();
        config.pre_save();
        let Ok(bytes) = serde_json::to_vec(&config) else {
            return;
        };
        _ = write_safe(&self.path, &bytes);
    }
}

pub(crate) fn try_read_json<T: std::fmt::Debug + Default + for<'de> Deserialize<'de>>(
    path: &Path,
) -> T {
    let Ok(data) = std::fs::read(path) else {
        return T::default();
    };
    serde_json::from_slice(&data).unwrap_or_default()
}

pub(crate) fn write_safe(path: &Path, content: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let mut temp = path.to_path_buf();
    temp.add_extension(format!(
        "{}",
        rand::rng().sample(rand::distr::Alphabetic) as char
    ));
    temp.add_extension("new");

    let mut temp_file = std::fs::File::create(&temp)?;

    temp_file.write_all(content)?;
    temp_file.flush()?;
    temp_file.sync_all()?;

    drop(temp_file);

    if let Err(err) = std::fs::rename(&temp, path) {
        _ = std::fs::remove_file(&temp);
        return Err(err);
    }

    Ok(())
}
