pub mod token_utils;
pub mod utils {
    use unicode_truncate::UnicodeTruncateStr;
    use crate::models::{ Project, ProjectCategory, ProjectImage};

    pub fn unicode_truncate(input: String, len: u64) -> String {
        let (rv, _w) = input.as_str().unicode_truncate(len as usize);
        rv.to_string()
    }


    #[derive(Clone)]
    pub enum Sorter{
        CreatedAt,
        UpdatedAt,
        DeletedAt,
    }

    impl Sorter{
        pub fn project(self) -> Box<dyn FnMut(&Project, &Project) -> std::cmp::Ordering>{
            match self{
                Sorter::CreatedAt => Box::new(| a, b | b.created_at.cmp(&a.created_at)),
                Sorter::UpdatedAt => Box::new(| a, b | b.updated_at.cmp(&a.updated_at)),
                Sorter::DeletedAt => Box::new(| a, b | b.deleted_at.cmp(&a.deleted_at)),
            }
        }
        pub fn project_category(self) -> Box<dyn FnMut(&ProjectCategory, &ProjectCategory) -> std::cmp::Ordering>{
            match self{
                Sorter::CreatedAt => Box::new(| a, b | b.created_at.cmp(&a.created_at)),
                Sorter::UpdatedAt => Box::new(| a, b | b.updated_at.cmp(&a.updated_at)),
                Sorter::DeletedAt => Box::new(| a, b | b.deleted_at.cmp(&a.deleted_at)),
            }
        }
        pub fn project_image(self) -> Box<dyn FnMut(&ProjectImage, &ProjectImage) -> std::cmp::Ordering>{
            match self{
                Sorter::CreatedAt => Box::new(| a, b | b.created_at.cmp(&a.created_at)),
                Sorter::UpdatedAt => Box::new(| a, b | b.updated_at.cmp(&a.updated_at)),
                Sorter::DeletedAt => Box::new(| a, b | b.deleted_at.cmp(&a.deleted_at)),
            }
        }
    }
}

pub mod view_utils {
    use chrono::format::strftime::StrftimeItems;
    use chrono::NaiveDateTime;
    use comrak::{markdown_to_html, ComrakOptions};
    use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};

    pub fn unicode_truncate_helper(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        // get parameter from helper or throw an error
        let to_truncate = h
            .param(0)
            .ok_or(RenderError::new("Param 0 is required for format helper."))?;
        let to_keep = h
            .param(1)
            .ok_or(RenderError::new("Param 1 is required for format helper."))?;

        let truncated = super::utils::unicode_truncate(
            to_truncate.value().as_str().unwrap().to_string(),
            to_keep.value().as_u64().unwrap(),
        );
        let rendered = format!("{}", truncated);
        out.write(rendered.as_ref())?;
        Ok(())
    }

    pub fn render_markdown(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        // get parameter from helper or throw an error
        let md = h
            .param(0)
            .ok_or(RenderError::new("Param 0 is required for format helper."))?;

        let html: String = markdown_to_html(
            &md.value().as_str().unwrap().to_string(),
            &ComrakOptions::default(),
        );
        let rendered = format!("{}", html);
        out.write(rendered.as_ref())?;
        Ok(())
    }

    pub fn format_date(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        // get parameter from helper or throw an error
        let iso_date = h
            .param(0)
            .ok_or(RenderError::new("Param 0 is required for format helper."))?;

        let dt: String = iso_date.value().as_str().unwrap().to_string();
        let dt_result = NaiveDateTime::parse_from_str(&dt, "%Y-%m-%dT%H:%M:%S%.6f");

        match dt_result {
            Err(_) => {
                out.write("")?;
                Ok(())
            },
            Ok(dt) => {
                let fmt_str = h
                    .param(1)
                    .ok_or(RenderError::new("Param 1 is required for format helper."))?;

                let fmt = StrftimeItems::new(&fmt_str.value().as_str().unwrap());
                let rendered = format!("{}", dt.format_with_items(fmt));
                out.write(rendered.as_ref())?;
                Ok(())
            }
        }
    }
    
    pub fn format_views_count(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        let param = h.param(0)
                     .ok_or(RenderError::new("Param 0 required for format helper."))?;
        let count = param.value() as f64;
        let base = 1000_f64;
        let divided = count / base;
        let precision = 2;
        
        if divided > 1.0 {
            out.write(format!("{:.1$}k", divided, precision))?;
        } else {
            out.write(format!("{}", count))?;
        }
        Ok(())
    }
}

pub mod iso_date_format {
    use chrono::{ NaiveDateTime };
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.6f";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => {
                let s = format!("{}", d.format(FORMAT));
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none()
        }
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.len() {
            0 => Ok(None),
            _ => {
                let dt_result = NaiveDateTime::parse_from_str(&s, FORMAT);
                match dt_result {
                    Ok(dt) => Ok(Some(dt)),
                    _ => Err(serde::de::Error::custom(format!("Cannot parse NaiveDateTime from str {}", &s))),
                }
            }
        }
    }
}
