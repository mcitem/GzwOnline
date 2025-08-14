use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 Category 表
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Category::Name)
                            .string()
                            .not_null()
                            .comment("分类名称"),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建 Collection 表
        manager
            .create_table(
                Table::create()
                    .table(Collection::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Collection::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Collection::CategoryId)
                            .integer()
                            .comment("藏品分类ID"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-collection-category_id")
                            .from(Collection::Table, Collection::CategoryId)
                            .to(Category::Table, Category::Id),
                    )
                    .col(
                        ColumnDef::new(Collection::Title)
                            .string()
                            .not_null()
                            .comment("标题"),
                    )
                    .col(
                        ColumnDef::new(Collection::Subtitle)
                            .string()
                            .comment("副标题"),
                    )
                    .col(
                        ColumnDef::new(Collection::Description)
                            .text()
                            .comment("描述"),
                    )
                    .col(
                        ColumnDef::new(Collection::Supplement)
                            .text()
                            .comment("补充说明"),
                    )
                    .col(
                        ColumnDef::new(Collection::MainImage)
                            .string()
                            .comment("主图"),
                    )
                    .col(
                        ColumnDef::new(Collection::ViewsCount)
                            .integer()
                            .default(0)
                            .comment("浏览次数"),
                    )
                    .col(
                        ColumnDef::new(Collection::LikesCount)
                            .integer()
                            .default(0)
                            .comment("点赞次数"),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建 PhotoAlbum 表
        manager
            .create_table(
                Table::create()
                    .table(PhotoAlbum::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PhotoAlbum::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PhotoAlbum::CollectionId)
                            .integer()
                            .comment("记录的收藏的藏品ID"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-photoalbum-collection_id")
                            .from(PhotoAlbum::Table, PhotoAlbum::CollectionId)
                            .to(Collection::Table, Collection::Id),
                    )
                    .col(
                        ColumnDef::new(PhotoAlbum::Photo)
                            .string()
                            .comment("图片地址"),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建 Feedback 表
        manager
            .create_table(
                Table::create()
                    .table(Feedback::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Feedback::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Feedback::CollectionId)
                            .integer()
                            .comment("来源藏品ID"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-feedback-collection_id")
                            .from(Feedback::Table, Feedback::CollectionId)
                            .to(Collection::Table, Collection::Id),
                    )
                    .col(ColumnDef::new(Feedback::Title).string().comment("反馈标题"))
                    .col(
                        ColumnDef::new(Feedback::Contact)
                            .string()
                            .comment("联系方式"),
                    )
                    .col(ColumnDef::new(Feedback::Content).text().comment("反馈内容"))
                    .col(
                        ColumnDef::new(Feedback::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Feedback::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PhotoAlbum::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Collection::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Category {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum Collection {
    Table,
    Id,
    CategoryId,
    Title,
    Subtitle,
    Description,
    Supplement,
    MainImage,
    ViewsCount,
    LikesCount,
}

#[derive(DeriveIden)]
pub enum PhotoAlbum {
    Table,
    Id,
    CollectionId,
    Photo,
}

#[derive(DeriveIden)]
pub enum Feedback {
    Table,
    Id,
    CollectionId,
    Title,
    Contact,
    Content,
    CreatedAt,
}
