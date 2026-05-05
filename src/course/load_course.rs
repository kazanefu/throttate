use super::*;
use bevy::asset::Asset;
use bevy::reflect::TypePath;
use anyhow::Error;
use bevy::asset::{AssetLoader, LoadContext, io::Reader};

#[derive(Asset, TypePath, Clone)]
pub struct RonText(pub String);

#[derive(Default, TypePath)]
pub struct RonTextLoader;

impl AssetLoader for RonTextLoader {
    type Asset = RonText;
    type Settings = ();
    type Error = Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _ctx: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let text = String::from_utf8(bytes)?;
        Ok(RonText(text))
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

#[derive(Resource, Default)]
pub struct CourseLoadState {
    pub index: Option<Handle<RonText>>,
    pub courses: Vec<(CourseEntry, Handle<RonText>)>,
}

pub fn start_load_courses(
    mut state: ResMut<CourseLoadState>,
    asset_server: Res<AssetServer>,
    mut texts: ResMut<Assets<RonText>>,
) {
    if let Some(index_handle) = state.index.take() {
        texts.remove(index_handle.id());
    }
    asset_server.reload("courses_ron/index.ron");
    state.index = Some(asset_server.load("courses_ron/index.ron"));
    state.courses.clear();
}

pub fn load_index(
    mut state: ResMut<CourseLoadState>,
    asset_server: Res<AssetServer>,
    mut texts: ResMut<Assets<RonText>>,
) {
    if !state.courses.is_empty() {
        return;
    }
    let Some(index_handle) = &state.index else {
        return;
    };

    // indexがまだロードされてなければ待つ
    let Some(index_text) = texts.get(index_handle) else {
        return;
    };

    match ron::de::from_str::<CourseList>(&index_text.0) {
        Ok(list) => {
            // courseのロード開始
            state.courses = list
                .0
                .iter()
                .map(|entry| {
                    let path = format!("courses_ron/{}", entry.path);
                    asset_server.reload(&path);
                    let handle = asset_server.load(&path);
                    texts.remove(handle.id());
                    (entry.clone(), handle)
                })
                .collect::<Vec<(CourseEntry, Handle<RonText>)>>();
        }
        Err(e) => {
            error!("parse index.ron failed: {}", e);
        }
    }
}

pub fn check_and_finalize(
    state: Res<CourseLoadState>,
    mut course_list_resource: ResMut<CourseListResource>,
    texts: Res<Assets<RonText>>,
    mut next_state: ResMut<NextState<crate::state::GameState>>,
) {
    if state.courses.is_empty() {
        return;
    }

    // 全部ロード完了してるかチェック
    let mut result = vec![];

    for (entry, handle) in &state.courses {
        let Some(text) = texts.get(handle) else {
            return;
        };

        match ron::de::from_str::<Course>(&text.0) {
            Ok(course) => {
                result.push((entry.clone(), course));
            }
            Err(e) => {
                error!("parse course {} failed: {}", entry.name, e);
                return;
            }
        }
    }

    // ソートして反映
    result.sort_by(|a, b| a.0.id.cmp(&b.0.id));
    course_list_resource.0 = result;

    // 全てロード完了したのでStartに遷移
    next_state.set(crate::state::GameState::Start);
}
