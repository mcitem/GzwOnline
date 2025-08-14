use crate::m20220101_000001_create_table::Category;

use calamine::{open_workbook, DataType, Reader, Xlsx};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const PATH: &str = ".\\migration\\category.xlsx";

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
                let name = row.get(1).map(|v| v.to_string());

                let query = Query::insert()
                    .into_table(Category::Table)
                    .columns(vec![Category::Id, Category::Name])
                    .values_panic(vec![id.into(), name.into()])
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
                    .from_table(Category::Table)
                    .and_where(Expr::col(Category::Id).eq(id))
                    .to_owned();

                db.execute(backend.build(&query)).await?;
            }
        }

        Ok(())
    }
}
