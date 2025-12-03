use askama::Template;
use axum::{Form, extract::Path, extract::State, response::IntoResponse};
use example_core::{
    AppError, DEFAULT_LANG, DefaultAppState, HtmlMetadata, HtmlTemplate, I18nMetadata, Lang,
};
use rcfe::mvccpb::KeyValue;
use rcfe::{Client, DefaultClient, GetOptions, KVClient, SortOrderOption, SortTargetOption};
use serde::{Deserialize, Serialize};
use serde_with::{NoneAsEmptyString, serde_as};
use tracing::error;

#[serde_as]
#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct RangeForm {

    #[serde(skip_serializing_if = "Option::is_none")]
    query_all: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    range_end: Option<String>,

    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,

    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<SortOrderOption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort_target: Option<SortTargetOption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    serializable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    keys_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    count_only: Option<bool>,

    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    min_mod_revision: Option<i64>,

    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_mod_revision: Option<i64>,

    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    min_create_revision: Option<i64>,

    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_create_revision: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<bool>,
}

impl Into<GetOptions> for RangeForm {
    fn into(self) -> GetOptions {
        let mut builder = rcfe::GetOptionsBuilder::default();
        if let Some(ref range_end) = self.range_end
            && !self.query_all.unwrap_or(false)
        {
            builder = builder.end_key(range_end.as_str());
        }

        if let Some(prefixed) = self.prefix
            && !self.query_all.unwrap_or(false)
        {
            builder = builder.prefix(prefixed);
        }

        if let Some(limit) = self.limit
            && limit > 0
        {
            builder = builder.limit(limit);
        }
        if let Some(revision) = self.revision {
            builder = builder.revision(revision);
        }
        if let Some(sort_order) = self.sort_order {
            builder = builder.sort_order(sort_order);
        }
        if let Some(sort_target) = self.sort_target {
            builder = builder.sort_target(sort_target);
        }
        if let Some(serializable) = self.serializable {
            builder = builder.serializable(serializable);
        }
        if let Some(keys_only) = self.keys_only {
            builder = builder.keys_only(keys_only);
        }
        if let Some(count_only) = self.count_only {
            builder = builder.count_only(count_only);
        }
        if let Some(min_mod_revision) = self.min_mod_revision {
            builder = builder.min_mod_revision(min_mod_revision);
        }
        if let Some(max_mod_revision) = self.max_mod_revision {
            builder = builder.max_mod_revision(max_mod_revision);
        }
        if let Some(min_create_revision) = self.min_create_revision {
            builder = builder.min_create_revision(min_create_revision);
        }
        if let Some(max_create_revision) = self.max_create_revision {
            builder = builder.max_create_revision(max_create_revision);
        }
        builder.build()
    }
}

#[derive(Template, Serialize, Default)]
#[template(path = "index.html")]
struct IndexPage {
    lang: Lang,
    header: String,
    form_title: String,
    query_all_label: String,
    key_label: String,
    key_placeholder: String,
    key_value: Option<String>,
    range_end_label: String,
    range_end_placeholder: String,
    range_end_value: Option<String>,
    limit_label: String,
    limit_placeholder: String,
    limit_value: Option<i64>,
    revision_label: String,
    revision_placeholder: String,
    revision_value: Option<i64>,
    sort_order_label: String,
    sort_order_none: String,
    sort_order_ascend: String,
    sort_order_descend: String,
    is_sort_order_none: bool,
    is_sort_order_ascend: bool,
    is_sort_order_descend: bool,
    sort_target_label: String,
    sort_target_key: String,
    sort_target_version: String,
    sort_target_create: String,
    sort_target_mod: String,
    sort_target_value: String,
    is_sort_target_key: bool,
    is_sort_target_version: bool,
    is_sort_target_create: bool,
    is_sort_target_mod: bool,
    is_sort_target_value: bool,
    serializable_label: String,
    serializable_placeholder: String,
    serializable_value: bool,
    keys_only_label: String,
    keys_only_placeholder: String,
    keys_only_value: bool,
    count_only_label: String,
    count_only_placeholder: String,
    count_only_value: bool,
    min_mod_revision_label: String,
    min_mod_revision_placeholder: String,
    min_mod_revision_value: Option<i64>,
    max_mod_revision_label: String,
    max_mod_revision_placeholder: String,
    max_mod_revision_value: Option<i64>,
    min_create_revision_label: String,
    min_create_revision_placeholder: String,
    min_create_revision_value: Option<i64>,
    max_create_revision_label: String,
    max_create_revision_placeholder: String,
    max_create_revision_value: Option<i64>,
    prefixed_label: String,
    prefixed_placeholder: String,
    prefixed_value: bool,
    query_all: bool,
    results: Option<Vec<(String, String)>>,
    result_header: String,
    result_key_label: String,
    result_value_label: String,
    submit_button: String,
    reset_button: String,
    home_url: String,
    q: Option<String>,
    total_label: String,
    total: i64,
    put_key_label: String,
    put_key_placeholder: String,
    put_value_label: String,
    put_value_placeholder: String,
    lease_label: String,
    lease_placeholder: String,
    prev_kv_label: String,
    put_section_title: String,
}

