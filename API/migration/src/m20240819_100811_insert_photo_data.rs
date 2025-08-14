use crate::m20220101_000001_create_table::PhotoAlbum;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

const PATH: &str = ".\\migration\\photo_album.xlsx";

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

                let collection_id = row.get(1).map(|v| v.as_i64().map(|v| v as i32)).unwrap();

                let photo = row.get(2).map(|v| v.to_string());

                let query = Query::insert()
                    .into_table(PhotoAlbum::Table)
                    .columns([PhotoAlbum::Id, PhotoAlbum::CollectionId, PhotoAlbum::Photo])
                    .values_panic([id.into(), collection_id.into(), photo.into()])
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
                    .from_table(PhotoAlbum::Table)
                    .and_where(Expr::col(PhotoAlbum::Id).eq(id))
                    .to_owned();

                db.execute(backend.build(&query)).await?;
            }
        }

        Ok(())
    }
}
