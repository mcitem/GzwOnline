use crate::m20220101_000001_create_table::Collection;

use calamine::{open_workbook, DataType, Reader, Xlsx};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const PATH: &str = ".\\migration\\collection.xlsx";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        let backend = manager.get_database_backend();
        let mut workbook: Xlsx<_> = open_workbook(PATH).map_err(|e| {
            println!("{:?}", e);
            DbErr::Custom("无法打开文件".to_string())
        })?;

        if let Some(Ok(sheet)) = workbook
            .sheet_names()
            .get(0)
            .map(|s| workbook.worksheet_range(s))
        {
            for row in sheet.rows().skip(1) {
                let id = row.get(0).map(|v| v.as_i64().map(|v| v as i32)).unwrap();

                let category_id = row.get(1).map(|v| v.as_i64().map(|v| v as i32)).unwrap();

                let title = row.get(2).map(|v| v.to_string());

                let subtitle = row.get(3).map(|v| v.to_string());

                let description = row.get(4).map(|v| v.to_string());

                let text = row.get(5).map(|v| v.to_string());

                let main_img = row.get(6).map(|v| v.to_string());

                let views_count = row.get(7).map(|v| v.as_i64().map(|v| v as i32)).unwrap();

                let like_count = row.get(8).map(|v| v.as_i64().map(|v| v as i32)).unwrap();

                let query = Query::insert()
                    .into_table(Collection::Table)
                    .columns([
                        Collection::Id,
                        Collection::CategoryId,
                        Collection::Title,
                        Collection::Subtitle,
                        Collection::Description,
                        Collection::Supplement,
                        Collection::MainImage,
                        Collection::ViewsCount,
                        Collection::LikesCount,
                    ])
                    .values_panic([
                        id.into(),
                        category_id.into(),
                        title.into(),
                        subtitle.into(),
                        description.into(),
                        text.into(),
                        main_img.into(),
                        views_count.into(),
                        like_count.into(),
                    ])
                    .to_owned();

                db.execute(backend.build(&query)).await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        let backend = manager.get_database_backend();
        let mut workbook: Xlsx<_> = open_workbook(PATH).map_err(|e| {
            println!("{:?}", e);
            DbErr::Custom("无法打开文件".to_string())
        })?;

        if let Some(Ok(sheet)) = workbook
            .sheet_names()
            .get(0)
            .map(|s| workbook.worksheet_range(s))
        {
            for row in sheet.rows().skip(1) {
                let id = row.get(0).map(|v| v.as_i64().map(|v| v as i32)).unwrap();

                let query = Query::delete()
                    .from_table(Collection::Table)
                    .and_where(Expr::col(Collection::Id).eq(id))
                    .to_owned();

                db.execute(backend.build(&query)).await?;
            }
        }

        Ok(())
    }
}