impl HtmlMetadata for IndexPage {
    fn namespace(&self) -> String {
        String::from("index")
    }

    fn lang(&self) -> Lang {
        self.lang.clone()
    }

    fn current_path(&self) -> String {
        String::from("")
    }

    fn q(&self) -> Option<String> {
        self.q.clone()
    }
}

impl I18nMetadata for IndexPage {}

impl IndexPage {
    fn new<L>(
        lang: Option<L>,
        range_form: RangeForm,
        results: Option<Vec<(String, String)>>,
        total: i64,
        has_lang: bool,
    ) -> Self
    where
        L: Into<Lang>,
    {
        let mut page = IndexPage::default();
        if let Some(lang) = lang {
            page.lang = lang.into();
        }
        page.header = page.nt("header");
        page.form_title = page.nt("form_title");
        page.query_all_label = page.nt("query_all_label");
        page.key_label = page.nt("key_label");
        page.key_placeholder = page.nt("key_placeholder");
        page.range_end_label = page.nt("range_end_label");
        page.range_end_placeholder = page.nt("range_end_placeholder");
        page.limit_label = page.nt("limit_label");
        page.limit_placeholder = page.nt("limit_placeholder");
        page.revision_label = page.nt("revision_label");
        page.revision_placeholder = page.nt("revision_placeholder");
        page.sort_order_label = page.nt("sort_order_label");
        page.sort_order_none = page.nt("sort_order_none");
        page.sort_order_ascend = page.nt("sort_order_ascend");
        page.sort_order_descend = page.nt("sort_order_descend");
        page.sort_target_label = page.nt("sort_target_label");
        page.sort_target_key = page.nt("sort_target_key");
        page.sort_target_version = page.nt("sort_target_version");
        page.sort_target_create = page.nt("sort_target_create");
        page.sort_target_mod = page.nt("sort_target_mod");
        page.sort_target_value = page.nt("sort_target_value");
        page.serializable_label = page.nt("serializable_label");
        page.serializable_placeholder = page.nt("serializable_placeholder");
        page.keys_only_label = page.nt("keys_only_label");
        page.keys_only_placeholder = page.nt("keys_only_placeholder");
        page.count_only_label = page.nt("count_only_label");
        page.count_only_placeholder = page.nt("count_only_placeholder");
        page.min_mod_revision_label = page.nt("min_mod_revision_label");
        page.min_mod_revision_placeholder = page.nt("min_mod_revision_placeholder");
        page.max_mod_revision_label = page.nt("max_mod_revision_label");
        page.max_mod_revision_placeholder = page.nt("max_mod_revision_placeholder");
        page.min_create_revision_label = page.nt("min_create_revision_label");
        page.min_create_revision_placeholder = page.nt("min_create_revision_placeholder");
        page.max_create_revision_label = page.nt("max_create_revision_label");
        page.max_create_revision_placeholder = page.nt("max_create_revision_placeholder");
        page.prefixed_label = page.nt("prefixed_label");
        page.prefixed_placeholder = page.nt("prefixed_placeholder");
        page.result_header = page.nt("result_header");
        page.result_key_label = page.nt("result_key_label");
        page.result_value_label = page.nt("result_value_label");
        page.submit_button = page.nt("submit_button");
        page.reset_button = page.nt("reset_button");
        page.lease_label = page.nt("lease_label");
        page.lease_placeholder = page.nt("lease_placeholder");
        page.prev_kv_label = page.nt("prev_kv_label");
        page.put_key_placeholder = page.nt("put_key_placeholder");
        page.put_key_label = page.nt("put_key_label");
        page.put_value_label = page.nt("put_value_label");
        page.put_value_placeholder = page.nt("put_value_placeholder");
        page.put_section_title = page.nt("put_section_title");
        page.results = results;

        let form = range_form.clone();
        page.key_value = form.key;
        page.range_end_value = form.range_end;
        page.limit_value = form.limit;
        page.revision_value = form.revision;
        page.is_sort_order_none =
            form.sort_order.is_none() || matches!(form.sort_order, Some(SortOrderOption::None));
        page.is_sort_order_ascend = matches!(form.sort_order, Some(SortOrderOption::Ascend));
        page.is_sort_order_descend = matches!(form.sort_order, Some(SortOrderOption::Descend));
        page.is_sort_target_key =
            form.sort_target.is_none() || matches!(form.sort_target, Some(SortTargetOption::Key));
        page.is_sort_target_version = matches!(form.sort_target, Some(SortTargetOption::Version));
        page.is_sort_target_create = matches!(form.sort_target, Some(SortTargetOption::Create));
        page.is_sort_target_mod = matches!(form.sort_target, Some(SortTargetOption::Mod));
        page.is_sort_target_value = matches!(form.sort_target, Some(SortTargetOption::Value));
        page.serializable_value = form.serializable.unwrap_or(false);
        page.keys_only_value = form.keys_only.unwrap_or(false);
        page.count_only_value = form.count_only.unwrap_or(false);
        page.min_mod_revision_value = form.min_mod_revision;
        page.max_mod_revision_value = form.max_mod_revision;
        page.min_create_revision_value = form.min_create_revision;
        page.max_create_revision_value = form.max_create_revision;
        page.prefixed_value = form.prefix.unwrap_or(false);
        page.query_all = form.query_all.unwrap_or(false);

        let lang_path = if has_lang && page.lang != page.default_language() {
            format!("/{}", page.lang)
        } else {
            String::from("/")
        };
        page.home_url = format!("{}", lang_path);
        match serde_urlencoded::to_string(&range_form) {
            Ok(q) => page.q = Some(q),
            Err(e) => error!("Failed to serialize range form: {}", e),
        }

        page.total_label = page.nt("total_label");
        page.total = total;
        page
    }
}

pub async fn page(
    State(state): State<DefaultAppState>,
    Form(form): Form<RangeForm>,
) -> Result<impl IntoResponse, AppError> {
    inner_page(DEFAULT_LANG, form, state.client, false).await
}

pub async fn page_with_lang(
    State(state): State<DefaultAppState>,
    Path(lang): Path<String>,
    Form(form): Form<RangeForm>,
) -> Result<impl IntoResponse, AppError> {
    inner_page(lang, form, state.client, true).await
}

fn kvs_to_results(kvs: Vec<KeyValue>) -> Vec<(String, String)> {
    kvs.into_iter()
        .map(|kv| {
            let key_str = String::from_utf8(kv.key.to_vec()).unwrap_or_default();
            let value_str = String::from_utf8(kv.value.to_vec()).unwrap_or_default();
            (key_str, value_str)
        })
        .collect()
}

async fn inner_page<L>(
    lang: L,
    form: RangeForm,
    client: DefaultClient,
    has_lang: bool,
) -> Result<impl IntoResponse, AppError>
where
    L: Into<Lang>,
{
    let mut client = client.get_kv_client();
    let lang = lang.into();
    let (values, total) = if let Some(query_all) = form.query_all
        && query_all
    {
        let response = client.get_all(Some(form.clone().into())).await?;
        let response = response.into_inner();
        let kvs = response.kvs;
        (Some(kvs_to_results(kvs)), response.count)
    } else {
        if let Some(key) = form.key.clone() {
            let options: GetOptions = form.clone().into();
            let response = client.get_with_options(key.as_str(), options).await?;
            let response = response.into_inner();
            let kvs = response.kvs;
            (Some(kvs_to_results(kvs)), response.count)
        } else {
            (None, 0)
        }
    };
    Ok(HtmlTemplate::new(IndexPage::new(
        Some(lang),
        form,
        values,
        total,
        has_lang,
    )))
}
